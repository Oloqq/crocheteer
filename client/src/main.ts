import "./style.css";
import { setupGui } from "./gui.ts";
import * as render from "./render3d.ts";

const display3d = render.init();
setupGui(display3d);
