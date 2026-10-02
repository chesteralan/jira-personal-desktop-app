import "@testing-library/jest-dom/vitest";

const localStorageStore = new Map<string, string>();
Object.defineProperty(window, "localStorage", {
  value: {
    getItem: (key: string): string | null => localStorageStore.get(key) ?? null,
    setItem: (key: string, value: string): void => {
      localStorageStore.set(key, value);
    },
    removeItem: (key: string): void => {
      localStorageStore.delete(key);
    },
    clear: (): void => {
      localStorageStore.clear();
    },
    get length(): number {
      return localStorageStore.size;
    },
    key: (index: number): string | null => {
      const keys = [...localStorageStore.keys()];
      return keys[index] ?? null;
    },
  },
  writable: true,
});

beforeEach(() => {
  localStorageStore.clear();
});

Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: (query: string): MediaQueryList => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: (): void => undefined,
    removeListener: (): void => undefined,
    addEventListener: (): void => undefined,
    removeEventListener: (): void => undefined,
    dispatchEvent: (): boolean => false,
  }),
});
