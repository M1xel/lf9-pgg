<script setup lang="ts">
import { ref } from 'vue'
import HeaderNav from './components/HeaderNav.vue'
import SideBar from './components/SideBar.vue'
import { RouterLink, RouterView } from 'vue-router'

const isTeacher = ref(true)
const isLoggedIn = ref(true)
</script>

<template>
  <div class="sticky top-0">
    <header>
      <HeaderNav :isLoggedIn="isLoggedIn" :isTeacher="isTeacher">
        <template #left-icon>
          <BoltIcon class="icon" />
          <!--Hier dann ein richtiges Logo rein-->
        </template>
      </HeaderNav>
    </header>
  </div>
  <SideBar :isLoggedIn="isLoggedIn" :isTeacher="isTeacher" />
  <div>
    <RouterView />
  </div>
</template>

<style scoped></style>
