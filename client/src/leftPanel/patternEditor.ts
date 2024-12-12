import * as monaco from "monaco-editor";
import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import TypeScriptWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import defaultPattern from "./demoPattern.txt?raw";
import theme from "monaco-themes/themes/Pastels on Dark.json";
monaco.editor.defineTheme("usedCustomTheme", theme as any);

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

  // monaco.editor.defineTheme("customTheme2", parseTmTheme);

  defineLanguage();

  const editor = monaco.editor.create(editorContainer, {
    value: pattern,
    language: "amigurumiCrochetLanguage",
    theme: "usedCustomTheme",
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

function defineLanguage() {
  monaco.languages.register({ id: "amigurumiCrochetLanguage" });
  monaco.languages.setMonarchTokensProvider("amigurumiCrochetLanguage", {
    defaultToken: "invalid",

    // Define the tokenization rules
    tokenizer: {
      root: [
        // Comments
        [/#[^\n]*/, "comment"],

        // Keywords
        [
          /\b(MR|FO|mark|goto|FLO|BLO|BL|ch|Ch|color|sc|inc|dec|attach)\b/,
          "keyword",
        ],

        // Special Symbols (e.g., R)
        [/\bR\b/, "symbol"],

        // Numbers
        [/\b\d+\b/, "number"],

        // Identifiers
        [/\b[A-Za-z_][A-Za-z0-9_]*\b/, "identifier"],

        // Operators and Delimiters
        [/:|,|\[|\]|\(|\)/, "delimiter"],

        // Brackets
        [/{|}/, "bracket"],

        // Whitespace
        { include: "@whitespace" },
      ],

      // Whitespace handling
      whitespace: [[/[ \t\r\n]+/, "white"]],
    },
  });

  monaco.languages.setLanguageConfiguration("amigurumiCrochetLanguage", {
    comments: {
      lineComment: "#",
      // Assuming no block comments; if there are, define them here
    },
    brackets: [
      ["{", "}"],
      ["[", "]"],
      ["(", ")"],
    ],
    autoClosingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"', notIn: ["string"] },
      { open: "'", close: "'", notIn: ["string", "comment"] },
    ],
    surroundingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
  });

  // Apply the custom theme
  monaco.editor.setTheme("customTheme");
}
