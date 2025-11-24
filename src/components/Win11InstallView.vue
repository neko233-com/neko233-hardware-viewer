<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('win11.installTitle')">{{ $t('win11.installTitle') }}</h1>
      <div class="cp-subtitle">{{ $t('win11.installSubtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-card full-width">
        <div class="cp-label">{{ $t('win11.bypassTpm') }}</div>
        <div class="desc-text">{{ $t('win11.bypassDesc') }}</div>
        <div class="cp-actions">
          <button class="cp-button" @click="applyBypass" :disabled="loading">
            {{ loading ? '...' : $t('win11.applyBypass') }}
          </button>
        </div>
      </div>

      <div class="cp-card full-width">
        <div class="cp-label">{{ $t('win11.officialDownload') }}</div>
        <div class="desc-text">{{ $t('win11.downloadDesc') }}</div>
        <div class="cp-actions">
          <a href="https://www.microsoft.com/software-download/windows11" target="_blank" class="cp-button outline">
            {{ $t('win11.openMicrosoft') }}
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const loading = ref(false);

async function applyBypass() {
  loading.value = true;
  try {
    await invoke('set_win11_bypass');
    alert(t('activation.success'));
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.full-width {
  grid-column: 1 / -1;
}

.desc-text {
  margin: 10px 0;
  color: #aaa;
  font-size: 0.9em;
}

.cp-actions {
  margin-top: 15px;
}

.outline {
  background: transparent;
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  text-decoration: none;
  display: inline-block;
  text-align: center;
}

.outline:hover {
  background: var(--cp-primary);
  color: #000;
}
</style>