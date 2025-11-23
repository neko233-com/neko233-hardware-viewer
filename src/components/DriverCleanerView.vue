<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('driver_cleaner.title')">{{ $t('driver_cleaner.title') }}</h1>
      <div class="cp-subtitle">{{ $t('driver_cleaner.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="actions-bar">
        <button class="cp-btn" @click="scanDrivers" :disabled="loading">
          {{ loading ? $t('driver_cleaner.scanning') : $t('driver_cleaner.scan') }}
        </button>
        <button class="cp-btn danger" @click="cleanAll('nvidia')" :disabled="loading || !hasNvidia">
          {{ $t('driver_cleaner.cleanNvidia') }}
        </button>
        <button class="cp-btn danger" @click="cleanAll('amd')" :disabled="loading || !hasAmd">
          {{ $t('driver_cleaner.cleanAmd') }}
        </button>
      </div>

      <div v-if="error" class="error-msg">{{ error }}</div>

      <div class="cp-grid" v-if="drivers.length > 0">
        <div v-for="driver in drivers" :key="driver.published_name" class="cp-card">
          <div class="cp-label">{{ $t('driver_cleaner.provider') }}</div>
          <div class="cp-value">{{ driver.provider_name }}</div>
          
          <div class="cp-label">{{ $t('driver_cleaner.version') }}</div>
          <div class="cp-value">{{ driver.version }}</div>
          
          <div class="cp-label">{{ $t('driver_cleaner.date') }}</div>
          <div class="cp-value">{{ driver.date }}</div>
          
          <div class="cp-label">{{ $t('driver_cleaner.oemId') }}</div>
          <div class="cp-value small">{{ driver.published_name }}</div>

          <div class="card-actions">
            <button class="cp-btn danger small" @click="uninstall(driver.published_name)" :disabled="loading">
              {{ $t('driver_cleaner.uninstall') }}
            </button>
          </div>
        </div>
      </div>
      <div v-else-if="!loading" class="empty-state">
        {{ $t('driver_cleaner.noDrivers') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

interface DriverInfo {
  published_name: string;
  original_name: string;
  provider_name: string;
  class_name: string;
  version: string;
  date: string;
}

const drivers = ref<DriverInfo[]>([]);
const loading = ref(false);
const error = ref('');

const hasNvidia = computed(() => drivers.value.some(d => d.provider_name.toLowerCase().includes('nvidia')));
const hasAmd = computed(() => drivers.value.some(d => d.provider_name.toLowerCase().includes('amd') || d.provider_name.toLowerCase().includes('advanced micro devices')));

const scanDrivers = async () => {
  loading.value = true;
  error.value = '';
  try {
    drivers.value = await invoke('scan_graphic_drivers');
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const uninstall = async (id: string) => {
  if (!confirm(`Are you sure you want to uninstall driver ${id}? Screen may flicker.`)) return;
  
  loading.value = true;
  try {
    await invoke('uninstall_driver', { publishedName: id });
    await scanDrivers();
  } catch (e) {
    error.value = 'Uninstall failed: ' + e;
  } finally {
    loading.value = false;
  }
};

const cleanAll = async (type: 'nvidia' | 'amd') => {
  if (!confirm(`WARNING: This will remove ALL ${type.toUpperCase()} display drivers. Your screen may go black or resolution may drop. Continue?`)) return;

  const targets = drivers.value.filter(d => {
    const p = d.provider_name.toLowerCase();
    if (type === 'nvidia') return p.includes('nvidia');
    if (type === 'amd') return p.includes('amd') || p.includes('advanced micro devices');
    return false;
  });

  if (targets.length === 0) return;

  loading.value = true;
  for (const driver of targets) {
    try {
      await invoke('uninstall_driver', { publishedName: driver.published_name });
    } catch (e) {
      console.error(e);
      error.value += `Failed to remove ${driver.published_name}\n`;
    }
  }
  await scanDrivers();
  loading.value = false;
};

onMounted(scanDrivers);
</script>

<style scoped>
.actions-bar {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.cp-btn {
  background: transparent;
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  padding: 8px 16px;
  cursor: pointer;
  font-family: 'Segoe UI', sans-serif;
  transition: all 0.3s;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.cp-btn:hover {
  background: var(--cp-primary);
  color: #000;
  box-shadow: 0 0 10px var(--cp-primary);
}

.cp-btn.danger {
  border-color: #ff3333;
  color: #ff3333;
}

.cp-btn.danger:hover {
  background: #ff3333;
  color: #fff;
  box-shadow: 0 0 10px #ff3333;
}

.cp-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.card-actions {
  margin-top: 10px;
  text-align: right;
}

.small {
  font-size: 0.8em;
  color: #888;
}

.error-msg {
  color: #ff3333;
  margin-bottom: 10px;
  white-space: pre-wrap;
}

.empty-state {
  color: #888;
  font-style: italic;
}
</style>