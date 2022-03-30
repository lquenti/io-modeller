use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::analyzer::linear_model::LinearModelJSON;
use crate::analyzer::Analysis;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::html_templater::ModelSummaryTemplate;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle};
use plotlib::view::ContinuousView;

use serde_json::json;

// TODO move me
// TODO dont copy
fn model_to_json(analyzed: &[Analysis]) -> String {
    json![analyzed
        .iter()
        .map(|a| LinearModelJSON {
            benchmark_type: a.benchmark.benchmark_type.clone(),
            is_read_op: a.benchmark.is_read_op,
            model: a.linear_model.clone(),
        })
        .collect::<Vec<LinearModelJSON>>()]
    .to_string()
}

fn save_analysis_model(analyzed: &[Analysis]) -> Result<(), io::Error> {
    let json_str = model_to_json(analyzed);
    let path = format!("{}/LinearModel.json", analyzed[0].benchmark.model_path);
    println!("{}", path);

    let mut output = File::create(path)?;
    write!(output, "{}", json_str)?;
    Ok(())
}

fn create_html_report_for_analysis_models(analyzed: &[Analysis]) -> String {
    let mut v = ContinuousView::new()
        .x_label("Access Sizes in Bytes")
        .y_label("Expected Speed in sec");

    for a in analyzed {
        // TODO: Again, here it is expected to be sorted.
        // Also a lot of redundancy
        let lm = &a.linear_model;
        let max_access_size = a.jsons[a.jsons.len() - 1].access_size_in_bytes as f64;
        let line = Plot::new(vec![
            (0.0f64, lm.b),
            (max_access_size, max_access_size * lm.a),
        ])
        .line_style(
            LineStyle::new()
                // TODO add more colours
                .colour("#ff0000")
                .linejoin(LineJoin::Round),
        )
        .legend(format!(
            "{}: {}",
            a.benchmark.benchmark_type,
            if a.benchmark.is_read_op {
                "read"
            } else {
                "write"
            }
        ));
        v = v.add(line);
    }
    Page::single(&v).to_svg().unwrap().to_string()
}

fn save_html_report_for_analysis_models(analyzed: &[Analysis]) -> Result<(), io::Error> {
    let html = ModelSummaryTemplate {
        analyzed,
        svg_all: create_html_report_for_analysis_models(analyzed),
    };
    let path = format!("{}/html/index.html", analyzed[0].benchmark.model_path);
    let mut output = File::create(path)?;
    write!(output, "{}", html.into_html_string())?;
    Ok(())
}

pub fn create_model(
    model_path: &str,
    benchmark_file_path: &str,
    benchmarker_path: &str,
    root: bool,
) -> Result<(), std::io::Error> {
    let mut analyzed: Vec<Analysis>;
    // create folders
    fs::create_dir_all(model_path)?;

    let mut parent = PathBuf::from(benchmark_file_path);
    parent.pop();
    fs::create_dir_all(parent)?;

    analyzed = Vec::new();

    let all_benchmarks = PerformanceBenchmark::get_all_benchmarks(
        model_path,
        benchmark_file_path,
        benchmarker_path,
        root,
    );
    for benchmark in all_benchmarks {
        // run benchmark
        benchmark.run_and_save_all_benchmarks()?;

        // Run analysis
        let res = Analysis::new_from_finished_benchmark(benchmark);
        res.save_html_report()?;
        analyzed.push(res);
    }
    save_analysis_model(&analyzed)?;
    save_html_report_for_analysis_models(&analyzed)?;

    Ok(())
}
