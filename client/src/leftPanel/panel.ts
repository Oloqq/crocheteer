import * as monaco from "monaco-editor";
import * as monacoEditor from "./patternEditor";

export function init(): monaco.editor.IStandaloneCodeEditor {
  const leftPanel = document.getElementById("left-panel") as HTMLElement;
  const resizer = document.getElementById("resizer") as HTMLElement;
  const editorContainer = document.getElementById(
    "editor-container"
  ) as HTMLElement;
  const editor = monacoEditor.init(editorContainer);
  initResizing(leftPanel, resizer, editor);
  return editor;
}

function throttle(fn: Function, limit: number) {
  let lastCall = 0;
  return function (this: any, ...args: any[]) {
    const now = new Date().getTime();
    if (now - lastCall >= limit) {
      lastCall = now;
      fn.apply(this, args);
    }
  };
}

function initResizing(
  leftPanel: HTMLElement,
  resizer: HTMLElement,
  editor: any
) {
  const STORAGE_KEY_WIDTH = "leftPanelWidth";
  let isResizing = false;

  const savedWidth = localStorage.getItem(STORAGE_KEY_WIDTH);
  if (savedWidth) {
    leftPanel.style.width = savedWidth;
    resizer.style.left = savedWidth;
  }

  const onMouseMove = throttle((e: MouseEvent) => {
    if (!isResizing) return;
    const app = document.getElementById("app") as HTMLElement;
    const appRect = app.getBoundingClientRect();
    let newWidth = e.clientX - appRect.left;

    const minWidth = 20;
    newWidth = Math.max(minWidth, newWidth);

    leftPanel.style.width = `${newWidth}px`;
    resizer.style.left = `${newWidth}px`;
    editor.layout();
    localStorage.setItem(STORAGE_KEY_WIDTH, `${newWidth}px`);
  }, 16);

  const stopResizing = () => {
    if (isResizing) {
      isResizing = false;
      document.body.style.cursor = "default";
      document.body.style.userSelect = "auto";
    }
  };

  resizer.addEventListener("mousedown", (e) => {
    e.preventDefault();
    isResizing = true;
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";
  });

  document.addEventListener("mousemove", onMouseMove);
  document.addEventListener("mouseup", stopResizing);
}
