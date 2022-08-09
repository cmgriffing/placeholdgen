<script lang="ts">
  import type { Collection } from "../../types/Collection";
  import { Collapsible } from "spaper";
  import { nanoid } from "nanoid";

  interface OpenedCollections {
    [key: string]: boolean;
  }

  const collections: Collection[] = [
    {
      name: "FOOOOOOOO",
      images: [],
      collection_id: nanoid(),
    },
  ];

  const openedCollections: OpenedCollections = {};
</script>

<div class="collections hide-checkboxes">
  {#each collections as collection}
    <Collapsible open={openedCollections[collection.collection_id]}>
      Content
      <div slot="trigger">Testing</div>
    </Collapsible>
  {/each}
</div>

<style>
  /* input[type="checkbox"] {
    display: none !important;
  } */
</style>
