<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('uninstall.title')">{{ $t('uninstall.title') }}</h1>
      <div class="cp-subtitle">{{ $t('uninstall.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="search-bar">
        <input 
          v-model="searchQuery" 
          type="text" 
          class="cp-input" 
          :placeholder="$t('uninstall.search')"
        />
        <button class="cp-btn" @click="refreshApps" :disabled="loading">
          {{ loading ? $t('loading') : $t('uninstall.refresh') }}
        </button>
      </div>

      <div v-if="loading" class="loading-text">{{ $t('loading') }}</div>
      
      <div v-else class="apps-list">
        <div v-for="(app, index) in filteredApps" :key="index" class="app-item">
          <div class="app-info">
            <div class="app-name">{{ app.name }}</div>
            <div class="app-meta">
              <span v-if="app.version">v{{ app.version }}</span>
              <span v-if="app.publisher"> | {{ app.publisher }}</span>
              <span v-if="app.install_date"> | {{ app.install_date }}</span>
            </div>
          </div>
          <button class="cp-btn-danger" @click="uninstall(app)">
            {{ $t('uninstall.uninstall') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface AppInfo {
  name: string;
  version: string;
  publisher: string;
  uninstall_string: string;
  install_date: string;
}

const apps = ref<AppInfo[]>([]);
const loading = ref(false);
const searchQuery = ref('');

const filteredApps = computed(() => {
  if (!searchQuery.value) return apps.value;
  const q = searchQuery.value.toLowerCase();
  return apps.value.filter(app => 
    app.name.toLowerCase().includes(q) || 
    app.publisher.toLowerCase().includes(q)
  );
});

async function refreshApps() {
  loading.value = true;
  try {
    apps.value = await invoke('get_installed_apps');
  } catch (e) {
    console.error(e);
    alert('Failed to load apps: ' + e);
  } finally {
    loading.value = false;
  }
}

async function uninstall(app: AppInfo) {
  if (!confirm(`Are you sure you want to uninstall "${app.name}"?`)) return;
  
  try {
    await invoke('uninstall_app', { uninstallString: app.uninstall_string });
  } catch (e) {
    alert('Failed to launch uninstaller: ' + e);
  }
}

onMounted(() => {
  refreshApps();
});
</script>

<style scoped>
.cp-container {
  padding: 20px;
  height: 100%;
  overflow-y: auto;
  color: #fff;
}

.cp-header {
  margin-bottom: 20px;
  border-bottom: 1px solid #333;
  padding-bottom: 10px;
}

.cp-title {
  font-size: 2.5em;
  margin: 0;
  color: var(--cp-primary);
  text-transform: uppercase;
  letter-spacing: 2px;
}

.cp-subtitle {
  color: #888;
  font-size: 1em;
  margin-top: 5px;
}

.search-bar {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.cp-input {
  flex: 1;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid #333;
  color: #fff;
  padding: 8px 12px;
  font-family: 'Courier New', Courier, monospace;
}

.cp-input:focus {
  border-color: var(--cp-primary);
  outline: none;
}

.cp-btn {
  background: rgba(0, 243, 255, 0.1);
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  padding: 8px 16px;
  cursor: pointer;
  transition: all 0.3s;
}

.cp-btn:hover {
  background: var(--cp-primary);
  color: #000;
}

.cp-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.apps-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.app-item {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid #333;
  padding: 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.3s;
}

.app-item:hover {
  border-color: var(--cp-primary);
  background: rgba(0, 243, 255, 0.05);
}

.app-name {
  font-weight: bold;
  font-size: 1.1em;
  color: #fff;
}

.app-meta {
  font-size: 0.8em;
  color: #888;
  margin-top: 4px;
}

.cp-btn-danger {
  background: rgba(255, 0, 0, 0.1);
  border: 1px solid #f00;
  color: #f00;
  padding: 6px 12px;
  cursor: pointer;
  transition: all 0.3s;
}

.cp-btn-danger:hover {
  background: #f00;
  color: #fff;
}

.loading-text {
  text-align: center;
  padding: 40px;
  color: var(--cp-primary);
  font-family: 'Courier New', Courier, monospace;
}
</style>
