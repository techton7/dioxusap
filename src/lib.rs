//! # Dioxusap (Dioxus Animation Platform)
//!
//! Type-safe, high-performance GSAP animation platform and timeline sequencer for Dioxus.
//!
//! ## Overview
//!
//! `dioxusap` brings the industrial-grade animation capabilities of **GSAP** (GreenSock Animation Platform)
//! into Dioxus with zero-IPC execution, full type safety, and automatic reactive lifecycle management.
//!
//! It serves as the master motion orchestrator across:
//! - **DOM UI**: Elastic dialogs, sliding tabs, accordion transitions ([`monoxus`], [`shadcn-dioxus`])
//! - **2D Node Graphs**: Auto-layout animated transitions, data flow pulses ([`nodoxus`])
//! - **3D Graphics & CAD**: Camera flight, exploded assembly sequences, material/lighting tweens ([`trioxus`])
//!
//! ## Architecture Highlights
//!
//! - **Thin Reactive Wrapper + Zero-IPC**: Rust sends a single declarative trigger; the browser/webview GPU compositor executes the 120fps animation loop with zero IPC overhead.
//! - **Type-Safe Builder & Easing**: Eliminates raw JavaScript string formatting with compile-time checked enums and builders.
//! - **Auto-Cleanup via `gsap.context()`**: Automatic tween reversion on component unmount via `use_drop`, eliminating memory leaks and orphaned animations.
//! - **Generic In-Engine Targeting**: Supports direct targeting of browser/engine properties (e.g. Three.js camera position) without piping updates through Rust.

#![warn(missing_docs)]

/// Early scaffold version of dioxusap.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
