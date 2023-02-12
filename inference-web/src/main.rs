use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use inference_web::{infer, Inference, CARTPOLE_MODEL_FILE_PATH};
use serde::{Deserialize, Serialize};
use tch::CModule;
use tracing::info;

#[tokio::main]
async fn main() {
    // setup tracing & logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    // load the traced model
    let state = AppState {
        model: tch::CModule::load_on_device(CARTPOLE_MODEL_FILE_PATH, tch::Device::Cpu).unwrap(),
    };
    let shared_state = Arc::new(state);

    // setup the web server
    let app = Router::new()
        .route("/inference", get(inference_endpoint))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct AppState {
    model: CModule,
}
type SharedState = Arc<AppState>;

/// HTTP handler that takes query parameters, runs inference, and returns a JSON response or
/// a 500 internal server error.
async fn inference_endpoint(
    State(state): State<SharedState>,
    query: Query<InferenceRequestQueryParams>,
) -> Result<impl IntoResponse, StatusCode> {
    infer(&state.model, &query.0.into())
        .map(|inference| Json(InferenceResponse::from(inference)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
#[derive(Deserialize)]
struct InferenceRequestQueryParams {
    position: f32,
    velocity: f32,
    angle: f32,
    angular_velocity: f32,
}

impl From<InferenceRequestQueryParams> for [f32; 4] {
    fn from(value: InferenceRequestQueryParams) -> Self {
        [
            value.position,
            value.velocity,
            value.angle,
            value.angular_velocity,
        ]
    }
}

#[derive(Serialize)]
struct InferenceResponse {
    left: f64,
    right: f64,
}

impl From<Inference> for InferenceResponse {
    fn from(value: Inference) -> Self {
        InferenceResponse {
            left: value.left,
            right: value.right,
        }
    }
}
