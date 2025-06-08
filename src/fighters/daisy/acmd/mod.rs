mod aerials;
mod ground;
mod tilts;

pub fn install() {
	aerials::install();
	ground::install();
	tilts::install();
}