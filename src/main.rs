mod invoice_maker;

use axum::http::Method;
use axum::{routing::get, routing::post, Json, Router, response::IntoResponse, body::Body};
use axum::http::{StatusCode, header::CONTENT_TYPE};
use tokio::fs;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(|| async { "Welcome to the Invoice Maker!" }))
        .route("/create_invoice", post(create_invoice))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_invoice(Json(payload): Json<invoice_maker::Invoice>) -> impl IntoResponse {
    println!("Yooo");
    let result = tokio::task::spawn_blocking(move || invoice_maker::create_invoice(payload))
        .await
        .unwrap();

    match result {
        Ok(_) => {
            let contents = fs::read("output.pdf").await.unwrap();
            fs::remove_file("output.pdf").await.unwrap();
            (
                StatusCode::OK,
                [(CONTENT_TYPE, "application/pdf")],
                Body::from(contents),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(CONTENT_TYPE, "text/plain")],
            Body::from(format!("Failed to create invoice: {:?}", e.to_string())),
        ),
    }
}
