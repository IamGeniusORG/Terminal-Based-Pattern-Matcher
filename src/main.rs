use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use regex::{RegexBuilder, Captures};
use colored::*;
use walkdir::WalkDir;
use rayon::prelude::*;
use std::path::PathBuf;

/// A blazing fast, multi-threaded, terminal-based pattern matcher!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The regex pattern to search for
    pattern: String,

    /// The file(s) or directory(ies) to search
    #[arg(required = true)]
    paths: Vec<String>,

    /// Perform a case-insensitive search
    #[arg(short = 'i', long)]
    ignore_case: bool,
}

fn main() {
    // 1. Professional CLI Argument Parsing using 'clap'
    let args = Args::parse();

    // 2. Compile the Regex once for all threads
    let re = match RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build() 
    {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("{} Invalid regex pattern: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // 3. Multi-File/Directory Traversal using 'walkdir'
    let mut files_to_search: Vec<PathBuf> = Vec::new();

    for path_str in &args.paths {
        for entry in WalkDir::new(path_str).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                files_to_search.push(entry.into_path());
            }
        }
    }

    if files_to_search.is_empty() {
        eprintln!("{}", "Warning: No files found to search.".yellow());
        std::process::exit(0);
    }

    // 4. Multi-Threading using 'rayon'
    // This splits the workload of scanning files across all available CPU cores!
    files_to_search.par_iter().for_each(|file_path| {
        let file_path_str = file_path.display().to_string();

        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return, // Silently skip files we cannot read (like binaries or permission errors)
        };

        let reader = BufReader::new(file);

        // Iterate over lines safely
        for (index, line_result) in reader.lines().enumerate() {
            if let Ok(line) = line_result {
                if re.is_match(&line) {
                    // Highlight the match
                    let highlighted_line = re.replace_all(&line, |caps: &Captures| {
                        caps[0].green().bold().to_string()
                    });

                    // Print in a standard grep-like format:  filepath:line_number: match_content
                    println!(
                        "{}:{} {}",
                        file_path_str.magenta(),
                        (index + 1).to_string().cyan(),
                        highlighted_line
                    );
                }
            }
        }
    });
}
