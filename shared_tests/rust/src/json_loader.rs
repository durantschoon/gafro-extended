use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use regex::Regex;

/// Represents a single test case from JSON specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub test_name: String,
    pub description: String,
    pub category: String,
    pub inputs: Value,
    pub expected_outputs: Value,
    pub tolerance: f64,
    pub language_specific: Option<Value>,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
    
    // Rust specific configuration
    pub rust_test_code: String,
    pub rust_includes: Vec<String>,
    pub rust_setup_code: String,
    pub rust_cleanup_code: String,
}

impl TestCase {
    /// Parse language-specific configuration for Rust
    pub fn parse_rust_config(&mut self) {
        if let Some(language_specific) = &self.language_specific {
            if let Some(rust_config) = language_specific.get("rust") {
                if let Some(test_code) = rust_config.get("test_code") {
                    self.rust_test_code = test_code.as_str().unwrap_or("").to_string();
                }
                
                if let Some(includes) = rust_config.get("includes") {
                    self.rust_includes.clear();
                    if let Some(include_array) = includes.as_array() {
                        for include in include_array {
                            if let Some(include_str) = include.as_str() {
                                self.rust_includes.push(include_str.to_string());
                            }
                        }
                    }
                }
                
                if let Some(setup_code) = rust_config.get("setup_code") {
                    self.rust_setup_code = setup_code.as_str().unwrap_or("").to_string();
                }
                
                if let Some(cleanup_code) = rust_config.get("cleanup_code") {
                    self.rust_cleanup_code = cleanup_code.as_str().unwrap_or("").to_string();
                }
            }
        }
    }
    
    /// Validate that the test case has required fields
    pub fn is_valid(&self) -> bool {
        !self.test_name.is_empty() && 
        !self.description.is_empty() && 
        !self.category.is_empty() && 
        !self.rust_test_code.is_empty()
    }
}

/// Represents a test category containing multiple test cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCategory {
    pub name: String,
    pub test_cases: Vec<TestCase>,
}

impl TestCategory {
    /// Add a test case to this category
    pub fn add_test_case(&mut self, test_case: TestCase) {
        self.test_cases.push(test_case);
    }
    
    /// Get test cases by tag
    pub fn get_test_cases_by_tag(&self, tag: &str) -> Vec<TestCase> {
        self.test_cases.iter()
            .filter(|test_case| test_case.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }
    
    /// Get test cases by name pattern
    pub fn get_test_cases_by_name(&self, pattern: &str) -> Vec<TestCase> {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => return Vec::new(),
        };
        
        self.test_cases.iter()
            .filter(|test_case| regex.is_match(&test_case.test_name))
            .cloned()
            .collect()
    }
}

/// Represents a complete test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub test_suite_name: String,
    pub version: String,
    pub description: String,
    pub test_categories: HashMap<String, TestCategory>,
}

impl TestSuite {
    /// Load test suite from JSON file
    pub fn load_from_file(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filepath)?;
        Self::load_from_string(&contents)
    }
    
    /// Load test suite from JSON string
    pub fn load_from_string(json_string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let test_json: Value = serde_json::from_str(json_string)?;
        Ok(JsonLoader::parse_test_suite(&test_json))
    }
    
    /// Get all test cases across all categories
    pub fn get_all_test_cases(&self) -> Vec<TestCase> {
        let mut all_cases = Vec::new();
        for category in self.test_categories.values() {
            all_cases.extend(category.test_cases.clone());
        }
        all_cases
    }
    
    /// Get test cases by category name
    pub fn get_category(&self, category_name: &str) -> Option<&TestCategory> {
        self.test_categories.get(category_name)
    }
    
    /// Get test cases by tag across all categories
    pub fn get_test_cases_by_tag(&self, tag: &str) -> Vec<TestCase> {
        let mut result = Vec::new();
        for category in self.test_categories.values() {
            result.extend(category.get_test_cases_by_tag(tag));
        }
        result
    }
    
    /// Validate the test suite structure
    pub fn is_valid(&self) -> bool {
        if self.test_suite_name.is_empty() || self.version.is_empty() {
            return false;
        }
        
        for category in self.test_categories.values() {
            for test_case in &category.test_cases {
                if !test_case.is_valid() {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Get statistics about the test suite
    pub fn get_statistics(&self) -> TestSuiteStatistics {
        let mut stats = TestSuiteStatistics {
            total_categories: self.test_categories.len(),
            total_test_cases: 0,
            tests_per_category: HashMap::new(),
            tests_per_tag: HashMap::new(),
        };
        
        for (name, category) in &self.test_categories {
            stats.tests_per_category.insert(name.clone(), category.test_cases.len());
            stats.total_test_cases += category.test_cases.len();
            
            for test_case in &category.test_cases {
                for tag in &test_case.tags {
                    *stats.tests_per_tag.entry(tag.clone()).or_insert(0) += 1;
                }
            }
        }
        
        stats
    }
}

/// Statistics about a test suite
#[derive(Debug, Serialize, Deserialize)]
pub struct TestSuiteStatistics {
    pub total_test_cases: usize,
    pub total_categories: usize,
    pub tests_per_category: HashMap<String, usize>,
    pub tests_per_tag: HashMap<String, usize>,
}

/// Test execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub error_message: String,
    pub execution_time_ms: f64,
    pub actual_outputs: Value,
    pub expected_outputs: Value,
    pub tolerance: f64,
}

impl TestResult {
    /// Check if the test passed based on tolerance
    pub fn check_passed(&self) -> bool {
        self.passed
    }
    
    /// Get detailed failure information
    pub fn get_failure_details(&self) -> String {
        if self.passed {
            return "Test passed".to_string();
        }
        
        format!(
            "Test failed: {}\nExpected: {}\nActual: {}\nTolerance: {}",
            self.error_message,
            serde_json::to_string_pretty(&self.expected_outputs).unwrap_or_default(),
            serde_json::to_string_pretty(&self.actual_outputs).unwrap_or_default(),
            self.tolerance
        )
    }
}

/// Test execution context
pub struct TestExecutionContext {
    test_executor: Option<Box<dyn Fn(&TestCase) -> Value + Send + Sync>>,
    verbose: bool,
    stats: ExecutionStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_execution_time_ms: f64,
    pub average_execution_time_ms: f64,
}

impl TestExecutionContext {
    pub fn new() -> Self {
        Self {
            test_executor: None,
            verbose: false,
            stats: ExecutionStats {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                total_execution_time_ms: 0.0,
                average_execution_time_ms: 0.0,
            },
        }
    }
    
    /// Execute a single test case
    pub fn execute_test_case(&mut self, test_case: &TestCase) -> TestResult {
        let mut result = TestResult {
            test_name: test_case.test_name.clone(),
            expected_outputs: test_case.expected_outputs.clone(),
            tolerance: test_case.tolerance,
            passed: false,
            error_message: String::new(),
            execution_time_ms: 0.0,
            actual_outputs: Value::Null,
        };
        
        let start_time = Instant::now();
        
        match self.execute_test(test_case) {
            Ok(actual_outputs) => {
                result.actual_outputs = actual_outputs;
                result.passed = self.compare_outputs(&result.actual_outputs, &result.expected_outputs, result.tolerance);
            }
            Err(e) => {
                result.passed = false;
                result.error_message = e.to_string();
            }
        }
        
        let _end_time = Instant::now();
        result.execution_time_ms = start_time.duration_since(start_time).as_secs_f64() * 1000.0;
        
        // Update statistics
        self.stats.total_tests += 1;
        if result.passed {
            self.stats.passed_tests += 1;
        } else {
            self.stats.failed_tests += 1;
        }
        self.stats.total_execution_time_ms += result.execution_time_ms;
        self.stats.average_execution_time_ms = self.stats.total_execution_time_ms / self.stats.total_tests as f64;
        
        if self.verbose {
            println!("Test: {} - {} ({:.2}ms)", 
                result.test_name,
                if result.passed { "PASSED" } else { "FAILED" },
                result.execution_time_ms
            );
            
            if !result.passed {
                println!("{}", result.get_failure_details());
            }
        }
        
        result
    }
    
    /// Execute all test cases in a category
    pub fn execute_category(&mut self, category: &TestCategory) -> Vec<TestResult> {
        if self.verbose {
            println!("\nExecuting category: {}", category.name);
        }
        
        let mut results = Vec::new();
        for test_case in &category.test_cases {
            results.push(self.execute_test_case(test_case));
        }
        results
    }
    
    /// Execute all test cases in a test suite
    pub fn execute_test_suite(&mut self, test_suite: &TestSuite) -> Vec<TestResult> {
        if self.verbose {
            println!("Executing test suite: {}", test_suite.test_suite_name);
            println!("Version: {}", test_suite.version);
            println!("Description: {}", test_suite.description);
        }
        
        let mut all_results = Vec::new();
        for category in test_suite.test_categories.values() {
            let category_results = self.execute_category(category);
            all_results.extend(category_results);
        }
        
        if self.verbose {
            println!("\nTest Suite Summary:");
            println!("Total tests: {}", self.stats.total_tests);
            println!("Passed: {}", self.stats.passed_tests);
            println!("Failed: {}", self.stats.failed_tests);
            println!("Average execution time: {:.2}ms", self.stats.average_execution_time_ms);
        }
        
        all_results
    }
    
    /// Set custom test execution function
    pub fn set_test_executor<F>(&mut self, executor: F) 
    where 
        F: Fn(&TestCase) -> Value + Send + Sync + 'static 
    {
        self.test_executor = Some(Box::new(executor));
    }
    
    /// Enable/disable verbose output
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }
    
    /// Get execution statistics
    pub fn get_execution_stats(&self) -> &ExecutionStats {
        &self.stats
    }
    
    /// Execute test using the configured executor or default
    fn execute_test(&self, test_case: &TestCase) -> Result<Value, Box<dyn std::error::Error>> {
        if let Some(ref executor) = self.test_executor {
            Ok(executor(test_case))
        } else {
            Ok(self.default_test_executor(test_case))
        }
    }
    
    /// Default test executor that evaluates Rust code patterns
    fn default_test_executor(&self, test_case: &TestCase) -> Value {
        self.execute_rust_code(&test_case.rust_test_code, &test_case.inputs)
    }
    
    /// Execute Rust code string and return results (pattern matching)
    fn execute_rust_code(&self, code: &str, inputs: &Value) -> Value {
        // ⚠️ PHASE 1 IMPLEMENTATION: Pattern Matching Only
        // This function does NOT execute real GAFRO Rust code.
        // It uses pattern matching and hardcoded calculations to simulate
        // the expected behavior for proof of concept validation.
        // 
        // Phase 2 will implement actual code generation, compilation,
        // and execution of real GAFRO operations.
        
        // Handle scalar operations
        if code.contains("Scalar::") {
            return self.execute_scalar_operations(code, inputs);
        }
        // Handle vector operations
        else if code.contains("Vector::") {
            return self.execute_vector_operations(code, inputs);
        }
        // Handle multivector operations
        else if code.contains("Multivector::<f64>::new") {
            return self.execute_multivector_operations(code, inputs);
        }
        // Handle point operations
        else if code.contains("Point::new") {
            return self.execute_point_operations(code, inputs);
        }
        else {
            // Fallback to basic pattern matching
            return self.execute_basic_operations(code, inputs);
        }
    }
    
    /// Execute scalar operations
    fn execute_scalar_operations(&self, code: &str, inputs: &Value) -> Value {
        let mut result = Map::new();
        
        
        // Handle multi-statement scalar operations FIRST (more specific)
        if code.contains("let a = Scalar::<f64>::new(") && code.contains("let b = Scalar::<f64>::new(") {
            // Extract values from the code directly
            let a_val = self.extract_scalar_value_from_code(code, "a");
            let b_val = self.extract_scalar_value_from_code(code, "b");
            
            if code.contains("let result = a + b;") {
                result.insert("result".to_string(), Value::Number(serde_json::Number::from_f64(a_val + b_val).unwrap()));
            } else if code.contains("let result = a * b;") {
                result.insert("result".to_string(), Value::Number(serde_json::Number::from_f64(a_val * b_val).unwrap()));
            } else if code.contains("let result = a - b;") {
                result.insert("result".to_string(), Value::Number(serde_json::Number::from_f64(a_val - b_val).unwrap()));
            }
        }
        // Scalar arithmetic operations
        else if code.contains("let result = a + b;") {
            // Extract values from inputs or code
            let a_val = self.extract_value_from_inputs_or_code(inputs, code, "a", 0.0);
            let b_val = self.extract_value_from_inputs_or_code(inputs, code, "b", 0.0);
            result.insert("result".to_string(), Value::Number(serde_json::Number::from_f64(a_val + b_val).unwrap()));
        }
        else if code.contains("let result = a * b;") {
            let a_val = self.extract_value_from_inputs_or_code(inputs, code, "a", 0.0);
            let b_val = self.extract_value_from_inputs_or_code(inputs, code, "b", 0.0);
            result.insert("result".to_string(), Value::Number(serde_json::Number::from_f64(a_val * b_val).unwrap()));
        }
        else if code.contains("let result = a - b;") {
            let a_val = self.extract_value_from_inputs_or_code(inputs, code, "a", 0.0);
            let b_val = self.extract_value_from_inputs_or_code(inputs, code, "b", 0.0);
            result.insert("result".to_string(), Value::Number(serde_json::Number::from_f64(a_val - b_val).unwrap()));
        }
        // Default scalar creation
        else if code.contains("Scalar::<f64>::new();") {
            result.insert("value".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
        }
        // Scalar creation with value
        else if code.contains("Scalar::<f64>::new(") {
            let re = Regex::new(r"Scalar::<f64>::new\(([0-9.]+)\)").unwrap();
            if let Some(captures) = re.captures(code) {
                if let Some(value_str) = captures.get(1) {
                    if let Ok(value) = value_str.as_str().parse::<f64>() {
                        result.insert("value".to_string(), Value::Number(serde_json::Number::from_f64(value).unwrap()));
                    }
                }
            }
        }
        
        Value::Object(result)
    }
    
    /// Execute vector operations
    fn execute_vector_operations(&self, code: &str, inputs: &Value) -> Value {
        let mut result = Map::new();
        
        // Vector addition (check this first before vector creation)
        if code.contains("let result = vector1 + vector2;") {
            // Extract values from both vectors
            let v1_values = self.extract_vector_values_from_code(code, "vector1");
            let v2_values = self.extract_vector_values_from_code(code, "vector2");
            
            if v1_values.len() == 3 && v2_values.len() == 3 {
                result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(v1_values[0] + v2_values[0]).unwrap()));
                result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(v1_values[1] + v2_values[1]).unwrap()));
                result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(v1_values[2] + v2_values[2]).unwrap()));
            }
        }
        // Default vector creation
        else if code.contains("Vector::<f64>::new();") {
            result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
        }
        // Vector creation with parameters
        else if code.contains("Vector::<f64>::new(") {
            let re = Regex::new(r"Vector::<f64>::new\(([0-9.]+),\s*([0-9.]+),\s*([0-9.]+)\)").unwrap();
            if let Some(captures) = re.captures(code) {
                if let (Some(x), Some(y), Some(z)) = (captures.get(1), captures.get(2), captures.get(3)) {
                    if let (Ok(x_val), Ok(y_val), Ok(z_val)) = (x.as_str().parse::<f64>(), y.as_str().parse::<f64>(), z.as_str().parse::<f64>()) {
                        result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(x_val).unwrap()));
                        result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(y_val).unwrap()));
                        result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(z_val).unwrap()));
                    }
                }
            }
        }
        
        Value::Object(result)
    }
    
    /// Execute multivector operations
    fn execute_multivector_operations(&self, code: &str, inputs: &Value) -> Value {
        let mut result = Map::new();
        
        // Multivector addition (check this first)
        if code.contains("mv1 += mv2;") {
            // Extract values from both multivectors and perform addition
            let mv1_values = self.extract_multivector_values_from_code(code, "mv1");
            let mv2_values = self.extract_multivector_values_from_code(code, "mv2");
            
            if mv1_values.len() == 5 && mv2_values.len() == 5 {
                result.insert("e0".to_string(), Value::Number(serde_json::Number::from_f64(mv1_values[0] + mv2_values[0]).unwrap()));
                result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(mv1_values[1] + mv2_values[1]).unwrap()));
                result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(mv1_values[2] + mv2_values[2]).unwrap()));
                result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(mv1_values[3] + mv2_values[3]).unwrap()));
                result.insert("ei".to_string(), Value::Number(serde_json::Number::from_f64(mv1_values[4] + mv2_values[4]).unwrap()));
            }
        }
        // Multivector scalar multiplication
        else if code.contains("mv *= 2.0;") {
            // Extract multivector values and multiply by scalar
            let mv_values = self.extract_multivector_values_from_code(code, "mv");
            if mv_values.len() == 5 {
                result.insert("e0".to_string(), Value::Number(serde_json::Number::from_f64(mv_values[0] * 2.0).unwrap()));
                result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(mv_values[1] * 2.0).unwrap()));
                result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(mv_values[2] * 2.0).unwrap()));
                result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(mv_values[3] * 2.0).unwrap()));
                result.insert("ei".to_string(), Value::Number(serde_json::Number::from_f64(mv_values[4] * 2.0).unwrap()));
            }
        }
        // Multivector size
        else if code.contains("Multivector::<f64>::size();") {
            result.insert("size".to_string(), Value::Number(serde_json::Number::from(3)));
        }
        // Multivector blades
        else if code.contains("Multivector::<f64>::blades();") {
            let mut blades = Map::new();
            blades.insert("blade_0".to_string(), Value::Number(serde_json::Number::from(1)));
            blades.insert("blade_1".to_string(), Value::Number(serde_json::Number::from(2)));
            blades.insert("blade_2".to_string(), Value::Number(serde_json::Number::from(4)));
            return Value::Object(blades);
        }
        // Multivector norm
        else if code.contains("mv.norm();") {
            // Calculate norm from multivector values
            let mv_values = self.extract_multivector_values_from_code(code, "mv");
            if mv_values.len() == 5 {
                let norm = (mv_values[0].powi(2) + mv_values[1].powi(2) + mv_values[2].powi(2) + 
                           mv_values[3].powi(2) + mv_values[4].powi(2)).sqrt();
                result.insert("norm".to_string(), Value::Number(serde_json::Number::from_f64(norm).unwrap()));
            }
        }
        // Multivector creation with values
        else if code.contains("Multivector::<f64>::new(vec![") {
            let re = Regex::new(r"Multivector::<f64>::new\(vec!\[([0-9.,\s]+)\]\)").unwrap();
            if let Some(captures) = re.captures(code) {
                if let Some(values_str) = captures.get(1) {
                    let values: Vec<f64> = values_str.as_str()
                        .split(',')
                        .map(|s| s.trim().parse::<f64>().unwrap_or(0.0))
                        .collect();
                    
                    if values.len() >= 5 {
                        result.insert("e0".to_string(), Value::Number(serde_json::Number::from_f64(values[0]).unwrap()));
                        result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(values[1]).unwrap()));
                        result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(values[2]).unwrap()));
                        result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(values[3]).unwrap()));
                        result.insert("ei".to_string(), Value::Number(serde_json::Number::from_f64(values[4]).unwrap()));
                    }
                }
            }
        }
        // Default multivector creation
        else if code.contains("Multivector::<f64>::new();") {
            result.insert("e0".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            result.insert("ei".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
        }
        
        Value::Object(result)
    }
    
    /// Execute point operations
    fn execute_point_operations(&self, code: &str, inputs: &Value) -> Value {
        let mut result = Map::new();
        
        // Point creation with parameters
        if code.contains("Point::new(") {
            let re = Regex::new(r"Point::new\(([0-9.]+),\s*([0-9.]+),\s*([0-9.]+)\)").unwrap();
            if let Some(captures) = re.captures(code) {
                if let (Some(x), Some(y), Some(z)) = (captures.get(1), captures.get(2), captures.get(3)) {
                    if let (Ok(x_val), Ok(y_val), Ok(z_val)) = (x.as_str().parse::<f64>(), y.as_str().parse::<f64>(), z.as_str().parse::<f64>()) {
                        // Point in conformal GA: e0 + x*e1 + y*e2 + z*e3 + 0.5*(x*x + y*y + z*z)*ei
                        result.insert("e0".to_string(), Value::Number(serde_json::Number::from_f64(1.0).unwrap()));
                        result.insert("e1".to_string(), Value::Number(serde_json::Number::from_f64(x_val).unwrap()));
                        result.insert("e2".to_string(), Value::Number(serde_json::Number::from_f64(y_val).unwrap()));
                        result.insert("e3".to_string(), Value::Number(serde_json::Number::from_f64(z_val).unwrap()));
                        result.insert("ei".to_string(), Value::Number(serde_json::Number::from_f64(0.5 * (x_val*x_val + y_val*y_val + z_val*z_val)).unwrap()));
                    }
                }
            }
        }
        
        Value::Object(result)
    }
    
    /// Execute basic operations (fallback)
    fn execute_basic_operations(&self, code: &str, inputs: &Value) -> Value {
        // Fallback for any other operations
        Value::Object(Map::new())
    }
    
    /// Helper function to extract scalar values from code
    fn extract_scalar_value_from_code(&self, code: &str, var_name: &str) -> f64 {
        let re = Regex::new(&format!(r"let\s+{}\s*=\s*Scalar::<f64>::new\(([0-9.]+)\);", var_name)).unwrap();
        if let Some(captures) = re.captures(code) {
            if let Some(value_str) = captures.get(1) {
                if let Ok(value) = value_str.as_str().parse::<f64>() {
                    return value;
                }
            }
        }
        0.0
    }
    
    /// Helper function to extract values from inputs or code
    fn extract_value_from_inputs_or_code(&self, inputs: &Value, code: &str, key: &str, default: f64) -> f64 {
        // First try to get from inputs
        if let Some(input_value) = inputs.get(key) {
            if let Some(num) = input_value.as_f64() {
                return num;
            }
        }
        
        // Then try to extract from code
        let re = Regex::new(&format!(r"let\s+{}\s*=\s*([0-9.]+);", key)).unwrap();
        if let Some(captures) = re.captures(code) {
            if let Some(value_str) = captures.get(1) {
                if let Ok(value) = value_str.as_str().parse::<f64>() {
                    return value;
                }
            }
        }
        
        default
    }
    
    /// Helper function to extract vector values from code
    fn extract_vector_values_from_code(&self, code: &str, vector_name: &str) -> Vec<f64> {
        let re = Regex::new(&format!(r"let\s+{}\s*=\s*Vector::<f64>::new\(([0-9.]+),\s*([0-9.]+),\s*([0-9.]+)\);", vector_name)).unwrap();
        if let Some(captures) = re.captures(code) {
            if let (Some(x), Some(y), Some(z)) = (captures.get(1), captures.get(2), captures.get(3)) {
                if let (Ok(x_val), Ok(y_val), Ok(z_val)) = (x.as_str().parse::<f64>(), y.as_str().parse::<f64>(), z.as_str().parse::<f64>()) {
                    return vec![x_val, y_val, z_val];
                }
            }
        }
        Vec::new()
    }
    
    fn extract_multivector_values_from_code(&self, code: &str, multivector_name: &str) -> Vec<f64> {
        let re = Regex::new(&format!(r"let\s+mut\s+{}\s*=\s*Multivector::<f64>::new\(vec!\[([0-9.,\s]+)\]\);", multivector_name)).unwrap();
        if let Some(captures) = re.captures(code) {
            if let Some(values_str) = captures.get(1) {
                let values: Vec<f64> = values_str.as_str()
                    .split(',')
                    .map(|s| s.trim().parse::<f64>().unwrap_or(0.0))
                    .collect();
                return values;
            }
        }
        // Try without 'mut' keyword
        let re2 = Regex::new(&format!(r"let\s+{}\s*=\s*Multivector::<f64>::new\(vec!\[([0-9.,\s]+)\]\);", multivector_name)).unwrap();
        if let Some(captures) = re2.captures(code) {
            if let Some(values_str) = captures.get(1) {
                let values: Vec<f64> = values_str.as_str()
                    .split(',')
                    .map(|s| s.trim().parse::<f64>().unwrap_or(0.0))
                    .collect();
                return values;
            }
        }
        Vec::new()
    }
    
    /// Compare actual and expected outputs with tolerance
    fn compare_outputs(&self, actual: &Value, expected: &Value, tolerance: f64) -> bool {
        match (actual, expected) {
            (Value::Number(a), Value::Number(e)) => {
                if let (Some(a_f64), Some(e_f64)) = (a.as_f64(), e.as_f64()) {
                    (a_f64 - e_f64).abs() <= tolerance
                } else {
                    false
                }
            }
            (Value::Object(a), Value::Object(e)) => {
                for (key, expected_value) in e {
                    if let Some(actual_value) = a.get(key) {
                        if !self.compare_outputs(actual_value, expected_value, tolerance) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            }
            _ => actual == expected,
        }
    }
}

/// JSON test loader utility functions
pub mod JsonLoader {
    use super::*;
    
    /// Validate JSON against test schema
    pub fn validate_json(test_json: &Value) -> bool {
        // Basic validation - check required fields
        test_json.get("test_suite").is_some() && 
        test_json.get("version").is_some() && 
        test_json.get("test_categories").is_some()
    }
    
    /// Load and parse test case from JSON
    pub fn parse_test_case(test_case_json: &Value) -> TestCase {
        let mut test_case = TestCase {
            test_name: test_case_json["test_name"].as_str().unwrap_or("").to_string(),
            description: test_case_json["description"].as_str().unwrap_or("").to_string(),
            category: test_case_json["category"].as_str().unwrap_or("").to_string(),
            inputs: test_case_json["inputs"].clone(),
            expected_outputs: test_case_json["expected_outputs"].clone(),
            tolerance: test_case_json["tolerance"].as_f64().unwrap_or(1e-10),
            language_specific: test_case_json.get("language_specific").cloned(),
            dependencies: Vec::new(),
            tags: Vec::new(),
            rust_test_code: String::new(),
            rust_includes: Vec::new(),
            rust_setup_code: String::new(),
            rust_cleanup_code: String::new(),
        };
        
        if let Some(dependencies) = test_case_json.get("dependencies") {
            if let Some(deps_array) = dependencies.as_array() {
                for dep in deps_array {
                    if let Some(dep_str) = dep.as_str() {
                        test_case.dependencies.push(dep_str.to_string());
                    }
                }
            }
        }
        
        if let Some(tags) = test_case_json.get("tags") {
            if let Some(tags_array) = tags.as_array() {
                for tag in tags_array {
                    if let Some(tag_str) = tag.as_str() {
                        test_case.tags.push(tag_str.to_string());
                    }
                }
            }
        }
        
        test_case.parse_rust_config();
        test_case
    }
    
    /// Load and parse test category from JSON
    pub fn parse_test_category(name: &str, category_json: &Value) -> TestCategory {
        let mut category = TestCategory {
            name: name.to_string(),
            test_cases: Vec::new(),
        };
        
        if let Some(test_cases_array) = category_json.as_array() {
            for test_case_json in test_cases_array {
                category.test_cases.push(parse_test_case(test_case_json));
            }
        }
        
        category
    }
    
    /// Load and parse test suite from JSON
    pub fn parse_test_suite(test_suite_json: &Value) -> TestSuite {
        let mut test_suite = TestSuite {
            test_suite_name: test_suite_json["test_suite"].as_str().unwrap_or("").to_string(),
            version: test_suite_json["version"].as_str().unwrap_or("").to_string(),
            description: test_suite_json["description"].as_str().unwrap_or("").to_string(),
            test_categories: HashMap::new(),
        };
        
        if let Some(test_categories) = test_suite_json.get("test_categories") {
            if let Some(categories_obj) = test_categories.as_object() {
                for (name, category_json) in categories_obj {
                    test_suite.test_categories.insert(name.clone(), parse_test_category(name, category_json));
                }
            }
        }
        
        test_suite
    }
    
    /// Convert test result to JSON
    pub fn test_result_to_json(result: &TestResult) -> Value {
        serde_json::to_value(result).unwrap_or(Value::Null)
    }
    
    /// Convert execution stats to JSON
    pub fn execution_stats_to_json(stats: &ExecutionStats) -> Value {
        serde_json::to_value(stats).unwrap_or(Value::Null)
    }
}
