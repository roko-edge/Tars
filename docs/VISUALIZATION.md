# Network Visualization

This guide describes how to generate and inspect network diagrams with the current
TARS visualization API. It covers the standalone binary, graph construction, DOT
export, Graphviz rendering, and operational limitations. The public API overview is
maintained in [`API.md`](API.md).

---

## 1. Requirements

Run the visualization binary from the repository root. Its output paths and
Graphviz configuration path are relative to the process working directory.

| Requirement | Purpose |
|---|---|
| Rust toolchain supporting edition 2024 | Builds and runs the package |
| Graphviz with `dot` on `PATH` | Renders DOT files into images |
| Writable `src/view/artifacts/` directory | Receives the generated files |
| `src/view/graph/flags.yaml` | Supplies Graphviz rendering arguments |

The Rust development shell includes Graphviz and is available through
`nix develop .#rust`. See [`GETTING_STARTED.md`](GETTING_STARTED.md) for environment
setup. Graphviz is not required when only saving an already constructed graph as
DOT through `save_dot()`.

---

## 2. Generate a Network Diagram

Run `cargo run --bin visualization` from the repository root. The binary constructs
its own model with five inputs, linear layers of eight and five outputs, and ReLU
activations after both layers. It does not load a saved model or consume the model
trained by the training binary.

The binary calls `network_to_graph()`, which constructs the graph, saves its DOT
representation, and invokes Graphviz. A successful run produces:

| Artifact | Path | Use |
|---|---|---|
| DOT source | `src/view/artifacts/graph.dot` | Inspect or render the graph with Graphviz tools |
| PNG image | `src/view/artifacts/network.png` | View the rendered network diagram |

Existing files at these paths may be overwritten. Parent directories are not
created automatically. The binary does not open an image viewer and has no
command-line options for selecting a model, output path, or image format.

---

## 3. Interpret the Diagram

| Diagram element | Meaning | Not represented |
|---|---|---|
| Input node | One input feature | The feature's numeric value |
| Linear-layer node | One output neuron of a linear layer | Bias and evaluated output |
| Directed edge | A linear weight connecting successive layers | A numeric weight label |
| Edge grayscale | Absolute weight magnitude, clamped to the range from zero to one | Weight sign |

Zero-weight edges are white. Edges with an absolute weight of one or greater are
black. Intermediate magnitudes use intermediate shades of gray. The configured
background is black, so large-magnitude edges can be difficult to distinguish.

Activations do not produce additional nodes. Node display labels are empty. The
current rendering configuration places layers from left to right and suppresses
edge arrowheads, although the underlying graph is directed.

Input values are not evaluated during graph extraction. The input slice supplies
the input-node count, which must match the first linear layer's input width. For
the standalone binary's model, the graph contains 18 nodes and 80 edges.

---

## 4. Use the Visualization API

The public types are available under `tars::view::graph`. `NetGraph` holds the graph,
a cloned model, and a copy of the supplied input data. Construction alone does not
extract topology or write files.

| Operation | Behavior | Important constraint |
|---|---|---|
| `Perceptron::new(id: usize) -> Self` | Creates a node descriptor | The identifier is private and its display label is empty |
| `NetGraph::new(model: Sequential, input_data: &[f32]) -> Self` | Creates an empty graph with stored model and input data | Does not perform a forward pass |
| `network_to_graph(&mut self) -> Result<(), Box<dyn std::error::Error>>` | Extracts topology, saves DOT, and renders PNG | Output paths are fixed; Graphviz is required |
| `save_dot(&self, path: &str) -> Result<(), std::io::Error>` | Writes the current graph as DOT | Does not extract topology or render an image |
| `compile_dot(&self, source: &str, destiny: &str) -> Result<(), Box<dyn std::error::Error>>` | Renders the supplied DOT file with Graphviz | Does not serialize the in-memory graph |
| `add_node(&mut self, p: Perceptron)` | Adds a node for manual graph construction | Registers the node in the manual index list |
| `add_edge(&mut self, from: usize, to: usize, w: f32)` | Adds an edge between manually registered nodes | Endpoints are positions in the manual index list |
| `plot(&self)` | Prints the graph's debug representation | Does not render or open a diagram |

For model visualization, create a fresh `NetGraph` with a compatible input slice
and call `network_to_graph()` once. This operation combines extraction and artifact
generation; there is no public extraction-only method.

For manual graph construction, add nodes before connecting them with `add_edge()`.
Save the result with `save_dot()`, then use `compile_dot()` if an image is required.
This route permits caller-selected source and destination paths without extracting
model topology.

Automatically extracted nodes are not registered in the manual index list. Do not
use `add_edge()` positions as indices into an automatically extracted network.
Calling `save_dot()` immediately after `NetGraph::new()` exports an empty graph.

---

## 5. DOT Export and Rendering Configuration

The private `graph::dot` module implements serialization and rendering as methods
on `NetGraph`. Users call `save_dot()` and `compile_dot()` through `NetGraph`; the
module itself and its `graph_to_dot()` serializer are not public interfaces.

`compile_dot()` reads the first YAML document in `src/view/graph/flags.yaml` and
passes the strings under `dot.flags` to the external `dot` process. The configuration
is read at runtime for each invocation.

| Setting group | Current configuration |
|---|---|
| Graph | Black background, left-to-right layout, spline edges, configured node and rank spacing |
| Nodes | Fixed-size circles with white outlines |
| Edges | Pen width of one and no arrowheads; per-edge grayscale is supplied by DOT export |
| Output | PNG selected by `-Tpng` |

The output format is selected by the Graphviz arguments, not the destination file
extension. Supplying an `.svg` destination while retaining `-Tpng` still produces
PNG data. Selecting another format requires a corresponding configuration change.
The fixed destination used by `network_to_graph()` remains `network.png`.

Raw DOT can also be inspected or rendered with Graphviz independently of TARS.
The YAML rendering settings are passed to Graphviz separately and are not embedded
in the exported DOT file; independent rendering must supply those settings to
reproduce the same appearance.

---

## 6. Failures and Operational Limits

| Symptom or condition | Current behavior | User action |
|---|---|---|
| `dot` is unavailable | Process startup returns an error | Provide Graphviz on `PATH` |
| Configuration or output directory is not found | File access returns an error | Run from the repository root and verify the required paths |
| Output cannot be written | File access or rendering fails | Verify destination permissions |
| Invalid YAML syntax | Configuration parsing returns an error | Correct the configuration syntax |
| Empty YAML, invalid `dot.flags`, or non-string flag entries | Configuration extraction can panic | Use a first document containing a sequence of string flags |
| Graphviz exits unsuccessfully | Rendering returns an error | Inspect Graphviz diagnostics and verify the source and arguments |
| Empty model, non-linear first module, or incompatible input width | Graph extraction panics | Supply a model beginning with a linear layer and a matching input slice |
| Invalid manual edge endpoints | Index lookup panics | Use positions of nodes added through `add_node()` |
| Extraction is repeated on the same instance | Existing graph state is retained and additional nodes are appended | Create a fresh `NetGraph` for each extraction |

The `Result` returned by `network_to_graph()` does not cover every invalid-input
case. Model dimensions are assumed to be internally consistent. Artifact generation
is not transactional: DOT output can remain after image rendering fails.

---

## 7. Related Documentation

| Document | Scope |
|---|---|
| [`README.md`](README.md) | User documentation entry point |
| [`API.md`](API.md) | Current public interfaces and stability boundaries |
| [`GETTING_STARTED.md`](GETTING_STARTED.md) | Development environment and executable commands |
| [`EXPERIMENTS.md`](EXPERIMENTS.md) | Built-in training configurations |
