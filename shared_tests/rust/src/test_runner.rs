use clap::{Parser, ValueEnum};
use std::path::Path;
use crate::json_loader::*;

#[derive(Parser)]
#[command(name = "gafro_test_runner")]
#[command(about = "A test runner for GAFRO JSON test specifications")]
#[command(version)]
pub struct Args {
    /// Test file to run
    pub test_file: String,
    
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Run only tests with specified tag
    #[arg(short, long)]
    pub tag: Option<String>,
    
    /// Run only tests in specified category
    #[arg(short, long)]
    pub category: Option<String>,
    
    /// Show detailed statistics
    #[arg(short, long)]
    pub stats: bool,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

pub fn print_usage() {
    println!("Usage: gafro_test_runner [options] <test_file.json>");
    println!("Options:");
    println!("  -v, --verbose     Enable verbose output");
    println!("  -t, --tag <tag>   Run only tests with specified tag");
    println!("  -c, --category <name>  Run only tests in specified category");
    println!("  -s, --stats       Show detailed statistics");
    println!("  -f, --format <format>  Output format (text, json)");
    println!("  -h, --help        Show this help message");
    println!();
    println!("Examples:");
    println!("  gafro_test_runner scalar_tests.json");
    println!("  gafro_test_runner -v -t basic vector_tests.json");
    println!("  gafro_test_runner -c vector_creation vector_tests.json");
}

pub fn print_test_suite_info(test_suite: &TestSuite) {
    println!("\n=== Test Suite Information ===");
    println!("Name: {}", test_suite.test_suite_name);
    println!("Version: {}", test_suite.version);
    println!("Description: {}", test_suite.description);
    
    let stats = test_suite.get_statistics();
    println!("Total Categories: {}", stats.total_categories);
    println!("Total Test Cases: {}", stats.total_test_cases);
    
    println!("\nCategories:");
    for (name, count) in &stats.tests_per_category {
        println!("  {}: {} tests", name, count);
    }
    
    if !stats.tests_per_tag.is_empty() {
        println!("\nTags:");
        for (tag, count) in &stats.tests_per_tag {
            println!("  {}: {} tests", tag, count);
        }
    }
    println!("==============================");
}

pub fn print_test_results(results: &[TestResult], show_stats: bool, format: &OutputFormat) {
    match format {
        OutputFormat::Text => print_test_results_text(results, show_stats),
        OutputFormat::Json => print_test_results_json(results, show_stats),
    }
}

fn print_test_results_text(results: &[TestResult], show_stats: bool) {
    println!("\n=== Test Results ===");
    
    let mut passed = 0;
    let mut failed = 0;
    let mut total_time = 0.0;
    
    for result in results {
        print!("[{}] {}", 
            if result.passed { "PASS" } else { "FAIL" }, 
            result.test_name
        );
        
        if show_stats {
            print!(" ({:.2}ms)", result.execution_time_ms);
        }
        println!();
        
        if result.passed {
            passed += 1;
        } else {
            failed += 1;
            println!("  Error: {}", result.error_message);
        }
        
        total_time += result.execution_time_ms;
    }
    
    println!("\nSummary:");
    println!("  Passed: {}", passed);
    println!("  Failed: {}", failed);
    println!("  Total: {}", passed + failed);
    println!("  Total Time: {:.2}ms", total_time);
    
    if passed + failed > 0 {
        println!("  Average Time: {:.2}ms", total_time / (passed + failed) as f64);
    }
    
    println!("===================");
}

fn print_test_results_json(results: &[TestResult], _show_stats: bool) {
    let mut output = serde_json::Map::new();
    
    let mut passed = 0;
    let mut failed = 0;
    let mut total_time = 0.0;
    
    let mut test_results = Vec::new();
    for result in results {
        test_results.push(JsonLoader::test_result_to_json(result));
        
        if result.passed {
            passed += 1;
        } else {
            failed += 1;
        }
        total_time += result.execution_time_ms;
    }
    
    output.insert("test_results".to_string(), serde_json::Value::Array(test_results));
    output.insert("summary".to_string(), serde_json::json!({
        "passed": passed,
        "failed": failed,
        "total": passed + failed,
        "total_time_ms": total_time,
        "average_time_ms": if passed + failed > 0 { total_time / (passed + failed) as f64 } else { 0.0 }
    }));
    
    println!("{}", serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap_or_default());
}

pub fn run_tests(args: Args) -> Result<i32, Box<dyn std::error::Error>> {
    // Check if file exists
    if !Path::new(&args.test_file).exists() {
        eprintln!("Error: Test file {} does not exist", args.test_file);
        return Ok(1);
    }
    
    // Load test suite
    println!("Loading test suite from: {}", args.test_file);
    let test_suite = TestSuite::load_from_file(&args.test_file)?;
    
    if !test_suite.is_valid() {
        eprintln!("Error: Invalid test suite");
        return Ok(1);
    }
    
    // Print test suite information
    print_test_suite_info(&test_suite);
    
    // Set up test execution context
    let mut context = TestExecutionContext::new();
    context.set_verbose(args.verbose);
    
    // Execute tests based on filters
    let results = if let Some(category_name) = &args.category {
        // Run specific category
        if let Some(category) = test_suite.get_category(category_name) {
            if let Some(tag) = &args.tag {
                // Filter by tag within category
                let test_cases = category.get_test_cases_by_tag(tag);
                let mut results = Vec::new();
                for test_case in test_cases {
                    results.push(context.execute_test_case(&test_case));
                }
                results
            } else {
                // Run all tests in category
                context.execute_category(category)
            }
        } else {
            eprintln!("Error: Category '{}' not found", category_name);
            return Ok(1);
        }
    } else if let Some(tag) = &args.tag {
        // Run all tests with specific tag
        let test_cases = test_suite.get_test_cases_by_tag(tag);
        let mut results = Vec::new();
        for test_case in test_cases {
            results.push(context.execute_test_case(&test_case));
        }
        results
    } else {
        // Run all tests
        context.execute_test_suite(&test_suite)
    };
    
    // Print results
    print_test_results(&results, args.stats, &args.format);
    
    // Return exit code based on results
    let all_passed = results.iter().all(|r| r.passed);
    Ok(if all_passed { 0 } else { 1 })
}
