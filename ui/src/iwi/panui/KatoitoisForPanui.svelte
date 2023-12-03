
<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { AppAgentClient, Record, ActionHash, EntryHash, AgentPubKey, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Katoitoi } from './types';
import KatoitoiDetail from './KatoitoiDetail.svelte';

export let panuiHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (panuiHash === undefined) {
    throw new Error(`The panuiHash input is required for the KatoitoisForPanui element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'get_katoitois_for_panui',
      payload: panuiHash
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching katoitois: ${error.data.data}.</span>
{:else if hashes.length === 0}
<span>No katoitois found for this panui.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <KatoitoiDetail katoitoiHash={hash}></KatoitoiDetail>
    </div>
  {/each}
</div>
{/if}
