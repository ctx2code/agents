#!/usr/bin/env -S cargo +nightly -Zscript
--- cargo
package.edition = "2024"
package.version = "0.0.1"

[dependencies]
anyhow = "1.0.100"
clap = { version = "4", features = ["derive"] }
duct = "1.1.1"
rig-core = "0.29.0"
rustyline = "17.0.2"
tokio = { version = "1.49.0", features = ["full"] }
---
//! Still under development.

// Always use anyhow::Result on top level functions.
use anyhow::{ Result, Context, };

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Run cargo commands for development purposes.
    Dev {
        /// Arguments to pass to cargo.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        else_args: Vec<String>,
    },
}

/// Run a cargo command for development purposes.
fn run_cargo(else_args: Vec<String>) -> Result<()> {
    let cargo_args =
        ["+nightly".into()].into_iter()
            .chain(
                else_args.into_iter())
            .chain(
                ["-Zscript",
                    "--manifest-path", std::env!("CARGO_MANIFEST_PATH")]
                    .into_iter().map(Into::into))
            .collect::<Vec<_>>();
    use duct::cmd;
    cmd("cargo", cargo_args)
        .run().map(|_| {})
        .context("Running cargo command")
}

async fn run_repl() -> Result<()> {
    agent::prepare();
    use rustyline::{ DefaultEditor, error::ReadlineError::*, };
    let mut rl = DefaultEditor::new().context("Creating line editor")?;
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).ok();
                println!("{}", agent::enter(&line).await?);
            }
            Err(Interrupted | Eof) => {
                return Ok(());
            }
            Err(err) => {
                return Err(err).context("Readline error");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;
    let args = Args::parse();
    match args.command {
        Some(Command::Dev { else_args }) => {
            run_cargo(else_args)
        }
        None => {
            run_repl().await
        }
    }
}

mod agent {
    use rig::{
        agent::Agent, providers::openrouter::CompletionModel,
    };
    use std::sync::LazyLock;
    static AGENT: LazyLock<Agent<CompletionModel>> = LazyLock::new(|| {
        use rig::providers::openrouter::Client;
        use crate::rig_prelude::*;
        Client::from_env()
            .agent("openai/gpt-5.2-chat")
            .build()
    });
    pub fn prepare() {
        LazyLock::force(&AGENT);
    }

    use anyhow::Result;

    pub async fn enter(input: &str) -> Result<String> {
        use crate::rig_prelude::*;
        match AGENT.prompt(input).await {
            Ok(response) => {
                Ok(response)
            }
            Err(err) => {
                Ok(format!("Error: {}", err))
            }
        }
    }
}

mod rig_prelude {
    //! Complements the rig prelude with other commonly used items.
    pub use rig::{
        prelude::*, completion::Prompt,
    };
}
