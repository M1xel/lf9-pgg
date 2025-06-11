<script setup lang="ts">
import Drawer from 'primevue/drawer'
import { ref } from 'vue'
import { useClassStore } from '@/stores/classStore'
import router from '@/router'

const store = useClassStore()

store.loadClasses()

defineProps<{
  isLoggedIn: boolean
  isTeacher: boolean
}>()

var elements = store.classInfoList
var visible = ref(true)
var closeIcon = ref(false)

function setClass(input: number) {
  store.setActiveClass(input)
  router.push({ name: 'about' })
}
</script>

<template>
  <Drawer
    v-model:visible="visible"
    :showCloseIcon="closeIcon"
    :modal="false"
    :dismissable="false"
    v-on:hide="visible = true"
    header="Sidebar"
    v-if="isLoggedIn"
  >
    <div v-if="isTeacher">
      <ul>
        <li v-for="item in elements">
          <p href="/about" @click="setClass(item.id)">{{ item.name }}</p>
        </li>
      </ul>
    </div>
    <div v-else>
      <ul>
        <li>
          <p href="/about">Offene/Abgegebene Evaluaionsbögen</p>
        </li>
        <li>
          <p href="/about">Notenübersicht</p>
        </li>
      </ul>
    </div>
  </Drawer>
</template>
