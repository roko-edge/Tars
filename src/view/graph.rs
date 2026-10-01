use crate::AnyModule;
use crate::sequential::Sequential;
use core::fmt;
use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::prelude::*;
use std::fmt::Debug;
use std::fs;
use std::io::Result;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct Perceptron {
    id: usize,
}

impl Perceptron {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl fmt::Display for Perceptron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/*
 * Note: All nodes are added to the graph in increasing order.
 * Ex: [h0, h1, h2, h3, ... ]
 *      0   1   2   3   ...
 * This is important becasue it allows us to find a node's index in O(1) time.
 * Read NetGraph's "add" method to verify
 * */

#[derive(Clone, Debug)]
pub struct NetGraph {
    graph: DiGraph<Perceptron, f32>,
    indices: Vec<NodeIndex>,
}

impl NetGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            indices: Vec::new(),
        }
    }

    pub fn network_to_graph(&mut self, model: &Sequential, input: &[f32]) {
        for i in 0..input.len() {
            self.add_node(Perceptron::new(i));
        }

        let mut number_neurons = input.len();
        let mut previous_layer_start = 0;

        for module in &model.modules {
            match module {
                AnyModule::Linear(linear) => {
                    let current_layer_start = number_neurons;

                    for o in &linear.weights {
                        let p = Perceptron::new(number_neurons);
                        self.add_node(p);

                        // FIX: w shouldn't be nammed like that
                        for (w, &weight) in o.iter().enumerate() {
                            self.add_edge(previous_layer_start + w, number_neurons, weight);
                        }
                        number_neurons += 1;
                    }
                    previous_layer_start = current_layer_start;
                }
                AnyModule::Activation(_) => {}
            }
        }
    }

    pub fn graph_to_dot(&self) -> String {
        let dot = Dot::with_attr_getters(
            &self.graph,
            &[Config::EdgeNoLabel],
            &|_, edge| {
                let weight = *edge.weight();

                let intensity = weight.abs().clamp(0.0, 1.0);
                let gray = ((1.0 - intensity) * 255.0) as u8;

                format!(r##"color="#{0:02x}{0:02x}{0:02x}""##, gray)
            },
            &|_, _| String::new(),
        );

        format!("{}", dot)
    }

    pub fn save_dot(&self, path: &str) -> Result<()> {
        let dot = self.graph_to_dot();
        fs::write(path, dot)
    }

    pub fn export_dot(&self, source_path: &str, destiny_path: &str) {
        let out = Command::new("dot")
            .arg(source_path)
            .arg("-Gbgcolor=#000000")
            .arg("-Grankdir=LR")
            .arg("-Gsplines=line")
            .arg("-Gnodesep=0.8")
            .arg("-Granksep=1.0 equally")
            .arg("-Nshape=circle")
            .arg("-Nfixedsize=true")
            .arg("-Nwidth=0.6")
            .arg("-Nheight=0.6")
            .arg("-Nfontcolor=white")
            .arg("-Ncolor=#FFFFFF")
            .arg("-Ecolor=#FFFFFF")
            .arg("-Efontcolor=white")
            .arg("-Earrowhead=none")
            .arg("-Tpng")
            .arg("-o")
            .arg(destiny_path)
            .status()
            .expect(
                "Couldn't execute the 'dot' command. Check if Graphviz is installed on your PATH",
            );

        if !out.success() {
            eprintln!("Couldn't convert from .dot to .svg",);
            return;
        }

        println!(
            "Successfully converted from .dot to .svg - Check {}",
            destiny_path
        );
    }

    pub fn add_node(&mut self, p: Perceptron) {
        let idx = self.graph.add_node(p);
        self.indices.push(idx);
    }

    // use Dot::with_attr_getters to set edge color

    pub fn add_edge(&mut self, from: usize, to: usize, w: f32) {
        let from_idx = self.indices[from];
        let to_idx = self.indices[to];
        self.graph.add_edge(from_idx, to_idx, w);
    }

    pub fn plot(&self) {
        println!("{:?}", self.graph);
    }
}
