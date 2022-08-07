<script lang="ts">
  import { Form, Input, Button } from "spaper";
  import { dialog } from "@tauri-apps/api";
  import { sites } from "../../stores/sites";
  import { nanoid } from "nanoid";

  let siteName: string;
  let localPath: string;
</script>

<h2 class="page-title">Create Site</h2>

<Input
  placeholder="Brad Fitt, Plattholders, AllPacinos, etc..."
  label="Site Name"
  on:input={(event) => {
    siteName = event.detail;
  }}
/>

<Button
  nativeType="button"
  type="secondary"
  on:click={async () => {
    const result = await dialog.open({
      directory: true,
      title: "Select local folder for Site assets",
      multiple: false,
    });

    if (typeof result === "string") {
      localPath = result;
    }
  }}
>
  Select local path
</Button>

<div class="row flex-right">
  <Button
    type="success"
    on:click={() => {
      console.log("testing", { localPath, siteName });
      sites.push({
        id: nanoid(),
        name: siteName,
        localPath,
      });
    }}>Continue -></Button
  >
</div>
