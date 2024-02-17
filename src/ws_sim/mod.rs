mod ball_sim;
mod sim;

use futures_util::FutureExt;
use futures_util::{select, sink::SinkExt, stream::StreamExt};
use pin_utils::pin_mut;
use tokio::net::TcpListener;
use tokio::time::{Duration, Instant};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;

use self::ball_sim::{BallControls, BallSimulation};
use self::sim::*;

#[tokio::main]
pub async fn serve_websocket() {
    let try_socket = TcpListener::bind("127.0.0.1:8080").await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection::<BallSimulation>(stream));
    }
}

fn handle_incoming(msg: Option<Result<Message, Error>>, simulation: &mut impl Simulation) {
    if let Some(msg_res) = msg {
        if let Ok(message) = msg_res {
            println!("Received a message: {:?}", message);
            simulation.react(message.to_string().as_str());
        } else {
            println!("Received not Ok message: {:?}, wtf?", msg_res);
        }
    } else {
        println!("Received None, wtf?");
    }
}

async fn handle_connection<S: Simulation>(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    let mut simulation = S::new();
    let mut interval_duration = Duration::from_millis(17);
    let mut last_tick = Instant::now();

    loop {
        let mut incoming_msg = read.next().fuse();
        let sleep_future = tokio::time::sleep_until(last_tick + interval_duration).fuse();
        pin_mut!(sleep_future);
        select! {
            msg = incoming_msg => handle_incoming(msg, &mut simulation),
            _ = sleep_future => {
                let dt = 1.0;
                if let Some(data) = simulation.step(dt) {
                    if write.send(Message::Text(data)).await.is_err() {
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
