# LLM Integration Guide

Football Manager AI now supports integration with Large Language Models (LLMs) for generating game data including players, teams, leagues, and more.

## Overview

The LLM integration allows the game to generate more realistic and varied game data by using OpenAI-compatible APIs. When enabled, the game will use LLM to generate:
- Player names, nationalities, and attributes
- Team names, abbreviations, and stadium information
- League structures
- Match commentary (future feature)
- News events (future feature)

## Configuration

### Method 1: Configuration File

1. Copy `config.json.example` to `config.json`:
   ```bash
   cp config.json.example config.json
   ```

2. Edit `config.json` with your API details:
   ```json
   {
     "url": "https://api.deepseek.com/v1",
     "apiKey": "sk-your-api-key-here",
     "modelName": "deepseek-chat"
   }
   ```

### Method 2: Environment Variable (Recommended)

Set the `LLM_API_KEY` environment variable (takes precedence over config file):

```bash
export LLM_API_KEY="sk-your-api-key-here"
```

To make it persistent, add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):
```bash
echo 'export LLM_API_KEY="sk-your-api-key-here"' >> ~/.bashrc
source ~/.bashrc
```

## Supported APIs

The integration supports any OpenAI-compatible API provider:

### DeepSeek (Recommended)
- URL: `https://api.deepseek.com/v1`
- Models: `deepseek-chat`, `deepseek-coder`
- Website: https://www.deepseek.com/

### 通义千问 (Alibaba Qwen)
- URL: `https://dashscope.aliyuncs.com/compatible-mode/v1`
- Models: `qwen-turbo`, `qwen-plus`, `qwen-max`
- Website: https://tongyi.aliyun.com/

### Azure OpenAI
- URL: `https://your-resource.openai.azure.com/openai/deployments/your-deployment`
- Models: Standard OpenAI model names
- Website: https://azure.microsoft.com/en-us/products/ai-services/openai-service

### OpenAI (Official)
- URL: `https://api.openai.com/v1`
- Models: `gpt-3.5-turbo`, `gpt-4`, `gpt-4-turbo-preview`
- Website: https://platform.openai.com/

### Other Third-Party Services
Any service providing an OpenAI-compatible API should work. Just update the `url` and `modelName` fields in your configuration.

## How It Works

### Game Initialization Flow

1. **Configuration Check**: When starting a new game, the system checks for LLM configuration
2. **Validation**: If configured, validates the API key and endpoint
3. **Generation**: Uses LLM to generate game data (league, teams, players)
4. **Fallback**: If LLM fails or is not configured, falls back to random generation

### Data Generation Process

```
LLM API Call → JSON Response → Parsed Data → Game State
     ↓
  Stream chunks (for real-time feedback in future)
```

### Example Generated Data

**Player:**
```json
{
  "name": "Marcus Rashford",
  "nationality": "England",
  "attributes": {
    "finishing": 165,
    "passing": 142,
    "pace": 171,
    "stamina": 150,
    "strength": 145
  }
}
```

**Team:**
```json
{
  "name": "Manchester United",
  "abbreviation": "MUN",
  "founded_year": 1878,
  "stadium": {
    "name": "Old Trafford",
    "capacity": 74310
  }
}
```

## Fallback Mechanism

If the LLM API is unavailable or returns an error, the game automatically falls back to the original random generation system. This ensures the game remains playable even without LLM access.

Common fallback scenarios:
- Invalid or missing API key
- Network connectivity issues
- API rate limiting
- Invalid JSON response from LLM
- Configuration errors

## Customization

### Adjusting Skill Levels

The default skill level for generated players is 100 (out of 200). This can be modified in the code:

```rust
// In src/ai/llm/generators/player_gen.rs
let player = self.player_gen.generate(position, 100, 25).await?;
//                                              ^^^  ^^^
//                                            skill  age
```

### Changing Prompts

Prompt templates are defined in `src/ai/llm/prompts.rs`. You can customize these to change the style of generated data:

```rust
pub fn generate_player(position: &str, skill_level: u16, age: u8) -> String {
    format!(
        "You are a football manager game data generator...
        // Customize your prompt here
        "
    )
}
```

### Adding New Attributes

To add new player attributes or team properties:

1. Update the struct in `src/ai/llm/generators/player_gen.rs` or `team_gen.rs`
2. Update the prompt template to request the new data
3. Update the parsing logic to extract the new field

## Troubleshooting

### "LLM configuration invalid" message

**Solution**: Check that:
- Your `config.json` exists and is valid JSON
- Your API key is set and longer than 10 characters
- The URL is correct and accessible

### API timeout errors

**Solution**: The default timeout is 30 seconds. To increase it, modify `src/ai/llm/client.rs`:

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(60))  // Increase to 60 seconds
    .build()
```

### Invalid JSON from LLM

**Solution**: The system includes fallback logic to extract JSON from markdown code blocks. If issues persist, try:
- Using a more capable model (e.g., `gpt-4` instead of `gpt-3.5-turbo`)
- Adjusting temperature settings (lower = more deterministic)
- Simplifying the prompt template

### Rate limiting

**Solution**: API providers often have rate limits. Consider:
- Adding delays between API calls
- Using a paid API tier
- Implementing request batching (future feature)

## Performance Considerations

### Generation Time

LLM generation is significantly slower than random generation:
- Random generation: ~1 second for full game setup
- LLM generation: ~30-60 seconds for 20 teams × 25 players = 500 API calls

### Optimization Tips

1. **Use caching**: Future versions will cache generated data
2. **Batch requests**: Generate multiple entities per API call (planned)
3. **Parallel requests**: Use async concurrency for multiple teams (planned)
4. **Selective LLM**: Use LLM for key data only, random for the rest

## Security

### API Key Protection

- **Never commit** `config.json` to version control (it's in `.gitignore`)
- Use environment variables in production
- Rotate API keys regularly
- Use separate keys for development and production

### Data Privacy

- LLM API calls send game generation prompts to external servers
- No personal user data is transmitted
- Generated game data is stored locally only

## Future Enhancements

Planned features for future releases:

1. **Streaming UI**: Real-time display of generation progress
2. **Match Commentary**: Live commentary using LLM
3. **News System**: Dynamic news generation
4. **Batch Generation**: Reduce API calls with batch requests
5. **Caching**: Store generated entities for reuse
6. **Custom Models**: Support for fine-tuned models
7. **Progressive Loading**: Generate data on-demand during gameplay

## Contributing

To extend the LLM integration:

1. **New Generators**: Add new generator types in `src/ai/llm/generators/`
2. **Prompt Templates**: Add templates in `src/ai/llm/prompts.rs`
3. **API Features**: Extend `src/ai/llm/client.rs` for new API capabilities
4. **Tests**: Add unit tests in `*_tests.rs` files

See the implementation plan for more details: `docs/plans/2026-02-01-llm-integration.md`

## Resources

- **Implementation Plan**: `docs/plans/2026-02-01-llm-integration.md`
- **Source Code**: `src/ai/llm/`
- **Configuration Example**: `config.json.example`
- **OpenAI API Docs**: https://platform.openai.com/docs/api-reference
