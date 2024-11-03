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
  const STORAGE_KEY_EDITOR_CONTENT = "editorContent";
  const savedContent = localStorage.getItem(STORAGE_KEY_EDITOR_CONTENT);

  const editor = monaco.editor.create(editorContainer, {
    value: savedContent ?? "# Type your pattern here",
    language: "plaintext",
    theme: "vs-dark",
    automaticLayout: true,
  });

  if (!savedContent) {
    laodDefaultContent(editor);
  }

  window.addEventListener("resize", () => {
    editor.layout();
  });

  setInterval(() => {
    localStorage.setItem(STORAGE_KEY_EDITOR_CONTENT, editor.getValue());
  }, 5000);

  return editor;
}

async function laodDefaultContent(editor: any) {
  const fileContent = await fetch("src/leftPanel/demoPattern.txt");
  const text = await fileContent.text();
  editor.setValue(text);
}
