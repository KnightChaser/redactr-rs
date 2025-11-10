// src/main.rs
use aho_corasick::AhoCorasick;
use clap::{Parser, ValueEnum};
use regex::Regex;

use globset::{Glob, GlobSetBuilder};
use redactr_rs::{
    engines::{aho::ACRedactor, regex::RegexRedactor},
    process,
};
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Clone, ValueEnum, Debug)]
enum Engine {
    Regex,
    AC,
}

#[derive(Parser, Debug)]
struct Args {
    /// File/directory path(s)
    paths: Vec<String>,

    /// Engine to use
    #[arg(long)]
    engine: Engine,

    /// Replacement token
    #[arg(long, default_value = "[REDACTED]")]
    token: String,

    /// Glob filter, e.g. **/*.md
    #[arg(long)]
    glob: Option<String>,

    /// In-place modify files
    #[arg(long)]
    in_place: bool,

    /// Emit JSON report to stdout
    #[arg(long)]
    json: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // compile patterns once (exercise lifetimes by borrowing them into engines)
    let email_re = Regex::new(r"(?i)[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}")?;
    let api_ac = AhoCorasick::new(["AKIA", "sk_live_", "ghp_", "xoxb-"])?;

    // glob filter
    let glob_ok = if let Some(g) = &args.glob {
        let mut b = GlobSetBuilder::new();
        b.add(Glob::new(g)?);
        Some(b.build()?)
    } else {
        None
    };

    let mut reports = Vec::new();

    for p in &args.paths {
        for entry in WalkDir::new(p)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(g) = &glob_ok {
                if !g.is_match(path) {
                    continue;
                }
            }

            let text = std::fs::read_to_string(path)?;
            let (out, replaced) = match args.engine {
                Engine::Regex => {
                    let engine = RegexRedactor::new("email", &email_re);
                    process(&engine, &text, &args.token)?
                }
                Engine::AC => {
                    let engine = ACRedactor::new("api-token", &api_ac);
                    process(&engine, &text, &args.token)?
                }
            };

            if args.in_place && replaced > 0 {
                std::fs::write(path, out)?;
            }

            reports.push((path.display().to_string(), replaced));
        }
    }

    if args.json {
        #[derive(Serialize)]
        struct Item<'a> {
            file: &'a str,
            replaced: usize,
        }

        let playload: Vec<Item> = reports
            .iter()
            .map(|(f, r)| Item {
                file: f,
                replaced: *r,
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&playload)?);
    } else {
        for (file, replaced) in reports {
            println!("{}: {}", file, replaced);
        }
    }

    Ok(())
}
