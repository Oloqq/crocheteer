import World from "./World";

let ws: WebSocket | undefined = undefined;

export function connect(url: string, scene: World, onconnect: any): WebSocket {
  ws = new WebSocket(url);

  ws.onopen = function (_event) {
    console.log("Connected to WebSocket server");
    // send("getparams");
    if (onconnect) {
      onconnect();
    }

    (window as any).mockReceive = (x: any) => {
      scene.parseMessage("change-centroids", x);
    };
  };

  ws.onmessage = function (event) {
    let dict = JSON.parse(event.data);
    console.debug(`received (${dict["key"]})`, dict["data"]);
    scene.parseMessage(dict["key"], dict["data"]);
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
