use super::NetGraph;
use petgraph::dot::{Config as DotOptions, Dot};
use serde::Deserialize;
use std::{error::Error, fs, process::Command};

#[derive(Deserialize)]
struct Config {
    dot: DotConfig,
}

#[derive(Deserialize)]
struct DotConfig {
    flags: Vec<String>,
}

impl NetGraph {
    // This function should extract dot compile params from a .yaml file

    fn graph_to_dot(&self) -> String {
        let dot = Dot::with_attr_getters(
            &self.graph,
            &[DotOptions::EdgeNoLabel],
            &|_, edge| {
                // TODO:
                // Refactor this to a external function
                let weight = *edge.weight();
                let intensity = weight.abs().clamp(0.0, 1.0);
                let gray = ((1.0 - intensity) * 255.0) as u8;
                format!(r##"color="#{0:02x}{0:02x}{0:02x}""##, gray)
            },
            &|_, _| String::new(),
        );
        format!("{}", dot)
    }

    pub fn save_dot(&self, path: &str) -> Result<(), std::io::Error> {
        let dot = self.graph_to_dot();
        fs::write(path, dot)?;
        Ok(())
    }

    fn extract_comp_flags() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let flags = fs::read_to_string("flags.yaml")?;
        let cfg: Config = serde_yaml_ng::from_str(&flags)?;
        Ok(cfg.dot.flags)
    }

    pub fn compile_dot(&self, source: &str, destiny: &str) -> Result<(), Box<dyn Error>> {
        let flags = Self::extract_comp_flags()?;

        let status = Command::new("dot")
            .arg(source)
            .args(&flags)
            .arg("-o")
            .arg(destiny)
            .status()?;

        if !status.success() {
            return Err("Graphviz failed to compile your graph".into());
        }

        Ok(())
    }
}
