# Paperplane 

A fast, local-first, open-source API workspace for modern developers.

## Why Paperplane?

Most API tools are bloated, slow, and cloud-locked.

Paperplane is:
- Fast (Rust + Tauri)
- Local-first (no forced cloud)
- Git-native (collections as files)
- Multi-protocol ready (REST → gRPC → WebSockets)

## MVP Features

- REST API client (GET, POST, headers, body)
- Lightweight desktop app
- Collections stored as YAML/JSON
- Environment variables (`{{base_url}}`)
- Clean response viewer
- Curl import
- CLI runner (planned)

## Tech Stack

- **Frontend**: React + TypeScript
- **Desktop**: Tauri
- **Core Engine**: Rust
- **HTTP**: reqwest
- **Storage**: local filesystem (Git-friendly)

## Structure
```
paperplane/
├── apps/
│ ├── desktop/ # Tauri UI (React + TS)
│ ├── cli/ # CLI runner (future)
│ └── cloud/ # Optional backend (future)
│
├── rust-core/ # Core execution engine (Rust)
│
├── packages/
│ ├── ui/ # Shared UI components
│ ├── core/ # Request orchestration logic
│ ├── storage/ # File + Git handling
│ ├── env/ # Environment variables
│ └── testing/ # Assertions + scripting
│
├── .github/
│ ├── workflows/ # CI/CD pipelines
│ └── ISSUE_TEMPLATE/
│
├── docs/ # Documentation
├── scripts/ # Dev scripts
└── README.md

```

## Vision

> A developer-native API workspace that handles REST, gRPC, GraphQL, and real-time protocols—fast.

## Status

MVP in progress

## Contributing

Open an issue before major changes. Follow conventional commits.

## 📄 License

MIT
