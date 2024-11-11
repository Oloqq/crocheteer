import * as monaco from "monaco-editor";
import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import TypeScriptWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import defaultPattern from "./demoPattern.txt?raw";

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

export function init(
  editorContainer: HTMLElement
): monaco.editor.IStandaloneCodeEditor {
  const STORAGE_KEY_EDITOR_CONTENT = "editorContent";
  const pattern =
    localStorage.getItem(STORAGE_KEY_EDITOR_CONTENT) ?? defaultPattern;

  const editor = monaco.editor.create(editorContainer, {
    value: pattern,
    language: "plaintext",
    theme: "vs-dark",
    automaticLayout: true,
  });

  window.addEventListener("resize", () => {
    editor.layout();
  });

  setInterval(() => {
    localStorage.setItem(STORAGE_KEY_EDITOR_CONTENT, editor.getValue());
  }, 5000);

  return editor;
}
