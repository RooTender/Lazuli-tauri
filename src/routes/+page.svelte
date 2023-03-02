<script lang="ts">
  import Tabs from "$lib/tabs/Tabs.svelte";
  import Tab from "$lib/tabs/Tab.svelte";
</script>

<div class="m-4">
    <Tabs activeTab="patients">
      <Tab title="appointments">TEST 1</Tab>
      <Tab title="patients">Test 2</Tab>
    </Tabs>
</div>
