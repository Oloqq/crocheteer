import "./style.css";
import { setupGui } from "./gui.ts";
import * as render from "./render3d.ts";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
  </div>
`;

setupGui();
render.initScene();
