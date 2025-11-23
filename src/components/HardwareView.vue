<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('title')">{{ $t('title') }}</h1>
      <div class="cp-subtitle">{{ $t('subtitle') }}</div>
    </div>

    <!-- Real-time Usage Section -->
    <div v-if="!loading && !error" class="cp-section">
      <div class="cp-section-title">{{ $t('labels.usage') }}</div>
      <div class="cp-grid">
        <div class="cp-card">
          <div class="cp-label">{{ $t('labels.cpuUsage') }}</div>
          <div class="cp-value">{{ usage.cpu_usage.toFixed(1) }}%</div>
          <div class="usage-bar">
            <div class="usage-fill" :style="{ width: usage.cpu_usage + '%' }"></div>
          </div>
        </div>
        <div class="cp-card">
          <div class="cp-label">{{ $t('labels.ramUsage') }}</div>
          <div class="cp-value">{{ (usage.memory_used / 1024 / 1024 / 1024).toFixed(1) }} / {{ (usage.memory_total / 1024 / 1024 / 1024).toFixed(1) }} GB</div>
          <div class="usage-bar">
            <div class="usage-fill" :style="{ width: (usage.memory_used / usage.memory_total * 100) + '%' }"></div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading-text">
      {{ $t('loading') }}
    </div>

    <div v-else-if="error" class="cp-section">
      <div class="cp-section-title" style="color: var(--cp-secondary)">{{ $t('error') }}</div>
      <div class="cp-value">{{ error }}</div>
    </div>

    <div v-else>
      <!-- Motherboard -->
      <div class="cp-section">
        <div class="cp-section-title">{{ $t('sections.motherboard') }}</div>
        <div class="cp-grid">
          <div v-for="(mobo, index) in info.motherboard" :key="index" class="cp-card">
            <div class="cp-label">{{ $t('labels.manufacturer') }}</div>
            <div class="cp-value">{{ mobo.Manufacturer }}</div>
            <div class="cp-label">{{ $t('labels.product') }}</div>
            <div class="cp-value">{{ mobo.Product }}</div>
            <div class="cp-label">{{ $t('labels.version') }}</div>
            <div class="cp-value">{{ mobo.Version }}</div>
            <a v-if="getDriverLink(mobo.Manufacturer, mobo.Product)" :href="getDriverLink(mobo.Manufacturer, mobo.Product) || undefined" target="_blank" class="driver-link">
              {{ $t('labels.downloadDriver') }}
            </a>
          </div>
        </div>
      </div>

      <!-- CPU -->
      <div class="cp-section">
        <div class="cp-section-title">{{ $t('sections.cpu') }}</div>
        <div class="cp-grid">
          <div v-for="(cpu, index) in info.cpu" :key="index" class="cp-card">
            <div class="cp-label">{{ $t('labels.model') }}</div>
            <div class="cp-value">{{ cpu.info.Name }}</div>
            <div class="cp-label">{{ $t('labels.manufacturer') }}</div>
            <div class="cp-value">{{ cpu.info.Manufacturer }}</div>
            <div class="cp-label">{{ $t('labels.coresThreads') }}</div>
            <div class="cp-value">{{ cpu.info.NumberOfCores }} / {{ cpu.info.NumberOfLogicalProcessors }}</div>
            <div class="cp-label">{{ $t('labels.clockSpeed') }}</div>
            <div class="cp-value">{{ cpu.info.MaxClockSpeed }} MHz</div>
            <div class="cp-score" :class="getScoreClass(cpu.score)">
              {{ $t('score') }}: {{ cpu.score_num }} ({{ $t('scores.' + cleanScore(cpu.score)) }})
            </div>
            <a v-if="getDriverLink(cpu.info.Manufacturer, cpu.info.Name)" :href="getDriverLink(cpu.info.Manufacturer, cpu.info.Name) || undefined" target="_blank" class="driver-link">
              {{ $t('labels.downloadDriver') }}
            </a>
          </div>
        </div>
      </div>

      <!-- GPU -->
      <div class="cp-section">
        <div class="cp-section-title">{{ $t('sections.gpu') }}</div>
        <div class="cp-grid">
          <div v-for="(gpu, index) in info.gpu" :key="index" class="cp-card">
            <div class="cp-label">{{ $t('labels.model') }}</div>
            <div class="cp-value">{{ gpu.info.Name }}</div>
            <div class="cp-label">{{ $t('labels.manufacturer') }}</div>
            <div class="cp-value">{{ gpu.info.AdapterCompatibility || $t('labels.unknown') }}</div>
            <div class="cp-label">{{ $t('labels.driverVersion') }}</div>
            <div class="cp-value">{{ gpu.info.DriverVersion }}</div>
            <div class="cp-label">{{ $t('labels.vram') }}</div>
            <div class="cp-value">
              {{ gpu.info.AdapterRAM ? (gpu.info.AdapterRAM / 1024 / 1024 / 1024).toFixed(2) + ' GB' : $t('labels.unknown') }}
            </div>
            <div class="cp-score" :class="getScoreClass(gpu.score)">
              {{ $t('score') }}: {{ gpu.score_num }} ({{ $t('scores.' + cleanScore(gpu.score)) }})
            </div>
            <a v-if="getDriverLink(gpu.info.AdapterCompatibility || gpu.info.Name, gpu.info.Name)" :href="getDriverLink(gpu.info.AdapterCompatibility || gpu.info.Name, gpu.info.Name) || undefined" target="_blank" class="driver-link">
              {{ $t('labels.downloadDriver') }}
            </a>
          </div>
        </div>
      </div>

      <!-- RAM -->
      <div class="cp-section">
        <div class="cp-section-title">
          {{ $t('sections.ram') }}
          <button class="cp-btn-small" @click="runMemoryDiagnostic" :title="$t('labels.memoryDiagnostic')">
            {{ $t('labels.checkMemory') }}
          </button>
        </div>
        <div class="cp-card">
          <div class="cp-grid">
            <div>
              <div class="cp-label">{{ $t('labels.total') }}</div>
              <div class="cp-value">{{ info.ram.total_gb }} GB</div>
            </div>
            <div>
              <div class="cp-label">{{ $t('labels.averageSpeed') }}</div>
              <div class="cp-value">{{ info.ram.avg_speed }} MHz</div>
            </div>
          </div>
          <div class="cp-score" :class="getScoreClass(info.ram.score)">
            {{ $t('score') }}: {{ info.ram.score_num }} ({{ $t('scores.' + cleanScore(info.ram.score)) }})
          </div>
          <div style="margin-top: 15px; border-top: 1px dashed #333; padding-top: 10px;">
            <div v-for="(mem, idx) in info.ram.info" :key="idx" style="margin-bottom: 10px; font-size: 0.9em; color: #aaa; border-bottom: 1px solid #222; padding-bottom: 5px;">
              <div style="color: var(--cp-primary); font-weight: bold; display: flex; justify-content: space-between;">
                <span>Slot {{ mem.DeviceLocator }} ({{ idx + 1 }})</span>
                <span v-if="mem.Status" :style="{ color: mem.Status === 'OK' ? '#0f0' : '#f00' }">[{{ mem.Status }}]</span>
              </div>
              <div>{{ $t('labels.manufacturer') }}: {{ mem.Manufacturer }}</div>
              <div>{{ $t('labels.model') }}: {{ mem.PartNumber }}</div>
              <div>{{ $t('labels.bankLabel') }}: {{ mem.BankLabel || 'N/A' }}</div>
              <div>{{ $t('labels.formFactor') }}: {{ mem.FormFactor || 'N/A' }}</div>
              <div>{{ $t('labels.dataWidth') }}: {{ mem.DataWidth ? mem.DataWidth + ' bit' : 'N/A' }}</div>
              <div>{{ $t('labels.totalWidth') }}: {{ mem.TotalWidth ? mem.TotalWidth + ' bit' : 'N/A' }}</div>
              <div>{{ $t('labels.timings') }}: N/A (OS Restriction)</div>
            <div class="cp-label">{{ $t('labels.serialNumber') }}</div>
            <div class="cp-value-row">
              <span class="cp-value text-truncate" :title="mem.SerialNumber">{{ mem.SerialNumber }}</span>
              <button v-if="mem.SerialNumber" class="copy-btn" @click="copyToClipboard(mem.SerialNumber)" title="Copy">📋</button>
            </div>
            <div>{{ $t('labels.capacity') }}: {{ (mem.Capacity / 1024 / 1024 / 1024).toFixed(0) }}GB</div>
              <div>{{ $t('labels.speed') }}: {{ mem.ConfiguredClockSpeed || mem.Speed }}MHz</div>
              <div>{{ $t('labels.voltage') }}: {{ mem.ConfiguredVoltage ? mem.ConfiguredVoltage / 1000 + 'V' : 'Unknown' }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Storage -->
      <div class="cp-section">
        <div class="cp-section-title">{{ $t('sections.disk') }}</div>
        <div class="cp-grid">
          <div v-for="(disk, index) in info.disks" :key="index" class="cp-card">
            <div class="cp-label">{{ $t('labels.model') }}</div>
            <div class="cp-value">{{ disk.info.Model }}</div>
            <div class="cp-label">{{ $t('labels.manufacturer') }}</div>
            <div class="cp-value">{{ disk.info.Manufacturer || 'Unknown' }}</div>
            <div class="cp-label">{{ $t('labels.serialNumber') }}</div>
            <div class="cp-value-row">
              <span class="cp-value text-truncate" :title="disk.info.SerialNumber">{{ disk.info.SerialNumber || 'Unknown' }}</span>
              <button v-if="disk.info.SerialNumber" class="copy-btn" @click="copyToClipboard(disk.info.SerialNumber)" title="Copy">📋</button>
            </div>
            <div class="cp-label">{{ $t('labels.capacity') }}</div>
            <div class="cp-value">
              {{ disk.info.Size ? (disk.info.Size / 1024 / 1024 / 1024).toFixed(2) + ' GB' : 'Unknown' }}
            </div>
            <div class="cp-label">{{ $t('labels.interface') }}</div>
            <div class="cp-value">{{ disk.info.InterfaceType || 'Unknown' }}</div>
            <div class="cp-score" :class="getScoreClass(disk.score)">
              {{ $t('score') }}: {{ disk.score_num }} ({{ $t('scores.' + cleanScore(disk.score)) }})
            </div>

            <a v-if="getDriverLink(disk.info.Manufacturer || disk.info.Model, disk.info.Model)" :href="getDriverLink(disk.info.Manufacturer || disk.info.Model, disk.info.Model) || undefined" target="_blank" class="driver-link">
              {{ $t('labels.downloadDriver') }}
            </a>
          </div>
        </div>
      </div>

      <!-- Sound -->
      <div class="cp-section">
        <div class="cp-section-title">{{ $t('sections.sound') }}</div>
        <div class="cp-grid">
          <div v-for="(snd, index) in info.sound" :key="index" class="cp-card">
            <div class="cp-label">{{ $t('labels.model') }}</div>
            <div class="cp-value">{{ snd.Name }}</div>
            <div class="cp-label">{{ $t('labels.manufacturer') }}</div>
            <div class="cp-value">{{ snd.Manufacturer }}</div>
            <div class="cp-label">{{ $t('labels.status') }}</div>
            <div class="cp-value">{{ snd.Status || 'Unknown' }}</div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { getDriverLink } from '../config/drivers';

const loading = ref(true);
const error = ref('');
const info = ref<any>(null);
const usage = ref({ cpu_usage: 0, memory_used: 0, memory_total: 1 });
let usageInterval: any = null;

const getScoreClass = (score: string) => {
  // Remove quotes if present (format!("{:?}") adds quotes)
  const s = score.replace(/"/g, '');
  switch (s) {
    case 'Excellent': return 'score-excellent';
    case 'Good': return 'score-good';
    case 'Average': return 'score-average';
    case 'Poor': return 'score-poor';
    default: return 'score-unknown';
  }
};

const cleanScore = (score: string) => {
  return score.replace(/"/g, '');
};

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    // Could add a toast notification here
  } catch (err) {
    console.error('Failed to copy: ', err);
  }
}

async function runMemoryDiagnostic() {
  if (!confirm('This will launch the Windows Memory Diagnostic tool. Continue?')) return;
  try {
    await invoke('run_memory_diagnostic');
  } catch (e) {
    alert('Failed to launch diagnostic tool: ' + e);
  }
}

const fetchUsage = async () => {
  try {
    const res: any = await invoke('get_system_usage');
    usage.value = res;
  } catch (e) {
    console.error('Failed to fetch usage', e);
  }
};

onMounted(async () => {
  try {
    // Invoke the Rust command
    const res = await invoke('get_hardware_info');
    console.log(res);
    info.value = res;
    
    // Start usage polling
    fetchUsage();
    usageInterval = setInterval(fetchUsage, 2000);
  } catch (e) {
    console.error(e);
    error.value = String(e);
  } finally {
    loading.value = false;
  }
});

onUnmounted(() => {
  if (usageInterval) clearInterval(usageInterval);
});
</script>

<style scoped>
.usage-bar {
  width: 100%;
  height: 6px;
  background-color: #333;
  margin-top: 5px;
  border-radius: 3px;
  overflow: hidden;
}

.usage-fill {
  height: 100%;
  background-color: var(--cp-primary);
  transition: width 0.5s ease;
}

.driver-link {
  display: inline-block;
  margin-top: 10px;
  padding: 5px 10px;
  background-color: rgba(0, 243, 255, 0.1);
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  text-decoration: none;
  font-size: 0.8em;
  transition: all 0.3s;
}

.driver-link:hover {
  background-color: var(--cp-primary);
  color: #000;
}

.cp-value-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.text-truncate {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
  display: inline-block;
  vertical-align: middle;
}

.copy-btn {
  background: transparent;
  border: 1px solid #555;
  color: #aaa;
  cursor: pointer;
  padding: 2px 6px;
  font-size: 0.8em;
  border-radius: 3px;
}

.copy-btn:hover {
  border-color: var(--cp-primary);
  color: var(--cp-primary);
}

.cp-btn-small {
  background: transparent;
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  padding: 2px 8px;
  font-size: 0.8em;
  cursor: pointer;
  margin-left: 10px;
  vertical-align: middle;
}

.cp-btn-small:hover {
  background: var(--cp-primary);
  color: #000;
}
</style>
