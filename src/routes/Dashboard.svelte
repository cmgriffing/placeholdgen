<script lang="ts">
  import { Button, Card } from "spaper";
  import { sites } from "../stores/sites";
</script>

<h2 class="page-title">Dashboard</h2>

{#each $sites as site}
  <Card title={site.name} image="https://www.fillmurray.com/400/200">
    Card Content
    <p slot="bottom">
      <Button isLink href={`#/sites/${site.id}/edit`}>Edit</Button>
      <Button>Export</Button>
    </p>
  </Card>
{:else}
  <h4>No Sites added yet.</h4>
  <Button isLink href="#/sites/create">Create Site</Button>
{/each}
