use std::{
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use colored::*;
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use termtree::Tree;
use tiktoken_rs::{cl100k_base, p50k_base, p50k_edit, r50k_base};
use prompt_macro::prompt;

mod prompts;

#[derive(Parser)]
#[clap(name = "taiko", version = "1.0.0", author = "Harsh Pratap Singh")]
struct Cli {
    #[arg()]
    path: PathBuf,
    #[clap(short, long)]
    include: Option<String>,
    #[clap(short, long)]
    exclude: Option<String>,
    #[clap(short='n', long)]
    encoding: Option<String>,
    #[clap(short, long)]
    output: Option<String>,
    #[clap(short, long, default_value = "code_review")]
    prompt_type: String,
}

fn main() {
    let args = Cli::parse();

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(120));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );

    spinner.set_message("Building prompt...");

    let create_tree = traverse_directory(&args.path, &args.include, &args.exclude);
    let (tree, files) = create_tree.unwrap_or_else(|e| {
        spinner.finish_with_message(format!("{}", "Failed!".red()));
        eprintln!(
            "{}{}{} {}",
            "(".bold().white(),
            "x".bold().red(),
            ")".bold().white(),
            format!("Failed to build directory tree: {}", e).red()
        );
        std::process::exit(1);
    });

    spinner.finish_with_message(format!("{}", "Done!".green()));

    let files_content = files.iter().map(|file| {
        format!("{}:\n\n{}\n", 
            file["path"].as_str().unwrap(),
            file["code"].as_str().unwrap())
    }).collect::<Vec<_>>().join("\n");

    let content = format!(
        "Project Path: {}\n\nProject Structure:\n{}\n\nFiles Content:\n{}",
        args.path.canonicalize().unwrap().display(),
        tree,
        files_content
    );

    let prompt = match args.prompt_type.as_str() {
        "code_review" => prompts::code_review(&content),
        "security" => prompts::security_audit(&content),
        "docs" => prompts::documentation(&content),
        _ => prompts::code_review(&content),
    };

    let (bpe, model_info) = match args.encoding.as_deref().unwrap_or("cl100k") {
        "cl100k" => (cl100k_base(), "for models - gpt-4-*, gpt-3.5-turbo-*"),
        "p50k" => (
            p50k_base(),
            "for models - text-davinci-*, code-davinci-* and more code models",
        ),
        "p50k_edit" => (
            p50k_edit(),
            "Edit models like text-davinci-edit-001, code-davinci-edit-001, text-curie-001, text-babbage-001 and more",
        ),
        "r50k" | "gpt2" => (r50k_base(), "GPT-3 models like davinci"),
        _ => (cl100k_base(), "for models - gpt-4-*, gpt-3.5-turbo-*"),
    };

    let token_count = bpe.unwrap().encode_with_special_tokens(&prompt).len();

    println!(
        "{}{}{} Token count: {}, Model info: {}",
        "(".bold().white(),
        "i".bold().blue(),
        ")".bold().white(),
        token_count.to_string().bold().yellow(),
        model_info
    );

    if let Some(output_path) = args.output {
        let file = std::fs::File::create(&output_path).expect("Failed to create output file");
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "{}", prompt).expect("Failed to write to output file");

        println!(
            "{}{}{} {}",
            "(".bold().white(),
            "✓".bold().green(),
            ")".bold().white(),
            format!("Prompt written to file: {}", output_path).green()
        );
    } else {
        println!("{}", prompt);
    }
}

#[inline]
fn wrap_code_block(code: &str, extension: &str) -> String {
    let backticks = "`".repeat(7);
    format!("{}{}\n{}\n{}", backticks, extension, code, backticks)
}

#[inline]
fn label<P: AsRef<Path>>(p: P) -> String {
    let path = p.as_ref();
    if path.file_name().is_none() {
        path.to_str().unwrap_or(".").to_owned()
    } else {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_owned()
    }
}

fn is_binary(path: &Path) -> bool {
    if let Ok(file) = fs::File::open(path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                for byte in line.bytes() {
                    if byte == b'\0' || !byte.is_ascii_graphic() {
                        return true;
                    }
                }
            }
        }
    }

    false 
}

fn traverse_directory(
    root_path: &PathBuf,
    include: &Option<String>,
    exclude: &Option<String>,
) -> Result<(String, Vec<serde_json::Value>)> {
    let mut files = Vec::new();

    let canonical_root_path = root_path.canonicalize()?;
    let gitignore_path = canonical_root_path.join(".gitignore");

    let mut ignore_builder = ignore::gitignore::GitignoreBuilder::new(&canonical_root_path);
    if let Ok(gitignore_file) = fs::File::open(&gitignore_path) {
        let gitignore_reader = io::BufReader::new(gitignore_file);
        for line in gitignore_reader.lines() {
            if let Ok(line) = line {
                let _ = ignore_builder.add_line(None, &line);
            }
        }
    }

    let tree = WalkBuilder::new(&canonical_root_path)
        .git_ignore(false)
        .build()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            let relative_path = entry
                .path()
                .strip_prefix(&canonical_root_path)
                .unwrap_or_else(|_| entry.path());

            let is_ignored = ignore_builder.build().unwrap().matched(relative_path, false).is_ignore();
            !is_ignored
        })
        .fold(Tree::new(label(&canonical_root_path)), |mut root, entry| {
            let path = entry.path();
            if let Ok(relative_path) = path.strip_prefix(&canonical_root_path) {
                let mut current_tree = &mut root;
                for component in relative_path.components() {
                    let component_str = component.as_os_str().to_string_lossy().to_string();
                    current_tree = if let Some(pos) = current_tree
                        .leaves
                        .iter_mut()
                        .position(|child| child.root == component_str)
                    {
                        &mut current_tree.leaves[pos]
                    } else {
                        let new_tree = Tree::new(component_str.clone());
                        current_tree.leaves.push(new_tree);
                        current_tree.leaves.last_mut().unwrap()
                    };
                }
                if path.is_file() && !is_binary(&path) { 
                    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                    if let Some(ref exclude_ext) = exclude {
                        let exclude_extensions: Vec<&str> =
                            exclude_ext.split(',').map(|s| s.trim()).collect();
                        if exclude_extensions.contains(&extension) {
                            return root;
                        }
                    }
                    if let Some(ref filter_ext) = include {
                        let filter_extensions: Vec<&str> =
                            filter_ext.split(',').map(|s| s.trim()).collect();
                        if !filter_extensions.contains(&extension) {
                            return root;
                        }
                    }
                    let code_bytes = fs::read(&path).expect("Failed to read file");
                    let code = String::from_utf8_lossy(&code_bytes);
                    let code_block = wrap_code_block(&code, extension);
                    if !code.trim().is_empty() && !code.contains(char::REPLACEMENT_CHARACTER) {
                        files.push(json!({
                            "path": path.display().to_string(),
                            "extension": extension,
                            "code": code_block,
                        }));
                    }
                }
            }
            root
        });

    Ok((tree.to_string(), files))
}
