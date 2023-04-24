<template>
  <div class="card">
    <div class="card-body">
      <h5 class="card-title m-2">{{ title }}</h5>
      <p class="card-text m-2">{{ text }}</p>
      <component :is="contentComponent" :contentProps="contentProps" />
    </div>
  </div>
</template>

<script>
import Home from './Home.vue'
import DeviceList from './DeviceList.vue'
import SystemStats from './SystemStats.vue'
export default {
  name: 'HomeCard',
  components: {
    Home,
    DeviceList,
    SystemStats
  },
  props: ['title', 'text', 'contentComponent'],
  setup(props) {
    return {}
  },
  data() {
    return { contentProps: {} }
  }
}
</script>
