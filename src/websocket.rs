// use futures_util::{SinkExt, StreamExt};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use tokio::net::TcpListener;
use tokio::time::{sleep_until, Duration, Instant};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
pub async fn websocket_stuff() {
    let try_socket = TcpListener::bind("127.0.0.1:8080").await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

fn calc_data(state: &mut ([f32; 3], f32)) -> [f32; 3] {
    state.0[1] += 0.3 * state.1;
    if state.0[1] > 5.0 {
        state.1 = -1.0;
    } else if state.0[1] < 0.0 {
        state.1 = 1.0;
    }
    state.0
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    let mut state = ([1.0, 0.5, 0.0], 1.0);
    let mut paused = false;

    loop {
        let start = Instant::now();

        if let Ok(Some(message)) = read.try_next().await {
            match message {
                Message::Text(txt) => {
                    if txt == "pause" {
                        paused = true;
                    } else if txt == "resume" {
                        paused = false;
                    }
                    // Process any other messages here
                }
                _ => {}
            }
        }

        if !paused {
            let pos_data = serde_json::json!(calc_data(&mut state)).to_string();

            if write.send(Message::Text(pos_data)).await.is_err() {
                println!("Connection done");
                break;
            }
        }

        let elapsed = start.elapsed();
        if let Some(remaining_sleep_duration) = Duration::from_millis(17).checked_sub(elapsed) {
            sleep_until(start + remaining_sleep_duration).await;
        }
    }
}
