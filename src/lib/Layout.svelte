<script lang="ts">
  import { Alert, Button } from "spaper";
  import type { Site } from "../types/Site";
  export let sites: Site[] = [];
</script>

<div class="row">
  <div class="col-3 col">
    <img
      class="app-logo"
      alt="app logo"
      src="https://www.fillmurray.com/250/250"
    />

    <Alert type="secondary">Menu</Alert>
    <div>
      <ul>
        <li><a href="#/">Dashboard</a></li>
        <li><a href="#/settings">Settings</a></li>
      </ul>
    </div>
    <div>
      <Alert type="secondary">Sites</Alert>
      <ul>
        {#each sites as site}
          <li>
            <a href={`#/sites/${site.id}/edit`}>{site.name}</a>
          </li>
        {/each}
      </ul>
      {#if !sites.length}
        <div class="no-sites-found">No Sites found.</div>
      {/if}

      <Button type="success" isLink href="#/sites/create" block>
        Create Site ->
      </Button>
    </div>
  </div>
  <div class="col-9 col">
    <slot />
  </div>
</div>

<style>
  .app-logo {
    margin-bottom: 1rem;
  }

  .no-sites-found {
    text-align: center;
  }
</style>
