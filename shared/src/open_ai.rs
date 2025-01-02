use std::process;

use crate::error::{AppError, AppResult};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, CreateEmbeddingRequestArgs,
    },
    Client,
};

const EMBEDDING_MODEL: &str = "text-embedding-3-small";

fn client() -> Client<OpenAIConfig> {
    let api_key = match std::env::var("OPEN_AI_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => {
            println!(
                "OPEN_AI_API_KEY is not set Please set the OPEN_AI_API_KEY environment variable"
            );
            process::exit(1);
        }
    };
    let config = OpenAIConfig::new().with_api_key(api_key.unwrap());
    Client::with_config(config)
}

fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> AppResult<i32> {
    if vec_a.len() != vec_b.len() {
        return Err(AppError::CosineSimilarityError(
            "ベクトルの長さが一致しません。".to_string(),
        ));
    }

    // ドット積を計算
    let dot_product: f32 = vec_a.iter().zip(vec_b).map(|(a, b)| a * b).sum();

    // ベクトルのノルムを計算
    let norm_a: f32 = vec_a.iter().map(|a| a * a).sum::<f32>().sqrt();
    let norm_b: f32 = vec_b.iter().map(|b| b * b).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return Err(AppError::CosineSimilarityError(
            "ノルムが0のため計算できません。".to_string(),
        ));
    }

    // コサイン類似度を計算
    let similarity = dot_product / (norm_a * norm_b);

    // 類似度を整数に変換して返す（スケール：100倍して小数点以下切り捨て）
    Ok((similarity * 100.0).round() as i32)
}

pub async fn embedding(input1: String, input2: String) -> AppResult<i32> {
    let request = CreateEmbeddingRequestArgs::default()
        .model(EMBEDDING_MODEL)
        .input([input1, input2])
        .build()
        .map_err(AppError::OpenAIError)?;

    let response = client()
        .embeddings()
        .create(request)
        .await
        .map_err(AppError::OpenAIError)?;

    let similarity = cosine_similarity(&response.data[0].embedding, &response.data[1].embedding)?;

    Ok(similarity)
}

pub async fn generate_question(user_answer: &str) -> AppResult<String> {
    // 質問を生成するためのプロンプト設定
    let prompt = format!(
        "次のテキストを答えとするような適切な質問を作成してください:\n\n答え: {}",
        user_answer
    );

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-mini")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant who specializes in generating questions based on given answers.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client().chat().create(request).await?;

    let mut generated_question = String::new();
    for choice in response.choices {
        if let Some(content) = choice.message.content {
            generated_question = content;
            break;
        }
    }

    Ok(generated_question)
}
