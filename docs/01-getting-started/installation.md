# Installation

> **Prerequisites:** [What is Rust?](what-is-rust.md)  
> **Next:** [Your First Project](your-first-project.md)

Rust is installed and managed through **rustup** — the official toolchain installer. Think of it as `pyenv` for Rust: it manages compiler versions, lets you switch between stable and nightly channels, and installs extra components like `rustfmt` and `clippy`.

---

## Linux

```bash
# Download and run the rustup installer script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The installer walks you through options. Accept the defaults (press `1`). When it finishes, reload your shell:

```bash
# Bash
source "$HOME/.cargo/env"

# Add permanently to your shell profile
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc    # bash
echo 'source "$HOME/.cargo/env"' >> ~/.zshrc     # zsh
```

**What gets installed:**
- `~/.cargo/bin/` — cargo, rustc, rustup, and other tools
- `~/.rustup/` — toolchain files (compiler, standard library)

**Linux dependencies.** Some crates need a C linker. Install it once:

```bash
# Ubuntu / Debian
sudo apt-get update && sudo apt-get install -y build-essential pkg-config

# Fedora / RHEL
sudo dnf groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

---

## macOS

```bash
# Option 1: rustup (recommended — same as Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

```bash
# Option 2: Homebrew (fixed version; no toolchain management)
brew install rust
```

macOS requires the Xcode Command Line Tools for the linker:

```bash
xcode-select --install
```

If you are on Apple Silicon (M1/M2/M3), the installer detects your architecture automatically and installs the correct `aarch64-apple-darwin` target.

---

## Windows

**Option 1: rustup-init.exe (recommended)**

1. Download the installer from [https://rustup.rs](https://rustup.rs)
2. Run `rustup-init.exe`
3. Accept the default installation when prompted

The installer adds `%USERPROFILE%\.cargo\bin` to your `PATH` automatically. Open a new terminal after installation.

**Prerequisite — C++ Build Tools.** Rust needs the MSVC linker:

- Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- In the installer, select **"Desktop development with C++"**
- Minimum required: MSVC compiler + Windows SDK

Alternatively, use the GNU toolchain (MinGW):

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**Option 2: winget**

```powershell
winget install Rustlang.Rustup
```

**Option 3: Chocolatey**

```powershell
choco install rustup.install
```

**Option 4: Scoop**

```powershell
scoop install rustup
```

---

## Verify the Installation

Run these in a new terminal after installation:

```bash
rustc --version
# rustc 1.78.0 (9b00956e5 2024-04-29)

cargo --version
# cargo 1.78.0 (54d8815d0 2024-04-09)

rustup --version
# rustup 1.27.0 (bbb9276d2 2024-03-08)
```

Compile and run a quick smoke test:

```bash
# Linux / macOS
echo 'fn main() { println!("Rust is working!"); }' > /tmp/hello.rs
rustc /tmp/hello.rs -o /tmp/hello
/tmp/hello
# Rust is working!
```

---

## Manage Your Toolchain with rustup

```bash
# Show what is installed
rustup show

# Update everything (compiler + components)
rustup update

# Install a specific version
rustup install 1.75.0

# Switch default toolchain
rustup default stable    # latest stable (recommended)
rustup default nightly   # nightly (needed for some experimental features)
rustup default 1.75.0   # pin to a specific version

# Override for one directory only (like pyenv's .python-version)
cd my-project
rustup override set 1.75.0

# Pin via file (commit this so teammates get the same version)
cat > rust-toolchain.toml << 'EOF'
[toolchain]
channel = "1.75.0"
components = ["rustfmt", "clippy"]
EOF

# Add a cross-compilation target
rustup target add aarch64-unknown-linux-gnu   # Linux ARM64 (e.g. AWS Graviton)
rustup target add x86_64-unknown-linux-musl   # static Linux binary
```

---

## Essential Components

Install these once — they make day-to-day development much smoother:

```bash
# Code formatter (like Black for Python)
rustup component add rustfmt

# Linter with helpful suggestions (like flake8/pylint, but much smarter)
rustup component add clippy

# Language server — powers IDE autocompletion and inline errors
rustup component add rust-analyzer

# Standard library source (enables "go to definition" into std)
rustup component add rust-src
```

Verify:

```bash
cargo fmt --version
cargo clippy --version
rust-analyzer --version
```

---

## Environment at a Glance

After setup, you should have:

```
~/.cargo/bin/
├── cargo        # build tool and package manager
├── rustc        # compiler
├── rustup       # toolchain manager
├── rustfmt      # formatter
├── cargo-clippy # linter (invoked as `cargo clippy`)
└── rust-analyzer # language server (used by your IDE)
```

---

## Troubleshooting

**`cargo` not found after installation**

```bash
source "$HOME/.cargo/env"   # Linux / macOS
# Or restart your terminal
```

**`linker 'cc' not found` on Linux**

```bash
sudo apt-get install build-essential   # Ubuntu / Debian
sudo dnf groupinstall "Development Tools"  # Fedora
```

**`linker 'link.exe' not found` on Windows**

Install Visual Studio Build Tools with the "Desktop development with C++" workload, or switch to the GNU toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

---

## See Also

- [Your First Project](your-first-project.md) — use your new installation
- [IDE Setup](ide-setup.md) — configure your editor
- [Troubleshooting](../05-reference/troubleshooting.md) — more common errors
