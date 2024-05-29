<script>
    const mixers = [
      { id: 1, productionRate: 20, temperature: 75, status: 'Running' },
      { id: 2, productionRate: 18, temperature: 70, status: 'Running' },
      { id: 3, productionRate: 22, temperature: 78, status: 'Stopped' },
      { id: 4, productionRate: 19, temperature: 74, status: 'Running' },
      { id: 5, productionRate: 21, temperature: 76, status: 'Maintenance' },
      { id: 6, productionRate: 17, temperature: 73, status: 'Running' },
    ];
  </script>
  
<div class="grid grid-rows-2 md:grid-cols-3 gap-4 p-4 h-full">
{#each mixers as mixer}
    <div class="bg-primary-400 shadow-lg rounded-lg p-4 border border-gray-200 h-full w-full">
    <h2 class="flex justify-center text-3xl font-semibold mb-10">Mixer {mixer.id}</h2>
    <p class="text-xl"><strong class="text-xl">Order ID: </strong>13</p>
    <p class="text-xl"><strong class="text-xl">Order olive weight: </strong>750 kg</p>
    <p class="text-xl"><strong class="text-xl">Customer: </strong>Ivan Ivanovic</p>
    <p class="text-xl"><strong class="text-xl">Time spent mixing: </strong>10 minutes</p>
    <p class="text-xl"><strong class="text-xl">Current Status: </strong>IN_PRODUCTION</p>
    <button class="mt-10 btn btn-primary text-xl bg-tertiary-500">More Details</button>
    </div>
{/each}
</div>