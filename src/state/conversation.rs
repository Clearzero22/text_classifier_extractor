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
        // Attach emotion to last user message first
        if let Some(msg) = self.state.messages.last_mut() {
            if matches!(msg.role, MessageRole::User) {
                msg.emotion = Some(emotion.clone());
            }
        }

        // Then add to history
        self.state.emotion_history.push(emotion);
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
