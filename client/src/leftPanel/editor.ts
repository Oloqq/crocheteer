import * as monaco from "monaco-editor";
import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import TypeScriptWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

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

export function init(editorContainer: HTMLElement) {
  const editor = monaco.editor.create(editorContainer, {
    value: `// Type your code here`,
    language: "javascript", // or your domain-specific language
    theme: "vs-dark",
    automaticLayout: true, // Automatically adjust layout
  });

  window.addEventListener("resize", () => {
    editor.layout();
  });

  return editor;
}
