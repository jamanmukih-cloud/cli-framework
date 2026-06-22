# CLI Framework 🖥️

Derive-based CLI framework with subcommands and completion.

## Features

- **Derive Macros**: Minimal boilerplate
- **Subcommands**: Nested command trees
- **Validation**: Arg validation + env vars
- **Completion**: Bash, Zsh, Fish

## Quick Start

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve { port: u16 },
    Build { target: String },
}
```

## License

MIT