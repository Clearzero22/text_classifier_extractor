use anyhow::Result;
use rig::completion::Prompt;
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
