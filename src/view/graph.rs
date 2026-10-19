use crate::AnyModule;
use crate::sequential::Sequential;
use core::fmt;
use petgraph::prelude::*;
use std::fmt::Debug;
mod dot;

// TODO:
// Split this module into dot operations and graph extract.

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
    model: Sequential,
    input_data: Vec<f32>,
    indices: Vec<NodeIndex>,
}

impl NetGraph {
    pub fn new(model: Sequential, input_data: &[f32]) -> Self {
        Self {
            graph: DiGraph::new(),
            model: model.clone(),
            input_data: input_data.to_vec(),
            indices: Vec::new(),
        }
    }

    // TODO:
    // This fn should return a error instead of Panic.
    fn check_input_size(model: &Sequential, input: &[f32]) {
        assert_eq!(model.modules[0].as_linear().in_sz, input.len());
    }

    fn create_node(g: &mut DiGraph<Perceptron, f32>, id: usize) -> NodeIndex {
        // TODO:
        // Check why this line bellow is being called every time
        // assert!(id >= 1, "Node's ID can't be negative!");
        let p = Perceptron::new(id);
        g.add_node(p)
    }

    fn create_edge(
        g: &mut DiGraph<Perceptron, f32>,
        from: NodeIndex,
        to: NodeIndex,
        weight: f32,
    ) -> EdgeIndex {
        assert!(from < to, "This should be a digraph");
        g.add_edge(from, to, weight)
    }

    // TODO:
    // Refactor this code to use tensor + iterators
    fn extract_graph(&mut self, mut number_neurons: usize, mut previous_layer_start: usize) {
        for module in &self.model.modules {
            match module {
                AnyModule::Linear(linear) => {
                    let current_layer_start: usize = number_neurons;

                    for o in &linear.weights {
                        Self::create_node(&mut self.graph, number_neurons);

                        for (w, &weight) in o.iter().enumerate() {
                            let from_idx = NodeIndex::new(previous_layer_start + w);
                            let to_idx = NodeIndex::new(number_neurons);
                            Self::create_edge(&mut self.graph, from_idx, to_idx, weight);
                        }
                        number_neurons += 1;
                    }
                    previous_layer_start = current_layer_start;
                }
                AnyModule::Activation(_) => {}
            }
        }
    }

    pub fn network_to_graph(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Self::check_input_size(&self.model, &self.input_data);

        // FIX:
        // Move this to .yaml comp flags
        let source = "src/view/artifacts/graph.dot";
        let destiny = "src/view/artifacts/network.png";

        // TODO:
        // Check what's wrong here:
        // Creates the first layer
        // (0..self.input_data.len())
        //     .map(|i| Self::create_node(&mut self.graph, i))
        //     .collect();

        for i in 0..self.input_data.len() {
            Self::create_node(&mut self.graph, i);
        }

        let number_neurons = self.input_data.len();
        let previous_layer_start = 0;

        Self::extract_graph(self, number_neurons, previous_layer_start);
        Self::save_dot(self, source)?;
        Self::compile_dot(self, source, destiny)?;
        Ok(())
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

    pub fn plot(&self) {
        println!("{:?}", self.graph);
    }
}
