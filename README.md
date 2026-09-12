# Dioxusap (Dioxus Animation Platform)

[![Crates.io](https://img.shields.io/crates/v/dioxusap.svg)](https://crates.io/crates/dioxusap)
[![Documentation](https://docs.rs/dioxusap/badge.svg)](https://docs.rs/dioxusap)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

**Type-safe, high-performance GSAP animation platform and timeline sequencer for Dioxus.**

`dioxusap` brings the industrial-grade animation capabilities of **GSAP** (GreenSock Animation Platform) into Dioxus with zero-IPC execution, full type safety, and automatic reactive lifecycle management.

Rather than being a simple button-bounce utility, `dioxusap` is designed as a **non-linear multi-track timeline orchestrator** connecting DOM UI, 2D node graphs, and 3D graphics into a unified, scrubbable time axis.

---

## 🗺️ Ecosystem Synergy Map

```text
                     ┌─────────────────────────────────────────┐
                     │   dioxusap (Timeline & Motion Sequencer)│
                     └────────────────────┬────────────────────┘
                                          │
       ┌──────────────────────────────────┼──────────────────────────────────┐
       ▼                                  ▼                                  ▼
[shadcn-dioxus / monoxus]            [nodoxus (Node Engine)]         [trioxus & CAD Viewport]
- Modal / Drawer elastic popups       - Sugiyama layout sliding       - 3D camera flight & orbits
- Accordion & Tab transitions        - Edge pulse flow animations    - Exploded assembly sequences
- Apple-style ScrollTrigger flows     - Viewport FitView gliding      - Lighting, materials, opacity
```

---

## ⚡ Core Architectural Pillars

### 1. "Thin Reactive Wrapper + Zero-IPC" Architecture
Rust does not attempt to reimplement 15 years of browser-specific transform physics, subpixel rendering hacks, and 120Hz lag-smoothing. Instead, **Rust sends a single declarative trigger command** to the browser/webview, where GSAP's native engine drives the 120fps animation loop on the GPU compositor.
- **Zero per-frame IPC overhead**: 0% Rust ↔ WebView communication during active playback.
- **Dioxus VDOM load**: 0 diffing and 0 re-renders while animating.

### 2. Type-Safe Rust Builder Pattern (No Raw `use_eval` Strings)
Developers never touch raw JavaScript strings. All eases, tweens, and timeline vars are strongly typed and verified by the Rust compiler:

```rust
// Type-safe, ergonomic chaining
gsap.to("#hero-card")
    .duration(0.8)
    .y(-20.0)
    .scale(1.05)
    .ease(Ease::ElasticOut(1.2, 0.4))
    .run();
```

### 3. Automatic Lifecycle Cleanup (`gsap.context()` + `use_drop`)
Similar to `@gsap/react`, `use_gsap` internally binds all tweens and timelines to a `gsap.context()`. When a Dioxus component unmounts (`use_drop`), `ctx.revert()` is called automatically, eliminating memory leaks, orphaned tweens, and zombie event listeners.

### 4. Generic Object & In-Engine Direct Target (Zero-IPC 3D Gliding)
GSAP can animate arbitrary JavaScript objects and properties beyond DOM selectors. For 3D viewports ([`trioxus`]) and node canvas coordinates ([`nodoxus`]), `dioxusap` targets in-engine object paths (e.g. `window.__trioxus.activeCamera.position`) directly in the webview, keeping high-frequency animation completely within the browser engine without IPC bottlenecks.

### 5. Blender-Style Timeline Control & Signal Scrubbing
Timelines are first-class citizens equipped with full scrubbing controls:
- `.seek(seconds: f64)`: Instant jump to a timestamp.
- `.progress(ratio: f64)`: 0.0 to 1.0 ratio scrubbing (ideal for slider/scrollbar sync).
- `.reverse()`, `.time_scale(rate: f64)`: Smooth backward playback and slow/fast motion.
- `use_timeline_scrubber`: Two-way synchronization with Dioxus reactive signals.

---

## 💻 Quick Code Preview (Target DX)

### Basic Scoped Tween
```rust
use dioxus::prelude::*;
use dioxusap::prelude::*;

#[component]
pub fn AnimatedCard() -> Element {
    let mut gsap = use_gsap();

    let on_hover = move |_| {
        gsap.to(".card")
            .y(-10.0)
            .scale(1.02)
            .duration(0.4)
            .ease(Ease::Power2Out)
            .run();
    };

    rsx! {
        div { class: "card", onmouseenter: on_hover,
            h2 { "Interactive Card" }
        }
    }
}
```

### Multi-Track Timeline Orchestration (CAD Assembly Preview)
```rust
use dioxus::prelude::*;
use dioxusap::prelude::*;

#[component]
pub fn CadAssemblyPlayer() -> Element {
    let timeline = use_timeline(|| {
        Timeline::new()
            // 1. [trioxus] 3D Camera flies into position
            .to_js("window.__trioxus.camera.position", TweenVars::new().prop("z", 100.0).duration(2.0))
            // 2. [monoxus] Spec modal slides into view at 1.5s
            .to("#spec-modal", TweenVars::new().x(0.0).opacity(1.0).duration(0.8), Some("1.5"))
            // 3. [trioxus] Casing separates into exploded view
            .to_js("window.__trioxus.casing.position", TweenVars::new().prop("y", 50.0).duration(2.0), Some("2.5"))
    });

    rsx! {
        // Scrub the entire multi-track sequence with a slider
        input {
            r#type: "range",
            min: "0",
            max: "1",
            step: "0.001",
            oninput: move |e| {
                if let Ok(v) = e.value().parse::<f64>() {
                    timeline.progress(v);
                }
            }
        }
    }
}
```

---

## 📁 Codebase Architecture

```text
animation/dioxusap/
├── Cargo.toml
├── release-plz.toml
├── .github/workflows/release-plz.yml
├── src/
│   ├── lib.rs               # Public exports & preludes
│   ├── hook.rs              # use_gsap(), use_timeline() with auto-cleanup
│   ├── easing.rs            # Type-safe Ease enums with canonical GSAP string format
│   ├── builder/
│   │   ├── mod.rs
│   │   ├── tween.rs         # gsap.to(), from(), from_to() builders
│   │   ├── timeline.rs      # Timeline chaining & sequencing
│   │   └── vars.rs          # TweenVars, TimelineVars serialization
│   └── plugins/
│       ├── mod.rs
│       ├── scroll_trigger.rs # ScrollTrigger reactive bindings
│       └── generic_target.rs # In-engine JavaScript object targeting
└── assets/                  # GSAP runtime injection helpers
```

---

## 🗺️ Roadmap & Implementation Phases

- [x] **Project Initialization**: Repository structure, dual MIT/Apache-2.0 licenses, CI release pipeline.
- [ ] **Phase 1: Minimal PoC (Core Tween & Evaluation)**:
  - Strongly typed `Ease` and `TweenVars` with canonical GSAP string formatting.
  - Fluent `TweenBuilder` and `TweenHandle` (`play()`, `pause()`, `reverse()`).
  - `use_gsap()` hook with scoped context and automatic unmount cleanup (`ctx.revert()`).
  - Idempotent runtime script injection guard.
- [ ] **Phase 2: Timeline & Scrubbing Sequencer**:
  - `TimelineBuilder` with relative position offsets (`"-=0.2"`, `"+=0.5"`).
  - First-class scrubbing API: `.seek()`, `.progress()`, `.time_scale()`.
  - Signal-bound scrubber hook (`use_timeline_scrubber`).
- [ ] **Phase 3: Ecosystem Integrations & Plugins**:
  - `to_js()` / generic in-engine target support for `trioxus` (3D) and `nodoxus` (Canvas).
  - Official `ScrollTrigger` reactive binding.
  - Interactive showcase and benchmark suite.

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

## ⚖️ Trademark Disclaimer

*GSAP is a trademark of GreenSock / Webflow. `dioxusap` is an independent open-source community project and is not affiliated with or endorsed by GreenSock or Webflow.*
