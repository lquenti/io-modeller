use std::fs;
use std::path::PathBuf;

use crate::analyzer::Analysis;
use crate::benchmark_wrapper::PerformanceBenchmark;

pub fn create_model(
    model_path: &str,
    benchmark_file_path: &str,
    benchmarker_path: &str,
    root: bool,
) -> Result<(), std::io::Error> {
    // create folders
    fs::create_dir_all(model_path)?;

    let mut parent = PathBuf::from(benchmark_file_path);
    parent.pop();
    fs::create_dir_all(parent)?;

    let mut analyzed: Vec<Analysis> = Vec::new();

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
        analyzed.push(res);
    }
    Analysis::all_to_file(&analyzed, model_path)?;
    println!("{}", Analysis::all_to_json(&analyzed));

    Ok(())
}
