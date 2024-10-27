import "./style.css";
import { setupGui } from "./gui.ts";
import * as render from "./render3d.ts";
import Scene from "./Scene.ts";
import PlushieRenderer from "./PlushieRenderer.ts";

const display3d = render.init();
setupGui(display3d);
const plushie = new PlushieRenderer();

new Scene("ws://127.0.0.1:8080", plushie);
