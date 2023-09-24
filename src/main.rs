/// Create Vehicle struct
struct Vehicle;

/// Implement the struct
impl LandCapable for Vehicle {}

impl WaterCapable for Vehicle {}

/// Create and implement trait | LandCapable
trait LandCapable {
    /// The method will have a shared reference to itself
    fn drive(&self) -> () {
        println!("DEFAULT method implementation: Engine starting...");
    }
}

/// Create and implement trait | WaterCapable
trait WaterCapable {
    fn float(&self) {
        println!("DEFAULT method implementation: Pressurizing...");
    }
}

// Create a trait for multi-terrain
/// Force MultiTerrain trait to implement both WaterCapable & LandCapable -> referred to as super traits
trait MultiTerrain: LandCapable + WaterCapable {}

struct HoverCraft;

impl MultiTerrain for HoverCraft {}

impl LandCapable for HoverCraft {}

impl WaterCapable for HoverCraft {}

/// Create ignition method with reference to Vehicle
#[allow(dead_code)]
fn ignition(vehicle: &dyn LandCapable) {
    vehicle.drive();
}

#[allow(dead_code)]
fn pressurize(vehicle: &dyn WaterCapable) {
    vehicle.float();
}

/// This method combines both methods that are implemented by the traits
fn ignition_pressurize(vehicle: &dyn MultiTerrain) {
    // Implement both methods
    vehicle.drive();
    vehicle.float();
}

fn main() {
    println!("****Console Output****");
    let hover_craft: HoverCraft = HoverCraft;
    ignition_pressurize(&hover_craft);
}