use anyhow::Result;
use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub struct MarkdownRenderer {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn render(&self, markdown: &str) -> Result<String> {
        let mut output = String::new();
        let parser = Parser::new(markdown);
        let mut in_code_block = false;
        let mut code_lang = String::new();
        let mut code_content = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    code_lang = lang.to_string();
                    code_content.clear();
                }
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                    in_code_block = false;
                    let highlighted = self.highlight_code(&code_content, &code_lang)?;
                    output.push_str(&highlighted);
                    output.push('\n');
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_content.push_str(&text);
                    } else {
                        output.push_str(&text);
                    }
                }
                Event::Start(Tag::Heading(level, _, _)) => {
                    output.push_str(&format!("\n{} ", "#".repeat(level as usize)));
                }
                Event::End(Tag::Heading(_, _, _)) => {
                    output.push('\n');
                }
                Event::Start(Tag::Emphasis) => output.push_str("*"),
                Event::End(Tag::Emphasis) => output.push_str("*"),
                Event::Start(Tag::Strong) => output.push_str("**"),
                Event::End(Tag::Strong) => output.push_str("**"),
                Event::Code(code) => {
                    output.push_str(&format!("`{}`", code));
                }
                Event::SoftBreak | Event::HardBreak => output.push('\n'),
                _ => {}
            }
        }

        Ok(output)
    }

    fn highlight_code(&self, code: &str, lang: &str) -> Result<String> {
        if lang.is_empty() {
            return Ok(code.to_string());
        }

        let syntax = self.syntax_set
            .find_syntax_by_token(lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut output = String::new();

        for line in LinesWithEndings::from(code) {
            let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set)?;
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            output.push_str(&escaped);
        }

        Ok(output)
    }
}

