use std::net::SocketAddr;

use tokio::runtime::Runtime;
use axum::{Router, routing::{get, post}, response::IntoResponse, extract::Extension};
use std::sync::{mpsc::{Receiver, SyncSender}};

pub fn start_server(chan: SyncSender<()>) -> Runtime {
    let rt = Runtime::new().unwrap();

    rt.spawn(async move {
        let app = Router::new()
            .route("/", get(root_route))
            .route("/toggle", post(toggle_route))
            .layer(Extension(chan));

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        dbg!(addr);

        axum::Server::bind(&addr)
          .serve(app.into_make_service())
          .await
          .unwrap();

    });

    rt
}

pub fn update_server(chan: &Receiver<()>, flag: &mut bool) {
  if let Ok(_) = chan.try_recv() {
      *flag = !*flag;
  }
}

async fn root_route() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("./index.html"))
}

async fn toggle_route(Extension(chan): Extension<SyncSender<()>>) -> axum::response::Html<&'static str> {
    chan.send(());
    axum::response::Html(include_str!("./index.html"))
}