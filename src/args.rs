use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, arg_required_else_help = true)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with LeetCode
    #[command(visible_alias = "-a")]
    Auth,
    /// Executes code with testcases
    #[command(visible_alias = "-r")]
    Run {
        #[arg(short, long)]
        /// File to execute
        file: Option<PathBuf>,
        #[arg(short, long)]
        /// Testcases to run
        testcase_file: Option<String>,
    },
    /// Submits code to LeetCode
    #[command(visible_alias = "-fs")]
    FastSubmit {
        #[arg(short, long)]
        /// File to submit
        file: Option<PathBuf>,
    },
    #[command(visible_alias = "-s")]
    Submit {
        #[arg(short, long)]
        /// File to submit
        file: Option<PathBuf>,
        #[arg(short, long)]
        /// Testcases to run
        testcase_file: Option<String>,
    },
    /// Save a question as HTML
    #[command(visible_alias = "-q")]
    Question {
        /// Question name
        question_name: String,
        /// Flag to not save the boilerplate code
        #[arg(short, long)]
        no_code_save: bool,
    },
    /// Save today's daily challenge as HTML
    #[command(visible_alias = "-d")]
    DailyChallenge {
        /// Flag to not save the boilerplate code
        #[arg(short, long)]
        no_code_save: bool,
    },
    /// Packs the solution with the question into a directory
    #[command(visible_alias = "-p")]
    Pack {
        #[arg(short, long)]
        /// File to pack
        file: Option<PathBuf>,
    },
}
