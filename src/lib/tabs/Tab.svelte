<script lang="ts">
    import { getContext } from "svelte";

    let activeTab: string = getContext('selectedTab');
    export let title: string;

    function Capitalize(str: string) {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }
</script>

<li class="mr-2">
    {#if activeTab === title}
    <p
        class="inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500"
        aria-current="page">{Capitalize(title)}</p>
    {:else}
    <p
        class="inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300"
        >{Capitalize(title)}</p>
    {/if}
</li>