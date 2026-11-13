import { save } from "@tauri-apps/plugin-dialog";
import { saveText, savePdf, type PdfBlock, type TranscriptTurn } from "./api";
import { parseReplicas, looksDiarized, speakerLabel } from "./diarize";
import { tr } from "../i18n";

type Names = Record<number, string> | undefined;

/** UTF-8 BOM and quoted cells for Excel; neutralize formula-like user content. */
export function timelineCsv(turns: TranscriptTurn[], names: Names): string {
  const cell = (value: string | number) => {
    const raw = String(value);
    const safe = /^[\s]*[=+@\-]/u.test(raw) ? `'${raw}` : raw;
    return `"${safe.replace(/"/g, '""')}"`;
  };
  const headers = tr().common.locale.startsWith("zh")
    ? ["姓名", "开始时间（秒）", "结束时间（秒）", "内容"]
    : ["Speaker", "Start (seconds)", "End (seconds)", "Text"];
  return "\uFEFF" + [headers, ...turns.map(turn => [speakerLabel(turn.speaker, names),
    turn.startSec.toFixed(3), turn.endSec.toFixed(3), turn.text])]
    .map(row => row.map(cell).join(",")).join("\r\n");
}

export async function exportTimelineCsv(name: string, turns: TranscriptTurn[], names?: Names) {
  const path = await save({ defaultPath: `${name}.csv`, filters: [{ name: "CSV", extensions: ["csv"] }] });
  if (path) await saveText(timelineCsv(turns, names), path);
}

function toTxt(text: string, names: Names): string {
  if (!looksDiarized(text)) return text;
  return parseReplicas(text)
    .map((r) => `${speakerLabel(r.speaker, names)} [${r.time}]: ${r.text}`)
    .join("\n\n");
}

function toMd(title: string, text: string, names: Names): string {
  const head = `# ${title}\n\n`;
  if (!looksDiarized(text)) return head + text;
  return (
    head +
    parseReplicas(text)
      .map((r) => `**${speakerLabel(r.speaker, names)}** _[${r.time}]_\n\n${r.text}`)
      .join("\n\n")
  );
}

export async function exportTxt(name: string, text: string, names?: Names) {
  const path = await save({
    defaultPath: `${name}.txt`,
    filters: [{ name: tr().common.pickerText, extensions: ["txt"] }],
  });
  if (path) await saveText(toTxt(text, names), path);
}

export async function exportMd(name: string, text: string, names?: Names) {
  const path = await save({
    defaultPath: `${name}.md`,
    filters: [{ name: "Markdown", extensions: ["md"] }],
  });
  if (path) await saveText(toMd(name, text, names), path);
}

// ── Экспорт «Итогов» (готовый markdown: саммари / протокол / задачи) ──

/** Лёгкая очистка markdown до читаемого текста (для .txt и PDF-абзацев). */
function mdToPlain(md: string): string {
  return md
    .replace(/^#{1,6}\s+/gm, "") // заголовки → просто строки
    .replace(/\*\*(.+?)\*\*/g, "$1")
    .replace(/_(.+?)_/g, "$1")
    .replace(/^- \[ \]\s*/gm, "☐ ")
    .replace(/^- \[[xX]\]\s*/gm, "☑ ")
    .replace(/^[-*]\s+/gm, "• ");
}

export async function exportArtifactTxt(name: string, md: string) {
  const path = await save({
    defaultPath: `${name}.txt`,
    filters: [{ name: tr().common.pickerText, extensions: ["txt"] }],
  });
  if (path) await saveText(mdToPlain(md), path);
}

export async function exportArtifactMd(name: string, md: string) {
  const path = await save({
    defaultPath: `${name}.md`,
    filters: [{ name: "Markdown", extensions: ["md"] }],
  });
  if (path) await saveText(md, path);
}

/** markdown → блоки PDF: заголовки становятся заголовками, остальное — абзацами. */
export async function exportArtifactPdf(title: string, md: string) {
  const path = await save({
    defaultPath: `${title}.pdf`,
    filters: [{ name: "PDF", extensions: ["pdf"] }],
  });
  if (!path) return;

  const blocks: PdfBlock[] = [];
  let body: string[] = [];
  const flush = () => {
    const text = mdToPlain(body.join("\n")).trim();
    if (text) blocks.push({ heading: null, time: null, body: text });
    body = [];
  };
  for (const line of md.split("\n")) {
    const h = line.match(/^#{1,6}\s+(.*)$/);
    if (h) {
      flush();
      blocks.push({ heading: h[1].trim(), time: null, body: "" });
    } else if (line.trim() === "") {
      flush();
    } else {
      body.push(line);
    }
  }
  flush();
  await savePdf(title, blocks.filter((b) => b.heading || b.body), path);
}

/** PDF генерируется в Rust со встроенным шрифтом (кириллица гарантирована). */
export async function exportPdf(title: string, text: string, names?: Names) {
  const path = await save({
    defaultPath: `${title}.pdf`,
    filters: [{ name: "PDF", extensions: ["pdf"] }],
  });
  if (!path) return;

  let blocks: PdfBlock[];
  if (looksDiarized(text)) {
    blocks = parseReplicas(text).map((r) => ({
      heading: speakerLabel(r.speaker, names),
      time: r.time,
      body: r.text,
    }));
  } else {
    blocks = text
      .split(/\n\n+/)
      .map((p) => p.trim())
      .filter(Boolean)
      .map((p) => ({ heading: null, time: null, body: p }));
  }
  await savePdf(title, blocks, path);
}
