<template>
  <div class="row">
    <div class="row">
      <div class="col-6">
        <div class="list-group" id="list-tab" role="tablist">
          <a
            class="list-group-item list-group-item-action active"
            id="list-home-list"
            data-bs-toggle="list"
            href="#list-home"
            role="tab"
            aria-controls="list-home"
            @click="cpuLoad"
            >Cpu Load</a
          >
          <a
            class="list-group-item list-group-item-action"
            id="list-profile-list"
            data-bs-toggle="list"
            href="#list-profile"
            role="tab"
            aria-controls="list-profile"
            @click="memoryUsage"
            >Memory Usage</a
          >
          <a
            class="list-group-item list-group-item-action"
            id="list-messages-list"
            data-bs-toggle="list"
            href="#list-messages"
            role="tab"
            aria-controls="list-messages"
            @click="socketUsage"
            >Socket Usage</a
          >
          <a
            class="list-group-item list-group-item-action"
            id="list-settings-list"
            data-bs-toggle="list"
            href="#list-settings"
            role="tab"
            aria-controls="list-settings"
            @click="uptime"
            >Uptime</a
          >
        </div>
      </div>
      <div class="col-4">
        <div class="tab-content" id="nav-tabContent">
          <div
            class="tab-pane fade show active"
            id="list-home"
            role="tabpanel"
            aria-labelledby="list-home-list"
          >
            Cpu load: {{ stats.cpuLoad }}
          </div>
          <div
            class="tab-pane fade"
            id="list-profile"
            role="tabpanel"
            aria-labelledby="list-profile-list"
          >
            Memory Usage: {{ stats.memoryUsage }}
          </div>
          <div
            class="tab-pane fade"
            id="list-messages"
            role="tabpanel"
            aria-labelledby="list-messages-list"
          >
            Socket Usage: {{ stats.socketUsage }}
          </div>
          <div
            class="tab-pane fade"
            id="list-settings"
            role="tabpanel"
            aria-labelledby="list-settings-list"
          >
            Uptime: {{ stats.uptime }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
export default {
  name: 'SystemStats',
  components: {},
  props: ['contentProps'],
  setup(props) {
    const stats = ref({})
    function cpuLoad() {
      fetch('http://127.0.0.1:8080/stats/cpu_load', { mode: 'cors', method: 'GET' })
        .then((res) => res.json())
        .then((json) => (this.stats.cpuLoad = json))
        .catch((err) => console.log(err))
    }
    function memoryUsage() {
      fetch('http://127.0.0.1:8080/stats/memory', { mode: 'cors', method: 'GET' })
        .then((res) => res.json())
        .then((json) => (this.stats.memoryUsage = json))
        .catch((err) => console.log(err))
    }
    function socketUsage() {
      fetch('http://127.0.0.1:8080/stats/socket', { mode: 'cors', method: 'GET' })
        .then((res) => res.json())
        .then((json) => (this.stats.socketUsage = json))
        .catch((err) => console.log(err))
    }
    function uptime() {
      fetch('http://127.0.0.1:8080/stats/uptime', { mode: 'cors', method: 'GET' })
        .then((res) => res.json())
        .then((json) => (this.stats.uptime = json))
        .catch((err) => console.log(err))
    }
    return {
      stats,
      cpuLoad,
      memoryUsage,
      socketUsage,
      uptime
    }
  },
  created() {
    fetch('http://127.0.0.1:8080/stats/cpu_load', { mode: 'cors', method: 'GET' })
      .then((res) => res.json())
      .then((json) => (this.stats.cpuLoad = json))
      .catch((err) => console.log(err))
  },
  data() {
    return {}
  }
}
</script>
