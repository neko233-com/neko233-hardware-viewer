<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" data-text="NEKO233 HARDWARE">NEKO233 HARDWARE</h1>
      <div class="cp-subtitle">SYSTEM DIAGNOSTICS & SCORING</div>
    </div>

    <div v-if="loading" class="loading-text">
      INITIALIZING SCAN...
    </div>

    <div v-else-if="error" class="cp-section">
      <div class="cp-section-title" style="color: var(--cp-secondary)">SYSTEM ERROR</div>
      <div class="cp-value">{{ error }}</div>
    </div>

    <div v-else>
      <!-- Motherboard -->
      <div class="cp-section">
        <div class="cp-section-title">Motherboard</div>
        <div class="cp-grid">
          <div v-for="(mobo, index) in info.motherboard" :key="index" class="cp-card">
            <div class="cp-label">Manufacturer</div>
            <div class="cp-value">{{ mobo.Manufacturer }}</div>
            <div class="cp-label">Product</div>
            <div class="cp-value">{{ mobo.Product }}</div>
            <div class="cp-label">Version</div>
            <div class="cp-value">{{ mobo.Version }}</div>
          </div>
        </div>
      </div>

      <!-- CPU -->
      <div class="cp-section">
        <div class="cp-section-title">CPU Unit</div>
        <div class="cp-grid">
          <div v-for="(cpu, index) in info.cpu" :key="index" class="cp-card">
            <div class="cp-label">Model</div>
            <div class="cp-value">{{ cpu.info.Name }}</div>
            <div class="cp-label">Cores / Threads</div>
            <div class="cp-value">{{ cpu.info.NumberOfCores }} / {{ cpu.info.NumberOfLogicalProcessors }}</div>
            <div class="cp-label">Clock Speed</div>
            <div class="cp-value">{{ cpu.info.MaxClockSpeed }} MHz</div>
            <div class="cp-score" :class="getScoreClass(cpu.score)">
              SCORE: {{ cpu.score }}
            </div>
          </div>
        </div>
      </div>

      <!-- GPU -->
      <div class="cp-section">
        <div class="cp-section-title">Graphics Unit</div>
        <div class="cp-grid">
          <div v-for="(gpu, index) in info.gpu" :key="index" class="cp-card">
            <div class="cp-label">Name</div>
            <div class="cp-value">{{ gpu.info.Name }}</div>
            <div class="cp-label">Driver</div>
            <div class="cp-value">{{ gpu.info.DriverVersion }}</div>
            <div class="cp-label">VRAM</div>
            <div class="cp-value">
              {{ gpu.info.AdapterRAM ? (gpu.info.AdapterRAM / 1024 / 1024 / 1024).toFixed(2) + ' GB' : 'Unknown' }}
            </div>
            <div class="cp-score" :class="getScoreClass(gpu.score)">
              SCORE: {{ gpu.score }}
            </div>
          </div>
        </div>
      </div>

      <!-- RAM -->
      <div class="cp-section">
        <div class="cp-section-title">Memory (RAM)</div>
        <div class="cp-card">
          <div class="cp-grid">
            <div>
              <div class="cp-label">Total Capacity</div>
              <div class="cp-value">{{ info.ram.total_gb }} GB</div>
            </div>
            <div>
              <div class="cp-label">Avg Speed</div>
              <div class="cp-value">{{ info.ram.avg_speed }} MHz</div>
            </div>
          </div>
          <div class="cp-score" :class="getScoreClass(info.ram.score)">
            SCORE: {{ info.ram.score }}
          </div>
          <div style="margin-top: 15px; border-top: 1px dashed #333; padding-top: 10px;">
            <div v-for="(mem, idx) in info.ram.info" :key="idx" style="margin-bottom: 5px; font-size: 0.9em; color: #aaa;">
              Slot {{ mem.DeviceLocator }}: {{ mem.Manufacturer }} {{ (mem.Capacity / 1024 / 1024 / 1024).toFixed(0) }}GB @ {{ mem.ConfiguredClockSpeed || mem.Speed }}MHz
            </div>
          </div>
        </div>
      </div>

      <!-- Storage -->
      <div class="cp-section">
        <div class="cp-section-title">Storage</div>
        <div class="cp-grid">
          <div v-for="(disk, index) in info.disks" :key="index" class="cp-card">
            <div class="cp-label">Model</div>
            <div class="cp-value">{{ disk.info.Model }}</div>
            <div class="cp-label">Size</div>
            <div class="cp-value">
              {{ disk.info.Size ? (disk.info.Size / 1024 / 1024 / 1024).toFixed(2) + ' GB' : 'Unknown' }}
            </div>
            <div class="cp-label">Interface</div>
            <div class="cp-value">{{ disk.info.InterfaceType || 'Unknown' }}</div>
            <div class="cp-score" :class="getScoreClass(disk.score)">
              SCORE: {{ disk.score }}
            </div>
          </div>
        </div>
      </div>

      <!-- Sound -->
      <div class="cp-section">
        <div class="cp-section-title">Audio Devices</div>
        <div class="cp-grid">
          <div v-for="(snd, index) in info.sound" :key="index" class="cp-card">
            <div class="cp-label">Name</div>
            <div class="cp-value">{{ snd.Name }}</div>
            <div class="cp-label">Status</div>
            <div class="cp-value">{{ snd.Status || 'Unknown' }}</div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const loading = ref(true);
const error = ref('');
const info = ref<any>(null);

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

onMounted(async () => {
  try {
    // Invoke the Rust command
    const res = await invoke('get_hardware_info');
    console.log(res);
    info.value = res;
  } catch (e) {
    console.error(e);
    error.value = String(e);
  } finally {
    loading.value = false;
  }
});
</script>

<style scoped>
/* Scoped styles if needed, but global cyberpunk.css handles most */
</style>
