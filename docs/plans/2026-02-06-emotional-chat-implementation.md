# Emotional-Aware Chat System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a dual-agent conversational AI that detects user emotions and dynamically adjusts response strategies.

**Architecture:** Two agents working together - EmotionDetector analyzes user input for sentiment, ChatAgent generates contextually appropriate responses based on detected emotion and conversation history.

**Tech Stack:** Rust, rig-core 0.11.1, tokio, serde, schemars, thiserror, chrono, GLM-4.7 API

---

## Task 1: Add Missing Dependencies

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add thiserror and chrono dependencies**

Edit `Cargo.toml`, add to `[dependencies]` section:

```toml
[dependencies]
rig-core = "0.11.1"
tokio = { version = "1.34", features = ["full"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = { version = "0.8", features = ["derive"] }
dotenv = "0.15"
thiserror = "1.0"
chrono = "0.4"
```

**Step 2: Run cargo check**

Run: `cargo check`
Expected: No errors, dependencies resolved

**Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "deps: add thiserror and chrono dependencies"
```

---

## Task 2: Create Module Directory Structure

**Files:**
- Create: `src/models/mod.rs`
- Create: `src/agents/mod.rs`
- Create: `src/state/mod.rs`
- Create: `src/strategy/mod.rs`

**Step 1: Create models module**

Create `src/models/mod.rs`:

```rust
//! Data models for the emotional chat system

pub mod message;

pub use message::{Message, MessageRole};
```

**Step 2: Create agents module**

Create `src/agents/mod.rs`:

```rust
//! Agent implementations for emotion detection and chat

pub mod emotion;
pub mod chat;

pub use emotion::EmotionDetector;
pub use chat::ChatAgent;
```

**Step 3: Create state module**

Create `src/state/mod.rs`:

```rust
//! Conversation state management

pub mod conversation;

pub use conversation::{ConversationManager, ConversationState, EmotionTrend};
```

**Step 4: Create strategy module**

Create `src/strategy/mod.rs`:

```rust
//! Response strategy selection

pub mod response;

pub use response::{ResponseStrategy, select_strategy};
```

**Step 5: Update main.rs to include modules**

Add to `src/main.rs` at top after imports:

```rust
mod models;
mod agents;
mod state;
mod strategy;
```

**Step 6: Run cargo check**

Run: `cargo check`
Expected: "error[E0433]: failed to resolve: use of undeclared crate or module `message`" etc.

**Step 7: Commit**

```bash
git add src/models/mod.rs src/agents/mod.rs src/state/mod.rs src/strategy/mod.rs src/main.rs
git commit -m "refactor: create module structure"
```

---

## Task 3: Implement Message Models

**Files:**
- Create: `src/models/message.rs`

**Step 1: Write the failing test**

Create `src/models/message.rs` with test:

```rust
use serde_json;
use super::super::SentimentClassification;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: i64,
    pub emotion: Option<SentimentClassification>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
            timestamp: 12345,
            emotion: None,
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("User"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_message_with_emotion() {
        use crate::Sentiment;

        let msg = Message {
            role: MessageRole::User,
            content: "Great!".to_string(),
            timestamp: 12345,
            emotion: Some(SentimentClassification {
                sentiment: Sentiment::Positive,
                confidence: 0.95,
            }),
        };

        assert!(msg.emotion.is_some());
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test message`
Expected: Tests should pass (this is simple data structure)

**Step 3: (Already done - implementation is complete)**

**Step 4: Run tests to verify they pass**

Run: `cargo test message`
Expected: PASS

**Step 5: Commit**

```bash
git add src/models/message.rs
git commit -m "feat: add Message and MessageRole models"
```

---

## Task 4: Implement ConversationState

**Files:**
- Create: `src/state/conversation.rs`

**Step 1: Write the implementation**

Create `src/state/conversation.rs`:

```rust
use crate::models::{Message, MessageRole};
use crate::SentimentClassification;

#[derive(Debug, Clone)]
pub struct ConversationState {
    pub messages: Vec<Message>,
    pub emotion_history: Vec<SentimentClassification>,
    pub started_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionTrend {
    Improving,
    Declining,
    Stable,
}

pub struct ConversationManager {
    state: ConversationState,
}

impl ConversationManager {
    pub fn new() -> Self {
        Self {
            state: ConversationState {
                messages: Vec::new(),
                emotion_history: Vec::new(),
                started_at: chrono::Utc::now().timestamp(),
            },
        }
    }

    pub fn add_message(&mut self, role: MessageRole, content: &str) {
        let msg = Message {
            role,
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            emotion: None,
        };
        self.state.messages.push(msg);
    }

    pub fn update_emotion(&mut self, emotion: SentimentClassification) {
        self.state.emotion_history.push(emotion);

        // Attach emotion to last user message
        if let Some(msg) = self.state.messages.last_mut() {
            if matches!(msg.role, MessageRole::User) {
                msg.emotion = Some(emotion);
            }
        }
    }

    pub fn get_recent_emotion_trend(&self) -> EmotionTrend {
        use crate::Sentiment;

        let recent = self.state.emotion_history.iter().rev().take(5).collect::<Vec<_>>();

        if recent.len() < 2 {
            return EmotionTrend::Stable;
        }

        let scores: Vec<i32> = recent.iter()
            .map(|e| match e.sentiment {
                Sentiment::Positive => 1,
                Sentiment::Neutral => 0,
                Sentiment::Negative => -1,
            })
            .collect();

        let recent_count = scores.len().min(3);
        let recent_avg: f32 = scores.iter().take(recent_count).sum::<i32>() as f32 / recent_count as f32;

        let earlier_count = scores.len().saturating_sub(3);
        let earlier_avg: f32 = if earlier_count > 0 {
            scores.iter().skip(recent_count).sum::<i32>() as f32 / earlier_count as f32
        } else {
            recent_avg
        };

        if recent_avg > earlier_avg + 0.3 {
            EmotionTrend::Improving
        } else if recent_avg < earlier_avg - 0.3 {
            EmotionTrend::Declining
        } else {
            EmotionTrend::Stable
        }
    }

    pub fn get_history(&self) -> &[Message] {
        &self.state.messages
    }
}

impl Default for ConversationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_manager_new() {
        let manager = ConversationManager::new();
        assert_eq!(manager.get_history().len(), 0);
    }

    #[test]
    fn test_add_message() {
        let mut manager = ConversationManager::new();
        manager.add_message(MessageRole::User, "Hello");
        assert_eq!(manager.get_history().len(), 1);
        assert_eq!(manager.get_history()[0].content, "Hello");
    }

    #[test]
    fn test_emotion_trend_stable() {
        let mut manager = ConversationManager::new();
        use crate::Sentiment;

        // Add neutral emotions
        manager.update_emotion(SentimentClassification {
            sentiment: Sentiment::Neutral,
            confidence: 0.5,
        });
        manager.update_emotion(SentimentClassification {
            sentiment: Sentiment::Neutral,
            confidence: 0.5,
        });

        assert_eq!(manager.get_recent_emotion_trend(), EmotionTrend::Stable);
    }

    #[test]
    fn test_emotion_trend_improving() {
        let mut manager = ConversationManager::new();
        use crate::Sentiment;

        // Start negative, end positive
        for _ in 0..3 {
            manager.update_emotion(SentimentClassification {
                sentiment: Sentiment::Negative,
                confidence: 0.8,
            });
        }
        for _ in 0..3 {
            manager.update_emotion(SentimentClassification {
                sentiment: Sentiment::Positive,
                confidence: 0.8,
            });
        }

        assert_eq!(manager.get_recent_emotion_trend(), EmotionTrend::Improving);
    }
}
```

**Step 2: Run cargo check**

Run: `cargo check`
Expected: PASS (no errors)

**Step 3: Run tests**

Run: `cargo test conversation`
Expected: All tests PASS

**Step 4: Commit**

```bash
git add src/state/conversation.rs
git commit -m "feat: implement ConversationManager with emotion trend tracking"
```

---

## Task 5: Implement ResponseStrategy

**Files:**
- Create: `src/strategy/response.rs`

**Step 1: Write the implementation**

Create `src/strategy/response.rs`:

```rust
use crate::{Sentiment, SentimentClassification, state::EmotionTrend};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseStrategy {
    Empathetic,
    Encouraging,
    Neutral,
    Cheerful,
}

impl ResponseStrategy {
    pub fn to_prompt(&self) -> &'static str {
        match self {
            ResponseStrategy::Empathetic => {
                "You are an empathetic listener. The user is going through a difficult time.
                Respond with warmth and understanding. Acknowledge their feelings and provide
                emotional support. Avoid giving unsolicited advice. Focus on being present and compassionate."
            }
            ResponseStrategy::Encouraging => {
                "You are an encouraging and positive guide. The user needs some motivation and hope.
                Respond with positivity and energy. Highlight the bright side and offer sincere
                encouragement. Help the user see a path forward."
            }
            ResponseStrategy::Cheerful => {
                "You are a cheerful and friendly chat companion. The user is in a good mood.
                Respond in a lighthearted, fun way. Share in their joy and keep the conversation
                engaging and energetic."
            }
            ResponseStrategy::Neutral => {
                "You are a polite and professional conversational assistant.
                Respond in a balanced, friendly manner. Focus on understanding the user's needs
                and providing helpful responses."
            }
        }
    }
}

pub fn select_strategy(
    emotion: &SentimentClassification,
    trend: EmotionTrend,
) -> ResponseStrategy {
    match (emotion.sentiment, trend) {
        (Sentiment::Negative, EmotionTrend::Declining) => ResponseStrategy::Empathetic,
        (Sentiment::Negative, EmotionTrend::Stable) => ResponseStrategy::Encouraging,
        (Sentiment::Positive, _) => ResponseStrategy::Cheerful,
        _ => ResponseStrategy::Neutral,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_strategy_negative_declining() {
        use crate::state::EmotionTrend;

        let emotion = SentimentClassification {
            sentiment: Sentiment::Negative,
            confidence: 0.8,
        };

        let strategy = select_strategy(&emotion, EmotionTrend::Declining);
        assert_eq!(strategy, ResponseStrategy::Empathetic);
    }

    #[test]
    fn test_select_strategy_negative_stable() {
        use crate::state::EmotionTrend;

        let emotion = SentimentClassification {
            sentiment: Sentiment::Negative,
            confidence: 0.8,
        };

        let strategy = select_strategy(&emotion, EmotionTrend::Stable);
        assert_eq!(strategy, ResponseStrategy::Encouraging);
    }

    #[test]
    fn test_select_strategy_positive() {
        use crate::state::EmotionTrend;

        let emotion = SentimentClassification {
            sentiment: Sentiment::Positive,
            confidence: 0.8,
        };

        let strategy = select_strategy(&emotion, EmotionTrend::Improving);
        assert_eq!(strategy, ResponseStrategy::Cheerful);
    }

    #[test]
    fn test_select_strategy_neutral() {
        use crate::state::EmotionTrend;

        let emotion = SentimentClassification {
            sentiment: Sentiment::Neutral,
            confidence: 0.5,
        };

        let strategy = select_strategy(&emotion, EmotionTrend::Stable);
        assert_eq!(strategy, ResponseStrategy::Neutral);
    }

    #[test]
    fn test_strategy_prompts() {
        let prompts = vec![
            ResponseStrategy::Empathetic.to_prompt(),
            ResponseStrategy::Encouraging.to_prompt(),
            ResponseStrategy::Neutral.to_prompt(),
            ResponseStrategy::Cheerful.to_prompt(),
        ];

        for prompt in prompts {
            assert!(!prompt.is_empty());
            assert!(prompt.len() > 50);
        }
    }
}
```

**Step 2: Run cargo check**

Run: `cargo check`
Expected: PASS

**Step 3: Run tests**

Run: `cargo test strategy`
Expected: All tests PASS

**Step 4: Commit**

```bash
git add src/strategy/response.rs
git commit -m "feat: implement ResponseStrategy with selection logic"
```

---

## Task 6: Implement EmotionDetector Agent

**Files:**
- Create: `src/agents/emotion.rs`

**Step 1: Write the implementation**

Create `src/agents/emotion.rs`:

```rust
use anyhow::Result;
use rig::providers::openai;
use crate::SentimentClassification;

pub struct EmotionDetector {
    client: openai::Client,
    model: String,
}

impl EmotionDetector {
    pub fn new(client: openai::Client, model: &str) -> Self {
        Self {
            client,
            model: model.to_string(),
        }
    }

    pub async fn analyze(&self, text: &str) -> Result<SentimentClassification> {
        let extractor = self.client
            .extractor::<SentimentClassification>(&self.model)
            .preamble(
                "You are a sentiment analysis expert. Analyze the emotional tone of the user's text.
                Return the sentiment type (Positive/Negative/Neutral) and a confidence score (0-1).
                Be accurate and thoughtful in your assessment."
            )
            .build();

        let result = extractor.extract(text).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_detector_new() {
        // This is a compile-time test only
        let api_key = "test-key";
        let base_url = "https://api.example.com";
        let client = openai::Client::from_url(api_key, base_url);
        let detector = EmotionDetector::new(client, "test-model");

        assert_eq!(detector.model, "test-model");
    }
}
```

**Step 2: Run cargo check**

Run: `cargo check`
Expected: PASS

**Step 3: Run tests**

Run: `cargo test emotion`
Expected: Tests PASS (only compile test, no API call)

**Step 4: Commit**

```bash
git add src/agents/emotion.rs
git commit -m "feat: implement EmotionDetector agent"
```

---

## Task 7: Implement ChatAgent

**Files:**
- Create: `src/agents/chat.rs`

**Step 1: Write the implementation**

Create `src/agents/chat.rs`:

```rust
use anyhow::Result;
use rig::providers::openai;
use crate::models::{Message, MessageRole};
use crate::strategy::ResponseStrategy;

pub struct ChatAgent {
    client: openai::Client,
    model: String,
}

impl ChatAgent {
    pub fn new(client: openai::Client, model: &str) -> Self {
        Self {
            client,
            model: model.to_string(),
        }
    }

    pub async fn respond(
        &self,
        user_input: &str,
        strategy: ResponseStrategy,
        history: &[Message],
    ) -> Result<String> {
        let context = self.build_context_prompt(history);

        let agent = self.client
            .agent(&self.model)
            .preamble(strategy.to_prompt())
            .context(&context)
            .build();

        let response = agent.prompt(user_input).await?;
        Ok(response)
    }

    fn build_context_prompt(&self, history: &[Message]) -> String {
        if history.is_empty() {
            return "This is a new conversation.".to_string();
        }

        let mut context = String::from("Recent conversation:\n");

        // Only keep last 5 messages to avoid token overflow
        for msg in history.iter().rev().take(5).rev() {
            let role = match msg.role {
                MessageRole::User => "User",
                MessageRole::Assistant => "Assistant",
            };
            context.push_str(&format!("{}: {}\n", role, msg.content));
        }

        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_agent_new() {
        let api_key = "test-key";
        let base_url = "https://api.example.com";
        let client = openai::Client::from_url(api_key, base_url);
        let agent = ChatAgent::new(client, "test-model");

        assert_eq!(agent.model, "test-model");
    }

    #[test]
    fn test_build_context_prompt_empty() {
        let api_key = "test-key";
        let base_url = "https://api.example.com";
        let client = openai::Client::from_url(api_key, base_url);
        let agent = ChatAgent::new(client, "test-model");

        let context = agent.build_context_prompt(&[]);
        assert!(context.contains("new conversation"));
    }

    #[test]
    fn test_build_context_prompt_with_messages() {
        use crate::models::Message;

        let api_key = "test-key";
        let base_url = "https://api.example.com";
        let client = openai::Client::from_url(api_key, base_url);
        let agent = ChatAgent::new(client, "test-model");

        let messages = vec![
            Message {
                role: MessageRole::User,
                content: "Hello".to_string(),
                timestamp: 1,
                emotion: None,
            },
            Message {
                role: MessageRole::Assistant,
                content: "Hi there!".to_string(),
                timestamp: 2,
                emotion: None,
            },
        ];

        let context = agent.build_context_prompt(&messages);
        assert!(context.contains("User: Hello"));
        assert!(context.contains("Assistant: Hi there!"));
    }
}
```

**Step 2: Run cargo check**

Run: `cargo check`
Expected: PASS

**Step 3: Run tests**

Run: `cargo test chat`
Expected: All tests PASS

**Step 4: Commit**

```bash
git add src/agents/chat.rs
git commit -m "feat: implement ChatAgent with context-aware responses"
```

---

## Task 8: Update main.rs for CLI Interface

**Files:**
- Modify: `src/main.rs`

**Step 1: Update main.rs**

Replace the entire `src/main.rs` with:

```rust
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
use strategy::{select_strategy, ResponseStrategy};

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

        // Analyze emotion
        let emotion = match emotion_detector.analyze(input).await {
            Ok(e) => e,
            Err(e) => {
                eprintln!("❌ Emotion detection failed: {}", e);
                continue;
            }
        };

        // Update state
        state_manager.add_message(MessageRole::User, input);
        state_manager.update_emotion(emotion.clone());

        // Select strategy
        let trend = state_manager.get_recent_emotion_trend();
        let strategy = select_strategy(&emotion, trend);

        // Generate response
        let response = match chat_agent.respond(input, strategy, state_manager.get_history()).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ Response generation failed: {}", e);
                continue;
            }
        };

        // Update and output
        state_manager.add_message(MessageRole::Assistant, &response);

        println!("📊 Emotion: {:?} (confidence: {:.2})", emotion.sentiment, emotion.confidence);
        println!("📈 Trend: {:?}", trend);
        println!("🎯 Strategy: {:?}", strategy);
        println!("🤖 Assistant: {}\n", response);
    }

    Ok(())
}
```

**Step 2: Run cargo check**

Run: `cargo check`
Expected: PASS

**Step 3: Run cargo build**

Run: `cargo build`
Expected: Builds successfully

**Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat: implement CLI interface with full chat loop"
```

---

## Task 9: Update .env.example

**Files:**
- Modify: `.env.example`

**Step 1: Update .env.example**

Replace `.env.example` with:

```bash
# Zhipu AI (GLM) API Configuration
OPENAI_API_KEY=your-api-key-here
OPENAI_BASE_URL=https://open.bigmodel.cn/api/paas/v4

# Model to use
MODEL=glm-4.7

# Alternative endpoints:
# - Standard GLM API: https://open.bigmodel.cn/api/paas/v4
# - Coding Plan API: https://open.bigmodel.cn/api/coding/paas/v4
# - Local Ollama: http://localhost:11434/v1
```

**Step 2: Commit**

```bash
git add .env.example
git commit -m "docs: update .env.example with new configuration options"
```

---

## Task 10: Final Build and Test

**Files:**
- None (verification step)

**Step 1: Build the project**

Run: `cargo build --release`
Expected: Successful build

**Step 2: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 3: Quick manual test**

Run: `cargo run --release`
Type: "Hello! How are you?"
Expected: Emotion detected, response generated

**Step 4: Final commit**

```bash
git add -A
git commit -m "feat: complete emotional-aware chat system implementation"
```

**Step 5: Push to remote**

```bash
git push origin feature/emotional-chat
```

---

## Summary

This implementation plan creates a complete emotional-aware chat system with:

1. **Dual Agent Architecture** - Separate agents for emotion detection and response generation
2. **Conversation State Management** - Tracks message history and emotion trends
3. **Dynamic Strategy Selection** - Chooses response style based on emotion and trend
4. **CLI Interface** - Interactive command-line chat interface
5. **Full Test Coverage** - Unit tests for all components

**Total commits: ~10**
**Estimated time: 1-2 hours**
**Key files created/modified: 12 files**
