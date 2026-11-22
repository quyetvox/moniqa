# 🖥️ MoniQA — macOS System & Hardware Resource Monitor

> **A lightweight, beautiful, and real-time macOS resource monitor built with Rust, Tauri v2, Svelte 5, and Tailwind CSS.**  
> *Specifically optimized for Apple Silicon (M1/M2/M3/M4/M5), with zero sudo requirements.*

---

## ✨ Features

- **⚡ CPU Load & Frequency:**
  - Real-time global usage % and per-core distribution bars.
  - Live clock frequencies (MHz) and Apple Silicon model name.
  - 30-second smooth trend sparkline.
- **💾 Memory & Swap:**
  - Physical RAM consumption (Used / Total / Available).
  - Virtual Swap memory tracking.
  - Memory pressure trend line.
- **🎮 GPU & Apple Silicon Telemetry:**
  - Hardware GPU active utilization % via IOKit & IOReport.
  - Real-time GPU power draw in Watts.
  - Dynamic Unified Memory allocation estimation.
- **🌡️ Thermals & Cooling:**
  - On-die CPU & GPU temperatures in Celsius (°C).
  - Active cooling fan RPM speeds (or passive cooling indicator on MacBook Air).
- **💽 Disk & 🌐 Network I/O:**
  - System-wide network download and upload throughput rates (B/s, KB/s, MB/s).
  - Storage partitions mount points and disk space capacity meters.
- **🔋 Battery & Power Health:**
  - Battery charge percentage and charging state.
  - Cycle count, battery health %, and internal battery temperature.
- **📋 Top Active Processes:**
  - Real-time top processes by CPU or Memory usage.
  - Instant search filter and sorting toggle.
- **🖥️ Dual UX Architecture:**
  - **Menubar Popover (340px):** Quick glance at essential meters right below the macOS status bar. Automatically dismisses on blur.
  - **Full Dashboard (960px):** Deep-dive metrics, historical trends, per-core bars, and full process tables.
- **🌙 Dark & ☀️ Light Mode:**
  - Apple SF Pro aesthetics with smooth glassmorphism and automatic or manual theme toggle.

---

## 🚀 Installation & Running

### Option 1: Install from `.dmg` (Prebuilt)

1. Download or locate `MoniQA_0.1.0_aarch64.dmg` under `src-tauri/target/release/bundle/dmg/`.
2. Double-click to mount the `.dmg` disk image.
3. Drag **MoniQA** into your `/Applications` folder.
4. **First Launch (Gatekeeper Note):**  
   Since this is a self-built app without an Apple Developer certificate, right-click `MoniQA.app` and choose **Open**, then confirm **Open** on the dialog.

---

## 🛠️ Development Setup

### Prerequisites
- macOS 13+ (Ventura, Sonoma, Sequoia)
- [Rust](https://www.rust-lang.org/) `1.80+` (`cargo`, `rustc`)
- [Node.js](https://nodejs.org/) `20+` & `npm`

### Commands

```bash
# 1. Clone or navigate to the repository
cd /Users/voquyt/Documents/projects/services/moniqa

# 2. Install frontend dependencies
npm install

# 3. Start development mode with hot reload
npm run tauri dev

# 4. Build optimized production release (.dmg)
npm run tauri build
```

The output DMG and App bundle will be generated at:
```
src-tauri/target/release/bundle/dmg/MoniQA_0.1.0_aarch64.dmg
src-tauri/target/release/bundle/macos/MoniQA.app
```

---

## 🏗️ Architecture

```
┌────────────────────────────────────────────────────────┐
│                        macOS                           │
│                                                        │
│  ┌──────────────────────┐   IPC Event   ┌────────────┐ │
│  │     Rust Core        │ ────────────► │  Svelte 5  │ │
│  │                      │               │   Runes    │ │
│  │ • sysinfo v0.40      │ ◄──────────── │            │ │
│  │ • macmon v0.12 (SMC) │  Invoke Cmd   └────────────┘ │
│  └──────────┬───────────┘                              │
│             │                                          │
│      ┌──────▼──────┐                                   │
│      │ Menubar Tray│ Mini Stats: CPU% + Temp           │
│      └─────────────┘                                   │
└────────────────────────────────────────────────────────┘
```

- **Backend (`src-tauri/`):**
  - Background async loop runs every 1,000ms.
  - Samples hardware counters using Apple `IOKit`, `IOReport`, and `AppleSMC`.
  - Serializes metrics into `SystemSnapshot` and broadcasts to Svelte via Tauri IPC.
- **Frontend (`src/`):**
  - Svelte 5 with fine-grained reactivity (`$state`, `$derived`, `$effect`).
  - Tailored Tailwind CSS dark/light theme.
  - Zero heavy third-party chart dependencies; lightweight SVG sparklines render at 60fps.

---

## 📄 License
MIT © 2026 MoniQA Project.
