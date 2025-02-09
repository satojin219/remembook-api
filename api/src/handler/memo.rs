use crate::{
    extractor::AuthorizedUser,
    model::{
        memo::{
            CreateMemoRequest, CreateMemoRequestWithIds, CreateQuestionRequest,
            CreateQuestionRequestWithIds, UpdateMemoRequest, UserAnswerRequest, UserAnswerResponse,
        },
        question::{QuestionResponse, QuestionsResponse},
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
    book::event::CreateBook,
    id::{BookId, MemoId, QuestionId},
    memo::event::{DeleteMemo, UpdateMemo},
    question::event::UpdateQuestion,
    user::event::UpdateCoin,
};
use registry::AppRegistry;
use shared::{
    error::{AppError, AppResult},
    open_ai::{embedding, generate_question},
};

pub async fn create_memo(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateMemoRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    let create_book_event = CreateBook {
        title: req.title.clone(),
        author: req.author.clone(),
        image_url: req.image_url.clone(),
        google_books_id: req.google_books_id.clone(),
    };

    let book_id = registry
        .book_repository()
        .create_book(create_book_event, user.id())
        .await?;

    let create_memo_event = CreateMemoRequestWithIds(user.id(), book_id, req.clone());

    let memo_id = registry
        .memo_repository()
        .create_memo(create_memo_event.into())
        .await?;

    let question_text = generate_question(&req.body).await?;

    let create_question_event = CreateQuestionRequestWithIds(
        user.id(),
        book_id,
        memo_id,
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

pub async fn update_memo(
    _user: AuthorizedUser,
    Path(memo_id): Path<MemoId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateMemoRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .memo_repository()
        .update_memo(UpdateMemo {
            memo_id,
            memo_text: req.body.clone(),
        })
        .await?;

    let question_text = generate_question(&req.body).await?;

    registry
        .question_repository()
        .update_question(UpdateQuestion {
            memo_id,
            question_text,
        })
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete_memo(
    _user: AuthorizedUser,
    Path(memo_id): Path<MemoId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .memo_repository()
        .delete_memo(DeleteMemo { memo_id })
        .await?;

    Ok(StatusCode::OK)
}

pub async fn get_question(
    _user: AuthorizedUser,
    Path(memo_id): Path<MemoId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<QuestionResponse>> {
    registry
        .question_repository()
        .get_by_memo_id(memo_id)
        .await
        .and_then(|q| match q {
            Some(q) => Ok(Json(q.into())),
            None => Err(AppError::EntityNotFound(
                "The specific question was not found".to_string(),
            )),
        })
}

pub async fn get_question_list(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<QuestionsResponse>> {
    let questions = registry
        .question_repository()
        .get_list_by_book_id(book_id)
        .await?
        .into_iter()
        .map(QuestionResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(QuestionsResponse { questions }))
}

pub async fn answer_question(
    user: AuthorizedUser,
    Path((memo_id, question_id)): Path<(MemoId, QuestionId)>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UserAnswerRequest>,
) -> AppResult<Json<UserAnswerResponse>> {
    registry
        .user_repository()
        .update_coin(UpdateCoin {
            user_id: user.id(),
            amount: -1,
        })
        .await?;
    let memo = registry.memo_repository().get_by_id(memo_id).await?;
    let score = embedding(req.user_answer.clone(), memo.as_ref().unwrap().to_string()).await?;

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
        memo: memo.unwrap(),
        user_answer: req.user_answer,
    }))
}
