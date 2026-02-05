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
