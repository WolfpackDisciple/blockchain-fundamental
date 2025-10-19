
# 🧱 Blockchain Fundamentals Learning Path

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Step-by-step blockchain implementation in Rust** - from basic concepts to educational mining simulation.

## 📖 Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Learning Levels](#learning-levels)
- [Quick Start](#quick-start)
- [Requirements](#requirements)
- [License](#license)

## 🎯 Overview

A series of 4 independent projects, each representing a level of blockchain understanding:

- **blockchain-level-1** - Basic blockchain chain
- **blockchain-level-2** - Cryptographic hashing  
- **blockchain-level-3** - Validation and timestamps
- **blockchain-level-4** - Proof-of-Work mining

> 💡 **Note**: Educational implementation - real projects are more complex.

## 📁 Project Structure

```markdown
blockchain-fundamental/
├── blockchain-level-1/     # Simple blockchain chain
│   ├── src/main.rs
│   └── Cargo.toml
├── blockchain-level-2/     # Blockchain with hashing
│   ├── src/main.rs
│   └── Cargo.toml
├── blockchain-level-3/     # Advanced validation
│   ├── src/main.rs
│   └── Cargo.toml
├── blockchain-level-4/     # Proof-of-Work mining
│   ├── src/main.rs
│   └── Cargo.toml
└── README.md
```

## 🚀 Learning Levels

### Level 1: Basic Chain Concept
**Goal**: Understand linked blocks

```rust
struct Block {
    id: u32,
    data: String,
    previous_id: u32
}
```

**Run**: `cd blockchain-level-1 && cargo run`

### Level 2: Cryptographic Integrity  
**Goal**: Add hashing for data protection

```rust
fn real_hash(data: &str) -> String {
    // SHA256 implementation
}
```

**Run**: `cd blockchain-level-2 && cargo run`

### Level 3: Advanced Validation
**Goal**: Add timestamps and comprehensive checks

```rust
struct Block {
    timestamp: u64,
    // ... other fields
}
```

**Run**: `cd blockchain-level-3 && cargo run`

### Level 4: Proof-of-Work Mining
**Goal**: Implement consensus algorithm

```rust
fn mine(&mut self) {
    // Proof-of-Work implementation
}
```

**Run**: `cd blockchain-level-4 && cargo run`

## 🛠 Quick Start

```bash
# Clone repository
git clone <repository-url>
cd blockchain-fundamental

# Run levels
cd blockchain-level-1 && cargo run 
cd blockchain-level-2 && cargo run
cd blockchain-level-3 && cargo run 
cd blockchain-level-4 && cargo run 

# Testing
cd blockchain-level-3 && cargo test 
cd blockchain-level-4 && cargo test 
```

## 📋 Requirements

- **Rust**: version 1.70 or newer
- **Cargo**: Rust package manager

## 🎯 Recommended Learning Order

1. **blockchain-level-1** - Basic blockchain structure
2. **blockchain-level-2** - Cryptographic fundamentals  
3. **blockchain-level-3** - Validation and security
4. **blockchain-level-4** - Consensus algorithms

## 🔧 For Developers

Each project is self-contained with:
- Independent dependencies
- Own Cargo.toml
- Isolated code
- Clear learning objective

## 🛡 License

**MIT License** - see [LICENSE](LICENSE) file for details

## 🤝 Contribution

1. Fork the repository
2. Create a feature branch
3. Open a Pull Request

---

<div align="center">

**«From simple to complex - the path to mastery»** 🚀

</div>
```
