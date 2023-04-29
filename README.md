# Getting Started with the Test File Generator

The Test File Generator is a simple command-line (CLI) program written in Rust that generates a file with random content. This can be useful for creating test files of specific sizes, for example, to test file upload and download speeds, or to test how an application handles large files.

## Prerequisites

- [Rust](https://www.rust-lang.org/): To compile and run the program, you need to have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Installation

1. Clone the Test File Generator repository or download the source code provided above.

2. Open a terminal and navigate to the directory containing the source code.

3. Compile the program by running the following command:

   ```
   cargo build --release
   ```

   This command will generate an optimized executable in the `target/release` directory.

4. (Optional) Add the `target/release` directory to your `PATH` environment variable to access the Test File Generator from any location.

## Usage

To use the Test File Generator, simply run the compiled binary:

```
./target/release/test_file_generator
```

The program will generate a file named `foo.txt` in the same directory as the binary, containing random content. The file will have a size of approximately 2GB (500,000 chunks of 4,096 bytes each).

As the file is being generated, the program will print progress information every 1,000 chunks (approximately every 4 MB), showing the current position, target size, and percentage of completion.

Example output:

```
- writing 0 of 2097152000 bytes (0.00%)
- writing 4096000 of 2097152000 bytes (0.20%)
- writing 8192000 of 2097152000 bytes (0.39%)
...
- writing 2093056000 of 2097152000 bytes (99.80%)
Operation complete!
```

## Customization

If you need to generate a file with a different name or size, you can modify the source code to change the `chunk_count` variable in the `main` function to the desired number of chunks, and/or change the filename in the `File::create` call. To apply the changes, recompile the program by running `cargo build --release` again.

## Limitations

- The program currently generates a fixed-size file of approximately 2GB. To generate files of different sizes, you need to modify the source code and recompile the program.

- The generated file is named `foo.txt`. To change the filename, you need to modify the source code and recompile the program.

- The program does not provide any command-line options to customize its behavior. All customizations must be done by editing the source code and recompiling the program.
