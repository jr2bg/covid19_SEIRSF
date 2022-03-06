use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Config;
use std::fs;
use toml;

mod model;
pub use crate::model::displ;
use crate::model::total_iter;

fn main() {
    
    let content = fs::read_to_string("model_config.toml")
        .expect("Something went wrong reading the file");

    let config : Config = toml::from_str(&content).unwrap();

    let mut univ:Univ = Univ::init(config.n_rows, config.n_cols);

    let mut persons = univ.populate_poss_mult_pers_one_cell(&config);

    let folder = total_iter::create_folder(); 

    match univ.export(0, &folder) {
        Ok(_) => (),
        Err(_) => println!("couldn't export universe"),
    };

    total_iter::iter(&mut univ, &config, &mut persons, &folder);
    
}
