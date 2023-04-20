<script setup></script>

<template>
  <Bar :data="data" :options="options" />
  <n-button type="primary" @click="get">Click me!</n-button>
  <p>data: {{ state.value }}</p>
</template>

<script lang="ts">
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarElement,
  CategoryScale,
  LinearScale
} from 'chart.js'
import { Bar } from 'vue-chartjs'
import { reactive } from 'vue'

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend)

export default {
  name: 'App',
  components: {
    Bar
  },
  setup() {
    const state = reactive({ value: '' })

    async function get() {
      const response = await fetch('http://127.0.0.1:8080/', { mode: "cors", method: 'GET' })
      const json = await response.text()
      console.log(json)
      state.value = json
    }
    return {
      state,
      get
    }
  },
  data() {
    return {
      data: {
        labels: ['January', 'February', 'March'],
        datasets: [{ data: [40, 20, 12] }]
      },
      options: {
        responsive: true
      }
    }
  }
}
</script>
