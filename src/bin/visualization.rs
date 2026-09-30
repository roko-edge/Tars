use std::io::Result;
use std::process::Command;
use tars::Sequential;
use tars::view::plot::*;

/*
Proposed design:
 *Input       Hidden        Output x0          h1_0          y0
  x1          h1_1          y1
  x2          h1_2
  x3          h1_3
 */

fn main() -> Result<()> {
    let model = Sequential::new(3)
        .linear(4)
        .relu()
        .linear(3)
        .sigmoid()
        .linear(3)
        .sigmoid()
        .linear(1);

    let input = vec![1.0, 2.0, 3.0];

    let mut net = NetGraph::new();
    net.sequential_to_graph(&model, &input);

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
