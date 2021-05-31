# qasm2tex

qasm2tex is a tool for generating quantum circuit diagrams in LaTeX from qasm source code files.
This tool targets the [qcircuit](https://ctan.org/pkg/qcircuit?lang=en) package and was inspired by MIT's [qasm2circ](https://www.media.mit.edu/quanta/qasm2circ/) tool.
The qasm2tex tool is not quite ready for real use, but can already generate correct LaTeX output for some simple examples.

## Dependencies

Only a recent Rust compiler (developed with 1.48.0) is required to build from source.
