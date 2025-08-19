use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Artifact {
    pub id: String,
    pub title: String,
    pub content_type: String,
    pub content: String,
}

pub struct ArtifactManager {
    temp_dir: TempDir,
}

impl ArtifactManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            temp_dir: TempDir::new()?,
        })
    }

    pub fn extract_artifacts(&self, text: &str) -> Vec<Artifact> {
        let mut artifacts = Vec::new();
        let mut in_artifact = false;
        let mut current_artifact: Option<Artifact> = None;
        let mut content_lines = Vec::new();

        for line in text.lines() {
            if line.contains("<artifact") {
                in_artifact = true;
                let id = self.extract_attribute(line, "identifier").unwrap_or_else(|| Uuid::new_v4().to_string());
                let title = self.extract_attribute(line, "title").unwrap_or("Untitled".to_string());
                let content_type = self.extract_attribute(line, "type").unwrap_or("text/plain".to_string());
                
                current_artifact = Some(Artifact {
                    id,
                    title,
                    content_type,
                    content: String::new(),
                });
                content_lines.clear();
            } else if line.contains("</artifact>") {
                in_artifact = false;
                if let Some(mut artifact) = current_artifact.take() {
                    artifact.content = content_lines.join("\n");
                    artifacts.push(artifact);
                }
                content_lines.clear();
            } else if in_artifact {
                content_lines.push(line);
            }
        }

        artifacts
    }

    pub fn display_artifact(&self, artifact: &Artifact) -> Result<()> {
        match artifact.content_type.as_str() {
            "text/html" | "application/vnd.ant.react" => {
                let file_path = self.temp_dir.path().join(format!("{}.html", artifact.id));
                
                let html_content = if artifact.content_type == "application/vnd.ant.react" {
                    self.wrap_react_component(&artifact.content)
                } else {
                    artifact.content.clone()
                };
                
                fs::write(&file_path, html_content)?;
                webbrowser::open(file_path.to_str().unwrap())?;
            }
            "text/javascript" | "text/typescript" => {
                let extension = if artifact.content_type.contains("typescript") { "ts" } else { "js" };
                let file_path = self.temp_dir.path().join(format!("{}.{}", artifact.id, extension));
                fs::write(&file_path, &artifact.content)?;
                println!("Saved {} artifact to: {}", extension.to_uppercase(), file_path.display());
            }
            _ => {
                println!("Artifact '{}' ({})", artifact.title, artifact.content_type);
                println!("{}", artifact.content);
            }
        }
        Ok(())
    }

    fn wrap_react_component(&self, content: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>React Component</title>
    <script src="https://unpkg.com/react@18/umd/react.development.js"></script>
    <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"></script>
    <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body>
    <div id="root"></div>
    <script type="text/babel">
        {content}
        ReactDOM.render(React.createElement(App || (() => React.createElement('div', null, 'Component not found'))), document.getElementById('root'));
    </script>
</body>
</html>"#,
            content = content
        )
    }

    fn extract_attribute(&self, line: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start) = line.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = line[start..].find('"') {
                return Some(line[start..start + end].to_string());
            }
        }
        None
    }
}


