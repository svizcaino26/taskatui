# taskatui

> a task tracker for those who live in the terminal

Taskatui is a lightweight TUI (terminal user interface) productivity app designed around clarity, keyboard-first navigation, and a zero-friction workflow.

Born from the frustration of bloated task apps, its goal is simple: **manage tasks without leaving your terminal.**

---

## 🧪 Status (WIP)

`taskatui` is currently in **active development**. Database layer complete, UI layer in progress.

What exists today:

* ✅ Core task + subtask storage
* ✅ Editing + deletion logic
* ✅ Data model & in-memory state manager

What’s missing:

* 🔨 UI rendering (ratatui) in progress
* ⌨️ Input handling & navigation logic
* 📂 Project / category grouping (future)

---

## ✨ Features (MVP)

* ✅ Local storage using SQLite (persistent, no cloud, no tracking)
* ✅ Create / edit tasks
* ✅ Add and manage subtasks
* ✅ Mark tasks and subtasks as completed
* ✅ Simple, Rust-native architecture (no JS, no bloat)
* ✅ Instant startup

### Planned / Next

* ⏳ Follow-up dates & reminders
* ⏳ Filtering & search
* ⏳ Keyboard shortcuts reference panel
* ⏳ Theming

---

## 🧱 Tech Stack

| Layer        | Tooling                 |
| ------------ | ----------------------- |
| Language     | Rust                    |
| UI framework | ratatui                 |
| Database     | SQLite (`sqlx`)         |
| Style        | Minimal, keyboard-first |

---

## 🚀 Why?

Because sometimes you don’t want a web app, sync account, background daemon, or telemetry just to store a couple of todos.

`taskatui` is purpose-built for developers who live in their terminal.

---

## 📦 Installation (TBA)

```sh
# coming soon
cargo install taskatui
```

---

## 🧭 Roadmap

The roadmap will grow as the project evolves, but early-stage goals include:

* Richer navigation (jumping between task groups)
* Archiving instead of deleting
* Export / import
* Optional encryption (file-level)
* Plugin-friendly architecture

---

## ❤️ License

MIT License — do whatever, just don’t sue.

---

## 🤝 Contributing

This project is currently pre-release, but feedback and ideas are welcome.

---

## Screenshots (coming soon)
