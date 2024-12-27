# `taiko`

A CLI tool to generate context of your entire local codebase for your LLM by uploading the prompt. 

Just edit the template file with your desired prompt and run `cargo build --release` and `./target/release/taiko <path to your directory> -o <name of output markdown file> ` to obtain a file with the message that you can directly upload to your LLM. Take note of the token count, you might need models with high context windows!

Features :
Respects .gitignore files to exclude unwanted files and directories,
Handles binary files gracefully,
Customize the prompt using Handlebar to generate Markdown,
Filter by extension,
Display's the token count. 

Run `cargo run -- --help` for know about flags.
