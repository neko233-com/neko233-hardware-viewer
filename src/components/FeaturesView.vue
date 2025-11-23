<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('features.title')">{{ $t('features.title') }}</h1>
      <div class="cp-subtitle">{{ $t('features.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-card">
        <div class="cp-label">{{ $t('features.fakeShutdown') }}</div>
        <div class="desc-text">{{ $t('features.fakeShutdownDesc') }}</div>
        <button class="cp-button danger" @click="startFakeMode('shutdown')">
          {{ $t('features.start') }}
        </button>
      </div>

      <div class="cp-card">
        <div class="cp-label">{{ $t('features.fakeUpdate') }}</div>
        <div class="desc-text">{{ $t('features.fakeUpdateDesc') }}</div>
        <button class="cp-button primary" @click="startFakeMode('update')">
          {{ $t('features.start') }}
        </button>
      </div>

      <div class="cp-card">
        <div class="cp-label">{{ $t('features.fakeBsod') }}</div>
        <div class="desc-text">{{ $t('features.fakeBsodDesc') }}</div>
        <button class="cp-button primary" @click="startFakeMode('bsod')">
          {{ $t('features.start') }}
        </button>
      </div>
    </div>

    <!-- Overlays -->
    <div v-if="mode === 'shutdown'" class="overlay shutdown-overlay">
      <div class="shutdown-content">
        <div class="spinner"></div>
        <div class="text">Shutting down...</div>
      </div>
    </div>

    <div v-if="mode === 'update'" class="overlay update-overlay">
      <div class="update-content">
        <div class="spinner"></div>
        <div class="text">Working on updates {{ updateProgress }}% complete</div>
        <div class="subtext">Don't turn off your computer</div>
      </div>
    </div>

    <div v-if="mode === 'bsod'" class="overlay bsod-overlay">
      <div class="bsod-content">
        <div class="sad-face">:(</div>
        <div class="bsod-text">
          Your PC ran into a problem and needs to restart. We're just collecting some error info, and then we'll restart for you.
        </div>
        <div class="bsod-progress">{{ updateProgress }}% complete</div>
        <div class="bsod-qr">
          <div class="qr-placeholder"></div>
          <div class="bsod-details">
            For more information about this issue and possible fixes, visit https://www.windows.com/stopcode
            <br><br>
            If you call a support person, give them this info:
            <br>
            Stop code: CRITICAL_PROCESS_DIED
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { appWindow } from '@tauri-apps/api/window';

const mode = ref<'none' | 'shutdown' | 'update' | 'bsod'>('none');
const updateProgress = ref(0);
let progressInterval: any = null;

function startFakeMode(m: 'shutdown' | 'update' | 'bsod') {
  mode.value = m;
  appWindow.setFullscreen(true);
  updateProgress.value = 0;
  
  if (progressInterval) clearInterval(progressInterval);
  progressInterval = setInterval(() => {
    if (updateProgress.value < 99) {
      updateProgress.value += Math.floor(Math.random() * 3);
      if (updateProgress.value > 100) updateProgress.value = 99;
    }
  }, 2000);
}

function exitFakeMode() {
  if (mode.value !== 'none') {
    mode.value = 'none';
    appWindow.setFullscreen(false);
    if (progressInterval) clearInterval(progressInterval);
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    exitFakeMode();
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
  if (progressInterval) clearInterval(progressInterval);
});
</script>

<style scoped>
.desc-text {
  margin: 10px 0;
  color: #aaa;
  font-size: 0.9em;
}

.cp-button {
  margin-top: 10px;
  width: 100%;
}

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 99999;
  cursor: none;
}

/* Shutdown */
.shutdown-overlay {
  background-color: #000;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-family: 'Segoe UI', sans-serif;
}
.shutdown-content {
  text-align: center;
}
.shutdown-overlay .spinner {
  width: 50px;
  height: 50px;
  border: 5px solid transparent;
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}
.shutdown-overlay .text {
  font-size: 1.5em;
}

/* Update */
.update-overlay {
  background-color: #0067c0; /* Win10/11 Blue */
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-family: 'Segoe UI', sans-serif;
}
.update-content {
  text-align: center;
}
.update-overlay .spinner {
  width: 50px;
  height: 50px;
  border: 5px solid transparent;
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}
.update-overlay .text {
  font-size: 1.5em;
  margin-bottom: 10px;
}
.update-overlay .subtext {
  font-size: 1em;
  opacity: 0.8;
}

/* BSOD */
.bsod-overlay {
  background-color: #0078d7;
  color: white;
  font-family: 'Segoe UI', sans-serif;
  padding: 10% 15%;
  box-sizing: border-box;
}
.sad-face {
  font-size: 10em;
  margin-bottom: 20px;
}
.bsod-text {
  font-size: 1.5em;
  margin-bottom: 20px;
  line-height: 1.4;
}
.bsod-progress {
  font-size: 1.2em;
  margin-bottom: 40px;
}
.bsod-qr {
  display: flex;
  align-items: center;
}
.qr-placeholder {
  width: 100px;
  height: 100px;
  background: white;
  margin-right: 20px;
}
.bsod-details {
  font-size: 0.9em;
  line-height: 1.4;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>