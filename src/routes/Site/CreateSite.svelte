<script lang="ts">
  import { Form, Input, Button } from "spaper";
  import { get } from "svelte/store";
  import { sites } from "../../stores/sites";
  import { db } from "../../stores/db";
  import { nanoid } from "nanoid";
  import { SitesService } from "../../services/data/sites";
  import { CollectionsService } from "../../services/data/collections";
  import { awaitableTimeout } from "../../utils/awaitable-timeout";
  import { push as routerPush } from "svelte-spa-router";

  let siteName: string;

  async function createSite() {
    const database = get(db);

    const siteService = new SitesService(database);
    const collectionService = new CollectionsService(database);

    await siteService.initialized;

    const newSite = {
      site_id: nanoid(),
      site_name: siteName,
    };

    console.log("before create: ", newSite);

    const createResult = await siteService.createSite(newSite);

    const collectionResult = await collectionService.createCollection(
      newSite.site_id,
      {
        collection_id: nanoid(),
        site_id: newSite.site_id,
        collection_name: "Default",
        collection_key: "",
      }
    );

    sites.push(newSite);

    console.log("sites after create: ", await siteService.getSites());

    routerPush(`#/sites/${newSite.site_id}/edit`);
  }
</script>

<h2 class="page-title">Create Site</h2>

<Input
  placeholder="Brad Fitt, Plattholders, AllPacinos, etc..."
  label="Site Name"
  on:input={(event) => {
    siteName = event.detail;
  }}
/>

<div class="row flex-right">
  <Button type="success" on:click={createSite}>Continue -></Button>
</div>
