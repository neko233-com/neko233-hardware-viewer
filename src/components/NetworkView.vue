<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('network.title')">{{ $t('network.title') }}</h1>
      <div class="cp-subtitle">{{ $t('network.subtitle') }}</div>
    </div>

    <!-- Public IP -->
    <div class="cp-section">
      <div class="cp-section-title">{{ $t('network.publicIp') }}</div>
      <div class="cp-card">
        <div class="value-container" style="justify-content: center; margin-bottom: 10px;">
            <div class="cp-value" style="font-size: 1.5em;">{{ publicIp || $t('loading') }}</div>
            <button v-if="publicIp" class="copy-btn" @click="copyToClipboard(publicIp)" :title="$t('sections.copy')">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
            </button>
        </div>
        <div class="actions">
          <button class="cp-btn" @click="fetchPublicIp">{{ $t('network.refresh') }}</button>
        </div>
      </div>
    </div>

    <!-- Local Interfaces -->
    <div class="cp-section">
      <div class="cp-section-title">{{ $t('network.localInterfaces') }}</div>
      <div v-if="loadingInterfaces" class="loading-text">{{ $t('loading') }}</div>
      <div v-else class="cp-grid">
        <div v-for="(iface, index) in interfaces" :key="index" class="cp-card">
          <div class="cp-label">{{ iface.name }}</div>
          <div class="cp-desc">{{ iface.description }}</div>
          <div class="cp-row">
            <span class="label">MAC:</span>
            <span class="value">{{ iface.mac_address }}</span>
          </div>
          <div class="cp-row" v-if="iface.ipv4 && iface.ipv4.length">
            <span class="label">IPv4:</span>
            <div class="value-container">
              <span class="value">{{ iface.ipv4.join(', ') }}</span>
              <button class="copy-btn" @click="copyToClipboard(iface.ipv4.join(', '))" :title="$t('sections.copy')">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
              </button>
            </div>
          </div>
          <div class="cp-row" v-if="iface.ipv6 && iface.ipv6.length">
            <span class="label">IPv6:</span>
            <div class="value-container">
              <span class="value">{{ iface.ipv6.join(', ') }}</span>
              <button class="copy-btn" @click="copyToClipboard(iface.ipv6.join(', '))" :title="$t('sections.copy')">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
              </button>
            </div>
          </div>
          <div class="cp-row" v-if="iface.gateway && iface.gateway.length">
            <span class="label">Gateway:</span>
            <div class="value-container">
              <span class="value">{{ iface.gateway.join(', ') }}</span>
              <button class="copy-btn" @click="copyToClipboard(iface.gateway.join(', '))" :title="$t('sections.copy')">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
              </button>
            </div>
          </div>
          <div class="cp-row" v-if="iface.dns && iface.dns.length">
            <span class="label">DNS:</span>
            <div class="value-container">
              <span class="value">{{ iface.dns.join(', ') }}</span>
              <button class="copy-btn" @click="copyToClipboard(iface.dns.join(', '))" :title="$t('sections.copy')">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
              </button>
            </div>
          </div>
          <div class="cp-row">
            <span class="label">Status:</span>
            <span class="value" :class="{ 'text-green': iface.status === 'Up', 'text-red': iface.status !== 'Up' }">
              {{ iface.status }}
            </span>
          </div>
          <div class="cp-row">
            <span class="label">Speed:</span>
            <span class="value">{{ iface.speed }}</span>
          </div>
        </div>
      </div>
      <div class="actions">
        <button class="cp-btn" @click="fetchInterfaces">{{ $t('network.refresh') }}</button>
      </div>
    </div>

    <!-- Connectivity Test -->
    <div class="cp-section">
      <div class="cp-section-title">{{ $t('network.connectivity') }}</div>
      <div class="actions mb-4">
        <button class="cp-btn" @click="runPingTest" :disabled="pinging">
          {{ pinging ? $t('network.pinging') : $t('network.pingAll') }}
        </button>
      </div>
      
      <div class="ping-grid">
        <div v-for="target in pingTargets" :key="target.host" class="ping-card" :class="getPingClass(target)">
          <div class="ping-host">{{ target.name }}</div>
          <div class="ping-addr">{{ target.host }}</div>
          <div class="ping-value">
            <span v-if="target.loading">...</span>
            <span v-else-if="target.latency_ms >= 0">{{ target.latency_ms }} ms</span>
            <span v-else>TIMEOUT</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

interface NetworkInterface {
  name: string;
  description: string;
  mac_address: string;
  ipv4: string[];
  ipv6: string[];
  gateway: string[];
  dns: string[];
  status: string;
  speed: string;
}

interface PingTarget {
  name: string;
  host: string;
  latency_ms: number;
  status: string;
  loading: boolean;
}

const interfaces = ref<NetworkInterface[]>([]);
const loadingInterfaces = ref(false);
const pinging = ref(false);
const publicIp = ref('');

const pingTargets = ref<PingTarget[]>([
  { name: 'Baidu (CN)', host: 'www.baidu.com', latency_ms: -1, status: '', loading: false },
  { name: 'Bilibili (CN)', host: 'www.bilibili.com', latency_ms: -1, status: '', loading: false },
  { name: 'Google (US)', host: 'www.google.com', latency_ms: -1, status: '', loading: false },
  { name: 'GitHub (US)', host: 'github.com', latency_ms: -1, status: '', loading: false },
  { name: 'Cloudflare (Global)', host: '1.1.1.1', latency_ms: -1, status: '', loading: false },
  { name: 'Steam (Global)', host: 'store.steampowered.com', latency_ms: -1, status: '', loading: false },
  { name: 'Local Gateway', host: '192.168.1.1', latency_ms: -1, status: '', loading: false },
]);

const fetchInterfaces = async () => {
  loadingInterfaces.value = true;
  try {
    interfaces.value = await invoke('get_network_interfaces_detailed');
  } catch (e) {
    console.error(e);
  } finally {
    loadingInterfaces.value = false;
  }
};

async function fetchPublicIp() {
  publicIp.value = '';
  try {
    const ip: string = await invoke('get_public_ip');
    publicIp.value = ip;
  } catch (e) {
    publicIp.value = 'Error';
  }
}

const runPingTest = async () => {
  pinging.value = true;
  // Reset
  pingTargets.value.forEach(t => {
    t.loading = true;
    t.latency_ms = -1;
  });

  try {
    const hosts = pingTargets.value.map(t => t.host);
    const results: any[] = await invoke('ping_hosts', { targets: hosts });
    
    results.forEach(res => {
      const target = pingTargets.value.find(t => t.host === res.host);
      if (target) {
        target.latency_ms = res.latency_ms;
        target.status = res.status;
        target.loading = false;
      }
    });
  } catch (e) {
    console.error(e);
    pingTargets.value.forEach(t => t.loading = false);
  } finally {
    pinging.value = false;
  }
};

const getPingClass = (target: PingTarget) => {
  if (target.loading) return 'ping-loading';
  if (target.latency_ms < 0) return 'ping-timeout';
  if (target.latency_ms < 50) return 'ping-good';
  if (target.latency_ms < 150) return 'ping-fair';
  return 'ping-poor';
};

const copyToClipboard = async (text: string) => {
  try {
    await writeText(text);
  } catch (err) {
    console.error('Failed to copy:', err);
  }
};

onMounted(() => {
  fetchInterfaces();
  fetchPublicIp();
});
</script>

<style scoped>
.cp-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 5px;
  font-size: 0.9rem;
}
.label {
  color: #888;
}
.value {
  color: #fff;
  font-family: 'Consolas', monospace;
}
.text-green { color: #00ff9d; }
.text-red { color: #ff0055; }

.ping-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 15px;
}

.ping-card {
  background: rgba(0, 0, 0, 0.6);
  border: 1px solid #333;
  padding: 15px;
  text-align: center;
  transition: all 0.3s ease;
}

.ping-host {
  font-weight: bold;
  color: #00e5ff;
  margin-bottom: 5px;
}

.ping-addr {
  font-size: 0.8rem;
  color: #666;
  margin-bottom: 10px;
}

.ping-value {
  font-size: 1.2rem;
  font-family: 'Consolas', monospace;
}

.ping-good { border-color: #00ff9d; box-shadow: 0 0 10px rgba(0, 255, 157, 0.2); }
.ping-good .ping-value { color: #00ff9d; }

.ping-fair { border-color: #ffd700; }
.ping-fair .ping-value { color: #ffd700; }

.ping-poor { border-color: #ff0055; }
.ping-poor .ping-value { color: #ff0055; }

.ping-timeout { border-color: #444; opacity: 0.7; }
.ping-timeout .ping-value { color: #888; }

.ping-loading { border-color: #00e5ff; animation: pulse 1.5s infinite; }

@keyframes pulse {
  0% { box-shadow: 0 0 0 0 rgba(0, 229, 255, 0.4); }
  70% { box-shadow: 0 0 0 10px rgba(0, 229, 255, 0); }
  100% { box-shadow: 0 0 0 0 rgba(0, 229, 255, 0); }
}

.value-container {
  display: flex;
  align-items: center;
  gap: 8px;
}

.copy-btn {
  background: transparent;
  border: 1px solid var(--primary-color);
  color: var(--primary-color);
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.7;
  transition: all 0.2s;
}

.copy-btn:hover {
  opacity: 1;
  background: rgba(0, 243, 255, 0.1);
}
</style>
