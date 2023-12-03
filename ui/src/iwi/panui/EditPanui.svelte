<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { Panui } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@material/mwc-textfield';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalPanuiHash!: ActionHash;

export let currentRecord!: Record;
let currentPanui: Panui = decode((currentRecord.entry as any).Present.entry) as Panui;

let title: string | undefined = currentPanui.title;
let content: string | undefined = currentPanui.content;

let errorSnackbar: Snackbar;

$: title, content;
$: isPanuiValid = true && title !== '' && content !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditPanui element`);
  }
  if (originalPanuiHash === undefined) {
    throw new Error(`The originalPanuiHash input is required for the EditPanui element`);
  }
});

async function updatePanui() {

  const panui: Panui = {
    title: title!,
    content: content!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'iwi',
      zome_name: 'panui',
      fn_name: 'update_panui',
      payload: {
        original_panui_hash: originalPanuiHash,
        previous_panui_hash: currentRecord.signed_action.hashed.hash,
        updated_panui: panui
      }
    });

    dispatch('panui-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the panui: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Panui</span>

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Title" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textfield>
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea>
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button
      raised
      label="Save"
      disabled={!isPanuiValid}
      on:click={() => updatePanui()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
