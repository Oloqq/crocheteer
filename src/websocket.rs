use futures_util::FutureExt;
// use futures_util::{SinkExt, StreamExt};
use futures_util::{select, sink::SinkExt, stream::StreamExt};
use pin_utils::pin_mut;
use tokio::net::TcpListener;
use tokio::time::{Duration, Instant};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;

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
    state.0[1] += 0.1 * state.1;
    if state.0[1] > 5.0 {
        state.1 = -1.0;
    } else if state.0[1] < 0.0 {
        state.1 = 1.0;
    }
    state.0
}

struct State {
    paused: bool,
}

fn handle_incoming(msg: Option<Result<Message, Error>>, state: &mut State) {
    if let Some(Ok(message)) = msg {
        match message.to_string().as_str() {
            "pause" => state.paused = true,
            "resume" => state.paused = false,
            _ => (),
        }
        // Handle incoming message
        println!("Received a message: {:?}", message);
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    let mut state = ([1.0, 0.5, 0.0], 1.0);
    let mut control = State { paused: false };
    let mut interval_duration = Duration::from_millis(17);
    let mut last_tick = Instant::now();

    loop {
        let mut incoming_msg = read.next().fuse();
        let sleep_future = tokio::time::sleep_until(last_tick + interval_duration).fuse();
        pin_mut!(sleep_future); // Pin the sleep future
        select! {
            msg = incoming_msg => handle_incoming(msg, &mut control),
            _ = sleep_future => {
                if !control.paused {
                    let pos_data = serde_json::json!(calc_data(&mut state)).to_string();
                    println!("{pos_data}");

                    if write.send(Message::Text(pos_data)).await.is_err() {
                        println!("Connection done");
                        break;
                    }
                }
                let computation_time = last_tick.elapsed();
                last_tick = Instant::now();
                interval_duration = Duration::from_millis(17).saturating_sub(computation_time);
            }
        }
    }
}
