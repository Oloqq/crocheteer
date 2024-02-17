let ws = undefined;
let tmpworld = undefined;

export function controlViaWebsocket(address, world) {
  ws = new WebSocket(address);
  tmpworld = world;

  ws.onopen = function (event) {
    console.log("Connected to WebSocket server");
  };

  ws.onmessage = function (event) {
    // console.log("Position data received: ", event.data);
    world.parse(JSON.parse(event.data));
  };

  ws.onerror = function (error) {
    console.log("WebSocket error: ", error);
  };
}

export function bruh(obj) {
  let id = tmpworld.getId(obj);
  send(`pos ${id} ${obj.position.x} ${obj.position.y} ${obj.position.z}`);
}

export function send(text) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(text);
  } else {
    // console.error('WebSocket is not open.');
  }
}