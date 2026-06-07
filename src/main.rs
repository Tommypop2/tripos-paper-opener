use argh::FromArgs;

use crate::paper::Paper;
pub mod paper;
pub mod year;
#[derive(FromArgs)]
/// Open a given tripos paper
struct Arguments {
    #[argh(positional)]
    module: String,
    #[argh(positional)]
    year: u32,
    #[argh(switch, short = 'c')]
    /// open the crib
    crib: bool,

    #[argh(switch, short = 'b')]
    /// open both question paper and crib
    both: bool,
}

fn main() {
    let args: Arguments = argh::from_env();
    let paper = Paper::new(args.module, args.year);
    if args.both {
        // open both q paper and crib
        // open in this order so that the foreground paper is whatever the user specified with --crib
        paper.open(!args.crib).unwrap();
        paper.open(args.crib).unwrap();
    } else {
        paper.open(args.crib).unwrap();
    }
}
