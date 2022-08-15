<script lang="ts">
  import "papercss/dist/paper.min.css";
  import { sites } from "./stores/sites";
  import { db } from "./stores/db";

  import Router from "svelte-spa-router";

  import Layout from "./lib/Layout.svelte";

  import NotFound from "./routes/NotFound.svelte";
  import Dashboard from "./routes/Dashboard.svelte";
  import Settings from "./routes/Settings.svelte";
  import CreateSite from "./routes/Site/CreateSite.svelte";
  import DeleteSite from "./routes/Site/DeleteSite.svelte";
  import EditSite from "./routes/Site/EditSite.svelte";
  import { onDestroy, onMount } from "svelte";
  import Database from "tauri-plugin-sqlite-api";
  import { SitesService } from "./services/data/sites";
  import { awaitableTimeout } from "./utils/awaitable-timeout";
  // import Knex from "knex";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Commands } from "./app-types";

  // const knex = Knex({ client: "sqlite" });

  const routes = {
    "/": Dashboard,
    "/settings": Settings,
    "/sites/create": CreateSite,
    "/sites/:siteId/edit": EditSite,
    "/sites/:siteId/edit/:activeTab": EditSite,
    "/sites/:siteId/delete": DeleteSite,
    "*": NotFound,
  };

  onMount(async () => {
    // sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
    const database = await Database.open("./main.db");

    // store db to svelte store
    db.setDatabase(database);

    // dev only
    if (false) {
      console.warn("DEV: DROPPING TABLES");
      const sitesService = new SitesService(database);
      await awaitableTimeout(100);
      await sitesService.dropTables();
    }

    const sitesService = new SitesService(database);
    await awaitableTimeout(100);

    const fetchedSites = await sitesService.getSites();

    sites.reset(fetchedSites);

    const all_images = await invoke(Commands.GetAllImages);
  });
</script>

<main class="main">
  <Layout sites={$sites}>
    <Router {routes} />
  </Layout>
</main>

<style>
  .main {
    height: 100vh;
    width: 100vw;
  }
</style>
