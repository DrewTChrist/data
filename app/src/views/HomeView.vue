<script setup>
import HomeCard from '../components/HomeCard.vue'
</script>

<template>
  <main>
    <div class="container">
      <div class="row row-cols-1">
        <div class="col m-2">
          <HomeCard title="Devices" text="Connected devices:" contentComponent="DeviceList" />
        </div>
        <div class="col m-2">
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
