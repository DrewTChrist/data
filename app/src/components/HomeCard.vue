<template>
  <div class="card">
    <div class="card-body">
      <h5 class="card-title m-2">{{ title }}</h5>
      <p class="card-text m-2">{{ text }}</p>
      <component :is="mycomponent" />
    </div>
  </div>
</template>

<script>
import Home from '../components/Home.vue'
import DeviceList from '../components/DeviceList.vue'
export default {
  name: 'HomeCard',
  components: {
    Home,
    DeviceList
  },
  props: ['title', 'text', 'mycomponent'],
  setup(props) {
    return {}
  },
  data() {
    return {}
  }
}
</script>
