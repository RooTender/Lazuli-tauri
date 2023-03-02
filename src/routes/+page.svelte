<script lang="ts">
  import Tabs from "$lib/tabs/Tabs.svelte";
  import Tab from "$lib/tabs/Tab.svelte";

  import Appointments from "$lib/Appointments.svelte";
</script>

<div class="m-4">
    <Tabs activate="patients">
      <Tab title="appointments"><Appointments /></Tab>
      <Tab title="patients"><!--W.I.P. <Patients />--></Tab>
    </Tabs>
</div>
