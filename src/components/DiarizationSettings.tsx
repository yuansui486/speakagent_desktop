import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { diarizationEmbedding, setDiarizationEmbedding, listModels, type DiarizationEmbedding } from "../lib/api";
import { useT } from "../i18n";

export default function DiarizationSettings() {
  const t = useT().diarization;
  const { data: selected, refetch, error } = useQuery({ queryKey: ["diarizationEmbedding"], queryFn: diarizationEmbedding });
  const { data: models } = useQuery({ queryKey: ["models"], queryFn: listModels, refetchInterval: 3000 });
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState("");
  async function choose(id: DiarizationEmbedding) {
    setSaving(true); setSaveError("");
    try { await setDiarizationEmbedding(id); await refetch(); }
    catch (e) { setSaveError(String(e)); }
    finally { setSaving(false); }
  }
  const choices = [
    { id: "diar-emb-eres2netv2" as const, name: t.quality, hint: t.qualityHint },
    { id: "diar-emb" as const, name: t.fast, hint: t.fastHint },
  ];
  return <section className="glass mt-6 rounded-xl border border-white/5 p-4">
    <h2 className="font-medium text-zinc-200">{t.title}</h2>
    <p className="mt-2 text-xs leading-relaxed text-zinc-400">{t.hint}</p>
    <div className="mt-3 space-y-2">
      {choices.map(choice => <button key={choice.id} disabled={saving || !selected}
        onClick={() => void choose(choice.id)} aria-pressed={selected === choice.id}
        className={`w-full rounded-lg border p-3 text-left disabled:opacity-50 ${selected === choice.id ? "border-amber-500/40 bg-amber-500/10" : "border-white/10 hover:bg-white/5"}`}>
        <span className="block text-sm text-zinc-200">{choice.name}</span>
        <span className="mt-1 block text-xs leading-relaxed text-zinc-400">{choice.hint}</span>
        <span className="mt-1 block text-xs text-zinc-500">{models?.find(model => model.id === choice.id)?.installed ? t.installed : t.automatic}</span>
      </button>)}
    </div>
    <p className="mt-3 text-xs leading-relaxed text-zinc-500">{t.note}</p>
    {(saveError || error) && <p role="alert" className="mt-3 break-words text-xs text-red-400">{t.saveFailed}: {saveError || String(error)}</p>}
  </section>;
}
