use crate::AnyModule;
use crate::sequential::Sequential;
use core::{fmt, num};
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::prelude::*;
use std::fmt::Debug;
use std::fs;
use std::io::Result;

// Graph vertices should be only Perceptron instances, while edges are f32.

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
        write!(f, "{}", self.id)
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
            indices: Default::default(),
        }
    }

    pub fn to_dot(&self) -> String {
        format!("{}", Dot::new(&self.graph))
    }

    pub fn save_dot(&self, path: &str) -> Result<()> {
        let dot = self.to_dot();
        fs::write(path, dot)
    }

    pub fn add_node(&mut self, p: Perceptron) {
        let idx = self.graph.add_node(p);
        self.indices.push(idx);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, w: f32) {
        let from_idx = self.indices[from];
        let to_idx = self.indices[to];
        self.graph.add_edge(from_idx, to_idx, w);
    }

    pub fn sequential_to_graph(&mut self, model: &Sequential, input: &[f32]) {
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

    pub fn plot(&self) {
        println!("{:?}", self.graph);
    }
}
