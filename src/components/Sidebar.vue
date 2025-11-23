<template>
  <div class="sidebar">
    <div class="logo">
      NEKO233
    </div>
    <div class="menu">
      <div 
        v-for="item in items" 
        :key="item.id" 
        class="menu-item" 
        :class="{ active: activeId === item.id }"
        @click="$emit('select', item.id)"
      >
        <span class="icon">{{ item.icon }}</span>
        <span class="label">{{ item.label }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  activeId: string,
  items: { id: string, label: string, icon: string }[]
}>();

defineEmits(['select']);
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

.logo {
  padding: 20px;
  font-size: 1.5em;
  font-weight: bold;
  color: var(--cp-primary);
  text-shadow: 0 0 10px var(--cp-primary);
  border-bottom: 1px solid #333;
  letter-spacing: 2px;
}

.menu {
  flex: 1;
  padding: 20px 0;
}

.menu-item {
  padding: 15px 20px;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  color: #aaa;
  border-left: 3px solid transparent;
}

.menu-item:hover {
  background: rgba(0, 243, 255, 0.1);
  color: #fff;
}

.menu-item.active {
  background: rgba(0, 243, 255, 0.15);
  color: var(--cp-primary);
  border-left: 3px solid var(--cp-primary);
  text-shadow: 0 0 5px var(--cp-primary);
}

.icon {
  margin-right: 10px;
  font-size: 1.2em;
}
</style>