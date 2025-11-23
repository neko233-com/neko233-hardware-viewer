<template>
  <div class="app-layout">
    <Sidebar :activeId="activeTab" :items="menuItems" @select="activeTab = $event" />
    <div class="content-area">
      <LanguageSelector class="lang-selector" />
      <component :is="activeComponent" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import Sidebar from "./components/Sidebar.vue";
import HardwareView from "./components/HardwareView.vue";
import OptimizationView from "./components/OptimizationView.vue";
import OfficeView from "./components/OfficeView.vue";
import ActivationView from "./components/ActivationView.vue";
import DriverCleanerView from "./components/DriverCleanerView.vue";
import Win11InstallView from "./components/Win11InstallView.vue";
import Win11TweaksView from "./components/Win11TweaksView.vue";
import FeaturesView from "./components/FeaturesView.vue";
import LanguageSelector from "./components/LanguageSelector.vue";

const { t } = useI18n();
const activeTab = ref('hardware');

const menuItems = computed(() => [
  { id: 'hardware', label: t('menu.hardware'), icon: '🖥️' },
  { id: 'optimization', label: t('menu.optimization'), icon: '🚀' },
  { id: 'driver_cleaner', label: t('menu.driver_cleaner'), icon: '🧹' },
  { id: 'office', label: t('menu.office'), icon: '📝' },
  { id: 'activation', label: t('menu.activation'), icon: '🔑' },
  { id: 'win11_install', label: t('menu.win11_install'), icon: '💿' },
  { id: 'win11_tweaks', label: t('menu.win11_tweaks'), icon: '⚙️' },
  { id: 'features', label: t('menu.features'), icon: '🎭' },
]);

const activeComponent = computed(() => {
  switch (activeTab.value) {
    case 'hardware': return HardwareView;
    case 'optimization': return OptimizationView;
    case 'driver_cleaner': return DriverCleanerView;
    case 'office': return OfficeView;
    case 'activation': return ActivationView;
    case 'win11_install': return Win11InstallView;
    case 'win11_tweaks': return Win11TweaksView;
    case 'features': return FeaturesView;
    default: return HardwareView;
  }
});
</script>

<style>
body {
  margin: 0;
  padding: 0;
  background-color: #050505;
  color: #e0e0e0;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  overflow: hidden;
}

.app-layout {
  display: flex;
  height: 100vh;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  position: relative;
  background: radial-gradient(circle at top right, #1a1a1a, #000);
}

.lang-selector {
  position: absolute;
  top: 20px;
  right: 20px;
  z-index: 100;
}
</style>
