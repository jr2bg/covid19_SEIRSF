use covid19_spatiotemp_simulator::Config;
use covid19_spatiotemp_simulator::Univ;
use covid19_spatiotemp_simulator::Model;
use std::fs;
use toml;
use clap::{ArgGroup, Parser};
// use log::{info, warn};

mod model;
pub use crate::model::displ;
use crate::model::total_iter;

/// Program to simulate the spatio-temporal dynamics of COVID using
/// different models
/// structure for the CLI arguments. It contains
/// - model to be simulated
/// - directory where the results will be stored
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("exec")
                .required(true)
                .args(&["seiqsf", "seisf"]),
        ))]
struct Cli {
    #[clap(long)]
    path: Option<std::path::PathBuf>,

    /// save in the cloud
    #[clap(long)]
    cloud : bool,

    /// seiqfs model
    #[clap(long)]
    seiqsf: bool,

    /// seisf model
    #[clap(long)]
    seisf: bool,
}

fn main() {
    // get information for the initial values for the simulation
    let content = fs::read_to_string("model_config.toml")
            .expect("Something went wrong reading the file");
    
    // get the cli arguments given by the user
    let args = Cli::parse();

    let config: Config = toml::from_str(&content).unwrap();

    // get the desired model
    let model = if args.seiqsf {
        Model::SEIQSF
    }  else if args.seisf {
        Model::SEISF
    } else {
        panic!("Given model not available")
    };
    //in this case, set p_e: config.get_p_e();

    // initialize the universe with the appropriate size
    let mut univ: Univ = Univ::init(config.n_rows, config.n_cols);

    // 
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
    println!("{:?}",args);
    let folder = total_iter::create_folder(
        args.cloud,
        &model
    );
    config.export(&folder);

    /*match univ.export(0, &folder) {
        Ok(_) => (),
        Err(_) => println!("couldn't export universe"),
    };*/   

    // compute the simulation for the steps given
    total_iter::iter(&model, &mut univ, &config, &mut persons, &folder);
}
