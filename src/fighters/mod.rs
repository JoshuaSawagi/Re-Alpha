mod chrom;
mod daisy;
mod falco;
mod ike;
mod lucina;
mod marth;
mod roy;
mod samus;
mod wolf;

pub fn install() {
    chrom::install();
    daisy::install();
    falco::install();
    ike::install();
    lucina::install();
    marth::install();
    roy::install();
    samus::install();
    wolf::install();
}