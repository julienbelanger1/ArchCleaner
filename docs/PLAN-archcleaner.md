# ArchCleaner Plan

## Overview
ArchCleaner is a fast, lightweight, and safe TUI specifically designed for Arch Linux system maintenance and cleanup. It helps users find and delete unused files safely.

## Project Type
BACKEND (CLI/TUI)

## Success Criteria
- Native rust binary providing a beautiful TUI.
- unified list layout targeting everywhere files can hide.
- Dynamically requests sudo via `sudo -S`.
- Ensures nothing critical to system function is removed.

## Tech Stack
- Rust (for speed and low footprint)
- Ratatui (for the TUI framework)
- Crossterm (terminal backend)
- Tokio (for async command execution)

## File Structure
- `src/main.rs`: Entry point
- `src/app.rs`: Application state
- `src/ui.rs`: Rendering logic
- `src/scanner.rs`: Scanner trait for all cleanup modules
- `src/scanners/pacman.rs`: Pacman package cleanup
- `src/scanners/journal.rs`: Systemd journal logs
- `src/scanners/cache.rs`: User directory cache
- `src/sudo.rs`: Privilege escalation

## Task Breakdown
- [ ] Initialize Cargo project and add dependencies. Agent: `backend-specialist`, Skill: `rust-pro`. INPUT: empty folder. OUTPUT: Cargo.toml. VERIFY: `cargo check` passes.
- [ ] Implement App State and basic TUI loop. Agent: `backend-specialist`, Skill: `rust-pro`. INPUT: Cargo.toml. OUTPUT: main.rs, app.rs. VERIFY: UI renders.
- [ ] Implement Scanners (Pacman, Journal, Cache). Agent: `backend-specialist`, Skill: `rust-pro`. INPUT: app.rs. OUTPUT: scanner.rs, pacman.rs, etc. VERIFY: Scanners output accurate byte counts.
- [ ] Implement UI unified list. Agent: `backend-specialist`, Skill: `rust-pro`. INPUT: scanners. OUTPUT: ui.rs. VERIFY: UI displays list of items correctly.
- [ ] Implement Sudo executor. Agent: `security-auditor`. INPUT: sudo.rs. OUTPUT: functional execution. VERIFY: password modal handles correct and incorrect passwords.

## Phase X: Verification
- Lint: `cargo clippy` pass
- Security: `cargo audit` pass (if available)
- Build: `cargo build --release` success
- Date: pending
