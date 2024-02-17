// const ws = new WebSocket("ws://127.0.0.1:8080");

// ws.onopen = function (event) {
//   console.log("Connected to WebSocket server");
// };

// ws.onmessage = function (event) {
//   console.log("Position data received: ", event.data);
//   const s1pos = JSON.parse(event.data);
//   s1.position.x = s1pos[0];
//   s1.position.y = s1pos[1];
//   s1.position.z = s1pos[2];
// };

// ws.onerror = function (error) {
//   console.log("WebSocket error: ", error);
// };