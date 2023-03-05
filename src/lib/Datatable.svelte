<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Grid from "gridjs-svelte"
    import { h } from "gridjs";
    import "gridjs/dist/theme/mermaid.css";
	import { onMount } from "svelte";
  
    const columns: any[] = [];
    const pagination = {
      limit: 10,
      summary: true
    };

    let data: any[] = [];
    let filteredData: Array<any>;
    let grid: any;

    onMount(() => {
        columns.push("Name");
        columns.push("Surname");
        columns.push("Pesel");
        columns.push("Phone");
        columns.push({
            name: 'Action',
            formatter: (_: any, row: any) => {
            return h('button', {
                className: 'py-2 px-4 border rounded-md bg-blue-200 hover:bg-blue-300',
                onClick: () => {
                alert(`Editing "${row.cells[0].data}" "${row.cells[1].data}"`)
                }
            }, 'Edit');
            },
            width: '100px'
        });

        fetch();
    });

    async function fetch() {
        let response: string = await invoke("fetch", { set: "test" });
        data = JSON.parse(response);
        console.log(data);
        console.log(columns);
    }

    function handleSearch(event: Event) {
        let searchQuery = (event.target as HTMLInputElement).value;
  
        filteredData = data.filter((row) => {
            return Object.values(row).some((val: any) => {
                return val.toString().toLowerCase().includes(searchQuery.toLowerCase());
            });
        });
        
        updateSearchBar();
    }

    function updateSearchBar()
    {
        let searchBar = document.getElementById('searchBar');
  
        if (filteredData.length > 0)
        {
            grid.updateConfig({
                data: filteredData
            }).forceRender();

            searchBar?.classList.remove('bg-red-100');
        }
        else 
        {
            searchBar?.classList.add('bg-red-100');
        }
    }
  </script>
  
  <div>
    <div class="flex flex-row space-x-2 mb-4 ">
      <div class="relative w-full">
        <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
          <svg aria-hidden="true" class="w-5 h-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
        </div>
        <input id="searchBar" on:input={handleSearch} type="text" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg block w-full pl-10 p-2.5" placeholder="Search...">
      </div>
      <button class="bg-blue-500 hover:bg-blue-700 hover:transition-all text-white py-2 px-4 rounded-lg">
        New
      </button>
    </div>
    
    <Grid sort {data} {pagination} bind:instance={grid} resizable {columns}/>
  </div>
  