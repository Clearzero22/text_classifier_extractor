# Emotional-Aware Chat System

A Rust-based conversational AI that detects user emotions and dynamically adjusts response strategies using Zhipu AI's GLM-4.7 API.

## Features

- 🎭 **Emotion Detection** - Analyzes user sentiment in real-time (Positive/Negative/Neutral)
- 📈 **Emotion Trend Tracking** - Monitors how emotions change over the conversation
- 🎯 **Dynamic Response Strategies** - Adapts conversation style based on emotional state:
  - **Empathetic** - For users in distress (negative + declining trend)
  - **Encouraging** - For users needing motivation (negative + stable trend)
  - **Cheerful** - For users in good mood (positive sentiment)
  - **Neutral** - Professional, balanced responses (default)
- 💬 **Context-Aware** - Maintains conversation history for coherent multi-turn dialogue
- 🛡️ **Error Handling** - Graceful fallback for API failures and edge cases

## Architecture

```
User Input
    ↓
EmotionDetector → SentimentClassification
    ↓
ConversationManager → EmotionTrend
    ↓
select_strategy() → ResponseStrategy
    ↓
ChatAgent → Context-Aware Response
    ↓
Output (emotion, trend, strategy, response)
```

## Tech Stack

- **Language**: Rust 2024 Edition
- **AI Framework**: rig-core 0.11.1
- **Runtime**: tokio 1.34
- **Serialization**: serde, schemars
- **Model**: Zhipu AI GLM-4.7

## Prerequisites

- Rust 1.93+
- Zhipu AI API key ([Get one here](https://open.bigmodel.cn/))

## Installation

```bash
# Clone the repository
git clone https://github.com/Clearzero22/text_classifier_extractor.git
cd text_classifier_extractor

# Create environment file
cp .env.example .env

# Edit .env with your API key
# OPENAI_API_KEY=your-api-key-here
```

## Configuration

Create a `.env` file in the project root:

```bash
# Required
OPENAI_API_KEY=your-zhipu-ai-api-key

# Optional (with defaults shown)
OPENAI_BASE_URL=https://open.bigmodel.cn/api/paas/v4
MODEL=glm-4.7
```

## Usage

```bash
# Run the application
cargo run --release

# Or in development mode
cargo run
```

### Example Session

```
🤖 Emotional-Aware Chat System
📊 Model: glm-4.7
💬 Type 'quit' or 'exit' to end

You: Hello! How are you today?
📊 Emotion: Positive (confidence: 0.95)
📈 Trend: Stable
🎯 Strategy: Cheerful
🤖 Assistant: Hello! I'm doing wonderfully, thank you for asking! 😊
It's such a pleasure to chat with someone so friendly. How can I brighten your day?

You: I'm feeling really down lately
📊 Emotion: Negative (confidence: 0.85)
📈 Trend: Declining
🎯 Strategy: Empathetic
🤖 Assistant: I hear you, and I want you to know that your feelings are completely valid.
It takes courage to share when you're struggling. Please know that I'm here to listen without judgment.

You: quit
👋 Goodbye!
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test emotion
cargo test conversation
cargo test strategy
```

### Project Structure

```
src/
├── main.rs              # Entry point, CLI interface
├── models/
│   └── message.rs       # Message and MessageRole types
├── agents/
│   ├── emotion.rs       # EmotionDetector using structured extraction
│   └── chat.rs          # ChatAgent with strategy-based responses
├── state/
│   └── conversation.rs  # ConversationManager, EmotionTrend
└── strategy/
    └── response.rs      # ResponseStrategy enum and selection logic
```

## API Integration

This project uses Zhipu AI's GLM-4.7 model through an OpenAI-compatible API:

- **Provider**: Zhipu AI (BigModel)
- **Documentation**: https://open.bigmodel.cn/
- **Structured Extraction**: Uses JSON schema for type-safe sentiment analysis

## Error Handling

The system includes robust error handling:

- **Short Text Handling**: Enhanced prompts for inputs < 5 characters
- **API Failures**: Automatic fallback to Neutral sentiment
- **Edge Cases**: Handles empty history, single emotion, and boundary conditions

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.

## Acknowledgments

- Built with [rig-core](https://github.com/drig-tech/rig) - LLM framework for Rust
- Powered by [Zhipu AI GLM-4.7](https://open.bigmodel.cn/)
