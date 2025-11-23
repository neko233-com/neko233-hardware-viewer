<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('colorManagement.title')">{{ $t('colorManagement.title') }}</h1>
      <div class="cp-subtitle">{{ $t('colorManagement.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-card">
        <div class="cp-label">{{ $t('colorManagement.title') }}</div>
        <div class="desc-text">{{ $t('colorManagement.desc') }}</div>
        <button class="cp-button primary" @click="openColorCpl">
          {{ $t('colorManagement.openColorCpl') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';

async function openColorCpl() {
  try {
    await invoke('open_color_cpl');
  } catch (e) {
    console.error('Failed to open color management:', e);
    alert('Failed to open: ' + e);
  }
}
</script>

<style scoped>
.desc-text {
  margin: 10px 0;
  color: #aaa;
  font-size: 0.9em;
}

.cp-button {
  margin-top: 10px;
  width: 100%;
}
</style>
