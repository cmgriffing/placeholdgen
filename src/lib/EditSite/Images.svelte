<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
  import { Button, Collapsible, Input, Modal } from "spaper";
  import { get } from "svelte/store";

  import type { Site } from "../../types/Site";
  import type { SiteImage } from "../../types/SiteImage";
  import type { Collection } from "../../types/Collection";

  import { db } from "../../stores/db";
  import { nanoid } from "../../utils/nanoid";
  import { Commands } from "../../app-types";
  import { ImagesService } from "../../services/data/images";
  import { resourceDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import { event } from "@tauri-apps/api";

  interface OpenedCollections {
    [key: string]: boolean;
  }

  const openedCollections: OpenedCollections = {};

  let imagesService = new ImagesService(get(db));
  let root = "";
  let showingRemoteImageModal = false;
  let showingSearchImageModal = false;
  let remoteImageUrl = "";
  let currentRemoteImageCollection = "";

  export let site: Site;
  export let collections: Collection[] = [];
  export let collectionImagesMap: Record<string, SiteImage[]> = {};

  onMount(async () => {
    console.log("collections in Images: ", collections);
    root = await resourceDir();
  });

  function addLocalImage(collectionId) {
    return async function (event: MouseEvent) {
      await imagesService.initialized;
      // Open a selection dialog for image files
      let selected = await open({
        multiple: true,
        filters: [
          {
            name: "Image",
            extensions: ["png", "jpeg", "jpg"],
          },
        ],
      });
      if (selected === null) {
        // user cancelled the selection
        console.log("canceled image selection");
        return;
      }
      if (!Array.isArray(selected)) {
        // user selected one file
        console.log("Single selected ", selected);
        selected = [selected];
      }
      for (let i = 0; i < selected.length; i++) {
        const newImage = {
          image_id: nanoid(),
          uploaded: true,
          original_file_name: selected[i],
          collection_id: collectionId,
          image_type: "png",
          source_url: "",
        };

        await imagesService.createImage(newImage);

        await invoke(Commands.AddLocalImage, {
          localImagePath: selected[i],
          collectionId,
          imageId: newImage.image_id,
          siteId: site?.site_id || "",
        });
      }
    };
  }

  async function addRemoteImage() {
    const newImage: SiteImage = {
      image_id: nanoid(),
      uploaded: false,
      source_url: remoteImageUrl,
      collection_id: currentRemoteImageCollection,
      image_type: "png",
      original_file_name: "",
    };

    const fetchResult = await invoke(Commands.AddRemoteImage, {
      remoteImage: newImage,
    });

    if (fetchResult) {
      const createResult = await imagesService.createImage(newImage);
      console.log({ createResult });

      showingRemoteImageModal = false;
    } else {
      // show error?
    }
  }
</script>

<div class="collections hide-checkboxes">
  {#each collections as collection}
    <Collapsible
      open={collections.length === 1 ||
        openedCollections[collection.collection_id]}
    >
      {#each collectionImagesMap[collection.collection_id] || [] as image}
        <img
          alt="image thing"
          class="collection-image"
          src={convertFileSrc(
            `${root}/collections/${image.collection_id}/${image.image_id}.png`
          )}
        />
      {/each}

      <div slot="trigger">
        {collection.collection_name}

        <Button
          size="small"
          type="success"
          on:click={addLocalImage(collection.collection_id)}
        >
          Add Local Image(s)
        </Button>

        <Button
          size="small"
          type="success"
          on:click={() => {
            showingRemoteImageModal = true;
            currentRemoteImageCollection = collection.collection_id;
          }}
        >
          Add Remote Image(s)
        </Button>
      </div>
    </Collapsible>
  {/each}

  <Modal
    bind:active={showingRemoteImageModal}
    title="Remote Image"
    class="modal-no-footer"
  >
    <form
      on:submit={(event) => {
        console.log("submitting the thing", event);
      }}
    >
      <label>
        Remote Image URL:
        <Input
          name="remote-image-url"
          id="remote-image-url"
          value={remoteImageUrl}
          on:input={(event) => {
            remoteImageUrl = event.detail;
          }}
        />
      </label>
      <div>
        <Button nativeType="button" block on:click={addRemoteImage}
          >Import</Button
        >
      </div>
    </form>
  </Modal>

  <Modal
    bind:active={showingSearchImageModal}
    title="Search Image"
    class="modal-no-footer"
  >
    <form
      on:submit={(event) => {
        console.log("submitting the thing", event);
      }}
    >
      <label>
        Remote Image URL:
        <Input
          name="remote-image-url"
          id="remote-image-url"
          value={remoteImageUrl}
          on:input={(event) => {
            remoteImageUrl = event.detail;
          }}
        />
      </label>
      <div>
        <Button nativeType="button" block on:click={addRemoteImage}
          >Import</Button
        >
      </div>
    </form>
  </Modal>
</div>

<style>
  /* input[type="checkbox"] {
    display: none !important;
  } */

  .collection-image {
    width: 100px;
    height: auto;
  }

  .modal-body footer {
    display: none;
  }
</style>
