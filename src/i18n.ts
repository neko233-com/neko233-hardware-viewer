import { createI18n } from 'vue-i18n'
import en from './locales/en'
import zh from './locales/zh'

const messages = {
  en,
  zh
}

// Detect language
const getBrowserLanguage = () => {
  const lang = navigator.language || 'zh-CN'
  if (lang.startsWith('zh')) {
    return 'zh'
  }
  return 'en'
}

const i18n = createI18n({
  legacy: false, // Use Composition API
  locale: getBrowserLanguage(),
  fallbackLocale: 'en',
  messages
})

export default i18n
