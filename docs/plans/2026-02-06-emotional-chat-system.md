# Emotional-Aware Conversational AI System

**Date**: 2026-02-06
**Status**: Design Phase
**Architecture**: Dual-Agent System

---

## Overview

A conversational AI system that detects user emotions and dynamically adjusts response strategies. Built with Rust, rig-core, and Zhipu AI's GLM-4.7 API.

---

## Architecture

### Components

```
┌─────────────────────────────────────────────────────────┐
│                    User Input                            │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              EmotionDetector Agent                       │
│  Analyzes user input → SentimentClassification          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│            ConversationManager                           │
│  - Maintains message history                            │
│  - Tracks emotion trends                                │
│  - Provides context to ChatAgent                        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│           ResponseStrategy Selector                      │
│  Empathetic | Encouraging | Neutral | Cheerful          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│               ChatAgent Agent                            │
│  Generates response based on emotion + history          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                    Response Output                       │
└─────────────────────────────────────────────────────────┘
```

### File Structure

```
src/
├── main.rs              # Entry point, CLI loop
├── agents/
│   ├── mod.rs
│   ├── emotion.rs       # EmotionDetector
│   └── chat.rs          # ChatAgent
├── state/
│   ├── mod.rs
│   └── conversation.rs  # ConversationManager
├── strategy/
│   ├── mod.rs
│   └── response.rs      # ResponseStrategy
└── models/
    ├── mod.rs
    └── message.rs       # Message, MessageRole
```

---

## Data Models

### Message

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: i64,
    pub emotion: Option<SentimentClassification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
}
```

### ConversationState

```rust
pub struct ConversationState {
    pub messages: Vec<Message>,
    pub emotion_history: Vec<SentimentClassification>,
    pub started_at: i64,
}
```

### ResponseStrategy

```rust
pub enum ResponseStrategy {
    Empathetic,   // For negative emotions
    Encouraging,  // For declining mood
    Neutral,      // Default
    Cheerful,     // For positive emotions
}
```

### EmotionTrend

```rust
pub enum EmotionTrend {
    Improving,   // Mood getting better
    Declining,   // Mood getting worse
    Stable,      // No significant change
}
```

---

## Agent Interfaces

### EmotionDetector

```rust
pub struct EmotionDetector {
    client: openai::Client,
    model: String,
}

impl EmotionDetector {
    pub fn new(client: openai::Client, model: &str) -> Self;

    pub async fn analyze(&self, text: &str)
        -> Result<SentimentClassification>;
}
```

**Prompt Template:**
```
You are a sentiment analysis expert. Analyze the emotional tone of the user's text.
Return sentiment type (Positive/Negative/Neutral) and confidence score (0-1).
```

### ChatAgent

```rust
pub struct ChatAgent {
    client: openai::Client,
    model: String,
}

impl ChatAgent {
    pub fn new(client: openai::Client, model: &str) -> Self;

    pub async fn respond(
        &self,
        user_input: &str,
        strategy: ResponseStrategy,
        history: &[Message]
    ) -> Result<String>;

    fn build_context_prompt(&self, history: &[Message]) -> String;
}
```

**Strategy Prompts:**

| Strategy | Prompt Focus |
|----------|--------------|
| **Empathetic** | Warm, understanding, emotional support |
| **Encouraging** | Positive, motivating, hopeful |
| **Cheerful** | Light, fun, energetic |
| **Neutral** | Professional, helpful, balanced |

---

## State Management

### ConversationManager

```rust
pub struct ConversationManager {
    state: ConversationState,
}

impl ConversationManager {
    pub fn new() -> Self;

    pub fn add_message(&mut self, role: MessageRole, content: &str);

    pub fn update_emotion(&mut self, emotion: SentimentClassification);

    pub fn get_recent_emotion_trend(&self) -> EmotionTrend;

    pub fn get_history(&self) -> &[Message];
}
```

### EmotionTrend Algorithm

```rust
// Calculate trend from recent emotions
// 1. Convert emotions to scores: Positive=1, Neutral=0, Negative=-1
// 2. Compare average of recent 3 vs earlier emotions
// 3. Return Improving/Declining/Stable based on threshold (±0.3)
```

---

## Strategy Selection Logic

```rust
fn select_strategy(
    emotion: &SentimentClassification,
    trend: EmotionTrend
) -> ResponseStrategy {
    match (emotion.sentiment, trend) {
        (Sentiment::Negative, EmotionTrend::Declining) => {
            ResponseStrategy::Empathetic
        }
        (Sentiment::Negative, EmotionTrend::Stable) => {
            ResponseStrategy::Encouraging
        }
        (Sentiment::Positive, _) => {
            ResponseStrategy::Cheerful
        }
        _ => ResponseStrategy::Neutral,
    }
}
```

---

## Main Loop Flow

```rust
async fn chat_loop(
    emotion_detector: &EmotionDetector,
    chat_agent: &ChatAgent,
    state_manager: &mut ConversationManager,
) -> Result<()> {
    loop {
        // 1. Get user input
        let input = get_user_input()?;

        // 2. Analyze emotion
        let emotion = emotion_detector.analyze(&input).await?;

        // 3. Update state
        state_manager.add_message(User, &input);
        state_manager.update_emotion(emotion.clone());

        // 4. Select strategy
        let trend = state_manager.get_recent_emotion_trend();
        let strategy = select_strategy(&emotion, trend);

        // 5. Generate response
        let response = chat_agent
            .respond(&input, strategy, state_manager.get_history())
            .await?;

        // 6. Update and output
        state_manager.add_message(Assistant, &response);
        println!("Assistant: {}", response);
    }
}
```

---

## Error Handling

```rust
#[derive(Error, Debug)]
pub enum ChatError {
    #[error("API error: {0}")]
    ApiError(#[from] rig::completion::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Invalid emotion result")]
    InvalidEmotion,
}

// Retry mechanism for transient failures
pub async fn with_retry<F, T>(
    operation: F,
    max_retries: u32,
) -> ChatResult<T>
where
    F: Fn() -> Pin<Box<dyn Future<Output = ChatResult<T>> + Send>>,
```

---

## Configuration

### Environment Variables

```bash
OPENAI_API_KEY=your-api-key
OPENAI_BASE_URL=https://open.bigmodel.cn/api/paas/v4
MODEL=glm-4.7
```

### Config Struct

```rust
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl Config {
    pub fn from_env() -> ChatResult<Self>;
}
```

---

## Dependencies

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

---

## Next Steps

1. **Create file structure** - Set up module directories
2. **Implement models** - Message, ConversationState
3. **Implement EmotionDetector** - Extractor-based emotion analysis
4. **Implement ChatAgent** - Strategy-based response generation
5. **Implement ConversationManager** - State and trend calculation
6. **Implement main loop** - CLI integration
7. **Add tests** - Unit tests for each component
8. **Add logging** - Request/response tracking with `tracing`

---

## Future Enhancements

- [ ] Streaming responses for better UX
- [ ] Multiple language support
- [ ] Emotion intensity levels (beyond 3-way classification)
- [ ] Long-term memory across sessions
- [ ] Web UI / TUI interface
- [ ] Voice input/output
- [ ] Export conversation history
- [ ] Custom strategy definitions
