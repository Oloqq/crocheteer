declare global {
  interface MonacoEnvironment {
    getWorker(_: string, label: string): Worker;
  }

  var MonacoEnvironment: MonacoEnvironment;
}

export {};
