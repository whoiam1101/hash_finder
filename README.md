# hash_finder
A high-performance Rust console application that calculates SHA-256 hashes for consecutive integers. Finds and outputs hash-number pairs where the hash ends with a specified number of trailing zeros. Supports customizable parameters and leverages concurrency for optimal performance.

## Building the Project

1. Ensure you have Rust installed (1.70.0 or later):
```bash
rustc --version
```

2. Clone the repository:
```bash
git clone https://github.com/whoiam1101/hash_finder.git
cd hash_finder
```

3. Build the project:
```bash
cargo build --release
```

## Running the Program

The program can be run with the following syntax:
```bash
./target/release/hash_finder -N <zeros> -F <results>
```

Parameters:
- `-N, --number-of-zeros`: Number of trailing zeros required in the hash
- `-F, --number-of-results`: Number of matching results to find before stopping

### Examples

Search for 5 hashes that end with 4 zeros:
```bash
./target/release/hash_finder -N 4 -F 5
```

Search for 10 hashes that end with 5 zeros:
```bash
./target/release/hash_finder --number-of-zeros 5 --number-of-results 10
```
