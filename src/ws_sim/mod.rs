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
pub async fn serve_websocket() {
    let try_socket = TcpListener::bind("127.0.0.1:8080").await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

struct State {
    paused: bool,
}

struct Simulation {
    // plushie: Plushie,
    ball_pos: [f32; 3],
    dy: f32,
}

impl Simulation {
    fn new() -> Self {
        Self {
            ball_pos: [1.0, 0.5, 0.0],
            dy: 1.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.ball_pos[1] += 0.1 * self.dy * dt;
        if self.ball_pos[1] > 5.0 {
            self.dy = -1.0;
        } else if self.ball_pos[1] < 0.0 {
            self.dy = 1.0;
        }
    }

    fn get_data(&self) -> [f32; 3] {
        self.ball_pos
    }
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
    } else {
        println!("wtf");
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    let mut simulation = Simulation::new();
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
                    simulation.update(1.0);
                    let pos_data = serde_json::json!(simulation.get_data()).to_string();

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
