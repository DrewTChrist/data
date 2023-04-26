<script setup>
import HomeCard from '../components/HomeCard.vue'
</script>

<template>
  <main>
    <div class="container">
      <div class="row row-cols-1 m-2">
        <div class="col mb-4">
          <HomeCard title="Devices" text="Connected devices:" contentComponent="DeviceList" />
        </div>
        <div class="col">
          <HomeCard title="Statistics" text="System statistics:" contentComponent="SystemStats" />
        </div>
      </div>
    </div>
  </main>
</template>

<script>
export default {
  created() {},
  data() {
    return {
      contentProps: {
        stats: {
          cpuLoad: {},
          memoryUsage: {},
          uptime: ''
        },
        devices: ['No devices']
      }
    }
  }
}
</script>
