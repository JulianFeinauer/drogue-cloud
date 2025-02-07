use super::*;

use anyhow::Context;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct HttpSink {
    client: reqwest::Client,
    sink: String,
}

#[async_trait]
impl Sink for HttpSink {
    type Error = reqwest::Error;

    #[allow(clippy::needless_lifetimes)]
    async fn publish<'a>(
        &self,
        _target: SinkTarget<'a>,
        event: Event,
    ) -> Result<PublishOutcome, SinkError<Self::Error>> {
        let response =
            cloudevents::binding::reqwest::event_to_request(event, self.client.post(&self.sink))?
                .send()
                .await
                .map_err(SinkError::Transport)?;

        log::debug!("Publish result: {:?}", response);

        match response.status().is_success() {
            true => Ok(PublishOutcome::Accepted),
            false => Ok(PublishOutcome::Rejected),
        }
    }
}

impl HttpSink {
    pub fn new() -> anyhow::Result<Self> {
        let sink = std::env::var("K_SINK").context("Missing variable 'K_SINK'")?;

        Ok(Self {
            client: reqwest::ClientBuilder::new().build()?,
            sink,
        })
    }
}
