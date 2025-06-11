<script setup lang="ts">
import Button from 'primevue/button'
import { useClassStore } from '@/stores/classStore'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'

const store = useClassStore()
</script>
<template>
  <div class="about">
    <h1>{{ store.classInfo?.name }}</h1>
    <Accordion value="0">
      <AccordionPanel value="0">
        <AccordionHeader>Lernfeld I</AccordionHeader>
        <AccordionContent>
          <Button>FE</Button>
          <Button>LF</Button>
          <Button>SuK</Button>
          <Button>Wug</Button>
        </AccordionContent>
      </AccordionPanel>
      <AccordionPanel value="1">
        <AccordionHeader>Lernfeld II</AccordionHeader>
        <AccordionContent>
          <Button>FE</Button>
          <Button>LF</Button>
          <Button>SuK</Button>
          <Button>Wug</Button>
        </AccordionContent>
      </AccordionPanel>
      <AccordionPanel value="2">
        <AccordionHeader>Lernfeld III</AccordionHeader>
        <AccordionContent>
          <Button>FE</Button>
          <Button>LF</Button>
          <Button>SuK</Button>
          <Button>Wug</Button>
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
  </div>
</template>

<style></style>
