<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('ranking.title')">{{ $t('ranking.title') }}</h1>
      <div class="cp-subtitle">{{ $t('ranking.subtitle') }}</div>
    </div>

    <div class="tabs">
      <button 
        class="tab-btn" 
        :class="{ active: activeTab === 'cpu' }"
        @click="activeTab = 'cpu'"
      >
        CPU {{ $t('ranking.ladder') }}
      </button>
      <button 
        class="tab-btn" 
        :class="{ active: activeTab === 'gpu' }"
        @click="activeTab = 'gpu'"
      >
        GPU {{ $t('ranking.ladder') }}
      </button>
    </div>

    <div class="cp-section">
      <div class="cp-grid ranking-grid">
        <div v-for="(item, index) in currentList" :key="index" class="cp-card ranking-card">
          <div class="rank-num" :class="'rank-' + (index + 1)">#{{ index + 1 }}</div>
          <div class="rank-info">
            <div class="rank-name">{{ item.name }}</div>
            <div class="rank-score">{{ item.score }} pts</div>
          </div>
          <div class="rank-bar-container">
            <div class="rank-bar" :style="{ width: (item.score / maxScore * 100) + '%' }"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

const activeTab = ref('cpu');

// Mock Data - In a real app, this would come from an API or JSON file
const cpuList = [
  { name: 'AMD Ryzen 9 9950X3D', score: 72000 },
  { name: 'Intel Core Ultra 9 285K', score: 68000 },
  { name: 'AMD Ryzen 9 9950X', score: 67000 },
  { name: 'Intel Core i9-14900K', score: 62000 },
  { name: 'AMD Ryzen 9 7950X3D', score: 60500 },
  { name: 'Intel Core i9-13900K', score: 59000 },
  { name: 'AMD Ryzen 7 9800X3D', score: 58500 },
  { name: 'AMD Ryzen 9 7950X', score: 58000 },
  { name: 'Intel Core i7-14700K', score: 54000 },
  { name: 'AMD Ryzen 7 7800X3D', score: 48000 },
];

const gpuList = [
  { name: 'NVIDIA GeForce RTX 5090', score: 58000 },
  { name: 'NVIDIA GeForce RTX 5080', score: 45000 },
  { name: 'NVIDIA GeForce RTX 4090', score: 38000 },
  { name: 'NVIDIA GeForce RTX 4080 Super', score: 32000 },
  { name: 'AMD Radeon RX 7900 XTX', score: 31000 },
  { name: 'NVIDIA GeForce RTX 4080', score: 30500 },
  { name: 'NVIDIA GeForce RTX 4070 Ti Super', score: 28000 },
  { name: 'AMD Radeon RX 7900 XT', score: 27000 },
];

const currentList = computed(() => activeTab.value === 'cpu' ? cpuList : gpuList);
const maxScore = computed(() => Math.max(...currentList.value.map(i => i.score)));

</script>

<style scoped>
.cp-container {
  padding: 20px;
  height: 100%;
  overflow-y: auto;
  color: #fff;
}

.cp-header {
  margin-bottom: 20px;
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

.tabs {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
  border-bottom: 1px solid #333;
}

.tab-btn {
  background: transparent;
  border: none;
  color: #888;
  font-size: 1.2em;
  padding: 10px 20px;
  cursor: pointer;
  font-family: 'Courier New', Courier, monospace;
  font-weight: bold;
  transition: all 0.3s;
  border-bottom: 2px solid transparent;
}

.tab-btn:hover {
  color: #fff;
}

.tab-btn.active {
  color: var(--cp-primary);
  border-bottom-color: var(--cp-primary);
  text-shadow: 0 0 10px var(--cp-primary);
}

.ranking-grid {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.ranking-card {
  display: flex;
  align-items: center;
  padding: 15px;
  background: rgba(0, 0, 0, 0.6);
  border: 1px solid #333;
  position: relative;
  overflow: hidden;
}

.rank-num {
  font-size: 1.5em;
  font-weight: bold;
  width: 60px;
  color: #888;
}

.rank-1 { color: #ffd700; text-shadow: 0 0 10px #ffd700; }
.rank-2 { color: #c0c0c0; text-shadow: 0 0 10px #c0c0c0; }
.rank-3 { color: #cd7f32; text-shadow: 0 0 10px #cd7f32; }

.rank-info {
  flex: 1;
  z-index: 2;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-right: 20px;
}

.rank-name {
  font-size: 1.1em;
  font-weight: bold;
}

.rank-score {
  font-family: 'Courier New', Courier, monospace;
  color: var(--cp-primary);
}

.rank-bar-container {
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  z-index: 1;
  opacity: 0.1;
}

.rank-bar {
  height: 100%;
  background: var(--cp-primary);
  transition: width 1s ease-out;
}
</style>
