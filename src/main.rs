use teleporter::{start_http_server, start_websocket_client};
use tokio::task;

#[tokio::main]
async fn main() {
    let http_server = task::spawn(start_http_server());
    let websocket_client= task::spawn(start_websocket_client());

    let _ = tokio::join!(http_server, websocket_client);
}
