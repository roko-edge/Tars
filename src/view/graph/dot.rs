use super::NetGraph;
use petgraph::dot::{Config as DotOptions, Dot};
use std::{error::Error, fs, process::Command};
use yaml_rust2::YamlLoader;

impl NetGraph {
    // This function should extract dot compile params from a .yaml file
    fn graph_to_dot(&self) -> String {
        let dot = Dot::with_attr_getters(
            &self.graph,
            &[DotOptions::EdgeNoLabel][..],
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

    fn save_dot(&self, path: &str) -> Result<(), std::io::Error> {
        let dot = self.graph_to_dot();
        fs::write(path, dot)?;
        Ok(())
    }

    fn extract_comp_flags() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("src/view/graph/flags.yaml")?;
        let docs = YamlLoader::load_from_str(&content)?;
        let config = &docs[0];

        let flags: Vec<String> = config["dot"]["flags"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();

        Ok(flags)
    }

    fn compile_dot(&self, source: &str, destiny: &str) -> Result<(), Box<dyn Error>> {
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

    pub fn export_topology(
        &self,
        source: &str,
        destiny: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::save_dot(self, source)?;
        Self::compile_dot(self, source, destiny)?;
        Ok(())
    }
}
