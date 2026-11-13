import { useEffect, useRef, useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Mic, Upload, Fingerprint, Trash2, Plus } from "lucide-react";
import { diarizationEmbedding, pickAudioFiles } from "../lib/api";
import { useT } from "../i18n";
import {
  listVoiceProfiles, enrollVoiceFile, renameVoiceProfile, deleteVoiceProfile, deleteVoiceSample,
  startVoiceRecording, finishVoiceRecording, cancelVoiceRecording, voiceRecording,
} from "../lib/voiceprints";

const button = "inline-flex items-center justify-center gap-2 rounded-lg border border-white/10 px-3 py-2 text-sm text-zinc-200 hover:bg-white/5 disabled:cursor-not-allowed disabled:opacity-40";
const input = "rounded-lg border border-white/10 bg-zinc-900 px-3 py-2 text-sm text-zinc-200 disabled:opacity-50";

export default function VoiceLibraryPage() {
  const all = useT();
  const t = all.voiceLibrary;
  const cache = useQueryClient();
  const profiles = useQuery({ queryKey: ["voiceProfiles"], queryFn: listVoiceProfiles });
  const model = useQuery({ queryKey: ["diarizationEmbedding"], queryFn: diarizationEmbedding });
  const [profileId, setProfileId] = useState("");
  const [name, setName] = useState("");
  const [busy, setBusy] = useState(false);
  const [recording, setRecording] = useState(false);
  const [seconds, setSeconds] = useState(0);
  const [error, setError] = useState("");
  const [renaming, setRenaming] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const [deleting, setDeleting] = useState<{ id: string; kind: "person" | "sample" } | null>(null);
  const captureForm = useRef<HTMLElement>(null);
  const locked = useRef(false);
  const ownedRecording = useRef(false);
  const mounted = useRef(true);
  const startedAt = useRef(0);
  const timerAction = useRef<() => void>(() => {});
  const selected = profiles.data?.find(p => p.id === profileId);
  const enrollmentName = (selected?.name ?? name).trim();
  const disabled = busy || recording;

  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
      // Only release the microphone if this page started the recording.
      if (ownedRecording.current) {
        ownedRecording.current = false;
        void cancelVoiceRecording().catch(console.error);
      }
    };
  }, []);

  async function run(action: () => Promise<void>) {
    if (locked.current) return;
    locked.current = true;
    setBusy(true); setError("");
    try { await action(); }
    catch (e) { if (mounted.current) setError(String(e)); }
    finally {
      locked.current = false;
      if (mounted.current) setBusy(false);
    }
  }
  async function refresh() {
    await cache.invalidateQueries({ queryKey: ["voiceProfiles"] });
  }
  function requireName() {
    if (enrollmentName) return true;
    setError(t.nameRequired);
    return false;
  }
  function importFiles() {
    if (!requireName()) return;
    void run(async () => {
      const paths = await pickAudioFiles();
      let target = profileId || null;
      try {
        for (const path of paths) {
          if (!mounted.current) break;
          const profile = await enrollVoiceFile(enrollmentName, target, path);
          // Multiple selected files become samples of the same person.
          target = profile.id;
          if (mounted.current) setProfileId(profile.id);
        }
      } finally { await refresh(); }
    });
  }
  function start() {
    if (!requireName()) return;
    void run(async () => {
      if (await voiceRecording()) throw new Error(t.otherRecording);
      if (!mounted.current) return;
      await startVoiceRecording();
      ownedRecording.current = true;
      if (!mounted.current) {
        await cancelVoiceRecording();
        ownedRecording.current = false;
        return;
      }
      startedAt.current = Date.now();
      setSeconds(0); setRecording(true);
    });
  }
  function finish() {
    if (!ownedRecording.current) return;
    void run(async () => {
      // Backend finish takes and stops the capture before extracting features.
      // Releasing ownership now avoids cancelling a subsequent recording if the
      // user leaves this page while feature extraction is still running.
      ownedRecording.current = false;
      try {
        const profile = await finishVoiceRecording(enrollmentName, profileId || null);
        if (mounted.current) setProfileId(profile.id);
      } finally {
        // Finish consumes the captured audio even when enrollment rejects it.
        ownedRecording.current = false;
        if (mounted.current) setRecording(false);
        await refresh();
      }
    });
  }
  function cancel() {
    void run(async () => {
      await cancelVoiceRecording();
      ownedRecording.current = false;
      if (mounted.current) setRecording(false);
    });
  }
  timerAction.current = finish;
  useEffect(() => {
    if (!recording) return;
    const tick = window.setInterval(() => {
      const elapsed = Math.min(60, Math.floor((Date.now() - startedAt.current) / 1000));
      setSeconds(elapsed);
      if (elapsed >= 60) timerAction.current();
    }, 250);
    return () => window.clearInterval(tick);
  }, [recording]);

  const modelLabel = (id: string) => id === "diar-emb-eres2netv2" ? "ERes2NetV2" : id === "diar-emb" ? "CAM++" : id;
  return <div className="mx-auto max-w-4xl p-8">
    <h1 className="flex items-center gap-3 text-2xl font-semibold"><Fingerprint className="text-amber-400" />{t.title}</h1>
    <p className="mt-3 text-sm leading-relaxed text-zinc-400">{t.intro}</p>
    <p className="mt-2 text-xs leading-relaxed text-zinc-500">{t.privacy}</p>
    <section ref={captureForm} className="glass mt-6 rounded-xl border border-white/10 p-5">
      <p className="text-sm text-zinc-300">{t.currentModel}: <span className="text-amber-400">{model.data ? modelLabel(model.data) : t.loading}</span></p>
      <p className="mt-2 text-xs leading-relaxed text-zinc-400">{t.modelHint}</p>
      <p className="mt-2 text-xs leading-relaxed text-zinc-400">{t.sampleHint}</p>
      <div className="mt-5 flex flex-wrap gap-4">
        <label className="flex flex-col gap-2 text-xs text-zinc-400">{t.target}
          <select className={input} value={profileId} disabled={disabled} onChange={e => setProfileId(e.target.value)}>
            <option value="">{t.newPerson}</option>
            {profiles.data?.map(p => <option key={p.id} value={p.id}>{p.name}</option>)}
          </select>
        </label>
        {!profileId && <label className="flex min-w-52 flex-1 flex-col gap-2 text-xs text-zinc-400">{t.name}
          <input className={input} value={name} maxLength={100} disabled={disabled} placeholder={t.namePlaceholder} onChange={e => setName(e.target.value)} />
        </label>}
      </div>
      <div className="mt-4 flex flex-wrap items-center gap-3">
        {recording ? <>
          <span role="status" className="text-sm text-red-400">● {t.recording} {seconds}/60 s</span>
          <button className={button} disabled={busy} onClick={finish}>{t.stopSave}</button>
          <button className={button} disabled={busy} onClick={cancel}>{t.cancel}</button>
        </> : <>
          <button className={button} disabled={busy || !model.data || !enrollmentName} onClick={importFiles}><Upload size={16} />{t.import}</button>
          <button className={button} disabled={busy || !model.data || !enrollmentName} onClick={start}><Mic size={16} />{t.record}</button>
        </>}
        {busy && <span role="status" className="text-xs text-zinc-400">{t.busy}</span>}
      </div>
    </section>
    {(error || profiles.error || model.error) && <div role="alert" className="mt-4 break-words rounded-lg border border-red-500/20 bg-red-500/5 p-3 text-sm text-red-400">
      {error || String(profiles.error || model.error)}
      <button className={`${button} ml-3`} disabled={disabled} onClick={() => { setError(""); void profiles.refetch(); void model.refetch(); }}>{t.retry}</button>
    </div>}
    {profiles.isPending && <p role="status" className="mt-6 text-zinc-400">{t.loading}</p>}
    {profiles.data?.length === 0 && <p className="mt-6 text-sm text-zinc-500">{t.empty}</p>}
    <div className="mt-6 space-y-4">
      {profiles.data?.map(p => <section key={p.id} className="rounded-xl border border-white/10 p-5">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <h2 className="font-medium text-zinc-200">{p.name}</h2>
          <div className="flex flex-wrap gap-2">
            <button className={button} disabled={disabled} onClick={() => { setProfileId(p.id); captureForm.current?.scrollIntoView({ behavior: "smooth" }); }}><Plus size={14} />{t.append}</button>
            <button className={button} disabled={disabled} onClick={() => {
              setRenaming(p.id); setRenameValue(p.name);
            }}>{t.rename}</button>
            <button className={button} disabled={disabled} onClick={() => {
              setDeleting({ id: p.id, kind: "person" });
            }}><Trash2 size={14} />{t.deletePerson}</button>
          </div>
        </div>
        {renaming === p.id && <form className="mt-3 flex flex-wrap gap-2" onSubmit={event => {
          event.preventDefault();
          if (!renameValue.trim()) { setError(t.nameRequired); return; }
          void run(async () => { await renameVoiceProfile(p.id, renameValue.trim()); setRenaming(null); await refresh(); });
        }}>
          <input className={input} aria-label={t.name} value={renameValue} disabled={disabled} maxLength={100} onChange={event => setRenameValue(event.target.value)} autoFocus />
          <button className={button} disabled={disabled || !renameValue.trim()} type="submit">{all.common.save}</button>
          <button className={button} disabled={disabled} type="button" onClick={() => setRenaming(null)}>{all.common.cancel}</button>
        </form>}
        {!p.samples.length && <p className="mt-3 text-xs text-zinc-500">{t.noSamples}</p>}
        <ul className="mt-3 divide-y divide-white/5">
          {p.samples.map(sample => <li key={sample.id} className="flex flex-wrap items-center justify-between gap-2 py-3 text-xs text-zinc-400">
            <div><span>{t.model}: {modelLabel(sample.modelId)}</span><span className="ml-4">{t.seconds(sample.speechSeconds)}</span></div>
            <button className={button} disabled={disabled} onClick={() => {
              setDeleting({ id: sample.id, kind: "sample" });
            }}><Trash2 size={14} />{t.deleteSample}</button>
          </li>)}
        </ul>
      </section>)}
    </div>
    {deleting && <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-6">
      <div role="dialog" aria-modal="true" aria-labelledby="voice-delete-title" className="w-full max-w-md rounded-xl border border-white/10 bg-zinc-900 p-6">
        <h2 id="voice-delete-title" className="font-medium">{deleting.kind === "person" ? t.confirmPerson : t.confirmSample}</h2>
        <div className="mt-5 flex justify-end gap-3">
          <button className={button} disabled={busy} onClick={() => setDeleting(null)} autoFocus>{all.common.cancel}</button>
          <button className={`${button} text-red-400`} disabled={busy} onClick={() => void run(async () => {
            if (deleting.kind === "person") {
              await deleteVoiceProfile(deleting.id);
              if (profileId === deleting.id) setProfileId("");
            } else await deleteVoiceSample(deleting.id);
            setDeleting(null); await refresh();
          })}>{all.common.delete}</button>
        </div>
        {error && <p role="alert" className="mt-3 break-words text-sm text-red-400">{error}</p>}
      </div>
    </div>}
  </div>;
}
