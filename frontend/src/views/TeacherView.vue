<script setup lang="ts">
import { useClassStore } from '@/stores/classStore'
import Card from 'primevue/card'
import router from '@/router'

const store = useClassStore()

store.loadClasses()

var elements = store.classInfoList

function setClass(input: number) {
  store.setActiveClass(input)
  router.push({ name: 'about' })
}
</script>

<template>
  <div>
    <Card v-for="item in elements" class="cards" href="/about" @click="setClass(item.id)">
      <template #title>{{ item.name }}</template>
      <template #content>
        <p class="m-0">Klassebereich von: {{ item.name }}</p>
      </template>
    </Card>
  </div>
</template>

<style>
.cards {
  outline-style: solid;
  margin-top: 20px;
  outline-color: gray;
}
</style>
