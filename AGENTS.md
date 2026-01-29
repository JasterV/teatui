# TeaTui: Agent Context & Project Overview

## Project Summary

**TeaTui** (Tea = The Elm Architecture) is an experimental Rust framework for building Terminal User Interfaces (TUIs) on top of [Ratatui](https://github.com/ratatui/ratatui). It reproduces the **Elm Architecture** (Model-Update-View) to provide a pure functional development experience in Rust.

## Tech Stack & Environment

* **Language**: Rust (Edition **2024**).
* **Workspace Structure**: The project is organized as a workspace containing the core `teatui` library and an `examples` directory.
* **Core Dependencies**:
* `ratatui`: Used for terminal rendering.
* `crossterm`: Handles terminal backend and event polling.
* `thiserror`: Utilized for structured, descriptive error handling across the runtime.

## Design Philosophy

* **Functional Purity**: The goal is to build TUI applications in a pure-functional style, completely removing the usage of `&mut` and procedural state management.
* **Minimal Mutability**: Mutability is discouraged in user-provided code; state transitions occur by returning new instances of the Model.
* **Message Passing as Side Effects**: Communication between internal processes is strictly handled via `std::sync::mpsc` channels. Message passing is considered the "highest form of side effect" in the design.

## Core Architecture

The framework operates through four concurrent actors (threads) that manage the application lifecycle:

1. **Model**: A single type representing the entire application state.
2. **View Actor**: Renders the Model into the terminal. It takes a pure function `&Model -> Widget` and redraws only when the Model changes.
3. **Update Actor**: Manages state transitions. It takes the current Model and an incoming Message, returning an `Update` enum (either `Update::Exit` or `Update::Next(NewModel, Option<Effect>)`).
4. **Effects Actor**: Processes side effects (e.g., IO) returned by the Update actor. It can optionally return a Message to trigger further state updates.
5. **Event Actor**: Translates raw terminal events (via `crossterm`) into application-specific Messages.

## Key Types & Signatures

* **`Update<M, E>`**: Enum signaling whether to exit or continue with a new state and optional effect.
* **`update(model: M, msg: Msg) -> Update<M, E>`**: The core logic for state changes.
* **`view(model: &M) -> Widget`**: The rendering logic.
* **`effects(model: &M, effect: E) -> Option<Msg>`**: The side-effect handler.

## Development Guide

When extending the framework or building apps with it, ensure that `update` and `view` functions remain pure. Any operation involving the "outside world" (files, network, etc.) must be modeled as an `Effect` and handled within the `effects` actor.
