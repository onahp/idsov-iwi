<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Panui } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let title: string = '';
let content: string = '';

let errorSnackbar: Snackbar;

$: title, content;
$: isPanuiValid = true && title !== '' && content !== '';

onMount(() => {
});

async function createPanui() {  
  const panuiEntry: Panui = { 
    title: title!,
    content: content!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'create_panui',
      payload: panuiEntry,
    });
    dispatch('panui-created', { panuiHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the panui: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Panui</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Title" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create Panui"
    disabled={!isPanuiValid}
    on:click={() => createPanui()}
  ></mwc-button>
</div>
