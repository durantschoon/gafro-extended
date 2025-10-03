// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

/*!
 * Canonical Output Library for GAFRO Extended (Rust)
 * 
 * This library provides consistent, configurable output formatting
 * for both C++ and Rust implementations to ensure identical output.
 */

// use std::fmt; // Not currently used

// Trait for types that can be printed as positions
pub trait PositionLike {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn frame_name(&self) -> Option<&'static str> { None }
}

/// Configuration for output precision and formatting
#[derive(Debug, Clone)]
pub struct Config {
    pub position_precision: usize,
    pub angle_precision: usize,
    pub distance_precision: usize,
    pub time_precision: usize,
    pub speed_precision: usize,
    pub scientific_threshold: f64,
    pub use_tau_convention: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            position_precision: Self::get_env_precision("GAFRO_POSITION_PRECISION", 1),
            angle_precision: Self::get_env_precision("GAFRO_ANGLE_PRECISION", 2),
            distance_precision: Self::get_env_precision("GAFRO_DISTANCE_PRECISION", 1),
            time_precision: Self::get_env_precision("GAFRO_TIME_PRECISION", 1),
            speed_precision: Self::get_env_precision("GAFRO_SPEED_PRECISION", 2),
            scientific_threshold: Self::get_env_float("GAFRO_SCIENTIFIC_THRESHOLD", 100.0),
            use_tau_convention: Self::get_env_bool("GAFRO_USE_TAU", true),
        }
    }
}

impl Config {
    /// Get precision from environment variable with fallback
    fn get_env_precision(env_var: &str, default: usize) -> usize {
        std::env::var(env_var)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }
    
    /// Get float from environment variable with fallback
    fn get_env_float(env_var: &str, default: f64) -> f64 {
        std::env::var(env_var)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }
    
    /// Get boolean from environment variable with fallback
    fn get_env_bool(env_var: &str, default: bool) -> bool {
        std::env::var(env_var)
            .ok()
            .and_then(|v| match v.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" => Some(false),
                _ => None,
            })
            .unwrap_or(default)
    }
}

/// Canonical output formatter for consistent cross-language output
pub struct CanonicalOutput {
    config: Config,
}

impl CanonicalOutput {
    /// Create a new canonical output formatter with default config
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    
    /// Create a new canonical output formatter with custom config
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }
    
    /// Get mutable reference to config for runtime changes
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
    
    /// Get immutable reference to config
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Format a 3D position
    pub fn position(&self, x: f64, y: f64, z: f64) -> String {
        format!(
            "({:.precision$}, {:.precision$}, {:.precision$})",
            x, y, z,
            precision = self.config.position_precision
        )
    }
    
    /// Format a distance with unit
    pub fn distance(&self, value: f64, unit: &str) -> String {
        if value.abs() >= self.config.scientific_threshold {
            format!("{:.precision$e} {}", value, unit, precision = self.config.distance_precision)
        } else {
            format!("{:.precision$} {}", value, unit, precision = self.config.distance_precision)
        }
    }
    
    /// Format an angle in degrees
    pub fn angle_degrees(&self, degrees: f64) -> String {
        format!("{:.precision$}°", degrees, precision = self.config.angle_precision)
    }
    
    /// Format an angle in tau fractions
    pub fn angle_tau(&self, tau_fraction: f64) -> String {
        format!("{:.precision$}τ", tau_fraction, precision = self.config.angle_precision)
    }
    
    /// Format an angle with both degrees and tau
    pub fn angle_combined(&self, degrees: f64, tau_fraction: f64) -> String {
        format!("{} ({})", self.angle_degrees(degrees), self.angle_tau(tau_fraction))
    }
    
    /// Format time with unit
    pub fn time(&self, value: f64, unit: &str) -> String {
        format!("{:.precision$} {}", value, unit, precision = self.config.time_precision)
    }
    
    /// Format speed with unit
    pub fn speed(&self, value: f64, unit: &str) -> String {
        format!("{:.precision$} {}", value, unit, precision = self.config.speed_precision)
    }
    
    /// Format in scientific notation
    pub fn scientific(&self, value: f64, precision: usize) -> String {
        format!("{:.precision$e}", value, precision = precision)
    }
    
    /// Format a section header
    pub fn section_header(&self, title: &str) -> String {
        format!("\n{}\n{}", title, "=".repeat(title.len()))
    }
    
    /// Tau constant value
    pub const TAU: f64 = 6.283185307179586; // 2π
    
    /// Convert degrees to tau fraction
    pub fn degrees_to_tau(&self, degrees: f64) -> f64 {
        degrees / 360.0
    }
    
    /// Convert tau fraction to degrees
    pub fn tau_to_degrees(&self, tau_fraction: f64) -> f64 {
        tau_fraction * 360.0 / Self::TAU
    }
    
    /// Format tau constant
    pub fn tau_constant(&self) -> String {
        format!("τ (tau = 2π) = {:.5}", Self::TAU)
    }
    
    /// Print utilities that ensure consistent formatting
    pub fn print_position(&self, label: &str, x: f64, y: f64, z: f64, frame: Option<&str>) {
        print!("✓ {}: {}", label, self.position(x, y, z));
        if let Some(frame) = frame {
            print!(" [{} frame]", frame);
        }
        println!();
    }
    
    /// Print a position-like object directly
    pub fn print_position_like<P: PositionLike>(&self, label: &str, pos: &P) {
        let frame = pos.frame_name();
        self.print_position(label, pos.x(), pos.y(), pos.z(), frame);
    }
    
    pub fn print_distance(&self, label: &str, value: f64, unit: &str) {
        println!("✓ {}: {}", label, self.distance(value, unit));
    }
    
    pub fn print_angle(&self, label: &str, degrees: f64) {
        if self.config.use_tau_convention {
            let tau_fraction = self.degrees_to_tau(degrees);
            println!("✓ {}: {}", label, self.angle_combined(degrees, tau_fraction));
        } else {
            println!("✓ {}: {}", label, self.angle_degrees(degrees));
        }
    }
    
    pub fn print_speed(&self, label: &str, value: f64) {
        println!("✓ {}: {}", label, self.speed(value, "m/s"));
    }
    
    pub fn print_time(&self, label: &str, value: f64) {
        println!("✓ {}: {}", label, self.time(value, "s"));
    }
    
    pub fn print_success(&self, message: &str) {
        println!("✅ {}", message);
    }
    
    pub fn print_error(&self, message: &str) {
        println!("❌ {}", message);
    }
    
    pub fn print_warning(&self, message: &str) {
        println!("🚫 {}", message);
    }
    
    /// Format a list item
    pub fn list_item(&self, index: usize, content: &str) -> String {
        format!("  {}. {}", index, content)
    }
    
    /// Set precision for different types
    pub fn set_precision(&mut self, position: usize, angle: usize, distance: usize, time: usize, speed: usize) {
        self.config.position_precision = position;
        self.config.angle_precision = angle;
        self.config.distance_precision = distance;
        self.config.time_precision = time;
        self.config.speed_precision = speed;
    }
    
    /// Set scientific notation threshold
    pub fn set_scientific_threshold(&mut self, threshold: f64) {
        self.config.scientific_threshold = threshold;
    }
    
    /// Set tau convention usage
    pub fn set_tau_convention(&mut self, use_tau: bool) {
        self.config.use_tau_convention = use_tau;
    }
}

impl Default for CanonicalOutput {
    fn default() -> Self {
        Self::new()
    }
}

/// Global canonical output instance for convenience
pub static mut GLOBAL_OUTPUT: Option<CanonicalOutput> = None;

/// Initialize global output with default config
pub fn init_global_output() {
    unsafe {
        GLOBAL_OUTPUT = Some(CanonicalOutput::new());
    }
}

/// Initialize global output with custom config
pub fn init_global_output_with_config(config: Config) {
    unsafe {
        GLOBAL_OUTPUT = Some(CanonicalOutput::with_config(config));
    }
}

/// Get global output instance (panics if not initialized)
pub fn global_output() -> &'static CanonicalOutput {
    unsafe {
        GLOBAL_OUTPUT.as_ref().expect("Global output not initialized. Call init_global_output() first.")
    }
}

/// Get mutable global output instance (panics if not initialized)
pub fn global_output_mut() -> &'static mut CanonicalOutput {
    unsafe {
        GLOBAL_OUTPUT.as_mut().expect("Global output not initialized. Call init_global_output() first.")
    }
}

/// Convenience macros for global output
#[macro_export]
macro_rules! print_position {
    ($label:expr, $x:expr, $y:expr, $z:expr) => {
        $crate::canonical_output::global_output().print_position($label, $x, $y, $z, None);
    };
    ($label:expr, $x:expr, $y:expr, $z:expr, $frame:expr) => {
        $crate::canonical_output::global_output().print_position($label, $x, $y, $z, Some($frame));
    };
}

#[macro_export]
macro_rules! print_distance {
    ($label:expr, $value:expr) => {
        $crate::canonical_output::global_output().print_distance($label, $value, "m");
    };
    ($label:expr, $value:expr, $unit:expr) => {
        $crate::canonical_output::global_output().print_distance($label, $value, $unit);
    };
}

#[macro_export]
macro_rules! print_angle {
    ($label:expr, $degrees:expr) => {
        $crate::canonical_output::global_output().print_angle($label, $degrees);
    };
}

#[macro_export]
macro_rules! print_speed {
    ($label:expr, $value:expr) => {
        $crate::canonical_output::global_output().print_speed($label, $value);
    };
}

#[macro_export]
macro_rules! print_time {
    ($label:expr, $value:expr) => {
        $crate::canonical_output::global_output().print_time($label, $value);
    };
}

#[macro_export]
macro_rules! print_success {
    ($msg:expr) => {
        $crate::canonical_output::global_output().print_success($msg);
    };
}

#[macro_export]
macro_rules! print_error {
    ($msg:expr) => {
        $crate::canonical_output::global_output().print_error($msg);
    };
}

#[macro_export]
macro_rules! print_warning {
    ($msg:expr) => {
        $crate::canonical_output::global_output().print_warning($msg);
    };
}
