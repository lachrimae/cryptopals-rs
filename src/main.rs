#![feature(type_ascription)]

mod cryptopals;
mod challenges;

#[macro_use]
extern crate clap;

pub use challenges::set1::*;
pub use challenges::set2::*;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Curran McConnell <curran.mcconnell@protonmail.com>")
        (about: "test the cryptopals lib I wrote")
        (@arg PROBLEMSET: -s --set +required +takes_value "the problemset to test")
    )
    .get_matches();

    match matches.value_of("PROBLEMSET") {
        Some("1") => set_one(),
        Some("2") => set_two(),
        Some("3") => set_three(None),
        Some("4") => set_four(),
        Some("5") => set_five(),
        Some("6") => set_six(),
        Some("7") => set_seven(),
        Some("8") => set_eight(),
        Some("9") => set_nine(),
        Some("10") => set_ten(),
        Some("11") => set_eleven(),
        Some("12") => set_twelve(),
        Some("13") => set_thirteen(),
        Some("14") => set_fourteen(),
        Some("16") => set_sixteen(),
        _ => {
            set_one();
            set_two();
            set_three(None);
            set_four();
            set_five();
            set_six();
            set_seven();
            set_eight();
            set_nine();
            set_ten();
            set_eleven();
            set_twelve();
            set_thirteen();
            set_fourteen();
            set_sixteen();
        }
    }
}
