<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Katoitoi } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let panuiHash!: ActionHash;


let katoitoi: string = '';

let errorSnackbar: Snackbar;

$: katoitoi, panuiHash;
$: isKatoitoiValid = true && katoitoi !== '';

onMount(() => {
  if (panuiHash === undefined) {
    throw new Error(`The panuiHash input is required for the CreateKatoitoi element`);
  }
});

async function createKatoitoi() {  
  const katoitoiEntry: Katoitoi = { 
    katoitoi: katoitoi!,
    panui_hash: panuiHash!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'create_katoitoi',
      payload: katoitoiEntry,
    });
    dispatch('katoitoi-created', { katoitoiHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the katoitoi: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Katoitoi</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Katoitoi" value={ katoitoi } on:input={e => { katoitoi = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create Katoitoi"
    disabled={!isKatoitoiValid}
    on:click={() => createKatoitoi()}
  ></mwc-button>
</div>
