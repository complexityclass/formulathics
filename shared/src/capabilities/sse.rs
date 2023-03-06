use async_sse::{decode, Event};
use async_std::io::Cursor;
use futures::StreamExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crux_core::{
    capability::{CapabilityContext, Operation},
    Capability,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SseRequest {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum SseResponse {
    Chunk(Vec<u8>),
    Done,
}

impl Operation for SseRequest {
    type Output = SseResponse;
}

pub struct ServerSentEvents<Ev> {
    context: CapabilityContext<SseRequest, Ev>,
}

impl<Ev> ServerSentEvents<Ev>
    where
        Ev: 'static,
{
    pub fn new(context: CapabilityContext<SseRequest, Ev>) -> Self {
        Self { context }
    }

    pub fn get_json<F, T>(&self, url: &str, make_event: F)
        where
            F: Fn(T) -> Ev + Clone + Send + 'static,
            T: DeserializeOwned,
    {
        unimplemented!();
    }
}

impl<Ef> Capability<Ef> for ServerSentEvents<Ef> {
    type Operation = SseRequest;
    type MappedSelf<MappedEv> = ServerSentEvents<MappedEv>;

    fn map_event<F, NewEvent>(&self, f: F) -> Self::MappedSelf<NewEvent>
        where
            F: Fn(NewEvent) -> Ef + Send + Sync + Copy + 'static,
            Ef: 'static,
            NewEvent: 'static,
    {
        ServerSentEvents::new(self.context.map_event(f))
    }
}