mod aerials;
mod ground;
mod throws;
//mod tilts;

pub fn install() {
    aerials::install();
    ground::install();
    throws::install();
    //tilts::install();
}