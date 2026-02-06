# Whistler IDE

An incredibly lightweight code editor that fits my workflow.

## Overview

This editor is meant to be minimalist and follow a mix of Apple and Arc's (The Browser Company) design language.

## Architecture

Whistler follows The Elm Architecture, which is also used in iced. This means that every part of the application wil be structured around four fundamental concepts:

- **State**: The data model of the app
- **Messages**: Events that can occur
- **Update logic**: How messages can interact with state
- **View logic**: How state can create UI widgets

## Tech Stack

iced (~0.13+):
> iced is a cross-platform GUI framework that works with rust
>
> it has The Elm Architecture built in
>
> GPU-accelerated rendering for lower RAM usage and faster loading
>
> Reactive updates: basically, the UI automatically updates in real time whenever the underlying data would change

### For Syntax Highlighting

**Tree Sitter** + other language grammars
> Languages needed:
>> `tree-sitter-python`
>>
>> `tree-sitter-rust`
>>
>> `tree-sitter-javascript`
>>
>> `tree-sitter-css`
>>
>> `tree-sitter-html`

### LSP Integration

**tower-lsp** (the client)
> Async/await support
>
> LSP Servers to support:
>> `pyright` or `pylsp` (python)
>>
>> `rust-analyzer` (rust)
>>
>> `typescript-language-server` (TS/JS)

### File system

**notify**, for:
> Cross-platform compatibility
>
> Detecting file changes outside of editor

### Serialization

**serde + serde_json/toml**, for:
> config file handling
>
> theme definitions (using CSS themes)
>
> project settings
>
> LSP message serialization

### Async runtime

**tokio**, for:
> required for LSP communication
>
> file I/O
>
> AI API requests (Gemini)

### AI integration

**reqwest**, for:
> HTTP client for Gemini API
>
> Async request handling

**tiktoken-rs** (optional)
> Token counts for context management as to not exceed token limits

### Utilities

**ropey**, for
> Efficient text rope data structure
>
> Fast instertions, deletions, and line/column indexing/counting

**similar**, for:
> Text diffing for finding/replacing
>
> Git conflict visualization (maybe in the future)

**dirs**, for:
> Cross-platform directory paths
>
> Config folder location

**syntect**, which will be used as a fallback for syntax highlighting and theme color schemes (can export them to CSS)
