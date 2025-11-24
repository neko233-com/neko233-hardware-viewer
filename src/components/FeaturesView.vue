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

      <div class="cp-card">
        <div class="cp-label">{{ $t('features.cyberpunk') }}</div>
        <div class="desc-text">{{ $t('features.cyberpunkDesc') }}</div>
        <button class="cp-button primary" @click="startFakeMode('cyberpunk')">
          {{ $t('features.start') }}
        </button>
      </div>
    </div>

    <div class="f11-hint">
      <span v-html="$t('features.f11Hint').replace('Ctrl + F11', '<span class=\'key\'>Ctrl + F11</span>')"></span>
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

    <div v-if="mode === 'cyberpunk'" class="overlay cyberpunk-overlay">
      <div class="cp-bg-anim"></div>
      <div class="cp-content">
        <h1 class="glitch" data-text="NEKO233">NEKO233</h1>
        <div class="scanner-line"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const mode = ref<'none' | 'shutdown' | 'update' | 'bsod' | 'cyberpunk'>('none');
const updateProgress = ref(0);
let progressInterval: any = null;

function startFakeMode(m: 'shutdown' | 'update' | 'bsod' | 'cyberpunk') {
  mode.value = m;
  getCurrentWindow().setFullscreen(true);
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
    getCurrentWindow().setFullscreen(false);
    if (progressInterval) clearInterval(progressInterval);
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.ctrlKey && e.key === 'F11') {
    e.preventDefault(); // Prevent default browser F11
    if (mode.value !== 'none') {
      exitFakeMode();
    } else {
      getCurrentWindow().isFullscreen().then(isFull => {
        getCurrentWindow().setFullscreen(!isFull);
      });
    }
  } else if (e.key === 'Escape') {
    if (mode.value !== 'none') {
      exitFakeMode();
    }
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
/* ... existing styles ... */
.f11-hint {
  margin-top: 20px;
  text-align: center;
  color: #555;
  font-family: 'Courier New', Courier, monospace;
  font-size: 0.9em;
  border: 1px dashed #333;
  padding: 10px;
}

.f11-hint .key {
  color: var(--cp-primary);
  border: 1px solid var(--cp-primary);
  padding: 2px 5px;
  border-radius: 3px;
  font-weight: bold;
}

.cyberpunk-overlay {
  background: #000;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 9999;
}

.cp-bg-anim {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: 
    linear-gradient(90deg, rgba(0,243,255,0.03) 1px, transparent 1px),
    linear-gradient(0deg, rgba(0,243,255,0.03) 1px, transparent 1px);
  background-size: 50px 50px;
  animation: gridMove 20s linear infinite;
}

@keyframes gridMove {
  0% { transform: translateY(0); }
  100% { transform: translateY(50px); }
}

.cp-content h1 {
  font-size: 5em;
  color: var(--cp-primary);
  text-shadow: 2px 2px #ff003c;
  position: relative;
}

.scanner-line {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 5px;
  background: var(--cp-primary);
  box-shadow: 0 0 15px var(--cp-primary);
  opacity: 0.5;
  animation: scan 3s ease-in-out infinite;
}

@keyframes scan {
  0% { top: 0%; opacity: 0; }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% { top: 100%; opacity: 0; }
}

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