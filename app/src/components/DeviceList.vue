<template>
  <table class="table">
    <thead>
      <tr>
        <th scope="col">#</th>
        <th scope="col">Name</th>
        <th scope="col">Protocol</th>
        <th scope="col">Device</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row"></th>
        <td colspan="4">No devices available</td>
      </tr>
    </tbody>
  </table>
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
