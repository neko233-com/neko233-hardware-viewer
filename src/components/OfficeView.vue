<script setup lang="ts">
// Office management view
</script>

<template>
  <div class="cp-container">
    <div class="cp-header">
      <h1 class="cp-title glitch" :data-text="$t('office.title')">{{ $t('office.title') }}</h1>
      <div class="cp-subtitle">{{ $t('office.subtitle') }}</div>
    </div>

    <div class="cp-section">
      <div class="cp-card">
        <div class="cp-label">{{ $t('office.portal') }}</div>
        <div class="cp-value">{{ $t('office.portalDesc') }}</div>
        <div class="actions">
          <a href="https://www.office.com" target="_blank" class="cp-btn">{{ $t('office.openPortal') }}</a>
        </div>
        <div class="info-text">
          {{ $t('office.portalInfo') }}
        </div>
      </div>
      
      <div class="cp-card">
        <div class="cp-label">{{ $t('office.tool') }}</div>
        <div class="cp-value">{{ $t('office.toolDesc') }}</div>
        <div class="actions">
          <a href="https://config.office.com/" target="_blank" class="cp-btn">{{ $t('office.openTool') }}</a>
        </div>
        <div class="info-text">
          {{ $t('office.toolInfo') }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.actions {
  margin-top: 15px;
}

.cp-btn {
  display: inline-block;
  background: transparent;
  border: 1px solid var(--cp-primary);
  color: var(--cp-primary);
  padding: 8px 16px;
  cursor: pointer;
  font-family: 'Segoe UI', sans-serif;
  transition: all 0.3s;
  text-transform: uppercase;
  letter-spacing: 1px;
  text-decoration: none;
}

.cp-btn:hover {
  background: var(--cp-primary);
  color: #000;
  box-shadow: 0 0 10px var(--cp-primary);
}

.info-text {
  font-size: 0.9em;
  color: #888;
  margin-top: 10px;
}
</style>