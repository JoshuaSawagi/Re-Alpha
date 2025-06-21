mod aerials;
mod ground;
mod smashes;
mod throw;
mod tilts;

pub fn install() {
    aerials::install();
    ground::install();
    smashes::install();
    throw::install();
    tilts::install();
}