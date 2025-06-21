mod aerials;
mod ground;
mod smashes;
mod specials;
mod tilts;

pub fn install() {
	aerials::install();
	ground::install();
	smashes::install();
	specials::install();
	tilts::install();
}
