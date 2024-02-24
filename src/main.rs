use clap::Parser;
use regex::Regex;
use rust_perf_comp::{
    config::{Args, Command},
    plot_perf_stats::{self, plot_vs_x},
};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.cmd {
        Command::CleanPerfStatJson {
            input_file,
            output_file,
        } => clean_perf_stat_json(input_file, output_file)?,
        Command::BoxPlotBranchVsBranchless {
            in_branch_json,
            in_branchless_json,
        } => plot_perf_stats::read_perf_stat_record_json(in_branch_json, in_branchless_json)?,
        Command::LineOverX {
            x_vals,
            json_dir,
            branching_prefix,
            branchless_prefix,
            save_to,
            plot_type,
        } => {
            println!("Producing function over {x_vals:?}");
            println!("Using json-files from {json_dir:?} match patterns {branching_prefix} & {branchless_prefix} with the expected suffix of [x].json (e.g. {branching_prefix}0.json");
            assert!(json_dir.exists(), "{json_dir:?} does not exist");
            assert!(json_dir.is_dir(), "{json_dir:?} is not a directory");

            let mut branching_files = vec![];
            let mut branchless_files = vec![];
            for x in &x_vals {
                let a = json_dir.join(PathBuf::from(format!("{branching_prefix}{x}.json")));
                assert!(a.exists(), "{a:?} Does not exist - Expects {json_dir:?} to contain two files per value in x_vals, e.g. {branching_prefix}{x}.json & {branchless_prefix}{x}.json");
                branching_files.push(a);

                let b = json_dir.join(PathBuf::from(format!("{branchless_prefix}{x}.json")));
                assert!(b.exists(), "{b:?} Does not exist - Expects {json_dir:?} to contain two files per value in x_vals, e.g. {branching_prefix}{x}.json & {branchless_prefix}{x}.json");
                branchless_files.push(b);
            }
            plot_vs_x(
                x_vals,
                branching_files,
                branchless_files,
                save_to,
                plot_type,
            )?;
        }
    }
    Ok(())
}

pub fn clean_perf_stat_json(
    in_file: PathBuf,
    output_file: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    // read json file
    assert!(in_file.exists());
    let json = std::fs::read_to_string(in_file)?;
    // Replace all commas in numbers with dots
    let re: Regex = Regex::new(r"(\d+),(\d+)")?;
    let json = re.replace_all(&json, "$1.$2");
    // Add a trailing comma to the end of each line except the last one
    let re: Regex = Regex::new("(\n)")?;
    let json = re.replace_all(&json, ",");
    let json = json.trim_end_matches(',');

    // Add bracket to the start and end of the json
    let final_json = format!("[{json}]");
    if let Some(out) = output_file {
        assert!(out.exists());
        std::fs::write(out, final_json)?;
    } else {
        println!("{final_json}");
    }
    Ok(())
}
