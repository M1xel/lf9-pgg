<script setup lang="ts">
import { LogInIcon, LogOutIcon, BoltIcon } from 'lucide-vue-next'

defineProps<{
  isLoggedIn: boolean
  isTeacher: boolean
}>()
</script>

<template>
  <nav class="header-container">
    <div class="left-icon">
      <!-- Platzhalter für ein Icon -->
      <slot name="left-icon"></slot>
    </div>
    <div class="nav-items">
      <button v-if="!isLoggedIn" class="nav-button">
        <component :is="LogInIcon" class="icon" />
        Login
      </button>

      <template v-else>
        <button class="nav-button">
          <component :is="LogOutIcon" class="icon" />
          Logout
        </button>

        <button v-if="isTeacher" class="nav-button">
          <component :is="BoltIcon" class="icon" />
          Lehrerverwaltung
        </button>
      </template>
    </div>
  </nav>
</template>

<style scoped>
.header-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
}

.nav-items {
  display: flex;
  gap: 1rem;
}

.nav-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: none;
  border: none;
  cursor: pointer;
  font-weight: bold;
}

.icon {
  width: 1rem;
  height: 1rem;
}
</style>
