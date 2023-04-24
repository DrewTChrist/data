<template>
  <nav class="navbar navbar-expand-lg bg-body-tertiary">
    <div class="container-fluid">
      <a class="navbar-brand" href="#">Navbar</a>
      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbarSupportedContent"
        aria-controls="navbarSupportedContent"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <!--<a class="nav-link active" aria-current="page" href="#">Home</a>-->
            <RouterLink to="/" class="nav-link" href="#">Home</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink to="/config" class="nav-link" href="#">Configuration</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink to="/about" class="nav-link" href="#">About</RouterLink>
          </li>
        </ul>
      </div>
    </div>
  </nav>
  <RouterView />
</template>

<script>
import { defineComponent } from 'vue'
import { RouterLink, RouterView } from 'vue-router'
export default defineComponent({
  name: 'NavMenu',
  components: {}
})
</script>
