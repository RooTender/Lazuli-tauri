<script lang="ts">
  import Tabs from "$lib/tabs/Tabs.svelte";
  import Tab from "$lib/tabs/Tab.svelte";
  import TabContent from "$lib/tabs/TabContent.svelte"
  import { setContext } from "svelte";
  import { writable, type Writable } from "svelte/store"

  export let activate: string = "appointments";
  export let activeTab: Writable<string> = writable(activate);

  setContext('selectedTab', activeTab);

	import Datatable from "$lib/Datatable.svelte";

  const tabs = ["appointments", "patients"];
</script>

<Tabs>
  {#each tabs as tab}
  <Tab title="{tab}" />
  {/each}
</Tabs>

{#each tabs as tab}
<TabContent activateOn="{tab}"><Datatable content="{tab}" /></TabContent>
{/each}