<script setup lang="ts">
import SettingsForm from './components/SettingsForm.vue'
</script>

<template>
  <main class="min-h-screen bg-gray-50 dark:bg-gray-900 p-6">
    <div class="card">
      <h1 class="main-title text-4xl text-center mb-8">
        <span class="text-emerald-600">RSI</span> Break 助手
      </h1>
      <SettingsForm class="settings-form"/>
    </div>
  </main>
</template>