use anyhow::Result;
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

mod models;
mod agents;
mod state;
mod strategy;

use agents::{ChatAgent, EmotionDetector};
use models::MessageRole;
use state::ConversationManager;
use strategy::select_strategy;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SentimentClassification {
    pub sentiment: Sentiment,
    pub confidence: f32,
}

struct Config {
    api_key: String,
    base_url: String,
    model: String,
}

impl Config {
    fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY not set"))?;

        let base_url = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://open.bigmodel.cn/api/paas/v4".to_string());

        let model = std::env::var("MODEL")
            .unwrap_or_else(|_| "glm-4.7".to_string());

        Ok(Self { api_key, base_url, model })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env()?;

    println!("🤖 Emotional-Aware Chat System");
    println!("📊 Model: {}", config.model);
    println!("💬 Type 'quit' or 'exit' to end\n");

    let client = openai::Client::from_url(&config.api_key, &config.base_url);
    let emotion_detector = EmotionDetector::new(client.clone(), &config.model);
    let chat_agent = ChatAgent::new(client, &config.model);
    let mut state_manager = ConversationManager::new();

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        let emotion = match emotion_detector.analyze(input).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!("❌ Emotion detection failed: {}", e);
                continue;
            }
        };

        state_manager.add_message(MessageRole::User, input);
        state_manager.update_emotion(emotion.clone());

        let trend = state_manager.get_recent_emotion_trend();
        let strategy = select_strategy(&emotion, trend);

        let response = match chat_agent.respond(input, strategy, state_manager.get_history()).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ Response generation failed: {}", e);
                continue;
            }
        };

        state_manager.add_message(MessageRole::Assistant, &response);

        println!("📊 Emotion: {:?} (confidence: {:.2})", emotion.sentiment, emotion.confidence);
        println!("📈 Trend: {:?}", trend);
        println!("🎯 Strategy: {:?}", strategy);
        println!("🤖 Assistant: {}\n", response);
    }

    Ok(())
}
