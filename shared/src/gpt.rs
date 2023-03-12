use serde::{Deserialize, Serialize};
use crux_http::Http;
use app::Event;
use crate::app;

const API_TOKEN: &'static str = "";
const COMPLETION_URI: &'static str = "https://api.openai.com/v1/chat/completions";
const GENERATION_URI: &'static str = "https://api.openai.com/v1/images/generations";

pub struct API {
    model: String,
    role: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

impl ChatCompletion {
    pub fn get_answer(&self) -> Option<&str> {
        self.choices.get(0).map(|choice| choice.message.content.as_str())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PictureMetadata {
    data: Vec<PicturePath>,
    created: i64,
}

impl PictureMetadata {
    pub fn get_uri(&self) -> Option<String> {
        let path = self.data.first()?;
        let url = path.url.clone();
        Some(url)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct PicturePath {
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PictureRequest {
    prompt: String,
    n: usize,
    size: String,
}

impl API {
    pub fn new() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            role: "user".to_string(),
        }
    }

    pub fn make_request(&self, question: &str, http: &Http<Event>) -> crux_http::RequestBuilder<Event, ChatCompletion> {
        let body = self.question_body(question);
        let cred = format!("Bearer {}", API_TOKEN);
        // Currently body and headers are not supported so hack is needed
        let fake_uri = format!("{}£{}£{}", COMPLETION_URI, cred, body);
        http.post(fake_uri)
            .header("Authorization", format!("Bearer {}", API_TOKEN))
            .header("Content-Type", "application/json")
            .body_string(body)
            .expect_json::<ChatCompletion>()
    }

    pub fn make_img_request(&self, query: &str, http: &Http<Event>) -> crux_http::RequestBuilder<Event, PictureMetadata> {
        let body = self.picture_body(query);
        let cred = format!("Bearer {}", API_TOKEN);
        // Currently body and headers are not supported so hack is needed
        let fake_uri = format!("{}£{}£{}", GENERATION_URI, cred, body);
        http.post(fake_uri)
            .header("Authorization", format!("Bearer {}", API_TOKEN))
            .header("Content-Type", "application/json")
            .body_string(body)
            .expect_json::<PictureMetadata>()
    }

    fn question_body(&self, question: &str) -> String {
        let req = CompletionRequest {
            model: String::from(self.model.clone()),
            messages: vec![
                CompletionMessage {
                    role: String::from(self.role.clone()),
                    content: String::from(question.to_string()),
                }
            ],
        };

        serde_json::to_string(&req).unwrap()
    }

    fn picture_body(&self, query: &str) -> String {
        let req = PictureRequest {
            prompt: String::from(query),
            n: 1,
            size: "1024x1024".to_string(),
        };
        serde_json::to_string(&req).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct CompletionMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<CompletionMessage>,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use super::{CompletionMessage, CompletionRequest};
    use crate::gpt;

    #[test]
    fn test_serialized_correctly() {
        let cli = gpt::API::new();
        assert_eq!(cli.question_body(""), "{\"model\":\"gpt-3.5-turbo\",\"messages\":[{\"role\":\"user\",\"content\":\"\"}]}");
    }
}