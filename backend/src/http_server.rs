use axum::{
    extract::State,
    response::{sse, IntoResponse, Sse},
    routing::get,
    BoxError, Router,
};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::tamper_switch;

// Routes and handlers

#[derive(Clone)]
struct RouteState {
    tamper_event_tx: broadcast::Sender<tamper_switch::Event>,
}

pub fn routes(tamper_event_tx: broadcast::Sender<tamper_switch::Event>) -> Router {
    let routes = Router::new()
        .route("/events", get(get_gateway_events))
        .with_state(RouteState { tamper_event_tx });
    Router::new().nest("/api/v1", routes)
}

async fn get_gateway_events(State(route_state): State<RouteState>) -> impl IntoResponse {
    let tamper_event_rx = BroadcastStream::new(route_state.tamper_event_tx.subscribe()).map(
        |e| -> Result<_, BoxError> {
            match e {
                Ok(e) => Ok(sse::Event::default().data(e.to_string())),
                Err(e) => Err(e.into()),
            }
        },
    );

    let sse = Sse::new(tamper_event_rx).keep_alive(sse::KeepAlive::new());

    sse.into_response()
}
