import * as dat from "dat.gui";
import * as comms from "./comms";

export let data = {
  paused: true,
};

export function setupGui() {
  const gui = new dat.GUI();

  gui
    .add(data, "paused")
    .name("Pause")
    .onChange((value) => {
      if (value) {
        comms.send("pause");
      } else {
        comms.send("resume");
      }
    });

  // let counter = 0;
  // const setCounter = (count: number) => {
  //   counter = count;
  //   element.innerHTML = `count is ${counter}`;
  // };
  // element.addEventListener("click", () => setCounter(counter + 1));
  // setCounter(0);
}
