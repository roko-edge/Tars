use std::io::Result;
use tars::sequential::*;
use tars::view::graph::*;

// TODO Create a module to dot to png handler
// TODO Move dot compilation flags to a separate structure

/*
Proposed design:
 *Input       Hidden        Output x0          h1_0          y0
  x1          h1_1          y1
  x2          h1_2
  x3          h1_3
 */

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

    let mut net = NetGraph::new();

    net.network_to_graph(&model, &input);

    let source = "src/view/artifacts/graph.dot";
    let destiny = "src/view/artifacts/network.png";

    net.graph_to_dot();
    net.save_dot(source)?;
    net.export_dot(source, destiny);

    net.plot();
    Ok(())
}
