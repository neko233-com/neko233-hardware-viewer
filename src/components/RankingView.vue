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

    <div class="search-bar">
      <input 
        v-model="searchQuery" 
        type="text" 
        class="cp-input" 
        :placeholder="$t('ranking.searchPlaceholder') || 'Search Hardware...'"
      />
    </div>

    <div class="cp-section">
      <div class="cp-grid ranking-grid">
        <div v-for="(item, index) in filteredList" :key="index" class="cp-card ranking-card">
          <div class="rank-num" :class="'rank-' + (index + 1)">#{{ index + 1 }}</div>
          <div class="rank-info">
            <div class="rank-name">
              {{ item.name }}
              <span v-if="activeTab === 'cpu' && item.hasIGPU !== undefined" 
                    class="igpu-badge" 
                    :class="{ 'has-igpu': item.hasIGPU, 'no-igpu': !item.hasIGPU }"
                    :title="item.hasIGPU ? $t('ranking.hasIGPU') : $t('ranking.noIGPU')">
                {{ item.hasIGPU ? 'iGPU' : 'No iGPU' }}
              </span>
            </div>
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
import { cpuRanking, gpuRanking } from '../config/rankingData';

interface RankingItem {
  name: string;
  score: number;
  hasIGPU?: boolean;
}

const activeTab = ref('cpu');
const searchQuery = ref('');

const currentList = computed<RankingItem[]>(() => {
  return activeTab.value === 'cpu' ? cpuRanking : gpuRanking;
});

const filteredList = computed<RankingItem[]>(() => {
  const query = searchQuery.value.toLowerCase();
  let list = currentList.value;
  
  if (query) {
    list = list.filter((item: RankingItem) => item.name.toLowerCase().includes(query));
  }
  
  return list.sort((a: RankingItem, b: RankingItem) => b.score - a.score);
});

const maxScore = computed(() => {
  if (filteredList.value.length === 0) return 100;
  return Math.max(...filteredList.value.map((i: RankingItem) => i.score));
});
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

.search-bar {
  margin-bottom: 20px;
  display: flex;
  justify-content: center;
}

.cp-input {
  width: 100%;
  max-width: 400px;
  padding: 10px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--cp-primary);
  color: #fff;
  font-family: inherit;
  font-size: 1em;
}

.cp-input:focus {
  outline: none;
  box-shadow: 0 0 10px var(--cp-primary);
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

.igpu-badge {
  font-size: 0.7em;
  padding: 2px 6px;
  border-radius: 4px;
  margin-left: 8px;
  vertical-align: middle;
  font-weight: normal;
}

.has-igpu {
  background-color: rgba(0, 255, 0, 0.1);
  border: 1px solid #0f0;
  color: #0f0;
}

.no-igpu {
  background-color: rgba(255, 0, 0, 0.1);
  border: 1px solid #f00;
  color: #f00;
}
</style>
