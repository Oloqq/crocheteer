import "./style.css";
import { setupGui } from "./gui.ts";
import * as render from "./render3d.ts";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
  </div>
`;

const display3d = render.init();
setupGui(display3d);
