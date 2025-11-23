<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('diagnostics.title')">{{ $t('diagnostics.title') }}</h1>
      <div class="cp-subtitle">{{ $t('diagnostics.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-section-title">{{ $t('diagnostics.sectionTitle') }}</div>
      <div class="cp-grid">
        
        <!-- Quick Check -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('diagnostics.quickCheck') }}</div>
          <div class="cp-value">{{ $t('diagnostics.noReboot') }}</div>
          <div class="cp-desc">{{ $t('diagnostics.quickCheckDesc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="runQuickCheck" :disabled="loading">
              {{ loading ? $t('diagnostics.testing') : $t('diagnostics.runQuick') }}
            </button>
          </div>
        </div>

        <!-- Full Check -->
        <div class="cp-card">
          <div class="cp-label">{{ $t('diagnostics.fullCheck') }}</div>
          <div class="cp-value">{{ $t('diagnostics.rebootRequired') }}</div>
          <div class="cp-desc">{{ $t('diagnostics.fullCheckDesc') }}</div>
          <div class="actions">
            <button class="cp-btn" @click="runFullCheck">
              {{ $t('diagnostics.openTool') }}
            </button>
          </div>
        </div>

      </div>
    </div>

    <div v-if="message" class="cp-message" :class="{ error: isError }">{{ message }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { useI18n } from 'vue-i18n';
import { useTasks } from '../composables/useTasks';

const { t } = useI18n();
const { addTask, updateTask } = useTasks();
const loading = ref(false);
const message = ref('');
const isError = ref(false);
let unlisten: any = null;

const runQuickCheck = async () => {
  loading.value = true;
  message.value = t('diagnostics.testing');
  isError.value = false;
  
  const taskId = 'mem_check';
  addTask(taskId, t('diagnostics.quickCheck'));

  // Listen for progress
  unlisten = await listen('memory_progress', (event: any) => {
    const progress = event.payload as number;
    updateTask(taskId, progress);
  });

  try {
    await invoke('quick_memory_check');
    message.value = t('diagnostics.testPassed');
    updateTask(taskId, 100, 'completed');
  } catch (e) {
    isError.value = true;
    message.value = t('diagnostics.testFailed') + e;
    updateTask(taskId, 0, 'failed', String(e));
  } finally {
    loading.value = false;
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }
};

onUnmounted(() => {
  if (unlisten) unlisten();
});

const runFullCheck = async () => {
  try {
    await invoke('run_memory_diagnostic');
  } catch (e) {
    alert('Failed to launch tool: ' + e);
  }
};
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

.cp-desc {
  font-size: 0.8em;
  color: #aaa;
  margin: 10px 0;
  height: 40px;
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

.cp-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.cp-message {
  margin-top: 20px;
  padding: 10px;
  border: 1px solid var(--cp-primary);
  background: rgba(0, 229, 255, 0.1);
  color: #fff;
}

.cp-message.error {
  color: #ff003c;
  border-color: #ff003c;
  background: rgba(255, 0, 60, 0.1);
}
</style>
