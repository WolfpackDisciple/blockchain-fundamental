# 🧱 Blockchain Fundamentals Learning Path / Фундаментальные Основы Блокчейна

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Поэтапная реализация блокчейна на Rust** - от базовых концепций до учебной симуляции майнинга. 
**Step-by-step blockchain implementation in Rust** - from basic concepts to educational mining simulation.

---

## 🇷🇺 Русская Версия

### 📖 Оглавление
- [Обзор](#обзор)
- [Структура проектов](#структура-проектов)
- [Уровни обучения](#уровни-обучения)
- [Быстрый старт](#быстрый-старт)
- [Требования](#требования)
- [Лицензия](#лицензия)

### 🎯 Обзор

Это серия из 4 независимых проектов, каждый из которых представляет собой очередной уровень понимания технологии блокчейн:

- **blockchain-level-1** - Базовая цепочка блоков
- **blockchain-level-2** - Криптографическое хеширование  
- **blockchain-level-3** - Валидация и временные метки
- **blockchain-level-4** - Proof-of-Work майнинг

> 💡 **Примечание**: Код отличается от реальных проектов. Реальные проекты сложнее. Мы идем от простого к сложному. Эти 4 фундаментальных проекта - только начало, далее будут добавлены более сложные вещи.

> 🤝 **Сообщество**: Я такой же человек как и вы. Если вы понимаете в этом больше меня, помогите улучшить мой код через Issues или Pull Requests.

### 📁 Структура проектов
blockchain-fundamental/
├── blockchain-level-1/ # Простейшая цепочка блоков
│ ├── src/main.rs
│ └── Cargo.toml
├── blockchain-level-2/ # Блокчейн с хешированием
│ ├── src/main.rs
│ └── Cargo.toml
├── blockchain-level-3/ # Расширенная валидация
│ ├── src/main.rs
│ └── Cargo.toml
├── blockchain-level-4/ # Майнинг Proof-of-Work
│ ├── src/main.rs
│ └── Cargo.toml
└── README.md

text

### 🚀 Уровни обучения

#### Level 1: Базовая концепция цепочки
**Цель**: Понять основную идею связанных блоков

```rust
struct Block {
    id: u32,
    data: String,
    previous_id: u32
}
Запуск:

bash
cd blockchain-level-1 && cargo run
Level 2: Криптографическая целостность
Цель: Добавить хеширование для защиты данных

rust
fn real_hash(data: &str) -> String {
    // SHA256 implementation
}
Запуск:

bash
cd blockchain-level-2 && cargo run
Level 3: Расширенная валидация
Цель: Добавить временные метки и комплексные проверки

rust
struct Block {
    timestamp: u64,
    // ... другие поля
}
Запуск:

bash
cd blockchain-level-3 && cargo run
Level 4: Proof-of-Work майнинг
Цель: Реализовать алгоритм консенсуса

rust
fn mine(&mut self) {
    // Proof-of-Work implementation
}
Запуск:

bash
cd blockchain-level-4 && cargo run
🛠 Быстрый старт
bash
# Клонирование
git clone <repository-url>
cd blockchain-fundamental

# Запуск уровней
cd blockchain-level-1 && cargo run && cd ..
cd blockchain-level-2 && cargo run && cd ..
cd blockchain-level-3 && cargo run && cd ..
cd blockchain-level-4 && cargo run && cd ..

# Тестирование
cd blockchain-level-3 && cargo test && cd ..
cd blockchain-level-4 && cargo test && cd ..
📋 Требования
Rust: версия 1.70 или новее

Cargo: менеджер пакетов Rust

🇺🇸 English Version
📖 Table of Contents
Overview

Project Structure

Learning Levels

Quick Start

Requirements

License

🎯 Overview
This is a series of 4 independent projects, each representing a successive level of blockchain technology understanding:

blockchain-level-1 - Basic blockchain chain

blockchain-level-2 - Cryptographic hashing

blockchain-level-3 - Validation and timestamps

blockchain-level-4 - Proof-of-Work mining

💡 Note: The code differs from real projects. Real projects are more complex. We go from simple to complex. These 4 fundamental projects are just the beginning, more advanced topics will be added later.

🤝 Community: I'm just like you. If you understand this better than me, please help improve my code through Issues or Pull Requests.

📁 Project Structure
text
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
🚀 Learning Levels
Level 1: Basic Chain Concept
Goal: Understand the basic idea of linked blocks

rust
struct Block {
    id: u32,
    data: String,
    previous_id: u32
}
Run:

bash
cd blockchain-level-1 && cargo run
Level 2: Cryptographic Integrity
Goal: Add hashing for data protection

rust
fn real_hash(data: &str) -> String {
    // SHA256 implementation
}
Run:

bash
cd blockchain-level-2 && cargo run
Level 3: Advanced Validation
Goal: Add timestamps and comprehensive checks

rust
struct Block {
    timestamp: u64,
    // ... other fields
}
Run:

bash
cd blockchain-level-3 && cargo run
Level 4: Proof-of-Work Mining
Goal: Implement consensus algorithm

rust
fn mine(&mut self) {
    // Proof-of-Work implementation
}
Run:

bash
cd blockchain-level-4 && cargo run
🛠 Quick Start
bash
# Clone repository
git clone <repository-url>
cd blockchain-fundamental

# Run levels
cd blockchain-level-1 && cargo run && cd ..
cd blockchain-level-2 && cargo run && cd ..
cd blockchain-level-3 && cargo run && cd ..
cd blockchain-level-4 && cargo run && cd ..

# Testing
cd blockchain-level-3 && cargo test && cd ..
cd blockchain-level-4 && cargo test && cd ..
📋 Requirements
Rust: version 1.70 or newer

Cargo: Rust package manager

🎯 Recommended Learning Order / Рекомендуемый Порядок Изучения
English:

blockchain-level-1 - Understand basic blockchain structure

blockchain-level-2 - Learn cryptographic fundamentals

blockchain-level-3 - Master validation and security

blockchain-level-4 - Explore consensus algorithms

Русский:

blockchain-level-1 - Понять базовую структуру блокчейна

blockchain-level-2 - Изучить криптографические основы

blockchain-level-3 - Разобраться с валидацией и безопасностью

blockchain-level-4 - Освоить консенсусные алгоритмы

🔧 For Developers / Для Разработчиков
English:
Each project is self-contained with:

Independent dependencies

Own Cargo.toml

Isolated code

Clear learning objective

Русский:
Каждый проект самодостаточен и имеет:

Независимые зависимости

Собственный Cargo.toml

Изолированный код

Четкую учебную цель

🛡 License / Лицензия
MIT License - see LICENSE file for details

🤝 Contribution / Вклад
English:
If you want to improve these learning materials:

Fork the repository

Create a branch for your improvement

Open a Pull Request with description of changes

Русский:
Если вы хотите улучшить эти учебные материалы:

Форкните репозиторий

Создайте ветку для вашего улучшения

Откройте Pull Request с описанием изменений

<div align="center">
«From simple to complex - the path to mastery»
«От простого к сложному - путь к мастерству» 🚀

</div> ```
