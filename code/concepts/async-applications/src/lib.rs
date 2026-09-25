//! Compile-checked excerpts for the website's Async Applications section.
//!
//! `sync_ui` owns the synchronous event loop and its application placeholders. `drain` uses
//! those same placeholders to isolate batching. `stale` owns a separate search state so request
//! identity can be studied without terminal setup. `coordination` covers channel and worker
//! lifetimes, and `handoff` covers temporary terminal release by a synchronous sole reader.
//! With the `http` feature, `http` supplies the reqwest adaptation and local HTTP fixture tests.
//!
//! These private modules are independent teaching excerpts. Run the complete application with
//! `cargo run -p async-applications`; it lives in `src/bin/background.rs`.

// Excerpts are compiled and tested here but are not wired into the runnable application's UI.
#![allow(dead_code)]

mod coordination;
mod drain;
mod handoff;
mod stale;
mod sync_ui;

#[cfg(feature = "http")]
mod http;
