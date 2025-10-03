// SPDX-FileCopyrightText: GAFRO Extended Implementation
//
// SPDX-License-Identifier: MPL-2.0

/*!
 * Autonomous robot navigation demonstrating Phase 2 type safety benefits (Rust)
 *
 * This demo shows how Phase 2 Modern Types prevent common navigation errors:
 * - Mixed coordinate frame operations
 * - Unit confusion in sensor fusion
 * - Angle convention mistakes in path planning
 * - Time/distance calculation errors
 */

// Import our utilities library
use std::f64::consts::PI;

// === Mathematical Constants ===
const TAU: f64 = 2.0 * PI; // τ = 2π

// Import canonical output for consistent formatting
use gafro_test_runner::canonical_output::{CanonicalOutput, PositionLike};

// === Type-Safe Coordinate Systems ===
trait Frame {
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy)]
struct WorldFrame;
impl Frame for WorldFrame {
    const NAME: &'static str = "world";
}

#[derive(Debug, Clone, Copy)]
struct RobotFrame;
impl Frame for RobotFrame {
    const NAME: &'static str = "robot";
}

#[derive(Debug, Clone, Copy)]
struct SensorFrame;
impl Frame for SensorFrame {
    const NAME: &'static str = "sensor";
}

// Allow dead code for demo purposes
#[allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct TypedPosition<F: Frame> {
    x: f64,
    y: f64,
    z: f64,
    _phantom: std::marker::PhantomData<F>,
}

impl<F: Frame> TypedPosition<F> {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            _phantom: std::marker::PhantomData,
        }
    }

    fn frame_name() -> &'static str {
        F::NAME
    }

    fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// Implement PositionLike trait for TypedPosition
impl<F: Frame> PositionLike for TypedPosition<F> {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    fn z(&self) -> f64 { self.z }
    fn frame_name(&self) -> Option<&'static str> { Some(F::NAME) }
}

impl<F: Frame> std::ops::Add for TypedPosition<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<F: Frame> std::ops::Sub for TypedPosition<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

type WorldPosition = TypedPosition<WorldFrame>;
type RobotPosition = TypedPosition<RobotFrame>;
type SensorPosition = TypedPosition<SensorFrame>;

// === Type-Safe SI Units ===
#[derive(Debug, Clone, Copy)]
struct SIQuantity<const M: i32, const L: i32, const T: i32> {
    value: f64,
}

impl<const M: i32, const L: i32, const T: i32> SIQuantity<M, L, T> {
    fn new(value: f64) -> Self {
        Self { value }
    }

    #[allow(dead_code)]
    fn mass_dim() -> i32 { M }
    #[allow(dead_code)]
    fn length_dim() -> i32 { L }
    #[allow(dead_code)]
    fn time_dim() -> i32 { T }
}

impl<const M: i32, const L: i32, const T: i32> std::ops::Add for SIQuantity<M, L, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.value + other.value)
    }
}

impl<const M: i32, const L: i32, const T: i32> std::ops::Sub for SIQuantity<M, L, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.value - other.value)
    }
}

// Simple multiplication and division implementations for demo
impl<const M: i32, const L: i32, const T: i32> std::ops::Mul<f64> for SIQuantity<M, L, T> {
    type Output = Self;
    
    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

impl<const M: i32, const L: i32, const T: i32> std::ops::Div<f64> for SIQuantity<M, L, T> {
    type Output = Self;
    
    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}

// For this demo, we'll implement specific operations needed
impl std::ops::Div<Time> for Length {
    type Output = Velocity;
    
    fn div(self, time: Time) -> Self::Output {
        Velocity::new(self.value / time.value)
    }
}

impl std::ops::Div<Velocity> for Length {
    type Output = Time;
    
    fn div(self, velocity: Velocity) -> Self::Output {
        Time::new(self.value / velocity.value)
    }
}

impl std::ops::Mul<Time> for AngularVelocity {
    type Output = Angle;
    
    fn mul(self, time: Time) -> Self::Output {
        Angle::new(self.value * time.value)
    }
}

type Length = SIQuantity<0, 1, 0>;
type Time = SIQuantity<0, 0, 1>;
type Velocity = SIQuantity<0, 1, -1>;
type AngularVelocity = SIQuantity<0, 0, -1>;

// Unit constructors
fn meters(v: f64) -> Length {
    Length::new(v)
}

fn seconds(v: f64) -> Time {
    Time::new(v)
}

fn meters_per_second(v: f64) -> Velocity {
    Velocity::new(v)
}

fn radians_per_second(v: f64) -> AngularVelocity {
    AngularVelocity::new(v)
}

// === Type-Safe Angles with Tau Convention ===
#[derive(Debug, Clone, Copy)]
struct Angle {
    radians: f64,
}

impl Angle {
    fn new(radians: f64) -> Self {
        Self { radians }
    }

    fn from_tau_fraction(fraction: f64) -> Self {
        Self::new(fraction * TAU)
    }

    fn from_degrees(degrees: f64) -> Self {
        Self::new(degrees * TAU / 360.0)
    }

    fn to_degrees(&self) -> f64 {
        self.radians * 360.0 / TAU
    }

    #[allow(dead_code)]
    fn to_tau_fraction(&self) -> f64 {
        self.radians / TAU
    }

    fn normalized(&self) -> Self {
        let norm = self.radians % TAU;
        Self::new(if norm < 0.0 { norm + TAU } else { norm })
    }
}

impl std::ops::Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.radians + other.radians)
    }
}

impl std::ops::Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.radians - other.radians)
    }
}

// === Autonomous Navigation Controller ===
struct AutonomousNavigationDemo {
    current_position: WorldPosition,
    current_heading: Angle,
    current_speed: Velocity,
}

impl AutonomousNavigationDemo {
    fn new(start_pos: WorldPosition, start_heading: Angle) -> Self {
        Self {
            current_position: start_pos,
            current_heading: start_heading,
            current_speed: meters_per_second(0.0),
        }
    }

    fn print_section(&self, title: &str) {
        println!("\n🤖 {}", title);
        println!("{}", "=".repeat(title.len() + 3));
    }

    fn demonstrate_coordinate_frame_safety(&self, output: &CanonicalOutput) {
        self.print_section("COORDINATE FRAME TYPE SAFETY");

        let world_target = WorldPosition::new(10.0, 5.0, 0.0);
        let robot_sensor_reading = RobotPosition::new(2.0, 1.0, 0.0);

        output.print_position_like("Current position", &self.current_position);
        output.print_position_like("Target position", &world_target);
        output.print_position_like("Sensor reading", &robot_sensor_reading);

        // This WILL compile - same coordinate frame
        let navigation_vector = world_target - self.current_position;
        println!("✅ Navigation vector: {}", output.position(navigation_vector.x, navigation_vector.y, navigation_vector.z));

        // This would NOT compile - different coordinate frames!
        // let invalid_vector = world_target - robot_sensor_reading;  // COMPILE ERROR!
        output.print_warning("Cannot subtract robot frame from world frame (compile-time prevention)");

        println!("Frame safety: {} operations verified", WorldPosition::frame_name());
    }

    fn demonstrate_unit_safety(&mut self, output: &CanonicalOutput) {
        self.print_section("SI UNIT SYSTEM SAFETY");

        let target_distance = meters(8.5);
        let travel_time = seconds(4.0);
        let required_speed = target_distance / travel_time;

        output.print_distance("Target distance", target_distance.value, "m");
        output.print_time("Travel time", travel_time.value);
        output.print_speed("Required speed", required_speed.value);

        // Dimensional analysis verification (compile-time)
        output.print_success(&format!("Speed dimensions: L^{} T^{} (verified at compile time)",
                Velocity::length_dim(), Velocity::time_dim()));

        // This would NOT compile - incompatible dimensions!
        // let invalid = target_distance + travel_time;  // COMPILE ERROR!
        output.print_warning("Cannot add distance to time (compile-time prevention)");

        self.current_speed = required_speed;
    }

    fn demonstrate_tau_angle_safety(&mut self, output: &CanonicalOutput) {
        self.print_section("TAU CONVENTION ANGLE SAFETY");

        let target_heading = Angle::from_degrees(90.0);  // Quarter turn
        let heading_error = target_heading - self.current_heading;

        output.print_angle("Current heading", self.current_heading.to_degrees());
        output.print_angle("Target heading", target_heading.to_degrees());
        output.print_angle("Heading error", heading_error.to_degrees());

        // Tau makes rotations intuitive
        let quarter_turn = Angle::from_tau_fraction(0.25);
        let half_turn = Angle::from_tau_fraction(0.5);
        let full_turn = Angle::from_tau_fraction(1.0);

        println!("\nTau Convention Benefits:");
        println!("  Quarter turn: {:.0}° = 0.25τ (intuitive!)", quarter_turn.to_degrees());
        println!("  Half turn: {:.0}° = 0.5τ (intuitive!)", half_turn.to_degrees());
        println!("  Full turn: {:.0}° = 1.0τ (intuitive!)", full_turn.to_degrees());

        // Compare with confusing π-based approach
        println!("\nTraditional π-based (confusing):");
        println!("  Quarter turn: π/2 = {:.2} (what fraction?)", PI / 2.0);
        println!("  Half turn: π = {:.2} (not obvious)", PI);
        println!("  Full turn: 2π = {:.2} (why multiply by 2?)", 2.0 * PI);

        self.current_heading = target_heading;
    }

    fn demonstrate_path_planning(&self, output: &CanonicalOutput) {
        self.print_section("TYPE-SAFE PATH PLANNING");

        let waypoints = vec![
            WorldPosition::new(2.0, 1.0, 0.0),
            WorldPosition::new(5.0, 3.0, 0.0),
            WorldPosition::new(8.0, 5.0, 0.0),
            WorldPosition::new(10.0, 5.0, 0.0),
        ];

        println!("Path waypoints (world frame):");
        let mut total_distance = 0.0;
        let mut previous_point = self.current_position;

        for (i, waypoint) in waypoints.iter().enumerate() {
            let segment_distance = previous_point.distance_to(waypoint);
            total_distance += segment_distance;

            println!("  {}. {} - segment: {}",
                    i + 1, output.position(waypoint.x, waypoint.y, waypoint.z), 
                    output.distance(segment_distance, "m"));

            previous_point = *waypoint;
        }

        let path_length = meters(total_distance);
        let estimated_time = path_length / self.current_speed;

        output.print_distance("Total path length", path_length.value, "m");
        output.print_speed("Current speed", self.current_speed.value);
        output.print_time("Estimated travel time", estimated_time.value);

        // Type safety ensures correct calculations (compile-time verification)
        output.print_success("Time calculation dimensionally verified");
    }

    fn demonstrate_obstacle_avoidance(&mut self, output: &CanonicalOutput) {
        self.print_section("TYPE-SAFE OBSTACLE AVOIDANCE");

        let obstacle_position = WorldPosition::new(6.0, 4.0, 0.0);
        let safety_distance = meters(2.0);
        let current_distance = meters(self.current_position.distance_to(&obstacle_position));

        output.print_position_like("Obstacle position", &obstacle_position);
        output.print_distance("Current distance to obstacle", current_distance.value, "m");
        output.print_distance("Required safety distance", safety_distance.value, "m");

        if current_distance.value < safety_distance.value {
            let avoidance_angle = Angle::from_tau_fraction(0.25);  // 90° turn
            let new_heading = self.current_heading + avoidance_angle;

            println!("⚠️  Obstacle too close! Executing avoidance maneuver.");
            println!("   Original heading: {:.1}°", self.current_heading.to_degrees());
            println!("   Avoidance turn: {:.1}° (τ/4)", avoidance_angle.to_degrees());
            println!("   New heading: {:.1}°", new_heading.to_degrees());

            self.current_heading = new_heading.normalized();
        } else {
            println!("✅ Safe distance maintained - no avoidance needed");
        }

        // Type safety prevents common errors
        println!("\n🛡️  Type Safety Benefits:");
        println!("   - Cannot compare distance to time (different dimensions)");
        println!("   - Cannot add positions from different coordinate frames");
        println!("   - Angle calculations use intuitive tau fractions");
        println!("   - All units verified at compile time");
    }

    fn demonstrate_sensor_fusion(&mut self, _output: &CanonicalOutput) {
        self.print_section("TYPE-SAFE SENSOR FUSION");

        // GPS reading (world frame)
        let gps_position = WorldPosition::new(7.2, 4.1, 0.0);
        let gps_accuracy = meters(1.5);

        // Odometry reading (distance traveled)
        let odometry_distance = meters(3.2);
        let odometry_heading = Angle::from_degrees(85.0);

        // IMU reading (angular velocity)
        let imu_angular_vel = radians_per_second(0.1);
        let measurement_time = seconds(2.0);

        println!("Sensor Inputs:");
        println!("  GPS: ({}, {}) ± {}m",
                gps_position.x, gps_position.y, gps_accuracy.value);
        println!("  Odometry: {}m at {:.1}°",
                odometry_distance.value, odometry_heading.to_degrees());
        println!("  IMU: {} rad/s for {}s",
                imu_angular_vel.value, measurement_time.value);

        // Fuse sensor data with type safety
        let estimated_angular_change = imu_angular_vel * measurement_time;
        let fused_heading = self.current_heading + estimated_angular_change;

        println!("\nFusion Results:");
        println!("  Angular change: {} rad", estimated_angular_change.radians);
        println!("  Fused heading: {:.1}°", fused_heading.to_degrees());

        // Type system ensures dimensional correctness (compile-time verification)
        println!("✅ Angular velocity × time = angle (dimensionally verified)");

        // Update position with type-safe calculations
        self.current_position = WorldPosition::new(
            self.current_position.x + odometry_distance.value * fused_heading.radians.cos(),
            self.current_position.y + odometry_distance.value * fused_heading.radians.sin(),
            self.current_position.z,
        );
        self.current_heading = fused_heading.normalized();

        println!("✓ Updated position: ({}, {}, {})",
                self.current_position.x, self.current_position.y, self.current_position.z);
    }

    fn print_navigation_summary(&self, output: &CanonicalOutput) {
        println!("\n📊 AUTONOMOUS NAVIGATION SUMMARY");
        println!("================================");
        println!("Final robot state:");
        output.print_position_like("Position", &self.current_position);
        output.print_angle("Heading", self.current_heading.to_degrees());
        output.print_speed("Speed", self.current_speed.value);

        println!("\n🏆 Type Safety Achievements:");
        println!("✅ Zero coordinate frame mixing errors");
        println!("✅ Zero dimensional analysis errors");
        println!("✅ Clear, intuitive angle calculations with tau");
        println!("✅ Compile-time verification of all calculations");
        println!("✅ Type-safe sensor fusion operations");

        println!("\n🚀 Ready for Production:");
        println!("This type-safe navigation system prevents the most common");
        println!("robotics programming errors that cause mission failures!");
    }
}

fn main() {
    // Initialize canonical output with same settings as C++ version
    let mut output = CanonicalOutput::new();
    // Precision settings now come from environment variables:
    // GAFRO_POSITION_PRECISION, GAFRO_ANGLE_PRECISION, etc.
    output.set_scientific_threshold(100.0);
    output.set_tau_convention(true);
    
    println!("🧭 GAFRO EXTENDED - AUTONOMOUS NAVIGATION TYPE SAFETY DEMO");
    println!("==========================================================");
    println!("Mathematical Convention: {}", output.tau_constant());
    println!("Demonstrating Phase 2 Modern Types for autonomous robot navigation.");

    // Initialize robot at origin facing east
    let start_position = WorldPosition::new(0.0, 0.0, 0.0);
    let start_heading = Angle::from_degrees(0.0);

    let mut demo = AutonomousNavigationDemo::new(start_position, start_heading);

    demo.demonstrate_coordinate_frame_safety(&output);
    demo.demonstrate_unit_safety(&mut output);
    demo.demonstrate_tau_angle_safety(&mut output);
    demo.demonstrate_path_planning(&output);
    demo.demonstrate_obstacle_avoidance(&output);
    demo.demonstrate_sensor_fusion(&mut output);
    demo.print_navigation_summary(&output);

    println!("\n📝 Phase 2 Benefits Demonstrated:");
    println!("1. Coordinate frame type safety prevents mixing world/robot/sensor frames");
    println!("2. SI unit system prevents dimensional analysis errors");
    println!("3. Tau convention makes angle calculations intuitive and clear");
    println!("4. Compile-time checks catch errors before deployment");
    println!("5. Type-safe sensor fusion prevents unit confusion");
}