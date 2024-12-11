declare module "monaco-themes" {
  export function parseTmTheme(rawTmThemeString: string): any;

  const defaultExport: {
    parseTmTheme: (rawTmThemeString: string) => any;
  };

  export default defaultExport;
}
