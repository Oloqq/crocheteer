import World from "./World";

let ws: WebSocket | undefined = undefined;

export function connect(url: string, scene: World): WebSocket {
  ws = new WebSocket(url);

  ws.onopen = function (_event) {
    console.log("Connected to WebSocket server");
    send("getparams");
  };

  ws.onmessage = function (event) {
    let dict = JSON.parse(event.data);
    console.debug(`received (${dict["key"]})`, dict["dat"]);
    scene.parseMessage(dict["key"], dict["dat"]);
  };

  ws.onerror = function (error) {
    console.error("WebSocket error: ", error);
  };

  return ws;
}

export function send(text: string) {
  console.debug(`sending: ${text}`);
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(text);
  } else {
    console.error("WebSocket is not open.");
  }
}
