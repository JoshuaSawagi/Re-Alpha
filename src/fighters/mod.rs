mod captain;
mod chrom;
mod daisy;
mod falco;
mod fox;
mod ike;
mod lucina;
mod inkling;
mod marth;
mod purin;
//mod rosetta;
mod roy;
mod samus;
mod sheik;
mod sonic;
mod wolf;
mod young_link;

pub fn install() {
    captain::install();
    chrom::install();
    daisy::install();
    falco::install();
    fox::install();
    ike::install();
    lucina::install();
    inkling::install();
    marth::install();
    purin::install();
    //rosetta::install();
    roy::install();
    samus::install();
    sheik::install();
    sonic::install();
    wolf::install();
    young_link::install();
}