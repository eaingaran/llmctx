# llmctx

[![Build Status](https://github.com/eaingaran/llmctx/actions/workflows/release.yaml/badge.svg)](https://github.com/eaingaran/llmctx/actions/workflows/release.yaml)

**llmctx** is a command-line tool that helps you prepare your source code for use with large language models (LLMs). It takes a directory of source code and generates a single output file that contains:

  * The file paths of all source files
  * The content of all source files

This makes it easy to pass the whole project/application/module as context for your questions to LLMs.

## Features

  * **Exclusion:** Exclude specific files and folders (e.g., `target`, `node_modules`, `.git`)
  * **Output:** Specify the output file name.
  * **Clipboard:** Copy the output directly to the clipboard.
  * **Compact Output:** Remove new lines and tabs for a more concise representation.
  * **Customizable Input:** Process any directory you want.

## Installation

1.  Make sure you have Rust installed. If not, follow the instructions at https://www.rust-lang.org/tools/install.
2.  Download the latest release binary from the GitHub releases page.
3.  Make the binary executable (e.g., `chmod +x llmctx`)
4.  Optionally, move the binary to a directory in your PATH.


## Usage

```bash
llmctx [OPTIONS] [INPUT_DIR]
```

**Options:**

  * `-e, --exclude <EXCLUDE>`: Comma-separated list of folders and files to exclude (case insensitive). Example: `target,node_modules,.git,cargo.toml` (default: '')
  * `-o, --output <OUTPUT>`: Path to the output file (default: `output.txt`)
  * `--clipboard`: Copy the output to the clipboard
  * `-c, --compact`: Generate compact output (remove new lines and tabs)
  * `INPUT_DIR`: Input directory to process (default: current directory)

**Example:**

To process the current directory and exclude `target` and `node_modules` directories, saving the output to `context.txt`:

```bash
llmctx -e target,node_modules -o context.txt
```

To process the my-project directory, remove new lines and tabs, and copy the output to the clipboard:

```bash
llmctx my-project -c --clipboard
```

## How it Works

llmctx walks through your source directory, reads each file, and formats the output. The output includes file paths and their contents, making it easy for LLMs to understand the structure and content of your code.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
