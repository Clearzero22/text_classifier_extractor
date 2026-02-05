# Source Module - Text Classifier Extractor

**Parent**: [text_classifier_extractor/](../CLAUDE.md)
**Last Updated**: 2026-02-06

---

## Module Overview

This module contains the core application logic for the text classifier. Currently organized as a single-file application (`main.rs`), it includes data models, API integration, and the main async runtime.

---

## Architecture

```
src/
└── main.rs
    ├── Data Models (lines 6-17)
    │   ├── Sentiment enum
    │   └── SentimentClassification struct
    │
    ├── Main Function (lines 19-53)
    │   ├── Environment setup
    │   ├── API client initialization
    │   ├── Agent builder (active)
    │   └── Extractor builder (commented)
    │
    └── Dependencies
        ├── anyhow::Result
        ├── rig::completion::Prompt
        ├── rig::providers::openai
        ├── schemars::JsonSchema
        └── serde traits
```

---

## Interfaces

### Public Types

```rust
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

pub struct SentimentClassification {
    pub sentiment: Sentiment,
    pub confidence: f32,
}
```

### Entry Points

```rust
#[tokio::main]
async fn main() -> Result<()>
```

**Purpose**: Application entry point that initializes the API client and executes the sentiment classification.

**Returns**: `Ok(())` on success, error via `anyhow::Result` on failure.

---

## Dependencies

### External Crates

| Crate | Usage | Lines |
|-------|-------|-------|
| `anyhow` | Error handling | 1, 20 |
| `rig` | LLM integration | 2, 24-32, 34 |
| `schemars` | JSON schema generation | 3, 6, 13 |
| `serde` | Serialization | 4, 6, 13 |
| `dotenv` | Environment loading | 21 |
| `tokio` | Async runtime | 19 |

### Internal Dependencies

None currently (monolithic structure).

---

## Key Functions

### `main()` (lines 20-53)

**Flow**:
1. Load environment variables from `.env`
2. Read `OPENAI_API_KEY` and `OPENAI_BASE_URL`
3. Initialize `openai::Client` with credentials
4. Build agent with model and preamble
5. Execute prompt and display response
6. Return `Result<()>`

**Environment Variables Required**:
- `OPENAI_API_KEY`: Zhipu AI API key
- `OPENAI_BASE_URL`: API endpoint (defaults provided if not set)

---

## Data Flow

```
┌─────────────┐
│   .env      │ Load
└──────┬──────┘
       │
       ▼
┌─────────────────────┐
│ Environment Loading │ (dotenv)
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ openai::Client      │ (rig-core)
│  - API Key          │
│  - Base URL         │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Agent Builder       │
│  - Model: glm-4.7   │
│  - Preamble         │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Prompt Execution    │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Response Output     │
└─────────────────────┘
```

---

## Active vs Commented Code

### Active Implementation (lines 29-36)
```rust
let comedian_agent = openai_client
    .agent("glm-4.7")
    .preamble("You are a comedian...")
    .build();

let response = comedian_agent.prompt("Entertain!").await?;
println!("{response}");
```

**Purpose**: Testing API connectivity with a simple chat agent.

### Commented Implementation (lines 40-50)
```rust
// let sentiment_classifier = openai_client
//     .extractor::<SentimentClassification>("glm-4.7")
//     .preamble("You are a sentiment analysis AI...")
//     .build();
//
// let result = sentiment_classifier.extract(text).await?;
```

**Purpose**: Structured extraction implementation (currently disabled).

---

## Testing

### Current State
No tests implemented yet.

### Recommended Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_serialization() {
        let sentiment = Sentiment::Positive;
        let json = serde_json::to_string(&sentiment).unwrap();
        assert!(json.contains("Positive"));
    }

    #[test]
    fn test_classification_schema() {
        let schema = schemars::schema_for!(SentimentClassification);
        assert!(schema.schema.is_some());
    }

    #[tokio::test]
    async fn test_agent_creation() {
        // Test agent builder with mock API
    }
}
```

---

## Refactoring Opportunities

### 1. Modularize into Separate Files

```
src/
├── main.rs              # Entry point only
├── models/
│   ├── mod.rs
│   └── sentiment.rs     # Sentiment types
├── agents/
│   ├── mod.rs
│   └── classifier.rs    # Classifier logic
└── config.rs            # Environment handling
```

### 2. Extract Configuration Logic

```rust
// config.rs
pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        // ...
    }
}
```

### 3. Create Classifier Trait

```rust
// classifier.rs
#[async_trait::async_trait]
pub trait Classifier<T> {
    async fn classify(&self, text: &str) -> Result<T>;
}
```

---

## Code Smells & Technical Debt

| Issue | Location | Severity | Suggested Fix |
|-------|----------|----------|---------------|
| Large main.rs | All lines | Medium | Split into modules |
| Missing tests | N/A | High | Add unit/integration tests |
| Hardcoded strings | Line 31 | Low | Move to constants or config |
| No error recovery | Line 34 | Medium | Add retry logic |
| Missing documentation | All | Medium | Add rustdoc comments |

---

## Navigation Breadcrumbs

**Up**: [Project Root](../CLAUDE.md)

**Siblings**:
- None currently (single module)

**Children**: None (flat structure)

---

## Future Enhancements

1. **Logging**: Add `tracing` for request/response logging
2. **Metrics**: Track classification latency and accuracy
3. **Batch Processing**: Support multiple texts in one request
4. **Caching**: Cache results for identical inputs
5. **CLI Interface**: Add command-line argument parsing
6. **Configuration File**: Support TOML/YAML config besides .env

---

## Notes

- **Typo in preamble** (line 31): "enterttain" should be "entertain"
- **Incomplete extraction**: Structured extractor code is commented out
- **API Key exposure**: `.env` file should never be committed
- **Edition**: Uses Rust 2024 edition features

---

**Maintenance**: Keep this file synchronized with `main.rs` changes. Update interface documentation when adding new types or functions.
