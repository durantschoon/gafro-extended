// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

/**
 * GAFRO Extended Utilities Library
 * 
 * This module provides convenient access to all utility types and functions
 * for GAFRO Extended development.
 */

// These modules are in the same directory, not in a subdirectory
// pub mod si_quantity;
// pub mod angle;
// pub mod canonical_output;

// Re-export commonly used types and functions
pub use crate::si_quantity::*;
pub use crate::angle::*;
pub use crate::canonical_output::*;

// Convenience re-exports
pub use crate::si_quantity::{
    Dimensionless, Mass, Length, Time, Velocity, Acceleration, 
    Force, Energy, Power, Pressure, Torque
};
