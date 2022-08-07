<script lang="ts">
  import "papercss/dist/paper.min.css";
  import { sites } from "./stores/sites";

  import Router from "svelte-spa-router";

  import Layout from "./lib/Layout.svelte";

  import NotFound from "./routes/NotFound.svelte";
  import Dashboard from "./routes/Dashboard.svelte";
  import Settings from "./routes/Settings.svelte";
  import CreateSite from "./routes/Site/CreateSite.svelte";
  import DeleteSite from "./routes/Site/DeleteSite.svelte";
  import EditSite from "./routes/Site/EditSite.svelte";

  const routes = {
    "/": Dashboard,
    "/settings": Settings,
    "/sites/create": CreateSite,
    "/sites/:siteId/edit": EditSite,
    "/sites/:siteId/edit/:activeTab": EditSite,
    "/sites/:siteId/delete": DeleteSite,
    "*": NotFound,
  };
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
