use std::io::Result;
use tars::sequential::*;
use tars::view::graph::*;

// TODO:
// Create a module to dot to png handler;
// Move dot compilation flags to a separate structure;
// Should the user be able to work with raw .dot for graph visualization?
// Hyprparams and Visualization config should be passed to a .yaml file - Ex: output format (svg/png, etc), source and
// destiny path.
// There should be a csv to raw data extractor or is it overbloated?

fn main() -> Result<()> {
    let model = Sequential::new(4)
        .linear(8)
        .relu()
        .linear(10)
        .relu()
        .linear(8)
        .relu()
        .linear(2)
        .relu();

    let input = vec![1.0, 0.676767, 1.0, 2.0];

    let mut net = NetGraph::new(model, &input);

    // generates a raw .dot file with network topology
    net.network_to_graph()?;
    // export .dot to .svg
    net.plot();

    Ok(())
}
