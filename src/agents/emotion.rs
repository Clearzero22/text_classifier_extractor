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
