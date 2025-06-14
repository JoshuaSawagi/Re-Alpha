mod aerials;
mod throw;
mod tilts;

pub fn install() {
    aerials::install();
    throw::install();
    tilts::install();
}