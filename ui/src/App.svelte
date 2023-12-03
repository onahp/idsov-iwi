<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import PanuiKatoa from './iwi/panui/PanuiKatoa.svelte';
  import CreatePanui from './iwi/panui/CreatePanui.svelte';

  import { clientContext } from './contexts';

  let client: AppAgentClient | undefined;

  let loading = true; 


  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect('', 'iwi');

    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main>
  {#if loading}
<div class="loader-container">
<mwc-circular-progress indeterminate />
    </div>
  {:else}
<div class="content-container">
<CreatePanui></CreatePanui>
<PanuiKatoa></PanuiKatoa>
</div>
  {/if}
</main>

<style>
.loader-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }

  .content-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    margin: 2rem;
  }
main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
