use covid19_SEIRSF::Config;
use covid19_SEIRSF::Univ;
use std::fs;
use toml;
use clap::Parser;

mod model;
pub use crate::model::displ;
use crate::model::total_iter;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let content = fs::read_to_string("model_config.toml")
            .expect("Something went wrong reading the file");
    
    let args = Cli::parse();

    let config: Config = toml::from_str(&content).unwrap();
    //in this case, set p_e: config.get_p_e();

    let mut univ: Univ = Univ::init(config.n_rows, config.n_cols);

    // let mut persons = univ.populate_poss_mult_pers_one_cell(&config);
    let mut persons = match &args.path {
        Some(pth) => {
            println!("+++++ Reading imported universe +++++");
            univ.read_imported_univ(pth, &config)
                .expect("couldn't load the universe")
            
        },
        None => {
            println!("+++++ Creating new universe +++++");
            univ.populate_poss_mult_pers_one_cell(&config)
        },
    };

    let folder = total_iter::create_folder();
    config.export(&folder);

    /*match univ.export(0, &folder) {
        Ok(_) => (),
        Err(_) => println!("couldn't export universe"),
    };*/

    total_iter::iter(&mut univ, &config, &mut persons, &folder);
}
