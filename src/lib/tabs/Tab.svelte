<script lang="ts">
    import { getContext } from "svelte";
    import  type { Writable } from "svelte/store";

    let activeTab: Writable<string> = getContext('selectedTab');
    export let title: string;

    function Capitalize(str: string) {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }
</script>

<li class="mr-2">
    {#if $activeTab === title}
    <!-- svelte-ignore a11y-missing-attribute -->
    <a
        class="inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500"
        aria-current="page">{Capitalize(title)}</a>
    {:else}
    <!-- svelte-ignore a11y-missing-attribute -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <a
        class="inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300"
        on:click={() => ($activeTab = title)}>{Capitalize(title)}</a>
    {/if}
</li>