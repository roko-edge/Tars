use petgraph::graph::DiGraph;
use std::io::Result;
use std::process::Command;
use tars::view::plot::*;

/*
Proposed design:
 *Input       Hidden        Output
  x0          h1_0          y0
  x1          h1_1          y1
  x2          h1_2
  x3          h1_3
 */

fn main() -> Result<()> {
    let mut net = NetGraph::new();

    let h0 = Perceptron::new(0, 1.0, 0.0);
    let h1 = Perceptron::new(1, 2.0, 0.0);
    let h2 = Perceptron::new(2, 3.0, 0.0);

    let h3 = Perceptron::new(3, 1.0, 0.0);
    let h4 = Perceptron::new(4, 2.0, 0.0);

    let h5 = Perceptron::new(5, 3.0, 0.0);

    net.add_node(h0);
    net.add_node(h1);
    net.add_node(h2);

    net.add_node(h3);
    net.add_node(h4);

    net.add_node(h5);

    net.add_edge(0, 3, 1.0);
    net.add_edge(0, 4, 1.0);

    net.add_edge(1, 3, 1.0);
    net.add_edge(1, 4, 1.0);

    net.add_edge(2, 3, 1.0);
    net.add_edge(2, 4, 1.0);

    net.add_edge(3, 5, 1.0);
    net.add_edge(4, 5, 1.0);

    let _dot = net.to_dot();

    let src_path: &str = "src/view/graph/network.dot";
    let output_path: &str = "src/view/graph/network.png";
    let input_format: &str = ".dot";
    let output_format: &str = ".png";

    // let _dot = Dot::with_config(&net, &[config::RankDir(RankDir::LR)]);

    net.save_dot(src_path)?;

    // TODO: Create images dir and add it to .gitignore
    // TODO: Create a module to dot to png handler
    // TODO: Move dot compilation flags to a separate structure

    let out = Command::new("dot")
        .arg(src_path)
        .arg("-Grankdir=LR")
        .arg("-Gsplines=true")
        .arg("-Tpng")
        .arg("-o")
        .arg(output_path)
        .status()
        .expect("Coulnd't execut the 'dot' command. Check if Graphviz is installed on your PATH");

    if !out.success() {
        eprintln!(
            "Couldn't convert from {} to {}",
            input_format, output_format
        );
    }

    println!(
        "Successfully converted from {} to {} - Check {}",
        input_format, output_format, output_path
    );

    net.plot();

    Ok(())
}
