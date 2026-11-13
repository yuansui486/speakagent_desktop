import { invoke } from "@tauri-apps/api/core";

export interface VoiceSample {
  id: string;
  modelId: string;
  speechSeconds: number;
  createdAt: number;
}
export interface VoiceProfile {
  id: string;
  name: string;
  samples: VoiceSample[];
  createdAt: number;
}
export interface VoiceMatch {
  speaker: number;
  profileId: string | null;
  name: string | null;
  score: number | null;
  reason: string;
}
export const listVoiceProfiles = () => invoke<VoiceProfile[]>("list_voice_profiles");
export const enrollVoiceFile = (name: string, profileId: string | null, path: string) =>
  invoke<VoiceProfile>("enroll_voice_file", { name, profileId, path });
export const renameVoiceProfile = (id: string, name: string) => invoke<void>("rename_voice_profile", { id, name });
export const deleteVoiceProfile = (id: string) => invoke<void>("delete_voice_profile", { id });
export const deleteVoiceSample = (id: string) => invoke<void>("delete_voice_sample", { id });
export const startVoiceRecording = () => invoke<void>("start_voice_recording");
export const finishVoiceRecording = (name: string, profileId: string | null) =>
  invoke<VoiceProfile>("finish_voice_recording", { name, profileId });
export const cancelVoiceRecording = () => invoke<void>("cancel_voice_recording");
export const voiceRecording = () => invoke<boolean>("voice_recording");
export const jobVoiceMatches = (jobId: string) => invoke<VoiceMatch[]>("job_voice_matches", { jobId });
