<template>
  <div class="sidebar">
    <div class="logo-container" :class="currentEffect">
      <div class="logo" data-text="NEKO233">NEKO233</div>
      <div class="logo-subtitle">HARDWARE_VIEWER</div>
      <div class="scanline"></div>
    </div>
    <div class="menu">
      <div 
        v-for="(item, index) in items" 
        :key="item.id" 
        class="menu-item" 
        :class="{ active: activeId === item.id, dragging: dragIndex === index, 'drag-over': dragOverIndex === index }"
        draggable="true"
        @dragstart="onDragStart($event, index)"
        @dragover.prevent="onDragOver($event, index)"
        @drop="onDrop($event, index)"
        @dragend="onDragEnd"
        @click="$emit('select', item.id)"
      >
        <span class="icon">{{ item.icon }}</span>
        <span class="label">{{ item.label }}</span>
      </div>
    </div>
    <div class="sidebar-footer">
      <!-- Task Manager Area -->
      <div v-if="activeTasks.length > 0" class="task-manager">
        <div class="task-header">{{ $t('taskManager.title') || 'TASKS' }}</div>
        <div v-for="task in activeTasks" :key="task.id" class="task-item">
          <div class="task-name">{{ task.name }}</div>
          <div class="task-progress-bar">
            <div class="task-progress-fill" :style="{ width: task.progress + '%' }"></div>
          </div>
        </div>
      </div>
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useTasks } from '../composables/useTasks';

const { activeTasks } = useTasks();

defineProps<{
  activeId: string,
  items: { id: string, label: string, icon: string }[]
}>();

const emit = defineEmits(['select', 'reorder']);

const effects = ['effect-neon', 'effect-hologram', 'effect-cyber', 'effect-matrix'];
const currentEffect = ref(effects[0]);
let effectInterval: any = null;
const dragIndex = ref<number | null>(null);
const dragOverIndex = ref<number | null>(null);

const onDragStart = (e: DragEvent, index: number) => {
  dragIndex.value = index;
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.dropEffect = 'move';
    e.dataTransfer.setData('text/plain', index.toString());
  }
};

const onDragOver = (_e: DragEvent, index: number) => {
  if (dragIndex.value !== null && dragIndex.value !== index) {
    dragOverIndex.value = index;
  }
};

const onDrop = (_e: DragEvent, index: number) => {
  if (dragIndex.value !== null && dragIndex.value !== index) {
    emit('reorder', { from: dragIndex.value, to: index });
  }
  dragIndex.value = null;
  dragOverIndex.value = null;
};

const onDragEnd = () => {
  dragIndex.value = null;
  dragOverIndex.value = null;
};

onMounted(() => {
  effectInterval = setInterval(() => {
    const randomIndex = Math.floor(Math.random() * effects.length);
    currentEffect.value = effects[randomIndex];
  }, 5000); // Change effect every 5 seconds
});

onUnmounted(() => {
  if (effectInterval) clearInterval(effectInterval);
});
</script>

<style scoped>
.sidebar {
  width: 250px;
  height: 100vh;
  background: rgba(0, 0, 0, 0.8);
  border-right: 1px solid var(--cp-primary);
  display: flex;
  flex-direction: column;
  backdrop-filter: blur(10px);
}

.menu {
  flex: 1;
  overflow-y: auto;
}

.sidebar-footer {
  padding: 20px;
  border-top: 1px solid #333;
  background: rgba(0, 0, 0, 0.5);
}

.task-manager {
  margin-bottom: 15px;
  padding-bottom: 15px;
  border-bottom: 1px dashed #333;
}

.task-header {
  font-size: 0.8em;
  color: #888;
  margin-bottom: 5px;
  text-transform: uppercase;
}

.task-item {
  margin-bottom: 5px;
}

.task-name {
  font-size: 0.9em;
  color: var(--cp-primary);
  margin-bottom: 2px;
}

.task-progress-bar {
  height: 4px;
  background: #333;
  border-radius: 2px;
  overflow: hidden;
}

.task-progress-fill {
  height: 100%;
  background: var(--cp-primary);
  transition: width 0.3s ease;
}

.logo-container {
  padding: 20px;
  border-bottom: 1px solid #333;
  background: #000;
  position: relative;
  overflow: hidden;
  height: 80px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  transition: all 0.5s ease;
}

.logo {
  font-size: 2em;
  font-weight: 900;
  color: #fff;
  letter-spacing: 4px;
  position: relative;
  font-family: 'Courier New', Courier, monospace;
  z-index: 2;
}

.logo-subtitle {
  font-size: 0.6em;
  color: var(--cp-primary);
  letter-spacing: 2px;
  opacity: 0.8;
  text-align: right;
  font-family: monospace;
  z-index: 2;
}

.scanline {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 2px;
  background: rgba(255, 255, 255, 0.1);
  animation: scan 3s linear infinite;
  z-index: 3;
  pointer-events: none;
}

/* Effect: Neon Pulse */
.effect-neon .logo {
  text-shadow: 0 0 5px #fff, 0 0 10px #fff, 0 0 20px var(--cp-tertiary), 0 0 30px var(--cp-tertiary), 0 0 40px var(--cp-tertiary);
  animation: neon-pulse 1.5s infinite alternate;
}
.effect-neon {
  box-shadow: inset 0 0 20px rgba(188, 19, 254, 0.2);
}

/* Effect: Hologram Flicker */
.effect-hologram .logo {
  color: rgba(0, 229, 255, 0.8);
  text-shadow: 2px 0 0 rgba(255, 0, 85, 0.5), -2px 0 0 rgba(0, 243, 255, 0.5);
  animation: hologram-flicker 0.1s infinite;
}
.effect-hologram .logo-subtitle {
  color: #fff;
  text-shadow: 0 0 5px var(--cp-primary);
}

/* Effect: Cyber Glitch (Original-ish) */
.effect-cyber .logo::before,
.effect-cyber .logo::after {
  content: attr(data-text);
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: #000;
}
.effect-cyber .logo::before {
  left: 2px;
  text-shadow: -1px 0 var(--cp-secondary);
  clip-path: inset(0 0 0 0);
  animation: glitch-anim-1 2s infinite linear alternate-reverse;
}
.effect-cyber .logo::after {
  left: -2px;
  text-shadow: -1px 0 var(--cp-primary);
  clip-path: inset(0 0 0 0);
  animation: glitch-anim-2 3s infinite linear alternate-reverse;
}

/* Effect: Matrix Data */
.effect-matrix .logo {
  color: var(--cp-success);
  text-shadow: 0 0 5px var(--cp-success);
  font-family: 'Consolas', monospace;
}
.effect-matrix .logo-subtitle {
  color: #0f0;
}
.effect-matrix {
  border-bottom: 1px solid #0f0;
}

@keyframes neon-pulse {
  from { text-shadow: 0 0 5px #fff, 0 0 10px #fff, 0 0 20px var(--cp-primary), 0 0 30px var(--cp-primary); }
  to { text-shadow: 0 0 2px #fff, 0 0 5px #fff, 0 0 10px var(--cp-primary), 0 0 15px var(--cp-primary); }
}

@keyframes hologram-flicker {
  0% { opacity: 0.9; transform: skewX(0deg); }
  20% { opacity: 0.8; transform: skewX(1deg); }
  40% { opacity: 0.95; transform: skewX(-1deg); }
  60% { opacity: 0.9; transform: skewX(0deg); }
  80% { opacity: 0.8; transform: skewX(2deg); }
  100% { opacity: 0.9; transform: skewX(0deg); }
}

@keyframes scan {
  0% { top: -10%; }
  100% { top: 110%; }
}

@keyframes glitch-anim-1 {
  0% { clip-path: inset(20% 0 80% 0); }
  20% { clip-path: inset(60% 0 10% 0); }
  40% { clip-path: inset(40% 0 50% 0); }
  60% { clip-path: inset(80% 0 5% 0); }
  80% { clip-path: inset(10% 0 70% 0); }
  100% { clip-path: inset(30% 0 20% 0); }
}

@keyframes glitch-anim-2 {
  0% { clip-path: inset(10% 0 60% 0); }
  20% { clip-path: inset(30% 0 20% 0); }
  40% { clip-path: inset(70% 0 10% 0); }
  60% { clip-path: inset(20% 0 50% 0); }
  80% { clip-path: inset(50% 0 30% 0); }
  100% { clip-path: inset(0% 0 80% 0); }
}

.menu {
  flex: 1;
  padding: 20px 0;
}

.menu-item {
  padding: 15px 20px;
  cursor: grab;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  color: #aaa;
  border-left: 3px solid transparent;
}

.menu-item:active {
  cursor: grabbing;
}

.menu-item.dragging {
  opacity: 0.5;
  background: var(--cp-secondary);
}

.menu-item.drag-over {
  border-top: 2px solid var(--cp-primary);
  transform: translateY(2px);
}

.menu-item:hover {
  background: linear-gradient(90deg, rgba(0, 243, 255, 0.1), transparent);
  color: #fff;
  padding-left: 25px;
}

.menu-item.active {
  background: linear-gradient(90deg, rgba(188, 19, 254, 0.2), transparent);
  color: #fff;
  border-left: 3px solid var(--cp-tertiary);
  text-shadow: 0 0 8px var(--cp-tertiary);
  box-shadow: inset 5px 0 10px -5px var(--cp-tertiary);
}

.icon {
  margin-right: 10px;
  font-size: 1.2em;
}


</style>