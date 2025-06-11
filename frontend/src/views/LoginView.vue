<script setup>
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Message from 'primevue/message'
import Password from 'primevue/password'
import Checkbox from 'primevue/checkbox'

import { FormField } from '@primevue/forms'
import { Form } from '@primevue/forms'
import { zodResolver } from '@primevue/forms/resolvers/zod'
import { z } from 'zod'
import { useToast } from 'primevue/usetoast'
import { ref } from 'vue'

const remember = ref(false)

const toast = useToast()

const resolver = zodResolver(
  z.object({
    username: z.string().min(1, { message: 'Username is required.' }),
    password: z.string().min(1, { message: 'Password is required.' }),
  }),
)

const onFormSubmit = ({ valid }) => {
  if (valid) {
    toast.add({ severity: 'success', summary: 'Form is submitted.', life: 3000 })
  }
}
</script>

<template>
  <div class="Login">
    <h1>This is the Login Page</h1>
  </div>
  <div class="card flex justify-center">
    <Form :resolver @submit="onFormSubmit" class="flex flex-col gap-4 w-full sm:w-56">
      <FormField
        v-slot="$field"
        as="section"
        name="username"
        initialValue=""
        class="flex flex-col gap-2"
      >
        <InputText type="text" placeholder="Username" />
        <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
          $field.error?.message
        }}</Message>
      </FormField>
      <FormField v-slot="$field" asChild name="password" initialValue="">
        <section class="flex flex-col gap-2">
          <Password type="text" placeholder="Password" :feedback="false" toggleMask fluid />
          <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
            $field.error?.message
          }}</Message>
        </section>
      </FormField>
      <Button type="submit" severity="secondary" label="Submit" />
    </Form>
    <div class="flex items-center gap-2">
      <Checkbox v-model="remember" inputId="rememberme" binary />
      <label for="rememberme"> Rememberme </label>
    </div>
  </div>
</template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
