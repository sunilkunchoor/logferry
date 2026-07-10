# IDE Setup

> **Prerequisites:** [Installation](installation.md)  
> **Next:** [Variables and Types](../02-rust-language-fundamentals/variables-and-types.md)

A good IDE setup makes a huge difference with Rust. The language server, **rust-analyzer**, provides real-time type inference, inline error squiggles, autocompletion, and refactoring — all driven by the same compiler that will ultimately build your code.

---

## VS Code (Recommended)

### Installation

1. Install [VS Code](https://code.visualstudio.com/)
2. Install the **rust-analyzer** extension — Extension ID: `rust-lang.rust-analyzer`
3. Optionally install **Even Better TOML** (`tamasfe.even-better-toml`) for `Cargo.toml` syntax highlighting

### Recommended Settings

Add to `.vscode/settings.json` in your project (or to your user settings):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### What rust-analyzer Gives You

- **Inline type hints** — see inferred types on every `let` binding (like Pylance for Python)
- **Autocompletion** — methods, fields, and imports suggested as you type
- **Red squiggles for compile errors** — no need to run `cargo build` to find errors
- **Inline documentation on hover** — see doc comments for any symbol
- **Go to definition / Find all references** — across your whole project
- **Automatic import insertion** — `use` statements added when you accept a completion
- **Refactoring** — rename, extract function, fill all match arms

### Useful Keyboard Shortcuts

| Action | Mac | Windows / Linux |
|---|---|---|
| Go to definition | `F12` | `F12` |
| Show all references | `⇧F12` | `Shift+F12` |
| Rename symbol | `F2` | `F2` |
| Trigger suggestions | `⌃Space` | `Ctrl+Space` |
| Format document | `⇧⌥F` | `Shift+Alt+F` |
| Open Problems panel | `⇧⌘M` | `Ctrl+Shift+M` |

---

## RustRover / IntelliJ

JetBrains offers two options:

- **RustRover** — standalone IDE dedicated to Rust (free for non-commercial use). Download at [jetbrains.com/rust](https://www.jetbrains.com/rust/).
- **IntelliJ IDEA** + **Rust plugin** — if you already have IntelliJ.

RustRover has the same feature set as rust-analyzer but embedded in JetBrains' polished UI, including integrated debugger (LLDB), profiler, and test runner with coverage. If you already live in IntelliJ for other languages, this is the natural choice.

---

## Neovim

```bash
# Install rust-analyzer binary (if not already done via rustup)
rustup component add rust-analyzer
```

In Neovim, using `lazy.nvim` + `nvim-lspconfig`, add to your Lua config:

```lua
require("lspconfig").rust_analyzer.setup({
    settings = {
        ["rust-analyzer"] = {
            checkOnSave = { command = "clippy" },
            cargo = { features = "all" },
        },
    },
})
```

For a full Rust Neovim setup, the [rustaceanvim](https://github.com/mrcjkb/rustaceanvim) plugin gives the richest experience — it adds DAP debugging, hover actions, and runnables directly in the editor.

---

## Verifying Your Setup

Open the `logferry` repository in your IDE:

1. Open `src/lib.rs`
2. Hover over a type like `IngestStats` — you should see its documentation pop up
3. Ctrl+click (or `F12`) on `ingest_chunk` to jump to its definition
4. Make a deliberate type error (e.g. assign `"hello"` to a `usize` field) — a red squiggle should appear within a few seconds

If these work, rust-analyzer is running correctly.

---

## Troubleshooting rust-analyzer in VS Code

**rust-analyzer not working**

1. Ensure the component is installed: `rustup component add rust-analyzer`
2. Reload the window: `Ctrl+Shift+P` → "Developer: Reload Window"
3. Check the output panel: `Ctrl+Shift+P` → "rust-analyzer: Show RA server logs"

**Slow startup**

rust-analyzer indexes the whole project on first open, which can take 30–60 seconds on large projects. The spinner in the status bar will disappear when it is ready.

---

## See Also

- [Your First Project](your-first-project.md) — build something to test your setup
- [Daily Workflow](../03-project-setup-and-tooling/daily-workflow.md) — the recommended inner loop for day-to-day development
