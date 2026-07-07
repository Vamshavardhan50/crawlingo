# Installation

## Python SDK

```bash
pip install crawlingo
```

Requires Python 3.8+ and glibc 2.28+ (Linux), VC++ Redistributable (Windows).

Pre-built wheels for:
- Linux: x86_64, aarch64
- macOS: x86_64, arm64
- Windows: AMD64

```bash
# Verify installation
python -c "import crawlingo; print(crawlingo.__version__)"
```

## Node.js SDK

```bash
npm install crawlingo
```

Requires Node.js 18+. Pre-built native addons are downloaded automatically for common platforms.

```bash
# Verify installation
node -e "const c = require('crawlingo'); console.log(Object.keys(c))"
```

## Rust SDK

Add to your `Cargo.toml`:

```toml
[dependencies]
crawlingo = "0.1"
```

Requires Rust 1.70+.

## Build from Source

```bash
git clone https://github.com/Vamshavardhan50/crawlingo.git
cd crawlingo

# Rust core
cargo build --release

# Python (from source)
cd sdk/python
pip install maturin
maturin develop --release

# Node.js (from source)
cd sdk/nodejs
npm install
npm run build
```

## Platform Requirements

| Platform | Requirements |
|----------|-------------|
| Linux | glibc 2.28+, Rust 1.70+ |
| macOS | 10.15+, Xcode CLI tools |
| Windows | VC++ Redist 2019+, Rust 1.70+ |

## Docker

```dockerfile
FROM python:3.12-slim
RUN pip install crawlingo
COPY script.py .
CMD ["python", "script.py"]
```
