use std::fmt::Display;

use tower_http::trace::OnResponse;
use tracing::info;

use crate::app::auth::Principal;

#[derive(Debug, Clone, Copy)]
pub struct LatencyLayer;

impl<B> OnResponse<B> for LatencyLayer {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        if let Some(principal) = response.extensions().get::<Principal>() {
            span.record("user_id", &principal.id);
        }
        info!(
            latency = %DurationWrapper(latency),
            status = %response.status(),
            "finished processing request",
        )
    }
}

struct DurationWrapper(std::time::Duration);

impl Display for DurationWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} ms", self.0.as_millis())
        } else {
            write!(f, "{} μs", self.0.as_micros())
        }
    }
}
