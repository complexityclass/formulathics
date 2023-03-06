use crate::capabilities::sse::ServerSentEvents;
use crux_core::render::Render;
use crux_http::Http;
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    Increment,
    Decrement,
    Reset,
}

#[derive(Default)]
pub struct Model {
    count: isize,
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub count: String,
}

#[derive(Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
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
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
            Event::Reset => model.count = 0,
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        Self::ViewModel {
            count: format!("Count is {}", model.count)
        }
    }
}