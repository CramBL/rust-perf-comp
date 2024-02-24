use clap::*;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    CleanPerfStatJson {
        #[arg(short, long)]
        input_file: PathBuf,
        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },
    BoxPlotBranchVsBranchless {
        #[arg(short = 'b', long = "in-branching-json")]
        in_branch_json: PathBuf,
        #[arg(short = 'l', long = "in-branchless-json")]
        in_branchless_json: PathBuf,
    },
    LineOverX {
        #[arg(required = true)]
        x_vals: Vec<u64>,
        #[arg(long)]
        json_dir: PathBuf,
        #[arg(long)]
        branching_prefix: String,
        #[arg(long)]
        branchless_prefix: String,
        #[arg(long)]
        save_to: PathBuf,
        #[arg(long)]
        plot_type: PlotType,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlotType {
    CpuInstructions,
    TimeBranchMisses,
    Merged,
}
