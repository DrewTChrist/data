<script setup>
import HomeCard from '../components/HomeCard.vue'
</script>

<template>
  <main>
    <div class="row row-cols-1 row-cols-md-3">
      <div class="col m-4">
        <HomeCard title="Devices" text="List of your devices" mycomponent="Home" />
      </div>
      <div class="col m-4">
        <HomeCard title="Other stuff" text="This is some different text" mycomponent="DeviceList" />
      </div>
      <div class="col m-4">
        <HomeCard title="Statistics" text="Some text" />
      </div>
      <div class="col m-4">
        <HomeCard title="Devices" text="Some text" />
      </div>
    </div>
  </main>
</template>
