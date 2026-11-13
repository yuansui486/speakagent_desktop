import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router-dom";
import { useT } from "../i18n";
import { jobVoiceMatches } from "../lib/voiceprints";

export default function VoiceMatches({ jobId }: { jobId: string }) {
  const all = useT();
  const t = all.voiceLibrary;
  const { data, error, isPending, refetch } = useQuery({
    queryKey: ["voiceMatches", jobId], queryFn: () => jobVoiceMatches(jobId), enabled: !!jobId, refetchOnMount: "always",
  });
  return <section className="my-4 rounded-xl border border-white/10 p-4">
    <div className="flex items-center justify-between gap-3">
      <h2 className="text-sm font-medium text-zinc-200">{t.matches}</h2>
      <Link className="text-xs text-amber-400 hover:underline" to="/voice-library">{t.title}</Link>
    </div>
    <p className="mt-2 text-xs leading-relaxed text-zinc-500">{t.snapshot}</p>
    {isPending && <p role="status" className="mt-2 text-xs text-zinc-500">{t.loading}</p>}
    {error && <div role="alert" className="mt-2 break-words text-xs text-red-400">{String(error)} <button className="underline" onClick={() => void refetch()}>{t.retry}</button></div>}
    {data?.length === 0 && <p className="mt-2 text-xs text-zinc-500">{t.noMatches}</p>}
    {!!data?.length && <ul className="mt-2 space-y-2">
      {data.map(match => <li key={match.speaker} className="flex flex-wrap items-baseline gap-x-4 gap-y-1 text-xs">
        <span className="text-zinc-400">{all.common.speaker(match.speaker)}</span>
        <span className={match.profileId ? "text-amber-400" : "text-zinc-400"}>{match.name ?? t.unknown}</span>
        {match.score !== null && <span className="text-zinc-500">{t.similarity}: {match.score.toFixed(3)}</span>}
        {!match.profileId && <span className="text-zinc-500">{t.reasons[match.reason as keyof typeof t.reasons] ?? `${t.unknownReason}: ${match.reason}`}</span>}
      </li>)}
    </ul>}
  </section>;
}
