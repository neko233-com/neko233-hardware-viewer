<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('win11.tweaksTitle')">{{ $t('win11.tweaksTitle') }}</h1>
      <div class="cp-subtitle">{{ $t('win11.tweaksSubtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-card">
        <div class="cp-label">{{ $t('win11.classicContext') }}</div>
        <div class="toggle-group">
          <button class="cp-button small" :class="{ active: classicContext }" @click="setClassicContext(true)">{{ $t('win11.on') }}</button>
          <button class="cp-button small" :class="{ active: !classicContext }" @click="setClassicContext(false)">{{ $t('win11.off') }}</button>
        </div>
      </div>

      <div class="cp-card">
        <div class="cp-label">{{ $t('win11.showExt') }}</div>
        <div class="toggle-group">
          <button class="cp-button small" :class="{ active: showExt }" @click="setShowExt(true)">{{ $t('win11.on') }}</button>
          <button class="cp-button small" :class="{ active: !showExt }" @click="setShowExt(false)">{{ $t('win11.off') }}</button>
        </div>
      </div>

      <div class="cp-card">
        <div class="cp-label">{{ $t('win11.showHidden') }}</div>
        <div class="toggle-group">
          <button class="cp-button small" :class="{ active: showHidden }" @click="setShowHidden(true)">{{ $t('win11.on') }}</button>
          <button class="cp-button small" :class="{ active: !showHidden }" @click="setShowHidden(false)">{{ $t('win11.off') }}</button>
        </div>
      </div>

      <div class="cp-card full-width warning-card">
        <div class="cp-label">{{ $t('win11.restartExplorer') }}</div>
        <button class="cp-button danger" @click="restartExplorer">{{ $t('win11.restartExplorerBtn') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const classicContext = ref(false); // We don't actually read the state yet, just toggle
const showExt = ref(false);
const showHidden = ref(false);

async function setClassicContext(enable: boolean) {
  try {
    await invoke('set_classic_context_menu', { enable });
    classicContext.value = enable;
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  }
}

async function setShowExt(enable: boolean) {
  try {
    await invoke('set_show_extensions', { enable });
    showExt.value = enable;
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  }
}

async function setShowHidden(enable: boolean) {
  try {
    await invoke('set_show_hidden_files', { enable });
    showHidden.value = enable;
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  }
}

async function restartExplorer() {
  try {
    await invoke('restart_explorer');
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  }
}
</script>

<style scoped>
.full-width {
  grid-column: 1 / -1;
}

.toggle-group {
  display: flex;
  gap: 5px;
  margin-top: 10px;
}

.small {
  padding: 5px 15px;
  font-size: 0.8em;
  opacity: 0.5;
}

.small.active {
  opacity: 1;
  background: var(--cp-primary);
  color: #000;
  box-shadow: 0 0 10px var(--cp-primary);
}

.warning-card {
  border-color: #ff4444;
}

.danger {
  background: rgba(255, 68, 68, 0.2);
  border: 1px solid #ff4444;
  color: #ff4444;
  width: 100%;
  margin-top: 10px;
}

.danger:hover {
  background: #ff4444;
  color: #fff;
  box-shadow: 0 0 15px #ff4444;
}
</style>