// The same catalogs the Rust side embeds (`src-tauri/src/i18n.rs`).
import en from "../locales/en.json";
import es from "../locales/es.json";
import gl from "../locales/gl.json";
import fr from "../locales/fr.json";
import ptBR from "../locales/pt-BR.json";
import ja from "../locales/ja.json";
import ko from "../locales/ko.json";
import zhCN from "../locales/zh-CN.json";
import tr from "../locales/tr.json";
import az from "../locales/az.json";
import hi from "../locales/hi.json";

export type Key = keyof typeof en;
type Catalog = Record<Key, string>;

const CATALOGS: Record<string, Catalog> = {
  en, es, gl, fr, "pt-BR": ptBR, ja, ko, "zh-CN": zhCN, tr, az, hi,
};

/** Language codes, each with its own name, for the picker. */
export const LANGUAGES = Object.entries(CATALOGS).map(([code, c]) => [code, c["language.name"]] as const);

let current: Catalog = en;

export function setLocale(code: string) {
  current = CATALOGS[code] ?? en;
  document.documentElement.lang = code;
}

export function t(key: Key, vars: Record<string, string | number> = {}): string {
  return Object.entries(vars).reduce((s, [k, v]) => s.replaceAll(`{${k}}`, String(v)), current[key] ?? en[key]);
}

/** Fill every element that names a key: `data-i18n` for text, `data-i18n-html`
 *  for text with inline markup, `data-i18n-aria` and `data-i18n-title` for attributes. */
export function translate(root: ParentNode = document) {
  for (const el of root.querySelectorAll<HTMLElement>("[data-i18n]")) el.textContent = t(el.dataset.i18n as Key);
  for (const el of root.querySelectorAll<HTMLElement>("[data-i18n-html]")) el.innerHTML = t(el.dataset.i18nHtml as Key);
  for (const el of root.querySelectorAll<HTMLElement>("[data-i18n-aria]")) el.setAttribute("aria-label", t(el.dataset.i18nAria as Key));
  for (const el of root.querySelectorAll<HTMLElement>("[data-i18n-title]")) el.title = t(el.dataset.i18nTitle as Key);
}
