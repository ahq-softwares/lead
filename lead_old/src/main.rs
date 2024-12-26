mod app;

use clap::{arg, Parser, Subcommand};

#[derive(Clone, Subcommand, Debug)]
pub enum Command {
  /// Run a Lead app
  Run {
    /// Should it be production?
    #[arg(short, long)]
    prod: bool,
    #[arg(short, long)]
    dir: Option<String>,
  },
  Docs,
  /// Create a new Lead app project
  New { dir: String },
}

#[derive(Parser)]
pub struct Arguments {
  #[clap(subcommand)]
  cmd: Option<Command>,
}

fn main() {
  let subcommand: Command = Arguments::parse().cmd.unwrap_or(Command::Run {
    prod: false,
    dir: None,
  });

  match subcommand {
    Command::Docs => {
      let Ok(home) = std::env::var("LEAD_HOME") else {
        panic!("LEAD_HOME not set");
      };

      #[cfg(windows)]
      let file = "lead_docs.exe";

      #[cfg(unix)]
      let file = "lead_docs";

      let file = format!("{}/versions/{}/{}", home, env!("CARGO_PKG_VERSION"), file);

      std::process::Command::new(file).spawn().unwrap().wait().unwrap();
    }
    Command::Run { prod, dir } => {
      let dir = dir.unwrap_or(".".into());
      app::run(dir, prod);
    }
    _ => todo!(),
  }
}
