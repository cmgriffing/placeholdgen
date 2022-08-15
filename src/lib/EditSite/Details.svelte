<script lang="ts">
  import { Form, Input, Button, Checkbox, Switch, Card } from "spaper";
  import type { Site } from "../../types/Site";
  import type { SiteImage } from "../../types/SiteImage";
  import type { Collection } from "../../types/Collection";

  export let site: Site;
  export let collections: Collection[];
  export let collectionImagesMap: Record<string, SiteImage[]> = {};

  let imageCount = 0;

  $: {
    if (collectionImagesMap) {
      imageCount = Object.values(collectionImagesMap).reduce(
        (count, images) => count + images.length,
        0
      );
    }
  }
</script>

<div class="row">
  <div class="col col-6">
    <Card title="Images" image="https://www.fillmurray.com/400/200">
      <div><span>{imageCount}</span></div>
      <p slot="bottom">
        <Button isLink href={`#/sites/${site?.site_id}/edit/images`}
          >Manage</Button
        >
      </p>
    </Card>
  </div>
  <div class="col col-6">
    <Card title="Collections" image="https://www.fillmurray.com/401/201">
      <div><span>{collections?.length || 0}</span></div>
      <p slot="bottom">
        <Button isLink href={`#/sites/${site?.site_id}/edit/images`}
          >Manage</Button
        >
      </p>
    </Card>
  </div>
</div>
