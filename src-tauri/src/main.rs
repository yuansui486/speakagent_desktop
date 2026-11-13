// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--transcription-worker") {
        speakagent_lib::engine::transcription::worker_main();
        return;
    }
    speakagent_lib::run()
}
