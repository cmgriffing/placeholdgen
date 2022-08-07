<script lang="ts">
  import type { Collection } from "../../types";
  import { Collapsible } from "spaper";
  import { nanoid } from "nanoid";

  interface OpenedCollections {
    [key: string]: boolean;
  }

  const collections: Collection[] = [
    {
      name: "FOOOOOOOO",
      images: [],
      collectionId: nanoid(),
    },
  ];

  const openedCollections: OpenedCollections = {};
</script>

<div class="collections hide-checkboxes">
  {#each collections as collection}
    <Collapsible open={openedCollections[collection.collectionId]}>
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
