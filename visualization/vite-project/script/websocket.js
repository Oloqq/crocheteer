let ws = undefined;

export function controlViaWebsocket(address, plushie) {
  ws = new WebSocket(address);

  ws.onopen = function (event) {
    console.log("Connected to WebSocket server");
  };

  ws.onmessage = function (event) {
    console.log("Position data received: ", event.data);
    const s1pos = JSON.parse(event.data);
    plushie.s1.position.x = s1pos[0];
    plushie.s1.position.y = s1pos[1];
    plushie.s1.position.z = s1pos[2];
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
