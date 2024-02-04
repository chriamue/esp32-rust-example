use axum::{response::Html, routing::get, Router};
use log::info;
use tokio::net::TcpListener;

const TCP_LISTENING_PORT: u16 = 80;

pub async fn server() -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{TCP_LISTENING_PORT}");

    let app = router();

    info!("Binding to {addr}...");
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

pub fn router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[cfg(test)]
mod tests {

    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_server() {
        let server = router();

        let req = Request::builder()
            .method("GET")
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = server
            .oneshot(req)
            .await
            .expect("failed to receive response");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
