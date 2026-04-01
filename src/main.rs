use rust_async_api::app;

#[tokio::main]
async fn main() {
    let app = app();

    let addr = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("could not start server");

    println!("listening on http://{}", addr);

    axum::serve(listener, app).await.expect("server error");
}
