<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('optimization.title')">{{ $t('optimization.title') }}</h1>
      <div class="cp-subtitle">{{ $t('optimization.subtitle') }}</div>
    </div>

    <div v-if="!isAdmin" class="admin-warning">
      <div class="warning-icon">⚠️</div>
      <div class="warning-text">
        <h3>{{ $t('optimization.adminRequired') }}</h3>
        <p>{{ $t('optimization.adminRequiredDesc') }}</p>
      </div>
    </div>

    <!-- Search Filter -->
    <div class="cp-section" style="margin-bottom: 20px;">
      <input 
        type="text" 
        v-model="searchQuery" 
        :placeholder="$t('optimization.searchPlaceholder')" 
        class="cp-input"
        style="width: 100%; padding: 10px; background: rgba(0,0,0,0.5); border: 1px solid var(--cp-primary); color: var(--cp-text); font-family: 'Rajdhani', sans-serif;"
      />
    </div>

    <div class="cp-section" v-if="filteredTweaks.length > 0">
      <div class="cp-section-title">{{ $t('optimization.tweaksTitle') }}</div>
      <div class="cp-grid">
        <div class="cp-card" v-for="tweak in filteredTweaks" :key="tweak.id">
          <div class="cp-label">{{ $t('optimization.' + tweak.id) }}</div>
          <div class="cp-desc" style="font-size: 0.8em; color: #aaa; margin-bottom: 10px;">{{ $t('optimization.' + tweak.id + '_desc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="applyTweak(tweak.id, true)" :disabled="loading">
              {{ $t('optimization.enable') }}
            </button>
            <button class="cp-btn" style="margin-left: 5px; background: rgba(255,0,0,0.2); border-color: #f00;" @click="applyTweak(tweak.id, false)" :disabled="loading">
              {{ $t('optimization.disable') }}
            </button>
          </div>
        </div>
      </div>
    </div>
    <div v-else class="cp-section">
        <div style="text-align: center; color: #aaa;">{{ $t('optimization.noResults') }}</div>
    </div>

    <div v-if="message" class="cp-message">{{ message }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const loading = ref(false);
const message = ref('');
const searchQuery = ref('');
const isAdmin = ref(true);

onMounted(async () => {
  isAdmin.value = await invoke('is_admin');
});

interface Tweak {
  id: string;
  category: 'privacy' | 'performance' | 'ui' | 'network';
}

const tweaks: Tweak[] = [
  { id: 'telemetry', category: 'privacy' },
  { id: 'windows_update', category: 'performance' },
  { id: 'hibernation', category: 'performance' },
  { id: 'game_dvr', category: 'performance' },
  { id: 'sticky_keys', category: 'ui' },
  { id: 'mouse_acceleration', category: 'ui' },
  { id: 'lock_screen', category: 'ui' },
  { id: 'bing_search', category: 'privacy' },
  { id: 'aero_shake', category: 'ui' },
  { id: 'timeline', category: 'privacy' },
  { id: 'cortana', category: 'privacy' },
  { id: 'location_tracking', category: 'privacy' },
  { id: 'advertising_id', category: 'privacy' },
  { id: 'feedback_diagnostics', category: 'privacy' },
  { id: 'suggested_apps', category: 'ui' },
  { id: 'meet_now', category: 'ui' },
  { id: 'news_interests', category: 'ui' },
  { id: 'wifi_sense', category: 'privacy' },
  { id: 'delivery_optimization', category: 'network' },
  { id: 'this_pc_desktop', category: 'ui' },
  { id: 'control_panel_desktop', category: 'ui' },
  { id: 'dark_mode', category: 'ui' },
  { id: 'transparency', category: 'ui' },
  { id: 'high_perf_plan', category: 'performance' },
  { id: 'fast_startup', category: 'performance' },
  { id: 'uac_dimming', category: 'ui' },
  { id: 'printer_spooler', category: 'performance' },
  { id: 'fax_service', category: 'performance' },
  { id: 'xps_services', category: 'performance' },
  { id: 'error_reporting', category: 'privacy' },
];

const filteredTweaks = computed(() => {
  if (!searchQuery.value) return tweaks;
  const q = searchQuery.value.toLowerCase();
  return tweaks.filter(t => t.id.toLowerCase().includes(q));
});

async function applyTweak(id: string, enable: boolean) {
  loading.value = true;
  message.value = '';
  try {
    const result: string = await invoke('apply_optimization', { id, enable });
    message.value = result;
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.actions {
  margin-top: 15px;
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

.cp-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.cp-message {
  margin-top: 20px;
  padding: 10px;
  border: 1px solid #333;
  background: rgba(0,0,0,0.5);
  color: #fff;
}

.admin-warning {
  background: rgba(255, 0, 0, 0.2);
  border: 2px solid #f00;
  padding: 15px;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 15px;
  animation: pulse 2s infinite;
}

.warning-icon {
  font-size: 2em;
}

.warning-text h3 {
  margin: 0 0 5px 0;
  color: #f00;
  text-transform: uppercase;
  letter-spacing: 2px;
}

.warning-text p {
  margin: 0;
  color: #fff;
}

@keyframes pulse {
  0% { box-shadow: 0 0 0 0 rgba(255, 0, 0, 0.4); }
  70% { box-shadow: 0 0 0 10px rgba(255, 0, 0, 0); }
  100% { box-shadow: 0 0 0 0 rgba(255, 0, 0, 0); }
}
</style>