use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

use crate::api::{ClaudeClient, Message, MessageContent, MessageRequest, ContentBlock, ResponseContent};
use crate::artifacts::{ArtifactManager, Artifact};
use crate::mcp::McpHandler;
use crate::markdown::MarkdownRenderer;

pub struct ChatApp {
    client: ClaudeClient,
    messages: Vec<Message>,
    input: String,
    artifacts: Vec<Artifact>,
    artifact_manager: ArtifactManager,
    mcp_handler: McpHandler,
    markdown_renderer: MarkdownRenderer,
    scroll_offset: usize,
}

impl ChatApp {
    pub fn new(client: ClaudeClient) -> Self {
        Self {
            client,
            messages: Vec::new(),
            input: String::new(),
            artifacts: Vec::new(),
            artifact_manager: ArtifactManager::new().expect("Failed to create artifact manager"),
            mcp_handler: McpHandler::new(),
            markdown_renderer: MarkdownRenderer::new(),
            scroll_offset: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            break;
                        }
                        KeyCode::Enter => {
                            if !self.input.trim().is_empty() {
                                let user_input = self.input.clone();
                                self.input.clear();
                                
                                // Add user message
                                self.messages.push(Message {
                                    role: "user".to_string(),
                                    content: MessageContent::Text(user_input.clone()),
                                });

                                // Send to Claude
                                if let Err(e) = self.send_message().await {
                                    self.messages.push(Message {
                                        role: "assistant".to_string(),
                                        content: MessageContent::Text(format!("Error: {}", e)),
                                    });
                                }
                            }
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Up => {
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                        KeyCode::Down => {
                            self.scroll_offset += 1;
                        }
                        KeyCode::Tab => {
                            if !self.artifacts.is_empty() {
                                let latest_artifact = &self.artifacts[self.artifacts.len() - 1];
                                let _ = self.artifact_manager.display_artifact(latest_artifact);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    async fn send_message(&mut self) -> Result<()> {
        let tools = ClaudeClient::get_tools();
        
        let request = MessageRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4000,
            messages: self.messages.clone(),
            tools: Some(tools),
        };

        let response = self.client.send_message(request).await?;
        
        let mut response_blocks = Vec::new();
        let mut full_text = String::new();

        for content in response.content {
            match content {
                ResponseContent::Text { text } => {
                    full_text.push_str(&text);
                    response_blocks.push(ContentBlock::Text { text });
                }
                ResponseContent::ToolUse { id, name, input } => {
                    // Handle tool call
                    let tool_result = self.mcp_handler.handle_tool_call(&name, &input).await?;
                    
                    response_blocks.push(ContentBlock::ToolUse {
                        id: id.clone(),
                        name,
                        input,
                    });
                    
                    response_blocks.push(ContentBlock::ToolResult {
                        tool_use_id: id,
                        content: tool_result,
                    });
                }
            }
        }

        // Extract artifacts from the full text
        let new_artifacts = self.artifact_manager.extract_artifacts(&full_text);
        self.artifacts.extend(new_artifacts);

        // Add assistant response
        if response_blocks.len() == 1 {
            if let ContentBlock::Text { text } = &response_blocks[0] {
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: MessageContent::Text(text.clone()),
                });
            } else {
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: MessageContent::Blocks(response_blocks),
                });
            }
        } else {
            self.messages.push(Message {
                role: "assistant".to_string(),
                content: MessageContent::Blocks(response_blocks),
            });
        }

        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(3),
                Constraint::Length(2),
            ])
            .split(f.size());

        // Chat history
        let mut chat_items = Vec::new();
        for message in &self.messages {
            let role_style = if message.role == "user" {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            };

            chat_items.push(ListItem::new(Line::from(vec![
                Span::styled(format!("{}: ", message.role), role_style)
            ])));

            match &message.content {
                MessageContent::Text(text) => {
                    let rendered = self.markdown_renderer.render(text).unwrap_or_else(|_| text.clone());
                    let lines: Vec<String> = rendered.lines().map(|s| s.to_string()).collect();
                    for line in lines {
                        chat_items.push(ListItem::new(Line::from(line)));
                    }
                }
                MessageContent::Blocks(blocks) => {
                    for block in blocks {
                        match block {
                            ContentBlock::Text { text } => {
                                let rendered = self.markdown_renderer.render(text).unwrap_or_else(|_| text.clone());
                                let lines: Vec<String> = rendered.lines().map(|s| s.to_string()).collect();
                                for line in lines {
                                    chat_items.push(ListItem::new(Line::from(line)));
                                }
                            }
                            ContentBlock::ToolUse { name, input, .. } => {
                                chat_items.push(ListItem::new(Line::from(
                                    Span::styled(
                                        format!("🔧 Tool: {} with input: {}", name, input),
                                        Style::default().fg(Color::Yellow)
                                    )
                                )));
                            }
                            ContentBlock::ToolResult { content, .. } => {
                                let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                                for line in lines {
                                    chat_items.push(ListItem::new(Line::from(
                                        Span::styled(line, Style::default().fg(Color::Magenta))
                                    )));
                                }
                            }
                        }
                    }
                }
            }

            chat_items.push(ListItem::new(Line::from(""))); // Empty line separator
        }

        let chat_list = List::new(chat_items)
            .block(Block::default().borders(Borders::ALL).title("Chat with Claude"))
            .style(Style::default().fg(Color::White));

        f.render_widget(chat_list, chunks[0]);

        // Input box
        let input_paragraph = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input (Press Enter to send, Ctrl+Q to quit, Tab to view latest artifact)"));

        f.render_widget(input_paragraph, chunks[1]);

        // Status
        let status_text = if self.artifacts.is_empty() {
            "No artifacts generated yet".to_string()
        } else {
            format!("{} artifact(s) available - Press Tab to view latest", self.artifacts.len())
        };

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Status"));

        f.render_widget(status, chunks[2]);
    }
}


