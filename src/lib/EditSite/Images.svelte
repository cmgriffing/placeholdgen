<script lang="ts">
  import type { Collection } from "../../types/Collection";
  import { Button, Collapsible } from "spaper";
  import { nanoid } from "../../utils/nanoid";
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Commands } from "../../app-types";
  import type { Site } from "../../types/Site";

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

  export let site: Site;
</script>

<div class="collections hide-checkboxes">
  {#each collections as collection}
    <Collapsible
      open={collections.length === 1 ||
        openedCollections[collection.collection_id]}
    >
      Content
      <div slot="trigger">
        Testing

        <Button
          size="small"
          type="success"
          on:click={async (event) => {
            // Open a selection dialog for image files
            const selected = await open({
              multiple: true,
              filters: [
                {
                  name: "Image",
                  extensions: ["png", "jpeg"],
                },
              ],
            });
            if (Array.isArray(selected)) {
              // user selected multiple files
              console.log("Multiple selected ", selected);

              for (let i = 0; i < selected.length; i++) {
                console.log("collection_id", collection.collection_id);
                console.log("HMMMMMM", {
                  localImagePath: selected[i],
                  collectionId: collection.collection_id,
                  imageId: nanoid(),
                  siteId: site?.site_id || "",
                });
                // await invoke(Commands.AddLocalImage, {
                //   localImagePath: selected[i],
                //   collectionId: collection.collection_id,
                //   imageId: nanoid(),
                //   siteId: site?.site_id || "",
                // });
              }
            } else if (selected === null) {
              // user cancelled the selection
              console.log("canceled ");
            } else {
              // user selected a single file
              console.log("Single selected ", selected);
            }
          }}
        >
          Add Local Image(s)
        </Button>

        <Button size="small" type="success" on:click={() => {}}>
          Add Remote Image(s)
        </Button>
      </div>
    </Collapsible>
  {/each}
</div>

<style>
  /* input[type="checkbox"] {
    display: none !important;
  } */
</style>
