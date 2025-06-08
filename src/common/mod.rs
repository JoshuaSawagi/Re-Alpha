mod airdodge;
mod cancels;
mod gentleman;
mod ground_movement;
mod guard;
mod guard_on;
mod landing;
mod jump_cancel;
mod cliff;
mod physics;
mod shield;
mod tech;
//pub mod momentum_transfer;

pub fn install() {
    airdodge::install();
    cancels::install();
    cliff::install();
    gentleman::install();
    ground_movement::install();
    guard::install();
    guard_on::install();
    landing::install();
    jump_cancel::install();
    physics::install();
    shield::install();
    tech::install();
}