<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('optimization.title')">{{ $t('optimization.title') }}</h1>
      <div class="cp-subtitle">{{ $t('optimization.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-section-title">{{ $t('optimization.sectionTitle') }}</div>
      <div class="cp-grid">
        
        <!-- Firewall -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('optimization.firewall') }}</div>
          <div class="cp-value">{{ firewallStatus ? $t('optimization.enabled') : $t('optimization.disabled') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="toggleFirewall(!firewallStatus)" :disabled="loading">
              {{ firewallStatus ? $t('optimization.disable') : $t('optimization.enable') }}
            </button>
          </div>
        </div>

        <!-- Cortana -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('optimization.cortana') }}</div>
          <div class="cp-value">{{ cortanaStatus ? $t('optimization.enabled') : $t('optimization.disabled') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="toggleCortana(!cortanaStatus)" :disabled="loading">
              {{ cortanaStatus ? $t('optimization.disable') : $t('optimization.enable') }}
            </button>
          </div>
        </div>

      </div>
    </div>

    <!-- System Performance Section -->
    <div class="cp-section">
      <div class="cp-section-title">{{ $t('optimization.performanceTitle') }}</div>
      <div class="cp-grid">
        
        <!-- Processor Optimization -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('optimization.processor') }}</div>
          <div class="cp-value">{{ $t('optimization.processorDesc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="runOptimization('optimize_processor')" :disabled="loading">
              {{ $t('optimization.apply') }}
            </button>
          </div>
        </div>

        <!-- High Performance Power Plan -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('optimization.powerPlan') }}</div>
          <div class="cp-value">{{ $t('optimization.powerPlanDesc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="runOptimization('enable_high_perf_plan')" :disabled="loading">
              {{ $t('optimization.enable') }}
            </button>
          </div>
        </div>

        <!-- File System Cache -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('optimization.fsCache') }}</div>
          <div class="cp-value">{{ $t('optimization.fsCacheDesc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="runOptimization('increase_fs_cache')" :disabled="loading">
              {{ $t('optimization.apply') }}
            </button>
          </div>
        </div>

        <!-- Large System Cache -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('optimization.sysCache') }}</div>
          <div class="cp-value">{{ $t('optimization.sysCacheDesc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="runOptimization('enable_large_system_cache')" :disabled="loading">
              {{ $t('optimization.enable') }}
            </button>
          </div>
        </div>

      </div>
    </div>

    <div v-if="message" class="cp-message">{{ message }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const firewallStatus = ref(false);
const cortanaStatus = ref(false);
const loading = ref(false);
const message = ref('');

const fetchStatus = async () => {
  try {
    firewallStatus.value = await invoke('get_firewall_status');
    cortanaStatus.value = await invoke('get_cortana_status');
  } catch (e) {
    console.error(e);
  }
};

const runOptimization = async (command: string) => {
  loading.value = true;
  message.value = '';
  try {
    const res: string = await invoke(command);
    message.value = res;
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    loading.value = false;
  }
};

const toggleFirewall = async (enable: boolean) => {
  loading.value = true;
  try {
    const res: string = await invoke('set_firewall_status', { enable });
    message.value = res;
    await fetchStatus();
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    loading.value = false;
  }
};

const toggleCortana = async (enable: boolean) => {
  loading.value = true;
  try {
    const res: string = await invoke('set_cortana_status', { enable });
    message.value = res;
    await fetchStatus();
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    loading.value = false;
  }
};

onMounted(fetchStatus);
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
</style>