use crate::{
    extractor::AuthorizedUser,
    model::{
        question::QuestionResponse,
        summary::{
            CreateQuestionRequest, CreateQuestionRequestWithIds, CreateSummaryRequest,
            CreateSummaryRequestWithIds, UpdateSummaryRequest, UserAnswerRequest,
            UserAnswerResponse,
        },
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{
    answer::event::CreateAnswer,
    id::{BookId, QuestionId, SummaryId},
    question::event::{CreateQuestion, UpdateQuestion},
    summary::event::{CreateSummary, DeleteSummary, UpdateSummary},
};
use registry::AppRegistry;
use shared::{
    error::{AppError, AppResult},
    open_ai::{embedding, generate_question},
};

/// ユーザーが入力した要約を登録し、要約から質問を生成する。
pub async fn create_summary(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateSummaryRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    let create_summary_event = CreateSummaryRequestWithIds(user.id(), book_id, req.clone());

    let summary_id = registry
        .summary_repository()
        .create_summary(create_summary_event.into())
        .await?;

    let question_text = generate_question(&req.body).await?;

    let create_question_event = CreateQuestionRequestWithIds(
        user.id(),
        book_id,
        summary_id,
        CreateQuestionRequest {
            body: question_text,
        },
    );

    registry
        .question_repository()
        .create_question(create_question_event.into())
        .await?;

    Ok(StatusCode::CREATED)
}

pub async fn update_summary(
    _user: AuthorizedUser,
    Path((_book_id, summary_id)): Path<(BookId, SummaryId)>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateSummaryRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .summary_repository()
        .update_summary(UpdateSummary {
            summary_id,
            summary_text: req.body.clone(),
        })
        .await?;

    let question_text = generate_question(&req.body).await?;

    registry
        .question_repository()
        .update_question(UpdateQuestion {
            summary_id,
            question_text,
        })
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete_summary(
    _user: AuthorizedUser,
    Path(summary_id): Path<SummaryId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .summary_repository()
        .delete_summary(DeleteSummary { summary_id })
        .await?;

    Ok(StatusCode::OK)
}

pub async fn get_question(
    _user: AuthorizedUser,
    Path(summary_id): Path<SummaryId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<QuestionResponse>> {
    registry
        .question_repository()
        .get_by_summary_id(summary_id)
        .await
        .and_then(|q| match q {
            Some(q) => Ok(Json(q.into())),
            None => Err(AppError::EntityNotFound(
                "The specific question was not found".to_string(),
            )),
        })
}

pub async fn answer_question(
    user: AuthorizedUser,
    Path((book_id, summary_id, question_id)): Path<(BookId, SummaryId, QuestionId)>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UserAnswerRequest>,
) -> AppResult<Json<UserAnswerResponse>> {
    let summary = registry.summary_repository().get_by_id(summary_id).await?;
    let score = embedding(
        req.user_answer.clone(),
        summary.as_ref().unwrap().to_string(),
    )
    .await?;

    registry
        .answer_repository()
        .create_answer(CreateAnswer {
            user_id: user.id(),
            question_id,
            answer_text: req.user_answer.clone(),
            score,
        })
        .await?;
      
    Ok(Json(UserAnswerResponse {
        score,
        summary: summary.unwrap(),
        user_answer: req.user_answer,
    }))
}
