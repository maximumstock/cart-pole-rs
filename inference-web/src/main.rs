use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use inference_web::{infer, CARTPOLE_MODEL_FILE_PATH};
use serde::Deserialize;
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
        model: tch::CModule::load(CARTPOLE_MODEL_FILE_PATH).unwrap(),
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

async fn inference_endpoint(
    State(state): State<SharedState>,
    query: Query<InferenceQueryParams>,
) -> Result<String, StatusCode> {
    infer(&state.model, &query.0.into())
        .map(|inference| {
            format!(
                "{{\"left\": {}, \"right\": {}}}͘",
                inference.left, inference.right
            )
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
#[derive(Deserialize)]
struct InferenceQueryParams {
    position: f32,
    velocity: f32,
    angle: f32,
    angular_velocity: f32,
}

impl From<InferenceQueryParams> for [f32; 4] {
    fn from(value: InferenceQueryParams) -> Self {
        [
            value.position,
            value.velocity,
            value.angle,
            value.angular_velocity,
        ]
    }
}
