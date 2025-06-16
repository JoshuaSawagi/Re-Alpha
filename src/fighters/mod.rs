mod captain;
mod chrom;
mod daisy;
mod falco;
mod ike;
mod lucina;
//mod mario;
mod marth;
mod purin;
mod roy;
mod samus;
mod sheik;
mod sonic;
mod wolf;

pub fn install() {
    captain::install();
    chrom::install();
    daisy::install();
    falco::install();
    ike::install();
    lucina::install();
    //mario::install();
    marth::install();
    purin::install();
    roy::install();
    samus::install();
    sheik::install();
    sonic::install();
    wolf::install();
}