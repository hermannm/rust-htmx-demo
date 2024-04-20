interface HtmxEvents {
  // https://v1.htmx.org/events/#htmx:responseError
  "htmx:responseError": CustomEvent<{ xhr: XMLHttpRequest }>;
}

declare global {
  interface HTMLElement {
    addEventListener<K extends keyof HtmxEvents>(
      type: K,
      listener: (this: Document, ev: HtmxEvents[K]) => void
    ): void;
  }
}

export {}; // Trick to make `declare global` above work
