<script>
import 'bootstrap/dist/css/bootstrap.min.css'
import { defineComponent } from 'vue'
import NavMenu from './components/NavMenu.vue'
export default defineComponent({
  components: {
    NavMenu
  }
})
</script>

<template>
  <header>
    <div class="wrapper">
      <NavMenu />
    </div>
  </header>
</template>

<style scoped></style>
