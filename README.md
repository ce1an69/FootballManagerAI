# FootballManagerAI

A terminal-based football management game written in Rust.

## Features

- Text-based terminal UI with ratatui
- Team and player management
- Match simulation
- League management
- Save/Load system
- **LLM Integration** - Generate game data using Large Language Models

## LLM Integration

Football Manager AI supports integration with OpenAI-compatible LLM APIs for generating more realistic and varied game data.

### Quick Setup

1. Set your API key via environment variable:
   ```bash
   export LLM_API_KEY="sk-your-api-key-here"
   ```

2. Or create `config.json`:
   ```bash
   cp config.json.example config.json
   # Edit config.json with your API details
   ```

3. Run the game - it will automatically use LLM if configured!

### Supported Providers

- DeepSeek (recommended)
- 通义千问 (Alibaba Qwen)
- Azure OpenAI
- OpenAI
- Any OpenAI-compatible API

### Documentation

See [docs/llm-integration.md](docs/llm-integration.md) for detailed configuration and usage instructions.

## Installation

```bash
cargo build --release
./target/release/football-manager-ai
```

## Development

See [docs/plans/](docs/plans/) for implementation plans and design documents.
