<script lang="ts">
  import { Form, Input, Button } from "spaper";
  import { get } from "svelte/store";
  import { dialog } from "@tauri-apps/api";
  import { sites } from "../../stores/sites";
  import { nanoid } from "nanoid";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Site } from "../../types/Site";
  import { Commands } from "../../app-types";
  import type { AppState } from "../../types/AppState";

  let siteName: string;
  let localPath: string;

  async function createSite() {
    sites.push({
      id: nanoid(),
      name: siteName,
      collections: [],
    });

    const appState: AppState = {
      sites: get(sites),
    };

    const result = await invoke(Commands.SaveAppState, { appState });

    console.log({ result });
  }
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
  <Button type="success" on:click={createSite}>Continue -></Button>
</div>
