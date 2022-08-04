<script lang="ts">
  import Router from "svelte-spa-router";

  import Dashboard from "./routes/Dashboard.svelte";
  import Settings from "./routes/Settings.svelte";
  import CreateSite from "./routes/Site/CreateSite.svelte";
  import DeleteSite from "./routes/Site/DeleteSite.svelte";
  import EditSite from "./routes/Site/EditSite.svelte";

  const routes = {
    "/": Dashboard,
    "/settings": Settings,
    "/site/create": CreateSite,
    "/site/:siteId/edit": EditSite,
    "/site/:siteId/delete": DeleteSite,
  };
</script>

<main>
  <Router {routes} />
</main>

<style>
</style>
