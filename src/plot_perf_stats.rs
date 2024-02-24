use plotlib::{
    page::Page,
    repr::Plot,
    style::{LineJoin, LineStyle, PointMarker, PointStyle},
    view::ContinuousView,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{write, Display},
    path::PathBuf,
};

use crate::config::PlotType;

#[derive(Clone, Copy)]
pub enum Cpu {
    Total,
    Core,
    Atom,
}
pub struct BranchingStyle;
impl BranchingStyle {
    pub const TOTAL_LEGEND: &'static str = "Branching TOTAL";
    pub const TOTAL_COLOR: &'static str = "#e59000";
    pub const TOTAL_POINT_SIZE: f32 = 4.;
    pub const TOTAL_LINE_WIDTH: f32 = 3.;

    pub const CORE_LEGEND: &'static str = "Branching CORE";
    pub const CORE_COLOR: &'static str = "#e5a73e";
    pub const CORE_POINT_SIZE: f32 = 2.5;
    pub const CORE_LINE_WIDTH: f32 = 1.5;

    pub const ATOM_LEGEND: &'static str = "Branching ATOM";
    pub const ATOM_COLOR: &'static str = "#e4ca9d";
    pub const ATOM_POINT_SIZE: f32 = 2.;
    pub const ATOM_LINE_WIDTH: f32 = 1.;

    pub const BRANCH_MISSES_COLOR: &'static str = "#d14419";

    pub fn color_cpu(cpu: Cpu) -> &'static str {
        match cpu {
            Cpu::Total => Self::TOTAL_COLOR,
            Cpu::Core => Self::CORE_COLOR,
            Cpu::Atom => Self::ATOM_COLOR,
        }
    }

    pub fn line_width_cpu(cpu: Cpu) -> f32 {
        match cpu {
            Cpu::Total => Self::TOTAL_LINE_WIDTH,
            Cpu::Core => Self::CORE_LINE_WIDTH,
            Cpu::Atom => Self::ATOM_LINE_WIDTH,
        }
    }

    pub fn point_size_cpu(cpu: Cpu) -> f32 {
        match cpu {
            Cpu::Total => Self::TOTAL_POINT_SIZE,
            Cpu::Core => Self::CORE_POINT_SIZE,
            Cpu::Atom => Self::ATOM_POINT_SIZE,
        }
    }

    pub fn legend_cpu(cpu: Cpu) -> &'static str {
        match cpu {
            Cpu::Total => Self::TOTAL_LEGEND,
            Cpu::Core => Self::CORE_LEGEND,
            Cpu::Atom => Self::ATOM_LEGEND,
        }
    }

    pub fn line_plot_cpu(val_over_x: Vec<(f64, f64)>, cpu: Cpu) -> Plot {
        Plot::new(val_over_x)
            .legend(Self::legend_cpu(cpu).to_string())
            .line_style(
                LineStyle::new()
                    .colour(Self::color_cpu(cpu))
                    .width(Self::line_width_cpu(cpu))
                    .linejoin(LineJoin::Round),
            )
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Circle)
                    .size(Self::point_size_cpu(cpu))
                    .colour(Self::color_cpu(cpu)),
            )
    }

    pub fn line_plot_branch_misses(val_over_x: Vec<(f64, f64)>) -> Plot {
        Plot::new(val_over_x)
            .legend("Branching: Branch misses".to_string())
            .line_style(
                LineStyle::new()
                    .colour(Self::BRANCH_MISSES_COLOR)
                    .width(2.)
                    .linejoin(LineJoin::Round),
            )
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Circle)
                    .size(1.0)
                    .colour(Self::BRANCH_MISSES_COLOR),
            )
    }

    pub fn line_plot_duration_s(val_over_x: Vec<(f64, f64)>) -> Plot {
        Plot::new(val_over_x)
            .legend("Branching: Duration [s]".to_string())
            .line_style(
                LineStyle::new()
                    .colour(Self::CORE_COLOR)
                    .width(2.)
                    .linejoin(LineJoin::Round),
            )
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Circle)
                    .size(1.0)
                    .colour(Self::CORE_COLOR),
            )
    }
}

pub struct BranchlessStyle;
impl BranchlessStyle {
    pub const TOTAL_LEGEND: &'static str = "Branchless TOTAL";
    pub const TOTAL_COLOR: &'static str = "#0369c5";
    pub const TOTAL_POINT_SIZE: f32 = 4.;
    pub const TOTAL_LINE_WIDTH: f32 = 3.;

    pub const CORE_LEGEND: &'static str = "Branchless CORE";
    pub const CORE_COLOR: &'static str = "#1691ff";
    pub const CORE_POINT_SIZE: f32 = 2.5;
    pub const CORE_LINE_WIDTH: f32 = 1.5;

    pub const ATOM_LEGEND: &'static str = "Branchless ATOM";
    pub const ATOM_COLOR: &'static str = "#88bae7";
    pub const ATOM_POINT_SIZE: f32 = 2.;
    pub const ATOM_LINE_WIDTH: f32 = 1.;

    pub const BRANCH_MISSES_COLOR: &'static str = "#5d00d1";

    pub fn color_cpu(cpu: Cpu) -> &'static str {
        match cpu {
            Cpu::Total => Self::TOTAL_COLOR,
            Cpu::Core => Self::CORE_COLOR,
            Cpu::Atom => Self::ATOM_COLOR,
        }
    }

    pub fn line_width_cpu(cpu: Cpu) -> f32 {
        match cpu {
            Cpu::Total => Self::TOTAL_LINE_WIDTH,
            Cpu::Core => Self::CORE_LINE_WIDTH,
            Cpu::Atom => Self::ATOM_LINE_WIDTH,
        }
    }

    pub fn point_size_cpu(cpu: Cpu) -> f32 {
        match cpu {
            Cpu::Total => Self::TOTAL_POINT_SIZE,
            Cpu::Core => Self::CORE_POINT_SIZE,
            Cpu::Atom => Self::ATOM_POINT_SIZE,
        }
    }

    pub fn legend_cpu(cpu: Cpu) -> &'static str {
        match cpu {
            Cpu::Total => Self::TOTAL_LEGEND,
            Cpu::Core => Self::CORE_LEGEND,
            Cpu::Atom => Self::ATOM_LEGEND,
        }
    }

    pub fn line_plot_cpu(val_over_x: Vec<(f64, f64)>, cpu: Cpu) -> Plot {
        Plot::new(val_over_x)
            .legend(Self::legend_cpu(cpu).to_string())
            .line_style(
                LineStyle::new()
                    .colour(Self::color_cpu(cpu))
                    .width(Self::line_width_cpu(cpu))
                    .linejoin(LineJoin::Round),
            )
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Square)
                    .size(Self::point_size_cpu(cpu))
                    .colour(Self::color_cpu(cpu)),
            )
    }

    pub fn line_plot_branch_misses(val_over_x: Vec<(f64, f64)>) -> Plot {
        Plot::new(val_over_x)
            .legend("Branchless: Branch misses".to_string())
            .line_style(
                LineStyle::new()
                    .colour(Self::BRANCH_MISSES_COLOR)
                    .width(2.)
                    .linejoin(LineJoin::Round),
            )
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Circle)
                    .size(1.0)
                    .colour(Self::BRANCH_MISSES_COLOR),
            )
    }

    pub fn line_plot_duration_s(val_over_x: Vec<(f64, f64)>) -> Plot {
        Plot::new(val_over_x)
            .legend("Branchless: Duration [s]".to_string())
            .line_style(
                LineStyle::new()
                    .colour(Self::CORE_COLOR)
                    .width(2.)
                    .linejoin(LineJoin::Round),
            )
            .point_style(
                PointStyle::new()
                    .marker(PointMarker::Circle)
                    .size(1.0)
                    .colour(Self::CORE_COLOR),
            )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PerfStatRecord {
    counter_value: String,
    unit: String,
    event: String,
    variance: Option<f32>,
    event_runtime: u64,
    pcnt_running: f32,
    metric_value: String,
    metric_unit: String,
}

pub fn read_perf_stat_record_json(
    in_branch_json: PathBuf,
    in_branchless_json: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let branching = std::fs::read_to_string(in_branch_json)?;
    let branchless = std::fs::read_to_string(in_branchless_json)?;
    let perf_stats_branching: Vec<PerfStatRecord> = serde_json::from_str(&branching)?;
    let perf_stats_branchless: Vec<PerfStatRecord> = serde_json::from_str(&branchless)?;

    let mut instructions_b_vs_bl: (u64, u64) = (0, 0);

    for e in perf_stats_branching {
        println!("{e:?}");
        if e.event == "cpu_core/instructions:u/" {
            let trimmed_counter_value = e.counter_value.split('.').next().unwrap();
            eprintln!("{trimmed_counter_value}");
            instructions_b_vs_bl.0 = trimmed_counter_value.parse().unwrap();
        }
    }

    for be in perf_stats_branchless {
        if be.event == "cpu_core/instructions:u/" {
            let trimmed_counter_value = be.counter_value.split('.').next().unwrap();
            eprintln!("{trimmed_counter_value}");
            instructions_b_vs_bl.1 = trimmed_counter_value.parse().unwrap();
        }
    }

    eprintln!(
        "Branching instructions = {}\nBrachless instructions = {}",
        instructions_b_vs_bl.0, instructions_b_vs_bl.1
    );

    use plotlib::repr::BarChart;
    use plotlib::style::BoxStyle;
    use plotlib::view::CategoricalView;

    let bar1 = BarChart::new(instructions_b_vs_bl.0 as f64)
        .label("Branching")
        .style(&BoxStyle::new().fill(BranchingStyle::CORE_COLOR));
    let bar2 = BarChart::new(instructions_b_vs_bl.1 as f64)
        .label("Branchless")
        .style(&BoxStyle::new().fill(BranchlessStyle::CORE_COLOR));

    let view = CategoricalView::new()
        .add(bar1)
        .add(bar2)
        .x_label("Instructions");

    Page::single(&view).save("bchar.svg")?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Magnitude {
    Eminus9,
    Eminus6,
    Eminus3,
    E0,
    E3,
    E6,
    E9,
    E12,
    E15,
    E18,
    E21,
    E24,
    E27,
}

impl Display for Magnitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Magnitude::Eminus9 => "E-9",
            Magnitude::Eminus6 => "E-6",
            Magnitude::Eminus3 => "E-3",
            Magnitude::E0 => "",
            Magnitude::E3 => "E3",
            Magnitude::E6 => "E6",
            Magnitude::E9 => "E9",
            Magnitude::E12 => "E12",
            Magnitude::E15 => "E15",
            Magnitude::E18 => "E18",
            Magnitude::E21 => "E21",
            Magnitude::E24 => "E24",
            Magnitude::E27 => "E27",
        };
        write(f, format_args!("{s}"))
    }
}

impl Magnitude {
    pub fn classify(num: f64) -> Self {
        if num == 0.0 {
            return Magnitude::E0;
        }

        let exponent = num.abs().log10().floor() as i32;
        match exponent {
            27.. => Magnitude::E27,
            24.. => Magnitude::E24,
            21.. => Magnitude::E21,
            18.. => Magnitude::E18,
            15.. => Magnitude::E15,
            12.. => Magnitude::E12,
            9.. => Magnitude::E9,
            6.. => Magnitude::E6,
            3.. => Magnitude::E3,
            0.. => Magnitude::E0,
            -3.. => Magnitude::Eminus3,
            -6.. => Magnitude::Eminus6,
            -9.. => Magnitude::Eminus9,
            _ => unreachable!("what kinda data is this?? num={num}"),
        }
    }

    pub fn scale(&self) -> f64 {
        match self {
            Magnitude::Eminus9 => 1e-9,
            Magnitude::Eminus6 => 1e-6,
            Magnitude::Eminus3 => 1e-3,
            Magnitude::E0 => 1e0,
            Magnitude::E3 => 1e3,
            Magnitude::E6 => 1e6,
            Magnitude::E9 => 1e9,
            Magnitude::E12 => 1e12,
            Magnitude::E15 => 1e15,
            Magnitude::E18 => 1e18,
            Magnitude::E21 => 1e21,
            Magnitude::E24 => 1e24,
            Magnitude::E27 => 1e27,
        }
    }
}

pub fn perf_stats_from_json_files(
    files: &[PathBuf],
) -> Result<Vec<Vec<PerfStatRecord>>, Box<dyn Error>> {
    let mut vec_perf = vec![];
    for f in files {
        let s = std::fs::read_to_string(f)?;
        let perf_data = serde_json::from_str(&s)?;
        vec_perf.push(perf_data);
    }
    Ok(vec_perf)
}

pub fn vals_from_perf_stats(
    perf_stat_runs: &[Vec<PerfStatRecord>],
    event: &str,
) -> Result<(Vec<f64>, Magnitude, u64), Box<dyn Error>> {
    let mut vals: Vec<u64> = vec![];
    for perf_stats in perf_stat_runs {
        for data in perf_stats {
            if data.event.contains(event) {
                if data.counter_value == "<not counted>" {
                    vals.push(0);
                } else {
                    let v = data.counter_value.split('.').next().unwrap();
                    vals.push(v.parse()?);
                }
            }
        }
    }

    let min: u64 = *vals.iter().min().unwrap();
    let max: u64 = *vals.iter().max().unwrap();

    let float_vals = vals.into_iter().map(|x| x as f64).collect();
    Ok((float_vals, Magnitude::classify(min as f64), max))
}

pub fn plot_vs_x(
    x_vals: Vec<u64>,
    branching_files: Vec<PathBuf>,
    branchless_files: Vec<PathBuf>,
    save_to: PathBuf,
    plot_type: PlotType,
) -> Result<(), Box<dyn Error>> {
    let ratio_vals: Vec<f64> = x_vals.into_iter().map(|x| x as f64).collect();

    let br_perf_stats: Vec<Vec<PerfStatRecord>> = perf_stats_from_json_files(&branching_files)?;
    let bl_perf_stats: Vec<Vec<PerfStatRecord>> = perf_stats_from_json_files(&branchless_files)?;

    match plot_type {
        PlotType::CpuInstructions => {
            let v = cpu_instructions_plot_view(ratio_vals, &br_perf_stats, &bl_perf_stats)?;
            Page::single(&v).save(save_to)?;
        }
        PlotType::TimeBranchMisses => {
            let v = time_branch_misses_plot_view(ratio_vals, &br_perf_stats, &bl_perf_stats)?;
            Page::single(&v).save(save_to)?;
        }
        PlotType::Merged => {
            todo!("This doesnt work, should switch to using plotters-rs");
            let cpu_instructions_v =
                cpu_instructions_plot_view(ratio_vals.clone(), &br_perf_stats, &bl_perf_stats)?;
            let time_branch_misses_v =
                time_branch_misses_plot_view(ratio_vals, &br_perf_stats, &bl_perf_stats)?;
            Page::empty()
                .dimensions(2, 1)
                .add_plot(&cpu_instructions_v)
                .add_plot(&time_branch_misses_v)
                .save(save_to)?;
        }
    }

    Ok(())
}

fn time_branch_misses_plot_view(
    ratio_vals: Vec<f64>,
    br_perf_stats: &[Vec<PerfStatRecord>],
    bl_perf_stats: &[Vec<PerfStatRecord>],
) -> Result<ContinuousView, Box<dyn Error>> {
    let (br_durations_x, bl_durations_x) =
        branching_branchless_durations_over_x(ratio_vals.clone(), br_perf_stats, bl_perf_stats)?;

    let br_frac_misses_core = frac_branch_misses_core_from_perf_stats(br_perf_stats)?;
    let bl_frac_misses_core = frac_branch_misses_core_from_perf_stats(bl_perf_stats)?;

    let br_frac_misses_x: Vec<(f64, f64)> = ratio_vals
        .clone()
        .into_iter()
        .zip(br_frac_misses_core)
        .collect();
    let bl_frac_misses_x: Vec<(f64, f64)> = ratio_vals
        .clone()
        .into_iter()
        .zip(bl_frac_misses_core)
        .collect();

    let l_br_branch_misses = BranchingStyle::line_plot_branch_misses(br_frac_misses_x);
    let l_bl_branch_misses = BranchlessStyle::line_plot_branch_misses(bl_frac_misses_x);

    let l_br_core_duration = BranchingStyle::line_plot_duration_s(br_durations_x);
    let l_bl_core_duration = BranchlessStyle::line_plot_duration_s(bl_durations_x);

    let cpu_instructions_plot = ContinuousView::new()
        .add(l_br_branch_misses)
        .add(l_bl_branch_misses)
        .add(l_br_core_duration)
        .add(l_bl_core_duration)
        .x_label("Ratio [True/False]")
        .y_label("Branch misses / Duration [s]")
        .y_range(0.0, 3.5)
        .y_max_ticks(16);
    Ok(cpu_instructions_plot)
}

fn frac_branch_misses_core_from_perf_stats(
    perf_stat_runs: &[Vec<PerfStatRecord>],
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut vals: Vec<f64> = vec![];
    for perf_stats in perf_stat_runs {
        for data in perf_stats {
            if data.event.contains("cpu_core/branch-misses") {
                if data.counter_value == "<not counted>" {
                    vals.push(0.);
                } else {
                    let v: f64 = data.metric_value.parse()?;
                    vals.push(v / 100.);
                }
            }
        }
    }
    Ok(vals)
}

fn durations_from_perf_stats(
    perf_stat_runs: &[Vec<PerfStatRecord>],
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut vals: Vec<f64> = vec![];
    for perf_stats in perf_stat_runs {
        for data in perf_stats {
            if data.event.contains("duration_time") {
                if data.counter_value == "<not counted>" {
                    vals.push(0.);
                } else {
                    let v = data.counter_value.split('.').next().unwrap();
                    let v: u64 = v.parse()?;
                    vals.push((v as f64) / 1_000_000_000.);
                }
            }
        }
    }
    Ok(vals)
}

type BrBlDurationsOverX = (Vec<(f64, f64)>, Vec<(f64, f64)>);
fn branching_branchless_durations_over_x(
    ratio_vals: Vec<f64>,
    br_perf_stats: &[Vec<PerfStatRecord>],
    bl_perf_stats: &[Vec<PerfStatRecord>],
) -> Result<BrBlDurationsOverX, Box<dyn Error>> {
    let br_durations_ms = durations_from_perf_stats(br_perf_stats)?;
    let bl_durations_ms = durations_from_perf_stats(bl_perf_stats)?;

    let br_durations_x: Vec<(f64, f64)> = ratio_vals
        .clone()
        .into_iter()
        .zip(br_durations_ms)
        .collect();
    let bl_durations_x: Vec<(f64, f64)> = ratio_vals
        .clone()
        .into_iter()
        .zip(bl_durations_ms)
        .collect();

    Ok((br_durations_x, bl_durations_x))
}

fn cpu_instructions_plot_view(
    ratio_vals: Vec<f64>,
    br_perf_stats: &[Vec<PerfStatRecord>],
    bl_perf_stats: &[Vec<PerfStatRecord>],
) -> Result<ContinuousView, Box<dyn Error>> {
    let (br_core_vals, br_min_mag, br_core_max) =
        vals_from_perf_stats(br_perf_stats, "cpu_core/instructions")?;
    let (bl_core_vals, bl_min_mag, bl_core_max) =
        vals_from_perf_stats(bl_perf_stats, "cpu_core/instructions")?;

    let (br_atom_vals, _, br_atom_max) =
        vals_from_perf_stats(br_perf_stats, "cpu_atom/instructions")?;
    let (bl_atom_vals, _, bl_atom_max) =
        vals_from_perf_stats(bl_perf_stats, "cpu_atom/instructions")?;

    let min_magnitude = br_min_mag.min(bl_min_mag);

    let br_core_scaled = br_core_vals.into_iter().map(|x| x / min_magnitude.scale());
    let br_atom_scaled = br_atom_vals.into_iter().map(|x| x / min_magnitude.scale());
    let bl_core_scaled = bl_core_vals.into_iter().map(|x| x / min_magnitude.scale());
    let bl_atom_scaled = bl_atom_vals.into_iter().map(|x| x / min_magnitude.scale());

    let max_scaled =
        (br_core_max + br_atom_max).max(bl_core_max + bl_atom_max) as f64 / min_magnitude.scale();

    let br_total_scaled: Vec<f64> = br_core_scaled
        .clone()
        .zip(br_atom_scaled.clone())
        .map(|(a, b)| a + b)
        .collect();
    let br_total_x: Vec<(f64, f64)> = ratio_vals
        .clone()
        .into_iter()
        .zip(br_total_scaled)
        .collect();
    let br_core_x: Vec<(f64, f64)> = ratio_vals.clone().into_iter().zip(br_core_scaled).collect();
    let mut br_atom_x: Vec<(f64, f64)> =
        ratio_vals.clone().into_iter().zip(br_atom_scaled).collect();
    br_atom_x.retain(|(_, a)| *a != 0.0);

    let bl_total_scaled: Vec<f64> = bl_core_scaled
        .clone()
        .zip(bl_atom_scaled.clone())
        .map(|(a, b)| a + b)
        .collect();
    let bl_total_x: Vec<(f64, f64)> = ratio_vals
        .clone()
        .into_iter()
        .zip(bl_total_scaled)
        .collect();
    let bl_core_x: Vec<(f64, f64)> = ratio_vals.clone().into_iter().zip(bl_core_scaled).collect();
    let mut bl_atom_x: Vec<(f64, f64)> = ratio_vals.into_iter().zip(bl_atom_scaled).collect();
    bl_atom_x.retain(|(_, b)| *b != 0.0);

    let l_total_branching = BranchingStyle::line_plot_cpu(br_total_x, Cpu::Total);
    let l_core_branching = BranchingStyle::line_plot_cpu(br_core_x, Cpu::Core);
    let l_atom_branching = BranchingStyle::line_plot_cpu(br_atom_x, Cpu::Atom);

    let l_total_branchless = BranchlessStyle::line_plot_cpu(bl_total_x, Cpu::Total);
    let l_core_branchless = BranchlessStyle::line_plot_cpu(bl_core_x, Cpu::Core);
    let l_atom_branchless = BranchlessStyle::line_plot_cpu(bl_atom_x, Cpu::Atom);

    let cpu_instructions_plot = ContinuousView::new()
        .add(l_total_branching)
        .add(l_core_branching)
        .add(l_atom_branching)
        .add(l_total_branchless)
        .add(l_core_branchless)
        .add(l_atom_branchless)
        .x_label("Ratio [True/False]")
        .y_label(format!("Instructions {min_magnitude}"))
        .y_range(0.0, max_scaled)
        .y_max_ticks(20);

    Ok(cpu_instructions_plot)
}
