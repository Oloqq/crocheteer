import "./style.css";
import { setupGui } from "./gui.ts";
import * as render from "./render3d.ts";
import World from "./World.ts";

const display3d = render.init();
setupGui(display3d);
new World("ws://127.0.0.1:8080", display3d);
