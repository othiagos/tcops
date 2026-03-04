# Team Clustered Orienteering Problem with Subgroups

A high-performance solver for the **TCOPS problem**, featuring both:

-   **Exact approaches** via Integer Linear Programming (ILP)
-   **Metaheuristics** using Variable Neighborhood Search (VNS)

The project is fully implemented in **Rust**, with auxiliary **Python
scripts** for instance visualization.

## Features

-   Exact mathematical modeling (ILP) using external solvers (SCIP, Gurobi)
-   VNS metaheuristic for large-scale instances
-   Clean modular architecture
-   JSON solution export support

## Requirements

To compile and run the project, you must install:

-   System dependencies (Linux)
-   Rust toolchain
-   Python (for auxiliary scripts)

## System Dependencies (Linux)

For Ubuntu/Debian systems:

``` bash
sudo apt update
sudo apt install build-essential libgfortran5 libclang-dev
```

## Rust Installation

If Rust is not installed, install it using the official toolchain:

``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation:

``` bash
rustup update
cargo --version
```

## Python Dependencies

It is recommended to use a virtual environment:

``` bash
# Create virtual environment
python3 -m venv venv
```

``` bash
# Activate
source venv/bin/activate
```

``` bash
# Install dependencies
pip install -r requirements.txt
```

## Commands

| Command   | Description |
|:---       |:--- |
| `make`                | Full pipeline: format + release tests + release build |
| `make check`          | Syntax checking |
| `make test`           | Run unit tests (Debug mode) |
| `make test-release`   | Run unit tests (Release mode) |
| `make build-dev`      | Build in Debug mode |
| `make build`          | Build optimized Release binary |
| `make lint`           | Run Clippy linter |
| `make run`            | Runs optimized Release version |
| `make run-dev`        | Runs Debug version |

###  Example: Heuristic Mode (VNS)

``` bash
make run input=data/instances/inst.tcops mode=heuristic
```

### Example: Exact Mode (SCIP)

``` bash
make run input=data/instances/inst.tcops mode=exact solver=scip
```

## Makefile Parameters

| Make Flag | Rust Equivalent   | Description   | Example |
|:---       |:---               |:---           |:---|
| `input`                   | `--input`                 | Instance file path                        | `data/inst.tcops` |
| `mode`                    | `--mode`                  | Execution mode (`exact` or `heuristic`)   | `heuristic` |
| `solver`                  | `--solver`                | Mathematical solver (`scip`, `gurobi`)    | `scip` |
| `max_iterations`          | `--max-iterations`        | Maximum VNS iterations                    | `500` |
| `max_shaking_intensity`   | `--max-shaking-intensity` | VNS shaking intensity                     | `3` |
| `show`                    | `--show`                  | Display graph with the solution found     | `show=1` |
| `save`                    | `--save`                  | Export solution as JSON                   | `save=1` |
