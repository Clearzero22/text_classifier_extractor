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
