use plotlib::{
    page::Page,
    repr::Plot,
    style::{LineJoin, LineStyle, PointMarker, PointStyle},
};
use plotters::{element::*, style::full_palette::PURPLE};
use plotters::{element::ComposedElement, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    ffi::OsStr,
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

pub trait CpuPlotStyle {
    fn cpu_total_style() -> ShapeStyle;
    fn cpu_core_style() -> ShapeStyle;
    fn cpu_atom_style() -> ShapeStyle;
    fn branch_misses_style() -> ShapeStyle;

    fn cpu_total_legend_style() -> impl Fn((i32, i32)) -> PathElement<(i32, i32)> {
        |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Self::cpu_total_style())
    }
    fn cpu_core_legend_style() -> impl Fn((i32, i32)) -> PathElement<(i32, i32)> {
        |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Self::cpu_core_style())
    }
    fn cpu_atom_legend_style() -> impl Fn((i32, i32)) -> PathElement<(i32, i32)> {
        |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Self::cpu_atom_style())
    }

    fn branch_misses_legend_style() -> impl Fn((i32, i32)) -> PathElement<(i32, i32)> {
        |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Self::branch_misses_style())
    }

    fn line_points_circle() -> impl Fn(
        (f64, f64),
        i32,
        ShapeStyle,
    ) -> ComposedElement<
        (f64, f64),
        SVGBackend<'static>,
        Circle<(i32, i32), i32>,
        Text<'static, (i32, i32), String>,
    > {
        |(x, y), s: i32, st: ShapeStyle| {
            // We want to construct a composed element on-the-fly
            EmptyElement::<(f64, f64), SVGBackend>::at((x, y))
            // At this point, the new pixel coordinate is established    
            + Circle::new((0,0),s,st.filled()) 
            + Text::new(format!("{y:.1}"), (10, 0), ("sans-serif", 10).into_font())
        }
    }

    fn line_points_triangle() -> impl Fn(
        (f64, f64),
        i32,
        ShapeStyle,
    ) -> ComposedElement<
        (f64, f64),
        SVGBackend<'static>,
        TriangleMarker<(i32, i32), i32>,
        Text<'static, (i32, i32), String>,
    > {
        |(x, y), s: i32, st: ShapeStyle| {
            // We want to construct a composed element on-the-fly
            EmptyElement::<(f64, f64), SVGBackend>::at((x, y))
            // At this point, the new pixel coordinate is established    
            + TriangleMarker::new((0,0), s, st.filled()) 
            + Text::new(format!("{y:.1}"), (10, 0), ("sans-serif", 10).into_font())
        }
    }
}
pub struct BranchingStyle;

impl CpuPlotStyle for BranchingStyle {
    fn cpu_total_style() -> ShapeStyle {
        Self::TOTAL_RGB.stroke_width(3)
    }

    fn cpu_core_style() -> ShapeStyle {
        Self::CORE_RGB.mix(0.8).stroke_width(2)
    }

    fn cpu_atom_style() -> ShapeStyle {
        Self::ATOM_RGB.mix(0.5).stroke_width(1)
    }

    fn branch_misses_style() -> ShapeStyle {
        Self::BRANCH_MISSES_RGB.stroke_width(2)
    }
}

impl BranchingStyle {
    pub const TOTAL_LEGEND: &'static str = "Branching TOTAL";
    pub const TOTAL_COLOR: &'static str = "#e59000";
    pub const TOTAL_RGB: RGBColor = RGBColor(0xe5, 0x90, 0);
    pub const TOTAL_POINT_SIZE: f32 = 4.;
    pub const TOTAL_LINE_WIDTH: f32 = 3.;

    pub const CORE_LEGEND: &'static str = "Branching CORE";
    pub const CORE_COLOR: &'static str = "#e5a73e";
    pub const CORE_RGB: RGBColor = RGBColor(0xe5, 0xa7, 0x3e);
    pub const CORE_POINT_SIZE: f32 = 2.5;
    pub const CORE_LINE_WIDTH: f32 = 1.5;

    pub const ATOM_LEGEND: &'static str = "Branching ATOM";
    pub const ATOM_COLOR: &'static str = "#e4ca9d";
    pub const ATOM_RGB: RGBColor = RGBColor(0xe4, 0xca, 0x9d);
    pub const ATOM_POINT_SIZE: f32 = 2.;
    pub const ATOM_LINE_WIDTH: f32 = 1.;

    pub const BRANCH_MISSES_COLOR: &'static str = "#d14419";
    pub const BRANCH_MISSES_RGB: RGBColor = RGBColor(0xd1, 0x44, 0x19);

    pub fn something() {
        println!("test");
    }

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

impl CpuPlotStyle for BranchlessStyle {
    fn cpu_total_style() -> ShapeStyle {
        Self::TOTAL_RGB.stroke_width(3)
    }

    fn cpu_core_style() -> ShapeStyle {
        Self::CORE_RGB.mix(0.8).stroke_width(2)
    }

    fn cpu_atom_style() -> ShapeStyle {
        Self::ATOM_RGB.mix(0.5).stroke_width(1)
    }

    fn branch_misses_style() -> ShapeStyle {
        Self::BRANCH_MISSES_RGB.stroke_width(2)
    }
}

impl BranchlessStyle {
    pub const TOTAL_LEGEND: &'static str = "Branchless TOTAL";
    pub const TOTAL_COLOR: &'static str = "#0369c5";
    pub const TOTAL_RGB: RGBColor = RGBColor(0x03, 0x69, 0xc5);
    pub const TOTAL_POINT_SIZE: f32 = 4.;
    pub const TOTAL_LINE_WIDTH: f32 = 3.;

    pub const CORE_LEGEND: &'static str = "Branchless CORE";
    pub const CORE_COLOR: &'static str = "#1691ff";
    pub const CORE_RGB: RGBColor = RGBColor(0x16, 0x91, 0xff);
    pub const CORE_POINT_SIZE: f32 = 2.5;
    pub const CORE_LINE_WIDTH: f32 = 1.5;

    pub const ATOM_LEGEND: &'static str = "Branchless ATOM";
    pub const ATOM_COLOR: &'static str = "#88bae7";
    pub const ATOM_RGB: RGBColor = RGBColor(0x88, 0xba, 0xe7);

    pub const ATOM_POINT_SIZE: f32 = 2.;
    pub const ATOM_LINE_WIDTH: f32 = 1.;

    pub const BRANCH_MISSES_COLOR: &'static str = "#5d00d1";
    pub const BRANCH_MISSES_RGB: RGBColor = RGBColor(0x5d, 0x00, 0xd1);

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
    save_to: &'static OsStr,
    plot_type: PlotType,
) -> Result<(), Box<dyn Error>> {
    let ratio_vals: Vec<f64> = x_vals.into_iter().map(|x| x as f64).collect();

    let br_perf_stats: Vec<Vec<PerfStatRecord>> = perf_stats_from_json_files(&branching_files)?;
    let bl_perf_stats: Vec<Vec<PerfStatRecord>> = perf_stats_from_json_files(&branchless_files)?;

    match plot_type {
        PlotType::CpuInstructions => {
            cpu_instructions_plot_view(save_to, ratio_vals, &br_perf_stats, &bl_perf_stats)?;
        }
        PlotType::TimeBranchMisses => {
            time_branch_misses_plot_view(save_to, ratio_vals, &br_perf_stats, &bl_perf_stats)?;
        }
        PlotType::Merged => {
            todo!("");
        }
    }

    Ok(())
}

fn time_branch_misses_plot_view(
    save_to: &'static OsStr,
    ratio_vals: Vec<f64>,
    br_perf_stats: &[Vec<PerfStatRecord>],
    bl_perf_stats: &[Vec<PerfStatRecord>],
) -> Result<(), Box<dyn Error>> {
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

    let root_drawing_area = SVGBackend::new(save_to, (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Duration vs. Branch Misses", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..103.0, 0.0..5.0)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_desc("Branch misses [%] / Duration [s]")
        .y_labels(10)
        .x_desc("True/False ratio [%]")
        .draw()?;

        chart
        .draw_series(LineSeries::new(
            br_frac_misses_x.clone(),
            RED,
        ))?
        .label("Branching: Branch Misses")
        .legend(BranchingStyle::branch_misses_legend_style());
        chart.draw_series(PointSeries::of_element(
            br_frac_misses_x.clone(),
            3,
            RED,
            &BranchingStyle::line_points_circle(),
        ))?;

        chart
        .draw_series(LineSeries::new(
            bl_frac_misses_x.clone(),
            PURPLE,
        ))?
        .label("Branchless: Branch Misses")
        .legend(BranchingStyle::branch_misses_legend_style());
        chart.draw_series(PointSeries::of_element(
            bl_frac_misses_x.clone(),
            3,
            PURPLE,
            &BranchingStyle::line_points_triangle(),
        ))?;


        chart
        .draw_series(LineSeries::new(
            br_durations_x.clone(),
            BranchingStyle::cpu_total_style(),
        ))?
        .label("Branching: Duration")
        .legend(BranchingStyle::cpu_total_legend_style());
        chart.draw_series(PointSeries::of_element(
            br_durations_x.clone(),
            0,
            BranchingStyle::cpu_total_style(),
            &BranchingStyle::line_points_circle(),
        ))?;


        chart
        .draw_series(LineSeries::new(
            bl_durations_x.clone(),
            BranchlessStyle::cpu_total_style(),
        ))?
        .label("Branchless: Duration")
        .legend(BranchlessStyle::cpu_total_legend_style());
        chart.draw_series(PointSeries::of_element(
            bl_durations_x.clone(),
            0,
            BranchlessStyle::cpu_total_style(),
            &BranchingStyle::line_points_circle(),
        ))?;

        chart
        .configure_series_labels()
        .position(SeriesLabelPosition::MiddleRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()?;


    Ok(())
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
    save_to: &'static OsStr,
    ratio_vals: Vec<f64>,
    br_perf_stats: &[Vec<PerfStatRecord>],
    bl_perf_stats: &[Vec<PerfStatRecord>],
) -> Result<(), Box<dyn Error>> {
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
    let br_atom_x: Vec<(f64, f64)> = ratio_vals.clone().into_iter().zip(br_atom_scaled).collect();

    // What to do about perf stat not counting cycles from the ATOM cpus?
    // br_atom_x.retain(|(_, a)| *a != 0.0);

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
    let bl_atom_x: Vec<(f64, f64)> = ratio_vals.into_iter().zip(bl_atom_scaled).collect();
    // What to do about perf stat not counting cycles from the ATOM cpus?
    // bl_atom_x.retain(|(_, b)| *b != 0.0);

    let root_drawing_area = SVGBackend::new(save_to, (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("CPU Instructions vs. True/False ratio", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..103.0, 0.0..max_scaled)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_desc(format!(
            "CPU Instructions {unit}",
            unit = match min_magnitude {
                Magnitude::E3 => "10 ^ 3".to_string(),
                Magnitude::E6 => "10 ^ 6".to_string(),
                Magnitude::E9 => "10 ^ 9".to_string(),
                Magnitude::E12 => "10 ^ 12".to_string(),
                Magnitude::E15
                | Magnitude::E18
                | Magnitude::E21
                | Magnitude::E24
                | Magnitude::E27 => format!("{min_magnitude}"),
                Magnitude::Eminus9 | Magnitude::Eminus6 | Magnitude::Eminus3 | Magnitude::E0 =>
                    String::new(),
            }
        ))
        .y_labels(10)
        .x_desc("True/False ratio [%]")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            br_total_x.clone(),
            BranchingStyle::cpu_total_style(),
        ))?
        .label("Branching CPU Total")
        .legend(BranchingStyle::cpu_total_legend_style());
    chart.draw_series(PointSeries::of_element(
        br_total_x.clone(),
        10,
        BranchingStyle::cpu_total_style(),
        &BranchingStyle::line_points_circle(),
    ))?;

    chart
        .draw_series(LineSeries::new(
            br_core_x.clone(),
            BranchingStyle::cpu_core_style(),
        ))?
        .label("Branching CPU Core")
        .legend(BranchingStyle::cpu_core_legend_style());
    chart.draw_series(PointSeries::of_element(
        br_core_x.clone(),
        4,
        BranchingStyle::cpu_core_style(),
        &BranchingStyle::line_points_circle(),
    ))?;

    chart
        .draw_series(LineSeries::new(
            br_atom_x.clone(),
            BranchingStyle::cpu_atom_style(),
        ))?
        .label("Branching CPU Atom")
        .legend(BranchingStyle::cpu_atom_legend_style());
    chart.draw_series(PointSeries::of_element(
        br_atom_x.clone(),
        4,
        BranchingStyle::cpu_atom_style(),
        &BranchingStyle::line_points_circle(),
    ))?;

    chart
        .draw_series(LineSeries::new(
            bl_total_x.clone(),
            BranchlessStyle::cpu_total_style(),
        ))?
        .label("Branchless CPU Total")
        .legend(BranchlessStyle::cpu_total_legend_style());
    chart.draw_series(PointSeries::of_element(
        bl_total_x.into_iter(),
        12,
        BranchlessStyle::cpu_total_style(),
        &BranchlessStyle::line_points_triangle(),
    ))?;

    chart
        .draw_series(LineSeries::new(
            bl_core_x.clone(),
            BranchlessStyle::cpu_core_style(),
        ))?
        .label("Branchless CPU Core")
        .legend(BranchlessStyle::cpu_core_legend_style());
    chart.draw_series(PointSeries::of_element(
        bl_core_x.into_iter(),
        5,
        BranchlessStyle::cpu_core_style(),
        &BranchlessStyle::line_points_triangle(),
    ))?;

    chart
        .draw_series(LineSeries::new(
            bl_atom_x.clone(),
            BranchlessStyle::cpu_atom_style(),
        ))?
        .label("Branchless CPU Atom")
        .legend(BranchlessStyle::cpu_atom_legend_style());
    chart.draw_series(PointSeries::of_element(
        bl_atom_x.into_iter(),
        5,
        BranchlessStyle::cpu_atom_style(),
        &BranchlessStyle::line_points_triangle(),
    ))?;

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()?;

    Ok(())
}
