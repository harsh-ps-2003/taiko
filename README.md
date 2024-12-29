# Taiko

A powerful CLI tool for generating AI prompts from your codebase. Perfect for code reviews, security audits, and documentation generation.

## Features

- 🔍 Generate structured prompts for code review
- 🛡️ Security audit prompt generation
- 📚 Documentation prompt generation
- 🎯 Smart file filtering and gitignore support
- 📊 Token counting for different LLM models
- 🚀 Fast and efficient codebase traversal

## Installation

### Using Cargo (Recommended)

```bash
cargo install taiko
```

### Building from Source

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

Features :
Respects .gitignore files to exclude 
unwanted files and directories,
Handles binary files gracefully,
Customize the prompt using Handlebar to 
generate Markdown,
Filter by extension,
Display's the token count.

```bash
# Generate a code review prompt
taiko . -o review.md

# Generate a security audit prompt
taiko . -p security -o security.md

# Generate documentation prompt
taiko . -p docs -o docs.md

# Filter specific file types
taiko . -i "rs,toml" -o review.md

# Exclude specific file types
taiko . -e "json,md" -o review.md

# Choose specific tokenizer
taiko . -n p50k -o review.md
```

### Options

```
Options:
  -i, --include <PATTERN>    Include only specific file extensions (comma-separated)
  -e, --exclude <PATTERN>    Exclude specific file extensions (comma-separated)
  -n, --encoding <MODEL>     Choose tokenizer model [default: cl100k]
  -o, --output <FILE>        Output file path
  -p, --prompt-type <TYPE>   Prompt type [default: code_review]
  -h, --help                 Print help
  -V, --version             Print version
```

### Available Tokenizers

- `cl100k`: For GPT-4 and GPT-3.5 Turbo models
- `p50k`: For text-davinci and code-davinci models
- `p50k_edit`: For edit models
- `r50k`/`gpt2`: For GPT-3 models like davinci

## License

MIT License
