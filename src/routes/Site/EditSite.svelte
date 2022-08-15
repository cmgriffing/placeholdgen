<script lang="ts">
  import { onMount } from "svelte";
  import { Tabs, Tab } from "spaper";
  import { invoke } from "@tauri-apps/api/tauri";
  import { get } from "svelte/store";

  import type { Site } from "../../types/Site";
  import type { Collection } from "../../types/Collection";
  import type { SiteImage } from "../../types/SiteImage";

  import EditSiteDetails from "../../lib/EditSite/Details.svelte";
  import EditSiteImages from "../../lib/EditSite/Images.svelte";
  import EditSiteExport from "../../lib/EditSite/Export.svelte";
  import { CollectionsService } from "../../services/data/collections";
  import { ImagesService } from "../../services/data/images";
  import { SitesService } from "../../services/data/sites";

  import { sites } from "../../stores/sites";
  import { db } from "../../stores/db";

  enum TabSection {
    Default = "default",
    Images = "images",
    Review = "review",
  }
  interface PageParams {
    siteId: string;
    activeTab: TabSection;
  }
  export let params: PageParams;
  let siteId = "";
  let site: Site;
  let collections: Collection[];
  let images: SiteImage[];
  let activeTabIndex = 0;

  let database = get(db);
  let collectionsService = new CollectionsService(database);
  let imagesService = new ImagesService(database);
  let sitesService = new SitesService(database);
  let collectionImagesMap: Record<string, SiteImage[]> = {};

  onMount(async () => {
    if (params.siteId) {
      siteId = params?.siteId;

      site = await sitesService.getSite(siteId);
      collections = await collectionsService.getCollections(siteId);

      console.log("collections in onMount", siteId, collections);

      await Promise.all(
        collections.map(async ({ collection_id }) => {
          collectionImagesMap[collection_id] =
            await imagesService.getCollectionImages(collection_id);
        })
      );
    }
  });

  $: {
    if (params.activeTab) {
      Object.values(TabSection).forEach((tabSectionValue, index) => {
        if (tabSectionValue === params.activeTab) {
          activeTabIndex = index;
        }
      });
    }
  }
</script>

<h2 class="page-title">Edit Site: {site?.site_name}</h2>

<Tabs activeTab={activeTabIndex} class="no-content-padding">
  <Tab label="Details">
    <EditSiteDetails {site} {collections} {collectionImagesMap} />
  </Tab>
  <Tab label="Images">
    <EditSiteImages {site} {collections} {collectionImagesMap} />
  </Tab>
  <Tab label="Export">
    <EditSiteExport {site} />
  </Tab>
</Tabs>

<style>
  .tabs .content {
    padding: 0 !important;
  }
</style>
