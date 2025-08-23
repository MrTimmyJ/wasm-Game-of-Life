# Conway's Game of Life
Made in Rust using Web Assembly.

Author: Timothy Johnson <br>
Date: May 13 2024 to June 3 2024

## Overview

&nbsp;&nbsp;&nbsp;&nbsp; An interactive, high-performance implementation of Conway’s Game of Life, built in Rust and compiled to WebAssembly, running seamlessly in the browser. The project emphasizes speed, modularity, and real-world deployment, featuring both a standard single-threaded version and an optimized multithreaded version using Rayon for parallel computation.

&nbsp;&nbsp;&nbsp;&nbsp; The app is hosted on a cloud-based Ubuntu server with Nginx, making it accessible as a live demo. It showcases how Rust’s memory safety and WebAssembly’s execution speed can be combined with modern web technologies to deliver efficient, interactive simulations directly in the browser.

🧩 Features

    ♻️ Multithreaded Simulation: Parallel universe updates powered by Rayon (with single-threaded fallback).

    🕸️ Rust → WebAssembly Integration: Compiled to .wasm and executed in-browser for maximum performance.

    🧮 Interactive Cell Control: Toggle cells in real-time with JavaScript event handling.

    ⏱️ Performance Profiling: Built-in timers log execution stats to the browser console.

    🌐 Live Deployment: Hosted on an Ubuntu server with Nginx for production-ready delivery.

    💻 Cross-Browser Support: Runs anywhere with WebAssembly-enabled browsers.

🔄 User Workflow

    Visit the hosted web demo.

    Watch Conway’s Game of Life evolve in real time.

    Click cells to toggle their state.

    Pause, resume, and experiment with different starting patterns.

    Modify rules or grid size in Rust → recompile → redeploy.

📁 Code Structure

.<br>
wasm-game-of-life/<br>
├── benches/<br> 
│   └── bench.rs &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Rust benchmark tests<br>
├── pkg/<br>
│   ├── package.json &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; JS package metadata for the WASM pkg<br>
│   ├── wasm_game_of_life.d.ts &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; TypeScript declarations for WASM exports<br>
│   ├── wasm_game_of_life.js &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Main JS wrapper for WASM module<br>
│   ├── wasm_game_of_life_bg.js &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Generated helper JS for WASM glue code<br>
│   ├── wasm_game_of_life_bg.wasm &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; The compiled WebAssembly binary<br>
│   └── wasm_game_of_life_bg.wasm.d.ts &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; TS declarations for wasm binary<br>
├── src/<br>
│   ├── lib.rs &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Core simulation logic and WASM bindings<br>
│   └── utils.rs &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Helper functions (logging, memory utils, etc.)<br>
├── tests/<br>
│   └── web.rs &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Tests for WASM/web interaction<br>
├── www/<br>
│   ├── bootstrap.js &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; JS entry point dynamically importing index.js<br>
│   ├── favicon.ico &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Site icon<br>
│   ├── index.html &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Main HTML page<br>
│   ├── index.js &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; JS logic importing WASM & running app<br>
│   ├── package-lock.json &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Node package lock for dependencies<br>
│   ├── package.json &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Node package metadata (for dev/build scripts)<br>
│   └── webpack.config.js &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Webpack bundler<br>
├── Cargo.toml &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Rust metadata and dependencies<br>
├── package-lock.json &nbsp;&nbsp;&nbsp;---&nbsp;&nbsp;&nbsp; Node package lock at project root<br>

⚙️ How It Works

🧱 Grid Model

    The universe is stored as a 1D Vec<Cell> with calculated 2D indexing

    Cells are either Alive or Dead (enum)

    Grid dimensions are defined by width and height

🚦 State Transitions

    On each tick, the simulation calculates the number of living neighbors

    Rules follow Conway’s original logic:

        Any live cell with 2–3 live neighbors survives

        Dead cells with exactly 3 live neighbors become alive

🧵 Multithreaded Updates

    Uses Rayon to divide the grid into parallel tasks (with an alternate std::thread implementation).

    The universe is divided into horizontal slices

    Each slice is processed in a separate thread using std::thread

    Slices are merged at the end to form the next universe state

⏱️ Performance Logging

    A Timer struct wraps execution steps

    Execution time is logged via web_sys::console::time() and .time_end()

🖼️ Screenshots / Visuals

![gameoflifebanner](https://github.com/user-attachments/assets/cc3586f0-08e3-4405-a1ba-5731e9ef112d)

<pre>
  ◻◻◻◻◻◻◻◻◻◻ 
  ◻◻◻◼◼◼◻◻◻◻
  ◻◻◻◼◻◼◻◻◻◻
  ◻◻◻◼◼◼◻◻◻◻
  ◻◻◻◻◻◻◻◻◻◻
</pre>

🧰 Technologies Used

    🦀 Rust	Systems-level language for performance and safety
    
    🕸️ WebAssembly	Target for compiling Rust to run in browsers
    
    🔁 wasm-bindgen	Bindings between JS and Rust/WASM
    
    🎲 Rayon / std::thread — multithreaded simulation engine.
    
    🖨️ web_sys — browser console integration.
    
    🧪 wasm-pack — build, test, and package Rust/WASM crates.

    🌐 Nginx — web server hosting .wasm and frontend assets.

    🐧 Ubuntu Server — cloud deployment environment.

    ⚡ JavaScript (ES6), HTML, CSS — interactive frontend integration.

🚀 Getting Started

    Prerequisites:

      Rust

      wasm-pack (Install via cargo install wasm-pack)

    Build & Run

      -Clone the project
      git clone https://github.com/MrTimmyJ/wasm-game-of-life
      cd wasm-game-of-life

      -Compile to WebAssembly
      wasm-pack build

      start local server with npm

  Get [Rust](https://www.rust-lang.org/tools/install)

🌱 Upcoming Features

    🖼️ Web frontend for cell visualization and interactivity

    🧮 Configurable tick rate and simulation rules

    🌐 Export/import grid states as JSON

    🔲 Dynamic grid resizing

    🧪 Web UI testing with headless browsers

📎 External Reference

[wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)

🪪 License

This project is dual-licensed under:

[MIT License](https://opensource.org/license/mit) <br>
[Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0)
