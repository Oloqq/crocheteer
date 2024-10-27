import Scene from "./Scene";

let ws: WebSocket | undefined = undefined;

export function connect(url: string, scene: Scene): WebSocket {
  ws = new WebSocket(url);

  ws.onopen = function (_event) {
    console.log("Connected to WebSocket server");
  };

  ws.onmessage = function (event) {
    let dict = JSON.parse(event.data);
    scene.parseMessage(dict["key"], dict["dat"]);
  };

  ws.onerror = function (error) {
    console.error("WebSocket error: ", error);
  };

  return ws;
}

export function send(text: string) {
  console.log(`sending: ${text}`, ws);
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(text);
  } else {
    console.error("WebSocket is not open.");
  }
}
