import { GuiData, setupGui } from "./gui.ts";
import * as render from "./render3d.ts";
import World from "./World.ts";
import * as monaco from "monaco-editor";

import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import TypeScriptWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

// Define MonacoEnvironment
self.MonacoEnvironment = {
  getWorker(_: string, label: string) {
    switch (label) {
      case "json":
        return new JsonWorker();
      case "typescript":
      case "javascript":
        return new TypeScriptWorker();
      default:
        return new EditorWorker();
    }
  },
};

setupSimulator();
// toggleButton();
// editor();
resizing();

function resizing() {
  document.addEventListener("DOMContentLoaded", () => {
    const toggleButton = document.getElementById(
      "toggle-panel"
    ) as HTMLButtonElement;
    const leftPanel = document.getElementById("left-panel") as HTMLElement;
    const resizer = document.getElementById("resizer") as HTMLElement;
    const editorContainer = document.getElementById(
      "editor-container"
    ) as HTMLElement;

    // Initialize Monaco Editor
    const editor = monaco.editor.create(editorContainer, {
      value: `// Type your code here`,
      language: "javascript", // or your domain-specific language
      theme: "vs-dark",
      automaticLayout: true, // Automatically adjust layout
    });

    // Toggle Panel Visibility
    toggleButton.addEventListener("click", () => {
      leftPanel.classList.toggle("hidden");
      if (leftPanel.classList.contains("hidden")) {
        // Optionally, hide the resizer when panel is hidden
        resizer.style.display = "none";
      } else {
        resizer.style.display = "block";
        editor.layout();
      }
    });

    // Drag to Resize Functionality
    let isResizing = false;

    const onMouseMove = (e: MouseEvent) => {
      if (!isResizing) return;
      const app = document.getElementById("app") as HTMLElement;
      const appRect = app.getBoundingClientRect();
      let newWidth = e.clientX - appRect.left;

      // Set minimum and maximum widths
      const minWidth = 200; // Minimum width of the left panel
      const maxWidth = 500; // Maximum width of the left panel

      newWidth = Math.max(minWidth, Math.min(newWidth, maxWidth));

      leftPanel.style.width = `${newWidth}px`;
      resizer.style.left = `${newWidth}px`; // Move resizer accordingly
      // editor.layout(); // Optional: Adjust editor if necessary
    };

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

    // Handle Window Resize
    window.addEventListener("resize", () => {
      // editor.layout();
    });
  });
}

function toggleButton() {
  const toggleButton = document.getElementById("toggle-panel")!;
  const leftPanel = document.getElementById("left-panel")!;

  toggleButton.addEventListener("click", () => {
    leftPanel.classList.toggle("hidden");
  });
}

function editor() {
  const leftPanel = document.getElementById("left-panel")!;
  const editorContainer = document.getElementById("editor-container")!;

  const editor = monaco.editor.create(editorContainer, {
    value: `// Type your code here`,
    language: "javascript", // or your domain-specific language
    theme: "vs-dark",
  });

  // Handle window resize
  window.addEventListener("resize", () => {
    editor.layout();
  });

  // Adjust editor layout when the panel is resized
  const resizeObserver = new ResizeObserver(() => {
    editor.layout();
  });
  resizeObserver.observe(leftPanel);

  // monaco.languages.register({ id: "myDSL" });

  // monaco.languages.setMonarchTokensProvider("myDSL", {
  //   // Define your language tokens here
  // });

  // editor.updateOptions({ language: "myDSL" });
}

function setupSimulator() {
  const display3d = render.init();
  let guiData = new GuiData();
  let world = new World("ws://127.0.0.1:8080", display3d, guiData);
  setupGui(display3d, guiData, world);
}
