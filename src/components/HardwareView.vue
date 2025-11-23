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
        <div class="cp-card">
          <div class="cp-label">{{ $t('labels.bootTime') }}</div>
          <div class="cp-value">{{ formatBootTime(bootTime) }}</div>
        </div>
        <div class="cp-card">
          <div class="cp-label">{{ $t('labels.upTime') }}</div>
          <div class="cp-value">{{ formatUptime(uptime) }}</div>
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
            <div class="cp-label">{{ $t('labels.chipset') }}</div>
            <div class="cp-value">{{ mobo.Chipset }}</div>
            <div class="cp-label">{{ $t('labels.version') }}</div>
            <div class="cp-value">{{ mobo.Version }}</div>
            
            <!-- Slots Info -->
            <div class="cp-label">{{ $t('labels.ssdSlots') }}</div>
            <div class="cp-value">{{ mobo.SsdSlots.used }} / {{ mobo.SsdSlots.total }} ({{ $t('labels.usedTotal') }})</div>
            <div class="slot-details" v-if="mobo.SsdSlots.details.length > 0">
              <div v-for="detail in mobo.SsdSlots.details" :key="detail" class="slot-item">{{ detail }}</div>
            </div>

            <div class="cp-label">{{ $t('labels.gpuSlots') }}</div>
            <div class="cp-value">{{ mobo.GpuSlots.used }} / {{ mobo.GpuSlots.total }} ({{ $t('labels.usedTotal') }})</div>
            <div class="slot-details" v-if="mobo.GpuSlots.details.length > 0">
              <div v-for="detail in mobo.GpuSlots.details" :key="detail" class="slot-item">{{ detail }}</div>
            </div>

            <a v-if="getDriverLink(mobo.Manufacturer, mobo.Product)" :href="getDriverLink(mobo.Manufacturer, mobo.Product) || undefined" target="_blank" class="driver-link">
              {{ $t('labels.downloadDriver') }}
            </a>
          </div>
        </div>
      </div>

      <!-- CPU -->
      <div class="cp-section">
        <div class="cp-section-title">
          {{ $t('sections.cpu') }}
          <button class="cp-icon-btn" @click="showCpuInfo = true" :title="$t('infoBtn.cpu')">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>
          </button>
        </div>
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
        <div class="cp-section-title">
          {{ $t('sections.gpu') }}
          <button class="cp-icon-btn" @click="showGpuInfo = true" :title="$t('infoBtn.gpu')">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>
          </button>
        </div>
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

    <!-- CPU Info Modal -->
    <div v-if="showCpuInfo" class="modal-overlay" @click.self="showCpuInfo = false">
      <div class="modal-content">
        <button class="close-btn" @click="showCpuInfo = false">×</button>
        <h2>{{ $t('cpuInfo.title') }}</h2>
        
        <div class="info-block">
          <h3>Intel</h3>
          <p><strong>Core i3/i5/i7/i9:</strong> {{ $t('cpuInfo.intelTiers') }}</p>
          <p><strong>Core Ultra:</strong> {{ $t('cpuInfo.intelUltra') }}</p>
          <p><strong>Generation:</strong> {{ $t('cpuInfo.intelGen') }}</p>
          <p><strong>Server:</strong> {{ $t('cpuInfo.intelServer') }}</p>
          <p><strong>Suffixes:</strong></p>
          <ul>
            <li><strong>F:</strong> {{ $t('cpuInfo.suffixF') }}</li>
            <li><strong>K:</strong> {{ $t('cpuInfo.suffixK') }}</li>
            <li><strong>KF:</strong> {{ $t('cpuInfo.suffixKF') }}</li>
          </ul>
        </div>

        <div class="info-block">
          <h3>AMD</h3>
          <p><strong>Ryzen 3/5/7/9:</strong> {{ $t('cpuInfo.amdTiers') }}</p>
          <p><strong>Generation:</strong> {{ $t('cpuInfo.amdGen') }}</p>
          <p><strong>Server:</strong> {{ $t('cpuInfo.amdServer') }}</p>
          <p><strong>Suffixes:</strong></p>
          <ul>
            <li><strong>X:</strong> {{ $t('cpuInfo.suffixX') }}</li>
            <li><strong>G:</strong> {{ $t('cpuInfo.suffixG') }}</li>
            <li><strong>X3D:</strong> {{ $t('cpuInfo.suffixX3D') }}</li>
          </ul>
        </div>

        <div class="info-block" style="border-top: 1px dashed #333; padding-top: 10px;">
          <p><strong>Timeline:</strong> {{ $t('cpuInfo.namingTimeline') }}</p>
        </div>
      </div>
    </div>

    <!-- GPU Info Modal -->
    <div v-if="showGpuInfo" class="modal-overlay" @click.self="showGpuInfo = false">
      <div class="modal-content wide-modal">
        <button class="close-btn" @click="showGpuInfo = false">×</button>
        <h2>{{ $t('gpuInfo.title') }}</h2>
        
        <div class="table-container">
          <table class="info-table">
            <thead>
              <tr>
                <th>Brand</th>
                <th>Tiers</th>
                <th>Series</th>
                <th>Suffixes</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td class="brand-nvidia">NVIDIA</td>
                <td>{{ $t('gpuInfo.nvidiaTiers') }}</td>
                <td>{{ $t('gpuInfo.nvidiaSeries') }}</td>
                <td>{{ $t('gpuInfo.nvidiaSuffix') }}</td>
              </tr>
              <tr>
                <td class="brand-amd">AMD</td>
                <td>{{ $t('gpuInfo.amdTiers') }}</td>
                <td>{{ $t('gpuInfo.amdSeries') }}</td>
                <td>{{ $t('gpuInfo.amdSuffix') }}</td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="info-block" style="border-top: 1px dashed #333; padding-top: 10px; margin-top: 15px;">
          <p><strong>Timeline:</strong> {{ $t('gpuInfo.timeline') }}</p>
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
const showCpuInfo = ref(false);
const showGpuInfo = ref(false);
const bootTime = ref(0);
const uptime = ref(0);
let usageInterval: any = null;

const formatBootTime = (timestamp: number) => {
  if (!timestamp) return '...';
  const date = new Date(timestamp * 1000);
  return date.toLocaleString();
};

const formatUptime = (seconds: number) => {
  if (!seconds) return '...';
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  return `${h}h ${m}m`;
};

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
    bootTime.value = await invoke('get_boot_time');
    uptime.value = await invoke('get_uptime');
    
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
.cp-icon-btn {
  background: rgba(0, 243, 255, 0.1);
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  border-radius: 4px;
  width: 24px;
  height: 24px;
  padding: 0;
  cursor: pointer;
  margin-left: 10px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s;
  box-shadow: 0 0 5px var(--cp-primary);
}

.cp-icon-btn:hover {
  background: var(--cp-primary);
  color: #000;
  box-shadow: 0 0 15px var(--cp-primary);
  transform: scale(1.1);
}



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



.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 10000;
}

.modal-content {
  background: #111;
  border: 1px solid var(--cp-primary);
  padding: 20px;
  width: 500px;
  max-width: 90%;
  position: relative;
  box-shadow: 0 0 20px rgba(0, 243, 255, 0.2);
  color: #eee;
}

.close-btn {
  position: absolute;
  top: 10px;
  right: 10px;
  background: transparent;
  border: none;
  color: #aaa;
  font-size: 1.5em;
  cursor: pointer;
}

.close-btn:hover {
  color: #fff;
}

.info-block {
  margin-bottom: 20px;
}

.info-block h3 {
  color: var(--cp-primary);
  border-bottom: 1px solid #333;
  padding-bottom: 5px;
}

.info-block ul {
  padding-left: 20px;
  color: #ccc;
}

.info-block li {
  margin-bottom: 5px;
}

.wide-modal {
  width: 800px;
  max-width: 95%;
}

.table-container {
  overflow-x: auto;
  margin-top: 15px;
}

.info-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9em;
}

.info-table th,
.info-table td {
  border: 1px solid #333;
  padding: 10px;
  text-align: left;
}

.info-table th {
  background: rgba(0, 229, 255, 0.1);
  color: var(--cp-primary);
}

.info-table td {
  background: rgba(0, 0, 0, 0.3);
}

.brand-nvidia {
  color: #76b900;
  font-weight: bold;
}

.brand-amd {
  color: #ff003c;
  font-weight: bold;
}

.slot-details {
  font-size: 0.8em;
  color: #888;
  margin-top: 5px;
  padding-left: 10px;
  border-left: 2px solid #333;
}
.slot-item {
  margin-bottom: 2px;
}
</style>
