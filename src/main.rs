use anyhow::Result;
use rig::{completion::Prompt, providers::openai};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod models;
mod agents;
mod state;
mod strategy;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct SentimentClassification {
    sentiment: Sentiment,
    confidence: f32,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok(); // Load environment variables securely

    // Use Zhipu AI's OpenAI-compatible API endpoint
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env file");
    let base_url = std::env::var("OPENAI_BASE_URL")
        .unwrap_or_else(|_| "https://open.bigmodel.cn/api/coding/paas/v4".to_string());
    let openai_client = openai::Client::from_url(&api_key, &base_url);

    let comedian_agent = openai_client
        .agent("glm-4.7")
        .preamble("You are a comedian here to enterttain the usr using humour and jokes.")
        .build();

    let response = comedian_agent.prompt("Entertain!").await?;

    println!("{response}");

    Ok(())

    // let sentiment_classifier = openai_client
    //     .extractor::<SentimentClassification>("glm-4.7")
    //     .preamble("You are a sentiment analysis AI. Classify the sentiment of the given text.")
    //     .build();

    // let text = "I absolutely loved the new restaurant. The food was amazing!";
    // let result = sentiment_classifier.extract(text).await?;

    // println!("Text: {}", text);
    // println!("Sentiment: {:?}", result.sentiment);
    // println!("Confidence: {:.2}", result.confidence);

    // Ok(())
}
