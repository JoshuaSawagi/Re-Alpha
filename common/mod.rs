mod cancels;
mod double_jump_cancel;
mod escape_air;
mod ground_movement;
mod guard;
mod guard_damage;
mod guard_on;
mod landing;
mod ledges;
mod physics;
mod shield;
mod tech;

pub fn install() {
    cancels::install();
    double_jump_cancel::install();
    escape_air::install();
    ground_movement::install();
    guard::install();
    guard_damage::install();
    guard_on::install();
    landing::install();
    ledges::install();
    physics::install();
    shield::install();
    tech::install();
}