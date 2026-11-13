// Лёгкий самописный i18n-слой (ZH/EN). Один источник истины — настройка `ui_lang`
// (SQLite на бэке); localStorage используется как мгновенное зеркало для первой
// отрисовки без «мигания». Язык влияет и на UI-строки, и на локаль дат/чисел, и —
// через ту же настройку на бэке — на язык вывода «Итогов встречи».
import { create } from "zustand";
import { setUiLanguage, uiLanguage, type UiLang } from "../lib/api";
import { voiceLibrary } from "./dict/voiceLibrary";
import { common } from "./dict/common";
import { models } from "./dict/models";
import { nav } from "./dict/nav";
import { statusBar } from "./dict/statusBar";
import { progress } from "./dict/progress";
import { home } from "./dict/home";
import { transcribe } from "./dict/transcribe";
import { dictation } from "./dict/dictation";
import { about } from "./dict/about";
import { mcp } from "./dict/mcp";
import { settings } from "./dict/settings";
import { onboarding } from "./dict/onboarding";
import { meeting } from "./dict/meeting";
import { resultView } from "./dict/resultView";
import { beautify } from "./dict/beautify";
import { speakers } from "./dict/speakers";
import { aiProvider } from "./dict/aiProvider";
import { modelSelector } from "./dict/modelSelector";
import { diagnostics } from "./dict/diagnostics";
import { overlay } from "./dict/overlay";
import { exportMenu } from "./dict/exportMenu";
import { dragDrop } from "./dict/dragDrop";
import { perf } from "./dict/perf";
import { acceleration } from "./dict/acceleration";
import { diarization } from "./dict/diarization";

// Словарь собирается из самодостаточных фрагментов (каждый несёт zh+en и сам
// проверяет их паритет). `en` defines the dictionary shape; `zh` must match it.
const en = {
  voiceLibrary: voiceLibrary.en,
  common: common.en,
  models: models.en,
  nav: nav.en,
  statusBar: statusBar.en,
  progress: progress.en,
  home: home.en,
  transcribe: transcribe.en,
  dictation: dictation.en,
  about: about.en,
  mcp: mcp.en,
  settings: settings.en,
  onboarding: onboarding.en,
  meeting: meeting.en,
  resultView: resultView.en,
  beautify: beautify.en,
  speakers: speakers.en,
  aiProvider: aiProvider.en,
  modelSelector: modelSelector.en,
  diagnostics: diagnostics.en,
  overlay: overlay.en,
  exportMenu: exportMenu.en,
  dragDrop: dragDrop.en,
  perf: perf.en,
  acceleration: acceleration.en,
  diarization: diarization.en,
};

const zh = {
  voiceLibrary: voiceLibrary.zh,
  common: common.zh,
  models: models.zh,
  nav: nav.zh,
  statusBar: statusBar.zh,
  progress: progress.zh,
  home: home.zh,
  transcribe: transcribe.zh,
  dictation: dictation.zh,
  about: about.zh,
  mcp: mcp.zh,
  settings: settings.zh,
  onboarding: onboarding.zh,
  meeting: meeting.zh,
  resultView: resultView.zh,
  beautify: beautify.zh,
  speakers: speakers.zh,
  aiProvider: aiProvider.zh,
  modelSelector: modelSelector.zh,
  diagnostics: diagnostics.zh,
  overlay: overlay.zh,
  exportMenu: exportMenu.zh,
  dragDrop: dragDrop.zh,
  perf: perf.zh,
  acceleration: acceleration.zh,
  diarization: diarization.zh,
};

export type Lang = UiLang; // "zh" | "en"
export type Dict = typeof en;

const dictionaries: Record<Lang, Dict> = { zh, en };
const STORAGE_KEY = "ui_lang";

function detectInitial(): Lang {
  try {
    const s = localStorage.getItem(STORAGE_KEY);
    if (s === "zh" || s === "en") return s;
    if (s === "ru") localStorage.setItem(STORAGE_KEY, "zh");
  } catch {
    /* ignore */
  }
  return "zh";
}

// Модульное зеркало текущего языка — для доступа из не-React утилит
// (format.ts, perf.ts, diarize.ts, api.ts, exporters.ts).
let _lang: Lang = detectInitial();
let languageRevision = 0;

function updateDocumentLanguage(lang: Lang): void {
  document.documentElement.lang = lang === "zh" ? "zh-CN" : lang;
}
updateDocumentLanguage(_lang);

/** Текущий язык (императивно, вне React). */
export const getLang = (): Lang => _lang;

/** Словарь текущего языка (императивно, для не-React утилит). */
export const tr = (): Dict => dictionaries[_lang];

interface LangState {
  lang: Lang;
  setLang: (l: Lang) => void;
}

export const useLang = create<LangState>((set) => ({
  lang: _lang,
  setLang: (l) => {
    languageRevision += 1;
    _lang = l;
    updateDocumentLanguage(l);
    try {
      localStorage.setItem(STORAGE_KEY, l);
    } catch {
      /* ignore */
    }
    set({ lang: l });
    // Персист в бэкенд (это же меняет язык вывода LLM). Ошибку глушим — UI уже переключён.
    void setUiLanguage(l).catch(() => {});
  },
}));

/** Хук словаря текущего языка — компонент ре-рендерится при смене языка. */
export function useT(): Dict {
  const lang = useLang((s) => s.lang);
  return dictionaries[lang];
}

/**
 * Сверка с бэкендом на старте: `ui_lang` в SQLite — источник истины. Вызывать один
 * раз при монтировании App. localStorage уже дал корректную первую отрисовку; здесь
 * лишь примиряем расхождение (например, после чистой установки или сброса localStorage).
 */
export async function syncLangFromBackend(): Promise<void> {
  const revision = languageRevision;
  try {
    const backend = await uiLanguage();
    // A user selection made while this request was pending takes precedence.
    if (revision !== languageRevision) return;
    if ((backend === "zh" || backend === "en") && backend !== _lang) {
      _lang = backend;
      updateDocumentLanguage(backend);
      try {
        localStorage.setItem(STORAGE_KEY, backend);
      } catch {
        /* ignore */
      }
      useLang.setState({ lang: backend });
    }
  } catch {
    /* ignore */
  }
}

// The recording overlay has its own webview and store. Follow language changes
// from other windows without writing the same preference back to the backend.
window.addEventListener("storage", (event) => {
  if (event.key !== STORAGE_KEY) return;
  const lang = event.newValue === "ru" ? "zh" : event.newValue;
  if (lang !== "zh" && lang !== "en") return;
  languageRevision += 1;
  _lang = lang;
  updateDocumentLanguage(lang);
  useLang.setState({ lang });
});
