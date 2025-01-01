use std::process;

use crate::error::{AppError, AppResult};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, CreateEmbeddingRequestArgs, Embedding,
    },
    Client,
};

const EMBEDDING_MODEL: &str = "text-embedding-3-small";

fn client() -> Client<OpenAIConfig> {
    let api_key = match std::env::var("OPEN_AI_API_KEY") {
        Ok(key) => Some(key),
        Err(_) => {
            println!(
                "{}: {}",
                "OPEN_AI_API_KEY is not set", "Please set the OPEN_AI_API_KEY environment variable"
            );
            process::exit(1);
        }
    };
    let config = OpenAIConfig::new().with_api_key(api_key.unwrap());
    Client::with_config(config)
}
pub async fn embedding(input1: String, input2: String) -> AppResult<Vec<Embedding>> {
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

    Ok(response.data)
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
