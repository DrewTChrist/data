<script>
import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.bundle.min.js'
import 'bootstrap-icons/font/bootstrap-icons.css'
import { defineComponent } from 'vue'
import NavMenu from './components/NavMenu.vue'
import Footer from './components/Footer.vue'
export default defineComponent({
  components: {
    NavMenu,
    Footer
  }
})
</script>

<template>
  <header>
    <div class="wrapper">
      <NavMenu />
      <Footer />
    </div>
  </header>
</template>

<style scoped></style>
