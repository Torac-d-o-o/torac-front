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
      <div class="bg-primary-400 shadow-lg rounded-lg p-6 border border-gray-200 h-full w-full">
        <h2 class="flex justify-center text-3xl font-semibold mb-6">Mixer {mixer.id}</h2>
        <div class="grid grid-cols-1 bg-secondary-400">
          <div class="border-b-2 grid grid-cols-2 place-items-start py-2">
            <p class="pl-2 text-xl"><strong>Customer:</strong></p>
            <p class="text-xl">Ivan Ivanovic</p>
          </div>
          <div class="border-b-2 grid grid-cols-2 py-2 place-items-start">
            <p class="pl-2 text-xl"><strong>Order ID:</strong></p>
            <p class="text-lg">13</p>
          </div>
          <div class="border-b-2 grid grid-cols-2 py-2 place-items-start">
            <p class="pl-2 text-xl"><strong>Order olive weight:</strong></p>
            <p class="text-lg">750 kg</p>
          </div>
          <div class="border-b-2 grid grid-cols-2 py-2 place-items-start">
            <p class="pl-2 text-xl"><strong>Time spent mixing:</strong></p>
            <p class="text-lg">10 minutes</p>
          </div>
          <div class="border-b-2 grid grid-cols-2 py-2 place-items-start">
            <p class="pl-2 text-xl"><strong>Current Status:</strong></p>
            <p class="text-lg">IN_PRODUCTION</p>
          </div>
        </div>
        <button class="mt-6 btn btn-primary text-xl bg-tertiary-500">More Details</button>
      </div>
    {/each}
  </div>
  