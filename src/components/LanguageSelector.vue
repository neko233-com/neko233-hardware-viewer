<template>
  <div class="language-selector">
    <el-select
      v-model="currentLocale"
      filterable
      placeholder="Select Language"
      class="lang-select"
      @change="handleLanguageChange"
      size="small"
    >
      <el-option
        v-for="item in options"
        :key="item.value"
        :label="item.label"
        :value="item.value"
      />
    </el-select>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()
const currentLocale = ref(locale.value)

const options = [
  { value: 'zh', label: '简体中文' },
  { value: 'en', label: 'English' }
]

const handleLanguageChange = (val: string) => {
  locale.value = val
}

// Sync if locale changes externally
watch(locale, (newVal) => {
  currentLocale.value = newVal as string
})
</script>

<style scoped>
.language-selector {
  position: absolute;
  top: 20px;
  right: 20px;
  z-index: 1000;
}

.lang-select {
  width: 140px;
}

/* Cyberpunk style overrides for Element Plus */
:deep(.el-input__wrapper) {
  background-color: rgba(0, 0, 0, 0.7) !important;
  box-shadow: 0 0 0 1px var(--cp-primary) inset !important;
  border-radius: 0 !important;
}

:deep(.el-input__inner) {
  color: var(--cp-primary) !important;
  font-family: 'Segoe UI', sans-serif;
  font-weight: bold;
}

:deep(.el-select__caret) {
  color: var(--cp-primary) !important;
}

:deep(.el-popper) {
  background-color: var(--cp-panel-bg) !important;
  border: 1px solid var(--cp-primary) !important;
}

:deep(.el-select-dropdown__item) {
  color: var(--cp-text-color) !important;
}

:deep(.el-select-dropdown__item.selected) {
  color: var(--cp-bg-color) !important;
  background-color: var(--cp-primary) !important;
}

:deep(.el-select-dropdown__item:hover) {
  background-color: rgba(0, 243, 255, 0.2) !important;
}
</style>
