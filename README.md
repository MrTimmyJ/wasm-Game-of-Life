# Conway's Game of Life
Made in Rust using Web Assembly.

Author: Timothy Johnson <br>
Date: May 13 2024 to June 3 2024

## Overview

A performant, multithreaded implementation of Conway’s Game of Life using Rust and WebAssembly.
This interactive cellular automaton runs directly in the browser, leveraging Rust’s memory safety and WebAssembly's execution speed to deliver a responsive simulation rendered in HTML/JavaScript.

This project simulates Conway's Game of Life with a focus on performance, modularity, and WebAssembly integration. Written in Rust and compiled to WebAssembly, it offers a fast and memory-safe runtime with multithreaded simulation logic.

Useful as an educational tool, playground for WebAssembly + Rust experiments, or foundation for more complex simulations.

🧩 Features

    ♻️ Multithreaded Update Logic: Uses native Rust threads to parallelize universe state updates

    🕸️ WebAssembly Integration: Compiles to .wasm for browser execution

    ◼️ ASCII Output for Debugging: Simple visual feedback of simulation state

    🧮 Cell Toggle Logic: Click to activate/deactivate individual cells (JS-side integration)

    ⏱️ Performance Profiling: Custom Timer struct logs execution times to the browser console

    💻 Cross-Platform: Runs on any browser with WebAssembly support

🔄 User Workflow

    Launch the web application (or serve locally)

    Watch Conway’s Game of Life evolve with real-time updates

    Click cells to toggle state

    Pause game state to stop game

    Modify update rules or grid size via Rust and recompile

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
    
    🎲 std::thread	For multithreading the simulation logic
    
    🖨️ web_sys	Access browser console logging
    
    🧪 wasm-pack	Build, test, and package WASM crate

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
