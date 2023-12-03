<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Katoitoi } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let katoitoiHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let katoitoi: Katoitoi | undefined;


let errorSnackbar: Snackbar;
  
$:  error, loading, record, katoitoi;

onMount(async () => {
  if (katoitoiHash === undefined) {
    throw new Error(`The katoitoiHash input is required for the KatoitoiDetail element`);
  }
  await fetchKatoitoi();
});

async function fetchKatoitoi() {
  loading = true;
  error = undefined;
  record = undefined;
  katoitoi = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'get_katoitoi',
      payload: katoitoiHash,
    });
    if (record) {
      katoitoi = decode((record.entry as any).Present.entry) as Katoitoi;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteKatoitoi() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'delete_katoitoi',
      payload: katoitoiHash,
    });
    dispatch('katoitoi-deleted', { katoitoiHash: katoitoiHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the katoitoi: ${e.data.data}`;
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
<span>Error fetching the katoitoi: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteKatoitoi()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Katoitoi:</strong></span>
    <span style="white-space: pre-line">{ katoitoi.katoitoi }</span>
  </div>

</div>
{/if}

