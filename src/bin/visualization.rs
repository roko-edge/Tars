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

    let h0 = Perceptron::new(0);
    let h1 = Perceptron::new(1);
    let h2 = Perceptron::new(2);

    let h3 = Perceptron::new(3);
    let h4 = Perceptron::new(4);

    let h5 = Perceptron::new(5);

    net.add_node(h0);
    net.add_node(h1);
    net.add_node(h2);

    net.add_node(h3);
    net.add_node(h4);

    net.add_node(h5);

    net.add_edge(0, 3, 1.0);
    net.add_edge(0, 4, 1.2);

    net.add_edge(1, 3, 2.3);
    net.add_edge(1, 4, 9.0);

    net.add_edge(2, 3, 0.1);
    net.add_edge(2, 4, 2.5);

    net.add_edge(3, 5, 0.3);
    net.add_edge(4, 5, 0.4);

    let _dot = net.to_dot();

    net.save_dot("src/view/artifacts/graph.dot")?;

    // TODO Create images dir and add it to .gitignore
    // TODO Create a module to dot to png handler
    // TODO Move dot compilation flags to a separate structure

    let out = Command::new("dot")
        .arg("src/view/artifacts/graph.dot")
        .arg("-Grankdir=LR")
        .arg("-Gsplines=true")
        .arg("-Tpng")
        .arg("-o")
        .arg("src/view/artifacts/network.png")
        .status()
        .expect("Coulnd't execut the 'dot' command. Check if Graphviz is installed on your PATH");

    let input_format: &str = ".dot";
    let output_format: &str = ".png";

    if !out.success() {
        eprintln!(
            "Couldn't convert from {} to {}",
            input_format, output_format
        );
    }

    println!(
        "Successfully converted from {} to {} - Check src/bin/artifacts/network.png",
        input_format, output_format
    );

    net.plot();

    Ok(())
}
