<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('activation.title')">{{ $t('activation.title') }}</h1>
      <div class="cp-subtitle">{{ $t('activation.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-card full-width">
        <div class="cp-label">{{ $t('activation.status') }}</div>
        <div class="cp-value large-text">{{ status }}</div>
        <div class="cp-actions">
          <button class="cp-button" @click="activate" :disabled="loading">
            {{ loading ? '...' : $t('activation.activate') }}
          </button>
        </div>
      </div>

      <div class="cp-card full-width">
        <div class="cp-label">{{ $t('activation.installKey') }}</div>
        <div class="input-group">
          <input v-model="productKey" :placeholder="$t('activation.enterKey')" class="cp-input" />
          <button class="cp-button" @click="installKey" :disabled="!isValidKey || loading">
            {{ $t('activation.installKey') }}
          </button>
        </div>
      </div>

      <div class="cp-card full-width">
        <div class="cp-label">{{ $t('activation.switchEdition') }}</div>
        <div class="edition-grid">
          <button v-for="(key, edition) in editions" :key="edition" class="cp-button outline" @click="switchEdition(key, edition)">
            {{ edition }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const status = ref('Checking...');
const productKey = ref('');
const loading = ref(false);

// Generic keys for edition switching (KMS/Generic)
const editions = {
  'Pro': 'W269N-WFGWX-YVC9B-4J6C9-T835X',
  'Home': 'TX9XD-98N7V-6WMQ6-BX7FG-H8Q99',
  'Enterprise': 'NPPR9-FWDCX-D2C8J-H872K-2YT43',
  'Education': 'NW6C2-QMPVW-D7KKK-3GKT6-VCFB2',
  'Pro Workstation': 'NRG8B-VKK3Q-CXVCJ-9G2XF-6Q84J'
};

const isValidKey = computed(() => {
  return productKey.value.length >= 20; // Basic length check
});

async function refreshStatus() {
  try {
    status.value = await invoke('get_activation_status');
  } catch (e) {
    status.value = 'Error retrieving status';
  }
}

async function installKey() {
  if (!productKey.value) return;
  loading.value = true;
  try {
    await invoke('install_product_key', { key: productKey.value });
    alert(t('activation.success'));
    refreshStatus();
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  } finally {
    loading.value = false;
  }
}

async function activate() {
  loading.value = true;
  try {
    await invoke('attempt_activation');
    alert(t('activation.success'));
    refreshStatus();
  } catch (e) {
    alert(t('activation.failed') + ': ' + e);
  } finally {
    loading.value = false;
  }
}

async function switchEdition(key: string, edition: string) {
  if (!confirm(`Switch to Windows ${edition}?`)) return;
  productKey.value = key;
  await installKey();
}

onMounted(refreshStatus);
</script>

<style scoped>
.full-width {
  grid-column: 1 / -1;
}

.large-text {
  font-size: 1.5em;
  margin: 10px 0;
  color: var(--cp-primary);
  text-shadow: 0 0 5px var(--cp-primary);
}

.input-group {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.cp-input {
  flex: 1;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--cp-border);
  color: var(--cp-text);
  padding: 8px;
  font-family: 'Share Tech Mono', monospace;
}

.cp-actions {
  margin-top: 10px;
}

.edition-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.outline {
  background: transparent;
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
}

.outline:hover {
  background: var(--cp-primary);
  color: #000;
}
</style>