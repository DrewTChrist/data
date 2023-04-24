<template>
  <ul class="list-group">
    <li v-for="device in contentProps.devices" class="list-group-item">{{ device }}</li>
    <!--<li class="list-group-item">A second item</li>
    <li class="list-group-item">A third item</li>
    <li class="list-group-item">A fourth item</li>
    <li class="list-group-item">And a fifth one</li>-->
  </ul>
</template>

<script>
export default {
  name: 'DeviceList',
  components: {},
  props: ['contentProps'],
  setup(props) {
    return {}
  },
  data() {
    return {}
  }
}
</script>
