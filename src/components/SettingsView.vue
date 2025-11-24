<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('settings.title')">{{ $t('settings.title') }}</h1>
      <div class="cp-subtitle">SYSTEM CONFIGURATION</div>
    </div>

    <div class="cp-section">
      <div class="cp-section-title">{{ $t('settings.general') || 'GENERAL' }}</div>
      <div class="cp-grid">
        
        <!-- Autostart -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('settings.autostart') }}</div>
          <div class="cp-value">{{ autostartEnabled ? $t('optimization.enabled') : $t('optimization.disabled') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="toggleAutostart">
              {{ autostartEnabled ? $t('optimization.disable') : $t('optimization.enable') }}
            </button>
          </div>
        </div>

        <!-- Auto Update -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('settings.autoUpdate') }}</div>
          <div class="cp-value">{{ autoUpdateEnabled ? $t('optimization.enabled') : $t('optimization.disabled') }}</div>
          <div class="actions">
            <label class="toggle-switch">
              <input type="checkbox" v-model="autoUpdateEnabled" @change="saveAutoUpdate">
              <span class="slider"></span>
            </label>
          </div>
        </div>

      </div>
    </div>

    <div class="cp-section">
      <div class="cp-section-title">{{ $t('settings.about') || 'ABOUT' }}</div>
      <div class="cp-card full-width">
        <div class="version-info">
          <div class="cp-label">{{ $t('settings.currentVersion') }}</div>
          <div class="cp-value large">{{ appVersion }}</div>
        </div>
        <div class="actions">
          <button class="cp-btn" @click="manualCheckUpdate" :disabled="checkingUpdate">
            {{ checkingUpdate ? $t('settings.checking') : $t('settings.checkUpdate') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Update Toast (Local to this view or global? Keeping global logic in App.vue, but trigger here) -->
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from '@tauri-apps/api/app';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const autostartEnabled = ref(false);
const autoUpdateEnabled = ref(true);
const appVersion = ref('0.0.0');
const checkingUpdate = ref(false);

const toggleAutostart = async () => {
  try {
    await invoke('set_autostart', { enable: !autostartEnabled.value });
    autostartEnabled.value = !autostartEnabled.value;
  } catch (e) {
    console.error(e);
    alert('Failed to set autostart: ' + e);
  }
};

const saveAutoUpdate = () => {
  localStorage.setItem('autoUpdate', String(autoUpdateEnabled.value));
};

const manualCheckUpdate = async () => {
  checkingUpdate.value = true;
  try {
    const update = await check();
    if (update) {
      const yes = confirm(`${t('settings.updateAvailable')} v${update.version}\n${t('settings.updateNow')}?`);
      if (yes) {
        await update.downloadAndInstall();
        await relaunch();
      }
    } else {
      alert(t('settings.noUpdate'));
    }
  } catch (e) {
    console.error(e);
    alert('Update check failed: ' + e);
  } finally {
    checkingUpdate.value = false;
  }
};

onMounted(async () => {
  appVersion.value = await getVersion();
  autostartEnabled.value = await invoke('check_autostart');
  
  const savedAuto = localStorage.getItem('autoUpdate');
  if (savedAuto !== null) {
    autoUpdateEnabled.value = savedAuto === 'true';
  }
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
  margin-bottom: 30px;
  border-bottom: 1px solid #333;
  padding-bottom: 10px;
}

.cp-title {
  font-size: 2.5em;
  margin: 0;
  font-family: 'Courier New', Courier, monospace;
  letter-spacing: 5px;
  color: #fff;
  text-shadow: 2px 2px var(--cp-primary);
}

.cp-subtitle {
  color: var(--cp-primary);
  letter-spacing: 2px;
  font-size: 0.8em;
}

.cp-section {
  margin-bottom: 30px;
}

.cp-section-title {
  font-size: 1.2em;
  color: var(--cp-primary);
  margin-bottom: 15px;
  border-left: 3px solid var(--cp-primary);
  padding-left: 10px;
}

.cp-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
}

.cp-card {
  background: rgba(0, 0, 0, 0.6);
  border: 1px solid #333;
  padding: 20px;
  transition: all 0.3s;
}

.cp-card.full-width {
  grid-column: 1 / -1;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.cp-card:hover {
  border-color: var(--cp-primary);
  box-shadow: 0 0 15px rgba(0, 229, 255, 0.2);
}

.cp-label {
  font-size: 1.1em;
  font-weight: bold;
  margin-bottom: 5px;
}

.cp-value {
  color: var(--cp-primary);
  font-family: 'Courier New', Courier, monospace;
  margin-bottom: 10px;
}

.cp-value.large {
  font-size: 1.5em;
}

.actions {
  margin-top: 15px;
  display: flex;
  gap: 10px;
}

.cp-btn {
  background: transparent;
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  padding: 5px 15px;
  cursor: pointer;
  font-family: inherit;
  transition: all 0.3s;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.cp-btn:hover {
  background: var(--cp-primary);
  color: #000;
  box-shadow: 0 0 10px var(--cp-primary);
}

/* Toggle Switch */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 34px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #333;
  transition: .4s;
  border: 1px solid #555;
}

.slider:before {
  position: absolute;
  content: "";
  height: 26px;
  width: 26px;
  left: 4px;
  bottom: 3px;
  background-color: #888;
  transition: .4s;
}

input:checked + .slider {
  background-color: rgba(0, 229, 255, 0.2);
  border-color: var(--cp-primary);
}

input:checked + .slider:before {
  transform: translateX(26px);
  background-color: var(--cp-primary);
}
</style>
