# Distribution

> **Prerequisites:** [logferry Walkthrough](logferry-walkthrough.md)  
> **Next:** [Cargo Cheatsheet](../05-reference/cargo-cheatsheet.md)

Once your PyO3 extension works correctly, this article shows how to build distributable wheels, write type stubs so Python type checkers understand your API, and optionally embed the extension inside a larger Python package.

---

## Building a Single-Platform Wheel

```bash
maturin build --release
# dist/logferry-0.1.0-cp311-cp311-linux_x86_64.whl
```

The wheel filename encodes the Python version and platform (`cp311` = CPython 3.11, `linux_x86_64`). It is a self-contained binary — no Rust toolchain needed on the machine that installs it.

```bash
pip install dist/logferry-*.whl
python -c "import logferry; print(logferry.ingest_logs.__doc__)"
```

---

## Cross-Platform Wheels with GitHub Actions

The standard approach: run `maturin build` on three OS runners and publish all three wheels to PyPI. The [maturin-action](https://github.com/PyO3/maturin-action) handles this in about 10 lines of YAML:

```yaml
# .github/workflows/publish.yml
name: Publish to PyPI
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: PyO3/maturin-action@v1
        with:
          command: build
          args: --release --out dist
      - uses: actions/upload-artifact@v4
        with:
          name: wheels-${{ matrix.os }}
          path: dist

  publish:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
        with:
          pattern: wheels-*
          merge-multiple: true
          path: dist
      - uses: PyO3/maturin-action@v1
        with:
          command: upload
          args: --non-interactive --skip-existing dist/*
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_API_TOKEN }}
```

---

## Embedding Inside a Larger Python Package

If `logferry` should live inside a bigger package (e.g. `mlops_toolkit`), use maturin's "mixed" layout:

```
mlops_toolkit/
├── Cargo.toml
├── pyproject.toml
├── src/
│   └── lib.rs              # rename #[pymodule] to `fn _logferry`
└── python/
    └── mlops_toolkit/
        ├── __init__.py
        └── py.typed
```

`pyproject.toml`:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "mlops-toolkit"
version = "0.1.0"

[tool.maturin]
python-source = "python"
module-name = "mlops_toolkit._logferry"
```

Rename the `#[pymodule]` function in `lib.rs` to `fn _logferry(...)`, then in `python/mlops_toolkit/__init__.py`:

```python
from mlops_toolkit._logferry import ingest_logs, validate_line, IngestStats

__all__ = ["ingest_logs", "validate_line", "IngestStats"]
```

Consumers write `from mlops_toolkit import ingest_logs` and never see that part of it is Rust.

```bash
pip install -e .     # editable install (rebuilds Rust on change)
maturin develop      # alternative during day-to-day development
```

---

## Writing Type Stubs for mypy / Pylance

PyO3 does not generate `.pyi` stubs automatically. Without them, type checkers see your extension as `Any`. Write a stub file alongside the compiled module:

```python
# logferry.pyi
from typing import Optional

class IngestStats:
    total_lines: int
    parsed_ok: int
    parse_errors: int
    validation_errors: int
    by_level: dict[str, int]
    avg_latency_ms: Optional[float]
    sample_errors: list[str]

def ingest_logs(
    lines: list[str],
    num_threads: int = ...,
) -> IngestStats: ...

def validate_line(line: str) -> bool: ...
```

Place it in the same directory as `logferry.so` (or in the package's `python/` directory for mixed layouts), and add a `py.typed` marker file:

```bash
touch python/logferry/py.typed
```

With this in place, Pylance and mypy see full types for `IngestStats`, `ingest_logs`, and `validate_line`.

---

## Publishing to PyPI

```bash
# Build optimised wheels for the current platform
maturin build --release

# Publish to PyPI (needs MATURIN_PYPI_TOKEN or a configured ~/.pypirc)
maturin publish

# Publish to TestPyPI first (recommended for first-time publishing)
maturin publish --repository testpypi
```

---

## `pyproject.toml` for `logferry`

The existing `pyproject.toml` is minimal. A production-ready version:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "logferry"
version = "0.1.0"
description = "A multi-threaded JSON log ingestor exposed to Python via PyO3"
readme = "README.md"
license = {text = "MIT"}
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Topic :: System :: Logging",
]

[tool.maturin]
features = ["pyo3/extension-module"]
```

---

## See Also

- [PyO3 Overview](pyo3-overview.md) — `maturin develop` for development builds
- [Cargo Cheatsheet](../05-reference/cargo-cheatsheet.md) — all maturin commands
- [Troubleshooting](../05-reference/troubleshooting.md) — common PyO3 / maturin errors
