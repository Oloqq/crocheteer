let ws = undefined;
let opened = false;

export function controlViaWebsocket(address, world) {
  ws = new WebSocket(address);

  ws.onopen = function (event) {
    console.log("Connected to WebSocket server");
    opened = true;
  };

  ws.onmessage = function (event) {
    let dict = JSON.parse(event.data);
    world.parseMessage(dict["key"], dict["dat"]);
  };

  ws.onerror = function (error) {
    console.error("WebSocket error: ", error);
  };
}

export function send(text) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(text);
  } else {
    console.error('WebSocket is not open.');
  }
}

export function isOpened() {
  return opened;
}