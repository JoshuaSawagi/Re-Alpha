mod aerials;
mod throw;
mod specials;

pub fn install() {
    aerials::install();
    throw::install();
    specials::install();
}