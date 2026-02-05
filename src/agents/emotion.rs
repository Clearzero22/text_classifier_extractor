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
        use crate::Sentiment;

        // 构建 prompt，对短文本提供更多上下文指导
        let input_prompt = if text.trim().len() < 5 {
            format!(
                "Analyze the sentiment of this very short text: \"{}\". \
                 Since the text is brief, consider common usage patterns. \
                 Is it Positive, Negative, or Neutral? Return a confidence score.",
                text
            )
        } else {
            text.to_string()
        };

        let system_prompt = if text.trim().len() < 5 {
            "You are a sentiment analysis expert specializing in brief text analysis. \
             For short inputs like names, greetings, or single words, use contextual clues. \
             Return sentiment type (Positive/Negative/Neutral) and confidence score (0-1). \
             Default to Neutral when uncertain, but set confidence to 0.5-0.7."
        } else {
            "You are a sentiment analysis expert. Analyze the emotional tone of the user's text. \
             Return the sentiment type (Positive/Negative/Neutral) and a confidence score (0-1). \
             Be accurate and thoughtful in your assessment."
        };

        let extractor = self.client
            .extractor::<SentimentClassification>(&self.model)
            .preamble(system_prompt)
            .build();

        // 尝试提取，如果失败则使用降级策略
        match extractor.extract(&input_prompt).await {
            Ok(result) => Ok(result),
            Err(e) => {
                // 检查错误类型 - 如果是反序列化错误（空响应或无效JSON），返回默认值
                let error_msg = e.to_string();
                if error_msg.contains("deserialize") || error_msg.contains("expected value") {
                    // 降级策略：返回 Neutral 情感，中等置信度
                    Ok(SentimentClassification {
                        sentiment: Sentiment::Neutral,
                        confidence: 0.5,
                    })
                } else {
                    // 其他错误类型（如网络错误）转换为 anyhow::Error 后向上传递
                    Err(anyhow::anyhow!("API error: {}", e))
                }
            }
        }
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
