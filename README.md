# Claude Chatbot

A feature-rich terminal-based chatbot client for Anthropic's Claude API, built with Rust. This chatbot provides an interactive terminal interface with advanced features including markdown rendering, artifact visualization, and MCP (Model Context Protocol) tool support.

## Features

### Core Functionality
- **Claude Sonnet 4 Integration**: Uses the latest `claude-sonnet-4-20250514` model
- **Interactive Terminal UI**: Built with `ratatui` for a modern terminal experience
- **Real-time Chat**: Seamless conversation flow with Claude
- **Error Handling**: Robust error handling and user feedback

### Rich Text Support
- **Markdown Rendering**: Full markdown support with proper formatting
- **Syntax Highlighting**: Code blocks with language-specific highlighting using `syntect`
- **Colored Output**: Differentiated styling for users, assistants, and system messages

### Artifact Display
- **HTML Artifacts**: Automatically opens HTML content in your default browser
- **React Components**: Wraps React/TypeScript components with necessary runtime
- **JavaScript/TypeScript**: Saves code artifacts to temporary files
- **Automatic Detection**: Extracts artifacts from Claude's responses automatically

### MCP Tool Support
- **Calculator**: Performs mathematical calculations (addition, subtraction, multiplication, division)
- **Weather**: Provides mock weather information for any location
- **Extensible Architecture**: Easy to add more tools

### User Interface
- **Scrollable Chat History**: Navigate through conversation history
- **Keyboard Shortcuts**: Intuitive controls for all operations
- **Status Bar**: Real-time feedback on artifacts and system status
- **Responsive Design**: Adapts to different terminal sizes

## Installation

### Prerequisites

- **Rust**: Version 1.70 or higher
- **Anthropic API Key**: Get one from [Anthropic Console](https://console.anthropic.com/)

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/claude-chatbot.git
cd claude-chatbot

# Build the project
cargo build --release

# The binary will be available at ./target/release/claude-chatbot
```

### Dependencies

This project uses the following key dependencies:

- `tokio` - Async runtime
- `reqwest` - HTTP client for API calls
- `serde` - JSON serialization/deserialization
- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal manipulation
- `syntect` - Syntax highlighting
- `pulldown-cmark` - Markdown parsing
- `anyhow` - Error handling

## Usage

### Basic Usage

```bash
# Using command line argument
./target/release/claude-chatbot --api-key YOUR_API_KEY

# Using environment variable (recommended)
export ANTHROPIC_API_KEY=your_api_key_here
./target/release/claude-chatbot
```

### Keyboard Controls

| Key | Action |
|-----|---------|
| `Enter` | Send message |
| `Ctrl+Q` | Quit application |
| `Tab` | View latest artifact in browser |
| `↑/↓` | Scroll through chat history |
| `Backspace` | Delete character |

### Using Tools

The chatbot comes with built-in tools that Claude can use:

**Calculator Example:**
```
You: What's 15 * 23?
Claude: I'll calculate that for you.
Tool: calculator with input: {"expression":"15*23"}
Result: 345
```

**Weather Example:**
```
You: What's the weather in Tokyo?
Claude: Let me check the weather for Tokyo.
Tool: weather with input: {"location":"Tokyo"}
Weather for Tokyo:
  temperature: 22°C
  condition: Partly cloudy
  humidity: 65%
  wind: 10 km/h NE
```

### Artifact Display

When Claude generates artifacts (HTML, React components, code), they are automatically:

1. **Detected and extracted** from the response
2. **Displayed in the status bar** with count
3. **Accessible via Tab key** to open in browser/editor

**Supported Artifact Types:**
- `text/html` - Opens in default browser
- `application/vnd.ant.react` - Wraps with React runtime and opens in browser
- `text/javascript` / `text/typescript` - Saves to temporary files

## Architecture

The project is organized into several modules:

```
src/
├── main.rs          # Entry point and CLI argument parsing
├── api.rs           # Claude API client and data structures
├── ui.rs            # Terminal user interface (ratatui)
├── artifacts.rs     # Artifact extraction and display
├── mcp.rs           # MCP tool implementations
└── markdown.rs      # Markdown rendering with syntax highlighting
```

### Key Components

#### API Client (`api.rs`)
- Handles HTTP requests to Claude API
- Manages message serialization/deserialization
- Supports tool calling and responses

#### UI Manager (`ui.rs`)
- Terminal interface built with `ratatui`
- Real-time rendering and event handling
- Keyboard input processing

#### Artifact Manager (`artifacts.rs`)
- Parses artifacts from Claude responses
- Handles different artifact types
- Manages temporary file creation and browser launching

#### MCP Handler (`mcp.rs`)
- Implements calculator and weather tools
- Extensible framework for adding new tools
- Async tool execution

#### Markdown Renderer (`markdown.rs`)
- Converts markdown to terminal-friendly format
- Syntax highlighting for code blocks
- Preserves formatting and structure

## Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `ANTHROPIC_API_KEY` | Your Anthropic API key | Yes |

### Customization

#### Adding New Tools

To add a new MCP tool, modify `src/mcp.rs`:

```rust
// Add to ClaudeClient::get_tools()
Tool {
    name: "my_tool".to_string(),
    description: "Description of my tool".to_string(),
    input_schema: serde_json::json!({
        "type": "object",
        "properties": {
            "param": {
                "type": "string",
                "description": "Parameter description"
            }
        },
        "required": ["param"]
    }),
}

// Add handler in McpHandler::handle_tool_call()
"my_tool" => self.my_tool(input).await,

// Implement the tool function
async fn my_tool(&self, input: &Value) -> Result<String> {
    let param = input["param"].as_str().unwrap_or("");
    // Tool logic here
    Ok(format!("Result: {}", param))
}
```

#### Changing Models

To use a different Claude model, modify the model string in `src/ui.rs`:

```rust
let request = MessageRequest {
    model: "claude-3-5-sonnet-20241022".to_string(), // Change this
    max_tokens: 4000,
    messages: self.messages.clone(),
    tools: Some(tools),
};
```

## Troubleshooting

### Common Issues

**API Key Issues:**
```bash
Error: API key required. Use --api-key or set ANTHROPIC_API_KEY
```
- Make sure your API key is correctly set
- Verify the key has proper permissions

**Network Issues:**
```bash
Error: API error: HTTP 401 Unauthorized
```
- Check your API key validity
- Ensure you have sufficient API credits

**Terminal Issues:**
- If UI appears broken, try resizing your terminal
- Ensure your terminal supports color output
- Use a modern terminal emulator

### Debug Mode

For debugging, you can build with debug symbols:

```bash
cargo build
RUST_LOG=debug ./target/debug/claude-chatbot
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Here are some areas where contributions would be particularly valuable:

- Additional MCP tools (file operations, web search, etc.)
- Enhanced markdown rendering
- Better artifact handling
- UI/UX improvements
- Performance optimizations
- Tests and documentation

### Development Setup

```bash
git clone https://github.com/yourusername/claude-chatbot.git
cd claude-chatbot
cargo build
cargo test
```

### Code Style

This project follows standard Rust conventions:
- Run `cargo fmt` before committing
- Run `cargo clippy` to check for common issues
- Add tests for new functionality

## Maintainer

**Wanjun Gu**  
Email: wanjun.gu@ucsf.edu

For questions, issues, or feature requests, please open an issue on GitHub or contact the maintainer directly.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Anthropic](https://www.anthropic.com/) for the Claude API
- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent terminal UI framework
- [syntect](https://github.com/trishume/syntect) for syntax highlighting
- The Rust community for amazing crates and tools

## Roadmap

- [ ] **Enhanced MCP Tools**: File operations, web search, code execution
- [ ] **Configuration File**: TOML-based configuration for settings
- [ ] **Chat History**: Persistent conversation history
- [ ] **Multiple Conversations**: Tab-based conversation management
- [ ] **Plugin System**: Dynamic tool loading
- [ ] **Web Interface**: Optional web-based UI
- [ ] **Docker Support**: Containerized deployment
- [ ] **CI/CD**: Automated testing and releases

---

**Made with Rust**


