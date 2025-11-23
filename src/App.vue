<template>
  <div class="app-layout">
    <div class="drag-region" data-tauri-drag-region></div>
    <Sidebar 
      :activeId="activeTab" 
      :items="sortedMenuItems" 
      @select="activeTab = $event"
      @reorder="handleReorder"
    >
      <template #footer>
        <div class="sidebar-controls">
          <button class="settings-btn-sidebar" @click="activeTab = 'settings'">
            <span class="icon">⚙️</span>
            <span class="label">{{ $t('buttons.settings') || 'SET' }}</span>
          </button>
          <div class="window-controls">
            <button class="minimize-btn-sidebar" @click="minimizeApp" title="Minimize">_</button>
            <button class="fullscreen-btn-sidebar" @click="toggleFullscreen" title="Fullscreen">□</button>
            <button class="exit-btn-sidebar" @click="exitApp" title="Exit">×</button>
          </div>
        </div>
      </template>
    </Sidebar>
    <div class="content-area">
      <LanguageSelector class="lang-selector" />
      <component :is="activeComponent" />
    </div>

    <!-- Settings Modal Removed -->
    
    <!-- Update Toast -->
    <div v-if="showToast" class="update-toast">
      <div class="toast-content">
        <span>{{ toastMessage }}</span>
        <button v-if="updateAvailable" class="cp-btn-small" @click="startUpdate">
          {{ $t('settings.updateNow') }}
        </button>
        <button class="close-toast" @click="showToast = false">×</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import { getVersion } from '@tauri-apps/api/app';
import Sidebar from "./components/Sidebar.vue";
import HardwareView from "./components/HardwareView.vue";
import OptimizationView from "./components/OptimizationView.vue";
import DiagnosticsView from "./components/DiagnosticsView.vue";
import SettingsView from "./components/SettingsView.vue";
import RankingView from "./components/RankingView.vue";
import OfficeView from "./components/OfficeView.vue";
import ActivationView from "./components/ActivationView.vue";
import DriverCleanerView from "./components/DriverCleanerView.vue";
import Win11InstallView from "./components/Win11InstallView.vue";
import Win11TweaksView from "./components/Win11TweaksView.vue";
import ColorManagementView from "./components/ColorManagementView.vue";
import FeaturesView from "./components/FeaturesView.vue";
import LanguageSelector from "./components/LanguageSelector.vue";

const { t } = useI18n();
const activeTab = ref('hardware');
const tabOrder = ref<string[]>([]);
// const showSettings = ref(false); // Moved to SettingsView
const autostartEnabled = ref(false);
const autoUpdateEnabled = ref(true);
const appVersion = ref('0.0.0');
// const checkingUpdate = ref(false); // Moved to SettingsView
const showToast = ref(false);
const toastMessage = ref('');
const updateAvailable = ref(false);

const exitApp = async () => {
  await appWindow.close();
};

const minimizeApp = async () => {
  await appWindow.minimize();
};

const toggleFullscreen = async () => {
  const isFullscreen = await appWindow.isFullscreen();
  await appWindow.setFullscreen(!isFullscreen);
};

// const openSettings = async () => { ... } // Removed

// const toggleAutostart = async () => { ... } // Moved to SettingsView

// const saveAutoUpdate = () => { ... } // Moved to SettingsView

// const manualCheckUpdate = async () => { ... } // Moved to SettingsView

// const startUpdate = async () => { ... } // Kept for Toast

const startUpdate = async () => {
  try {
    await installUpdate();
    await appWindow.close(); 
  } catch (e) {
    alert('Update failed: ' + e);
  }
};

const autoCheckLoop = async () => {
  if (!autoUpdateEnabled.value) return;
  
  try {
    const { shouldUpdate, manifest } = await checkUpdate();
    if (shouldUpdate) {
      // Check game mode
      const isGame: boolean = await invoke('is_game_running');
      if (isGame) {
        console.log('Game detected, suppressing update notification');
        return;
      }
      
      toastMessage.value = `${t('settings.updateAvailable')} v${manifest?.version}`;
      updateAvailable.value = true;
      showToast.value = true;
    }
  } catch (e) {
    console.error('Auto update check failed', e);
  }
};

onMounted(async () => {
  appVersion.value = await getVersion();
  autostartEnabled.value = await invoke('check_autostart');
  
  const savedOrder = await invoke('load_tab_order') as string[];
  if (savedOrder && savedOrder.length > 0) {
    tabOrder.value = savedOrder;
  }

  const savedAuto = localStorage.getItem('autoUpdate');
  if (savedAuto !== null) {
    autoUpdateEnabled.value = savedAuto === 'true';
  }

  if (autoUpdateEnabled.value) {
    // Check after a small delay to let app load
    setTimeout(autoCheckLoop, 5000);
  }

  // Disable context menu (Right Click)
  document.addEventListener('contextmenu', (e) => {
    e.preventDefault();
  });

  // Global Ctrl + F11 Fullscreen Listener
  window.addEventListener('keydown', async (e) => {
    if (e.ctrlKey && e.key === 'F11') {
      e.preventDefault();
      e.stopPropagation(); // Stop propagation
      const isFullscreen = await appWindow.isFullscreen();
      await appWindow.setFullscreen(!isFullscreen);
    }
  }, true); // Use capture phase
});

const baseMenuItems = computed(() => [
  { id: 'hardware', label: t('menu.hardware'), icon: '🖥️' },
  { id: 'ranking', label: t('menu.ranking') || 'RANKING', icon: '🏆' },
  { id: 'optimization', label: t('menu.optimization'), icon: '🚀' },
  { id: 'diagnostics', label: t('menu.diagnostics') || 'DIAGNOSTICS', icon: '🩺' },
  { id: 'driver_cleaner', label: t('menu.driver_cleaner'), icon: '🧹' },
  { id: 'office', label: t('menu.office'), icon: '📝' },
  { id: 'activation', label: t('menu.activation'), icon: '🔑' },
  { id: 'win11_install', label: t('menu.win11_install'), icon: '💿' },
  { id: 'win11_tweaks', label: t('menu.win11_tweaks'), icon: '⚙️' },
  { id: 'color_management', label: t('menu.color_management'), icon: '🎨' },
  { id: 'features', label: t('menu.features'), icon: '🎭' },
]);

const sortedMenuItems = computed(() => {
  if (tabOrder.value.length === 0) return baseMenuItems.value;
  
  const orderMap = new Map(tabOrder.value.map((id, index) => [id, index]));
  
  return [...baseMenuItems.value].sort((a, b) => {
    const indexA = orderMap.has(a.id) ? orderMap.get(a.id)! : 999;
    const indexB = orderMap.has(b.id) ? orderMap.get(b.id)! : 999;
    return indexA - indexB;
  });
});

const handleReorder = async ({ from, to }: { from: number, to: number }) => {
  const currentOrder = sortedMenuItems.value.map(i => i.id);
  const item = currentOrder.splice(from, 1)[0];
  currentOrder.splice(to, 0, item);
  
  tabOrder.value = currentOrder;
  await invoke('save_tab_order', { order: currentOrder });
};

const activeComponent = computed(() => {
  switch (activeTab.value) {
    case 'hardware': return HardwareView;
    case 'ranking': return RankingView;
    case 'optimization': return OptimizationView;
    case 'diagnostics': return DiagnosticsView;
    case 'settings': return SettingsView;
    case 'driver_cleaner': return DriverCleanerView;
    case 'office': return OfficeView;
    case 'activation': return ActivationView;
    case 'win11_install': return Win11InstallView;
    case 'win11_tweaks': return Win11TweaksView;
    case 'color_management': return ColorManagementView;
    case 'features': return FeaturesView;
    default: return HardwareView;
  }
});
</script>

<style>
body {
  margin: 0;
  padding: 0;
  background-color: transparent; /* Allow transparency */
  color: #e0e0e0;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  overflow: hidden;
}

.app-layout {
  display: flex;
  height: 100vh;
  background: rgba(5, 5, 5, 0.95); /* Semi-transparent background */
  border: 1px solid #333; /* Cyberpunk border */
  border-radius: 10px;
  overflow: hidden;
  position: relative;
}

.drag-region {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 30px;
  z-index: 9999;
  -webkit-app-region: drag;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  position: relative;
  background: radial-gradient(circle at top right, #1a1a1a, #000);
  padding-top: 30px; /* Space for drag region */
}

.bottom-controls {
  position: absolute;
  bottom: 20px;
  left: 20px;
  z-index: 10000;
  display: flex;
  gap: 15px;
}

.exit-btn, .minimize-btn {
  background: #ff003c;
  color: #fff;
  border: none;
  padding: 10px 20px;
  font-family: 'Courier New', Courier, monospace;
  font-weight: bold;
  font-size: 1.2em;
  cursor: pointer;
  clip-path: polygon(10% 0, 100% 0, 100% 70%, 90% 100%, 0 100%, 0 30%);
  transition: all 0.2s ease;
  text-transform: uppercase;
  letter-spacing: 2px;
  box-shadow: 0 0 10px rgba(255, 0, 60, 0.5);
}

.minimize-btn {
  background: #00e5ff;
  color: #000;
  box-shadow: 0 0 10px rgba(0, 229, 255, 0.5);
}

.exit-btn:hover {
  background: #ff3366;
  transform: scale(1.05);
  box-shadow: 0 0 20px rgba(255, 0, 60, 0.8);
}

.sidebar-controls {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.settings-btn-sidebar {
  background: transparent;
  border: 1px solid #ffb300;
  color: #ffb300;
  padding: 10px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: all 0.2s;
  font-family: inherit;
  text-transform: uppercase;
  font-weight: bold;
}

.settings-btn-sidebar:hover {
  background: rgba(255, 179, 0, 0.1);
  box-shadow: 0 0 10px rgba(255, 179, 0, 0.3);
}

.window-controls {
  display: flex;
  gap: 10px;
}

.minimize-btn-sidebar, .exit-btn-sidebar, .fullscreen-btn-sidebar {
  flex: 1;
  padding: 8px;
  border: none;
  cursor: pointer;
  font-weight: bold;
  font-size: 1.2em;
  transition: all 0.2s;
}

.minimize-btn-sidebar {
  background: #00e5ff;
  color: #000;
  clip-path: polygon(0 0, 100% 0, 100% 70%, 80% 100%, 0 100%);
}

.fullscreen-btn-sidebar {
  background: #fcee0a;
  color: #000;
  clip-path: polygon(10% 0, 90% 0, 90% 100%, 10% 100%);
}

.exit-btn-sidebar {
  background: #ff003c;
  color: #fff;
  clip-path: polygon(20% 0, 100% 0, 100% 100%, 0 100%, 0 30%);
}

.minimize-btn-sidebar:hover {
  background: #33efff;
  box-shadow: 0 0 10px rgba(0, 229, 255, 0.5);
}

.fullscreen-btn-sidebar:hover {
  background: #fff766;
  box-shadow: 0 0 10px rgba(252, 238, 10, 0.5);
}

.exit-btn-sidebar:hover {
  background: #ff3366;
  box-shadow: 0 0 10px rgba(255, 0, 60, 0.5);
}

.lang-selector {
  position: absolute;
  top: 40px; /* Adjusted for drag region */
  right: 20px;
  z-index: 100;
}

.settings-modal {
  width: 400px;
  border: 1px solid var(--cp-primary, #00e5ff);
  box-shadow: 0 0 30px rgba(0, 229, 255, 0.2);
}

.setting-item {
  margin: 20px 0;
  font-size: 1.1em;
}

.setting-item label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
}

.setting-item input[type="checkbox"] {
  width: 20px;
  height: 20px;
  accent-color: var(--cp-primary, #00e5ff);
}

.version-check {
  border-top: 1px dashed #333;
  padding-top: 20px;
  margin-top: 30px;
}

.update-toast {
  position: fixed;
  bottom: 80px;
  right: 20px;
  background: rgba(0, 0, 0, 0.9);
  border: 1px solid var(--cp-primary, #00e5ff);
  padding: 15px 20px;
  border-radius: 5px;
  z-index: 20000;
  box-shadow: 0 0 20px rgba(0, 229, 255, 0.3);
  animation: slideIn 0.3s ease-out;
}

.toast-content {
  display: flex;
  align-items: center;
  gap: 15px;
}

.close-toast {
  background: transparent;
  border: none;
  color: #aaa;
  cursor: pointer;
  font-size: 1.2em;
}

.close-toast:hover {
  color: #fff;
}

@keyframes slideIn {
  from { transform: translateX(100%); opacity: 0; }
  to { transform: translateX(0); opacity: 1; }
}
</style>
