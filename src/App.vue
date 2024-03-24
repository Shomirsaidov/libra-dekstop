<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>

  <div class="flex justify-center">
    <img src="./assets/logo.png" width="180" />
  </div>

  <div class="flex space-x-8 my-4 justify-center">
    <button class="bg-blue-300 rounded-xl p-2">
      <h1>
        <router-link :to="'/qr'" class="text-blue-900 font-black">Scan QR</router-link>
      </h1>
    </button>
    <button class="bg-blue-300 rounded-xl p-2">
      <h1>
        <router-link :to="'/bar'" class="text-blue-900 font-black">Scan Barcode</router-link>
      </h1>
    </button>

    <button class="bg-blue-300 rounded-xl p-2">
      <h1>
        <router-link :to="'/database'" class="text-blue-900 font-black">Database</router-link>
      </h1>
    </button>
  </div>



  <RouterView />
</template>

<style scoped>

</style>
