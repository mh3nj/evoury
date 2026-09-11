import en from "./locales/en.json";

export type Locale = "en";
export type TranslationMap = typeof en;

const translations: Record<Locale, TranslationMap> = { en };

let currentLocale: Locale = "en";
let currentTranslations: TranslationMap = en;

export function setLocale(locale: Locale) {
  currentLocale = locale;
  currentTranslations = translations[locale] || en;
}

export function getLocale(): Locale {
  return currentLocale;
}

export function t(path: string, params?: Record<string, string | number>): string {
  const keys = path.split(".");
  let value: any = currentTranslations;
  for (const key of keys) {
    value = value?.[key];
  }
  if (typeof value !== "string") return path;
  if (!params) return value;
  return Object.entries(params).reduce((str, [k, v]) => str.replace(`{${k}}`, String(v)), value);
}

export function useT() {
  return { t, locale: currentLocale, setLocale };
}
