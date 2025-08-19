use anyhow::Result;
use serde_json::Value;

pub struct McpHandler;

impl McpHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle_tool_call(&self, name: &str, input: &Value) -> Result<String> {
        match name {
            "calculator" => self.calculator(input).await,
            "weather" => self.weather(input).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }

    async fn calculator(&self, input: &Value) -> Result<String> {
        let expression = input["expression"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing expression"))?;

        // Simple calculator implementation
        match self.evaluate_expression(expression) {
            Ok(result) => Ok(format!("Result: {}", result)),
            Err(e) => Ok(format!("Error: {}", e)),
        }
    }

    async fn weather(&self, input: &Value) -> Result<String> {
        let location = input["location"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing location"))?;

        // Mock weather data
        let weather_data = vec![
            ("temperature", "22°C"),
            ("condition", "Partly cloudy"),
            ("humidity", "65%"),
            ("wind", "10 km/h NE"),
        ];

        let mut result = format!("Weather for {}:\n", location);
        for (key, value) in weather_data {
            result.push_str(&format!("  {}: {}\n", key, value));
        }

        Ok(result)
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64> {
        // Simple expression evaluator - in a real implementation, use a proper parser
        let cleaned = expr.replace(" ", "");
        
        if cleaned.contains('+') {
            let parts: Vec<&str> = cleaned.split('+').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse()?;
                let b: f64 = parts[1].parse()?;
                return Ok(a + b);
            }
        }
        
        if cleaned.contains('-') {
            let parts: Vec<&str> = cleaned.split('-').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse()?;
                let b: f64 = parts[1].parse()?;
                return Ok(a - b);
            }
        }
        
        if cleaned.contains('*') {
            let parts: Vec<&str> = cleaned.split('*').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse()?;
                let b: f64 = parts[1].parse()?;
                return Ok(a * b);
            }
        }
        
        if cleaned.contains('/') {
            let parts: Vec<&str> = cleaned.split('/').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse()?;
                let b: f64 = parts[1].parse()?;
                return Ok(a / b);
            }
        }
        
        // Try parsing as a single number
        cleaned.parse::<f64>().map_err(|e| anyhow::anyhow!("Parse error: {}", e))
    }
}

