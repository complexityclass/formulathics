use std::fmt::format;
use crate::capabilities::sse::ServerSentEvents;
use crate::gpt;
use crux_core::render::Render;
use crux_http::Http;
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
pub enum Event {
    Reset,
    Ask(String),
    Gen(String),
    #[serde(skip)]
    Set(crux_http::Result<crux_http::Response<gpt::ChatCompletion>>),
    #[serde(skip)]
    SetImage(crux_http::Result<crux_http::Response<gpt::PictureMetadata>>),
}

#[derive(Default)]
pub struct Model {
    response: Option<String>,
    img_url: Option<String>,
    questions_number: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub answer: String,
    pub img_url: String,
    pub spent: String,
}

#[derive(Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
    pub http: Http<Event>,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::Ask(question) => {
                model.questions_number += 1;
                gpt::API::new().make_request(&question, &caps.http).send(Event::Set);
            },
            Event::Gen(query) => {
                model.questions_number += 1;
                gpt::API::new().make_img_request(&query, &caps.http).send(Event::SetImage);

            },
            Event::Set(Ok(mut completion)) => {
                let completion = completion.take_body().unwrap();
                model.response = completion.get_answer().and_then(|s| Some(s.to_string()));
                caps.render.render();
            },
            Event::Set(Err(_)) => {
                println!("Ooops");
                panic!("Oh no something went wrong");
            },
            Event::SetImage(Ok(mut metadata)) => {
                let metadata = metadata.take_body().unwrap();
                model.img_url = metadata.get_uri();
                caps.render.render();
            }
            Event::SetImage(Err(_)) => {
                println!("Ooops no image");
            },
            Event::Reset => {
                model.questions_number = 0;
                model.response = None;
                model.img_url = None;
            }
        };
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        let answer = if let Some(value) = &model.response {
            if value.is_empty() {
                format!("")
            } else {
                format!("Answer: {}", value.to_string())
            }
        } else {
            format!("AI is out of office :(")
        };

        let spent = (*&model.questions_number as f64) * 0.002;

        let img_url = if let Some(url) = &model.img_url {
            url.to_string()
        } else {
            "https://example.com/icon.png".to_string()
        };

        Self::ViewModel {
            answer: answer,
            img_url: img_url,
            spent: format!("Already spent {} £", spent),
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use serde::{Deserialize, Serialize};
    use crate::gpt;

    #[test]
    fn test_serialized_correctly() {}
}