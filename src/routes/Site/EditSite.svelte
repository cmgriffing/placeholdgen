<script lang="ts">
  import { Tabs, Tab } from "spaper";
  import type { Site } from "../../types/Site";
  import { sites } from "../../stores/sites";
  import { invoke } from "@tauri-apps/api/tauri";

  import EditSiteDetails from "../../lib/EditSite/Details.svelte";
  import EditSiteImages from "../../lib/EditSite/Images.svelte";
  import EditSiteExport from "../../lib/EditSite/Export.svelte";

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
  let activeTabIndex = 0;

  $: {
    if (params.siteId) {
      siteId = params?.siteId;
      site = $sites.find((_site) => _site.id === params?.siteId);
    }
    if (params.activeTab) {
      Object.values(TabSection).forEach((tabSectionValue, index) => {
        if (tabSectionValue === params.activeTab) {
          activeTabIndex = index;
        }
      });
    }
  }
</script>

<h2 class="page-title">Edit Site: {site?.name}</h2>

<Tabs activeTab={activeTabIndex} class="no-content-padding">
  <Tab label="Details">
    <EditSiteDetails {site} />
  </Tab>
  <Tab label="Images">
    <EditSiteImages />
  </Tab>
  <Tab label="Export">
    <EditSiteExport />
  </Tab>
</Tabs>

<style>
  .tabs .content {
    padding: 0 !important;
  }
</style>
