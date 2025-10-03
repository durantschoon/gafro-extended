// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

/**
 * GAFRO Extended Rust Library
 * 
 * This library provides Rust implementations of GAFRO Extended utilities
 * and test infrastructure.
 */

pub mod json_loader;
pub mod test_runner;
pub mod utilities;

// Re-export utilities for easy access
pub use utilities::*;
