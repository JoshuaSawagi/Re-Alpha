mod aerials;
mod ground;
mod smash;
mod throw;

pub fn install() {
	aerials::install();
	ground::install();
	smash::install();
	throw::install();
}