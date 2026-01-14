import { init, register } from 'svelte-i18n';

import en from './i18n/en.json';
import zhCN from './i18n/zh-CN.json';

register('en', () => Promise.resolve(en));
register('zh-CN', () => Promise.resolve(zhCN));

function getInitialLocale(): string {
  if (typeof window === 'undefined') {
    return 'en';
  }

  // First check localStorage for user preference
  const savedLocale = localStorage.getItem('locale');
  if (savedLocale) {
    return savedLocale;
  }

  // Then detect system language
  const browserLang = navigator.language || (navigator as any).userLanguage || '';

  // Check for Chinese variants
  if (browserLang.startsWith('zh')) {
    return 'zh-CN';
  }

  // Check for exact match
  if (browserLang === 'en' || browserLang.startsWith('en-')) {
    return 'en';
  }

  // Default to English
  return 'en';
}

init({
  fallbackLocale: 'en',
  initialLocale: getInitialLocale(),
});
