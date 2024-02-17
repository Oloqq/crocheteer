let ws = undefined;

export function controlViaWebsocket(address, world) {
  ws = new WebSocket(address);

  ws.onopen = function (event) {
    console.log("Connected to WebSocket server");
  };

  ws.onmessage = function (event) {
    console.log("Position data received: ", event.data);
    world.parse(JSON.parse(event.data));
  };

  ws.onerror = function (error) {
    console.log("WebSocket error: ", error);
  };
}

export function send(text) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(text);
  } else {
    console.error('WebSocket is not open.');
  }
}

// export function send(ws, text) {
//   if (ws && ws.readyState === WebSocket.OPEN) {
//     ws.send(text);
//   } else {
//     console.error('WebSocket is not open.');
//   }
// }
