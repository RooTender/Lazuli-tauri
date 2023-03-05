<script lang="ts">
  import Tabs from "$lib/tabs/Tabs.svelte";
  import Tab from "$lib/tabs/Tab.svelte";
  import TabContent from "$lib/tabs/TabContent.svelte"
  import { setContext } from "svelte";
  import { writable, type Writable } from "svelte/store"

  export let activate: string = "appointments";
  export let activeTab: Writable<string> = writable(activate);

  setContext('selectedTab', activeTab);

  import Appointments from "$lib/Appointments.svelte";
	import Datatable from "$lib/Datatable.svelte";
</script>

<Tabs>
  <Tab title="appointments" />
  <Tab title="patients" />
</Tabs>
<TabContent activateOn="appointments"><Appointments /></TabContent>
<TabContent activateOn="patients"><Datatable /></TabContent>
