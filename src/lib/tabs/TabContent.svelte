<script lang="ts">
    import { getContext } from "svelte";
    import  type { Writable } from "svelte/store";

    export let activateOn: string;

    let activeTab: Writable<string> = getContext('selectedTab');
</script>
{#if $activeTab === activateOn}
<div>
    <slot />
</div>
{/if}