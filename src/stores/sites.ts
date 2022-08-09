import { writable } from "svelte/store";
import type { Site } from "../types/Site";

function createSites() {
  const { subscribe, set, update } = writable<Site[]>([]);

  return {
    subscribe,
    push: (site: Site) =>
      update((sites) => {
        sites.push(site);
        return sites;
      }),
    reset: (sites: Site[] = []) => set([...sites]),
  };
}

export const sites = createSites();
