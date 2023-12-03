<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import PanuiDetail from './PanuiDetail.svelte';
import type { PanuiSignal } from './types';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchPanuis();
  client.on('signal', signal => {
    if (signal.zome_name !== 'panui') return;
    const payload = signal.payload as PanuiSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Panui') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchPanuis() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'get_panui_katoa',
      payload: null,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the panuis: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No panuis found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <PanuiDetail panuiHash={hash}  on:panui-deleted={() => fetchPanuis()}></PanuiDetail>
    </div>
  {/each}
</div>
{/if}

