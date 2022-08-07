import { writable } from "svelte/store";
import type { Site } from "../types";

function createSites() {
  const { subscribe, set, update } = writable<Site[]>([]);

  return {
    subscribe,
    push: (site: Site) =>
      update((sites) => {
        sites.push(site);

        // sync to file system?

        return sites;
      }),
    reset: () => set([]),
  };
}

export const sites = createSites();
