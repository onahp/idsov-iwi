<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Panui } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditPanui from './EditPanui.svelte'; 

const dispatch = createEventDispatcher();

export let panuiHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let panui: Panui | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, panui;

onMount(async () => {
  if (panuiHash === undefined) {
    throw new Error(`The panuiHash input is required for the PanuiDetail element`);
  }
  await fetchPanui();
});

async function fetchPanui() {
  loading = true;
  error = undefined;
  record = undefined;
  panui = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'get_panui',
      payload: panuiHash,
    });
    if (record) {
      panui = decode((record.entry as any).Present.entry) as Panui;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deletePanui() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'delete_panui',
      payload: panuiHash,
    });
    dispatch('panui-deleted', { panuiHash: panuiHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the panui: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the panui: {error.data.data}</span>
{:else if editing}
<EditPanui
  originalPanuiHash={ panuiHash}
  currentRecord={record}
  on:panui-updated={async () => {
    editing = false;
    await fetchPanui()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditPanui>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deletePanui()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Title:</strong></span>
    <span style="white-space: pre-line">{ panui.title }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Content:</strong></span>
    <span style="white-space: pre-line">{ panui.content }</span>
  </div>

</div>
{/if}
