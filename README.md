# Taiko

A CLI tool to generate LLM prompts from the codebase.

## Features

- 🔍 Generate structured prompts of the codebase
- 🎯 Smart file filtering and gitignore support
- 📊 Token counting for different LLM models
- 📁 Flexible file and directory exclusion

# Whats this for?

* The primary purpose of generated markdown files is to serve as input to a LLM, such as those provided by OpenAI (GPT-4, GPT-3.5 Turbo), Anthropic (Claude), or others. The Markdown file contains a structured representation of the codebase, ready to be fed into an LLM.

* The token count is crucial. LLMs have a limited context window (e.g., GPT-4 has variants with 8k, 32k, and 128k token limits). The precise token count tells you exactly how much of a model's context window this codebase representation will consume. This is far more accurate than naive character counting or word counting. Knowing the token count allows you to
choose the right model - If your token count exceeds a model's limit, you know you need to use a model with a larger context window, or you need to reduce the size of the input (more on this below), avoid errors - attempting to send more tokens than a model can handle will result in an error and estimate costs - many LLM APIs charge based on token usage. The accurate token count allows you to estimate the cost of processing your codebase.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/harshpratapsingh/taiko
cd taiko
```

2. Build and install:
```bash
cargo install --path .
```

## Usage

```bash
# Generate a code review prompt
taiko . -o review.md

# Generate a security audit prompt
taiko . -p security -o security.md

# Generate documentation prompt
taiko . -p docs -o docs.md

# Exclude patterns
taiko . -e "Cargo.lock,target,.md" -o review.md
# This will exclude:
# - Specific file: "Cargo.lock"
# - Directory: "target"
# - File extension: ".md" (all markdown files)

# Choose specific tokenizer
taiko . -n p50k -o review.md
```

### Options

```
Options:
  -i, --include <PATTERN>      Include only specific file extensions (comma-separated)
  -e, --exclude <PATTERN>      Exclude patterns (comma-separated):
                              - file.ext: Exclude specific file
                              - .ext: Exclude all files with extension
                              - dir: Exclude directory
  -n, --encoding <MODEL>       Choose tokenizer model [default: cl100k]
  -o, --output <FILE>          Output file path
  -p, --prompt-type <TYPE>     Prompt type [default: code_review]
  -h, --help                   Print help
  -V, --version               Print version
```

### Exclude Pattern Examples

```bash
# Exclude specific files
taiko . -e "Cargo.lock,package-lock.json" -o review.md

# Exclude file extensions
taiko . -e ".md,.json,.lock" -o review.md

# Exclude directories
taiko . -e "target,node_modules,dist" -o review.md

# Mix different patterns
taiko . -e "Cargo.lock,.md,target,dist,.json" -o review.md
```

### Available Tokenizers

- `cl100k`: For GPT-4 and GPT-3.5 Turbo models
- `p50k`: For text-davinci and code-davinci models
- `p50k_edit`: For edit models
- `r50k`/`gpt2`: For GPT-3 models like davinci

## License

MIT License
