# Neko233 Hardware Viewer (Cyberpunk Edition)

A modern, Cyberpunk-themed hardware diagnostic tool for Windows, built with **Rust**, **Tauri**, and **Vue 3**.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue)

## ✨ Features

- **Cyberpunk Aesthetic**: Dark mode, neon glows, and glitch effects.
- **Comprehensive Hardware Scan**:
  - **Motherboard**: Manufacturer, Model, Version, Serial.
  - **CPU**: Name, Cores, Threads, Clock Speed, Scoring.
  - **GPU**: Name, Driver, VRAM, Scoring.
  - **RAM**: Capacity, Speed, Slot Information, Scoring.
  - **Storage**: Model, Size, Interface, Scoring.
  - **Audio**: Sound device enumeration.
- **Performance Scoring**: Automatic grading (Excellent/Good/Average/Poor) based on hardware specs.
- **Portable**: Single `.msi` installer or executable.

## 🛠️ Tech Stack

- **Backend**: Rust (Tauri, WMI, Sysinfo)
- **Frontend**: Vue 3, TypeScript, Vite
- **Styling**: Custom CSS (Cyberpunk Theme)

## 🚀 Getting Started

### Prerequisites

- **Node.js** (v16+)
- **Rust** (latest stable)
- **Visual Studio C++ Build Tools** (for Windows development)

### Development

1. Install dependencies:

   ```powershell
   npm install
   ```

2. Run in development mode:

   ```powershell
   npm run tauri dev
   # Or use the helper script:
   .\dev.cmd
   ```

### Build for Production

To create a Windows installer (`.msi`):

```powershell
npm run tauri build
# Or use the helper script:
.\build-windows.cmd
```

The output will be in `src-tauri/target/release/bundle/msi/`.

## 📂 Project Structure

- `src-tauri/`: Rust backend code.
  - `src/main.rs`: Main application logic and hardware detection commands.
- `src/`: Vue frontend code.
  - `components/HardwareView.vue`: Main UI component.
  - `assets/cyberpunk.css`: Global styling.
- `.github/workflows/`: CI/CD configuration for GitHub Actions.

## 📝 License

MIT License

