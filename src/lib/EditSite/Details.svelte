<script lang="ts">
  import { Form, Input, Button, Checkbox, Switch, Card } from "spaper";
  import type { Site } from "../../types";

  export let site: Site;
</script>

<div class="row">
  <div class="col col-6">
    <Card title="Images" image="https://www.fillmurray.com/400/200">
      <div><span>42</span></div>
      <p slot="bottom">
        <Button isLink href={`#/sites/${site?.id}/edit/images`}>Manage</Button>
      </p>
    </Card>
  </div>
  <div class="col col-6">
    <Card title="Collections" image="https://www.fillmurray.com/401/201">
      <div><span>1</span></div>
      <p slot="bottom">
        <Button isLink href={`#/sites/${site?.id}/edit/images`}>Manage</Button>
      </p>
    </Card>
  </div>
</div>
