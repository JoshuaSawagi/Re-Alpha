mod cancels;
mod escape_air;
mod ground_movement;
mod landing;
mod physics;
mod shield;
mod tech;

pub fn install() {
    cancels::install();
    escape_air::install();
    ground_movement::install();
    landing::install();
    physics::install();
    shield::install();
    tech::install();
}

/* I need to fix cliff, jump_cancel, gentleman, guard, and guard_on, because they crash on switch*/