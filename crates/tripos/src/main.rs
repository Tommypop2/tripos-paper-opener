use argh::FromArgs;

use crate::{
    module::Module,
    paper::{Paper, PaperType, PaperUrl},
};
pub mod module;
pub mod open_method;
pub mod paper;
pub mod year;
pub mod ep;

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

    #[argh(switch, short = 'e')]
    /// toggle for Examples Paper mode
    examples_paper: bool,

    #[argh(switch, short = 'b')]
    /// open both question paper and crib
    both: bool,

    #[argh(switch, short = 't')]
    /// open question paper and crib together (on same page)
    together: bool,
}

fn main() {
    
    let args: Arguments = argh::from_env();

    let paper_type =
    match args.examples_paper {
        true => PaperType::ExamplesPaper(args.year),
        false => PaperType::TriposPaper,
    };

    let paper = Paper::new(Module::new(args.module), args.year, paper_type);
    // Work out what method to open paper with
    let open_method = if args.together {
        open_method::OpenMethod::Together
    } else if args.both {
        open_method::OpenMethod::Both
    } else if args.crib {
        open_method::OpenMethod::Crib
    } 
    else {
        open_method::OpenMethod::QP
    };


    match open_method {
        open_method::OpenMethod::Together => paper.open(PaperUrl::Together).unwrap(),
        open_method::OpenMethod::Both => {
            // open both q paper and crib
            // open in this order so that the foreground paper is whatever the user specified with --crib
            // TODO: encode in OpenMethod::Both state which one should be foreground
            paper
                .open(if args.crib {
                    PaperUrl::QP
                } else {
                    PaperUrl::Crib
                })
                .unwrap();
            paper
                .open(if args.crib {
                    PaperUrl::Crib
                } else {
                    PaperUrl::QP
                })
                .unwrap();
        }
        open_method::OpenMethod::Crib => paper.open(PaperUrl::Crib).unwrap(),
        open_method::OpenMethod::QP => paper.open(PaperUrl::QP).unwrap(),
    }
}
