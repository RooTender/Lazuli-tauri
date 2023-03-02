<script lang="ts">
  import Tabs from "$lib/tabs/Tabs.svelte";
  import Tab from "$lib/tabs/Tab.svelte";
</script>

<div class="m-4">
    <Tabs activeTab="patients">
      <Tab title="appointments" />
      <Tab title="patients" />
    </Tabs>
</div>
