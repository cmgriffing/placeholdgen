import type Database from "tauri-plugin-sqlite-api";

import { writable } from "svelte/store";

function createDb() {
  const { subscribe, set, update } = writable<Database>();

  return {
    subscribe,
    setDatabase: (database: Database) => set(database),
  };
}

export const db = createDb();
