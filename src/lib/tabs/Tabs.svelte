<script lang="ts">
    import { setContext } from "svelte";
    import { writable, type Writable } from "svelte/store"

    export let activate: string;
    export let activeTab: Writable<string> = writable(activate);

    setContext('selectedTab', activeTab);
</script>

<div class="text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
    <ul class="flex flex-wrap -mb-px">
        <slot />
    </ul>
</div>