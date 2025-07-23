import { createI18n } from 'vue-i18n';
import en from './locales/en.json';
import zh from './locales/zh.json';

type MessageSchema = typeof en;

const i18n = createI18n<[MessageSchema], 'en' | 'zh'>({
  legacy: false,
  locale: localStorage.getItem('user-language') || 'zh', // 从localStorage获取语言，默认为中文
  fallbackLocale: 'en',
  messages: {
    en,
    zh,
  },
});

export default i18n; 