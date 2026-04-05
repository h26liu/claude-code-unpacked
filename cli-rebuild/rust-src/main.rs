mod agent_loop;
mod catalog;
mod config;
mod protocol;
mod providers;
mod session;
mod tools;

use std::io::{self, Write};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{agent_loop::{inspect_catalog, render_trace, run_task}, config::{load_config, render_config_summary}};

#[derive(Parser, Debug)]
#[command(name = "cli-rebuild")]
#[command(about = "Rust clean-room Claude Code-like CLI runtime")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Inspect,
    Config,
    Trace {
        #[arg(long)]
        task: String,
        #[arg(long, default_value = "default")]
        scenario: String,
        #[arg(long, default_value_t = false)]
        plan_mode: bool,
        #[arg(long = "memory")]
        memory: Vec<String>,
    },
    Run {
        #[arg(long)]
        task: String,
        #[arg(long)]
        scenario: Option<String>,
        #[arg(long)]
        plan_mode: bool,
        #[arg(long = "memory")]
        memory: Vec<String>,
    },
    Chat,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let loaded = load_config()?;

    match cli.command {
        Command::Inspect => {
            println!("{}", inspect_catalog());
        }
        Command::Config => {
            println!("{}", render_config_summary(&loaded)?);
        }
        Command::Trace { task, scenario, plan_mode, memory } => {
            println!("{}", render_trace(&loaded.config, &task, &scenario, plan_mode, &memory));
        }
        Command::Run { task, scenario, plan_mode, memory } => {
            let scenario = scenario.unwrap_or_else(|| loaded.config.session.scenario.clone());
            let plan_mode = if plan_mode { true } else { loaded.config.session.plan_mode };
            let extra_memory = if memory.is_empty() { loaded.config.session.memory.clone() } else { memory };
            let result = run_task(&loaded.config, &task, &scenario, plan_mode, &extra_memory, false).await?;
            println!("{}", result.final_message);
        }
        Command::Chat => {
            println!("Interactive chat mode. Type `exit` to quit.");
            loop {
                print!("> ");
                io::stdout().flush()?;
                let mut task = String::new();
                io::stdin().read_line(&mut task)?;
                let task = task.trim().to_string();
                if task.is_empty() || task == "exit" {
                    break;
                }
                let result = run_task(
                    &loaded.config,
                    &task,
                    &loaded.config.session.scenario,
                    loaded.config.session.plan_mode,
                    &loaded.config.session.memory,
                    true,
                ).await?;
                println!("{}", result.final_message);
            }
        }
    }

    Ok(())
}
