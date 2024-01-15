use async_openai::{
    config::{Config, OpenAIConfig},
    Client,
};

// Create a OpenAI client with API Key

pub struct OpenAI {
    client: Box<Client<OpenAIConfig>>,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        return OpenAI {
            client: Box::new(client),
        };
    }
}
