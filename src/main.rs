use covid19_SEIRSF::Cell;
use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pos;
use std::fs;
use toml;
//use serde_derive::Deserialize;

mod model;
pub use crate::model::displ;
use crate::model::total_iter;

fn main() {

    //match fs::remove_dir_all("tests") {
    //    Ok(()) => (),
    //    Err(_) => (),
    //}
    //println!("start");

    //fs::remove_dir_all("tests");
    
    let content = fs::read_to_string("model_config.toml")
        .expect("Something went wrong reading the file");
    //println!("after toml config read");

    let config : Config = toml::from_str(&content).unwrap();
    //println!("config toml unwrap");
    //println!("{:?}", config);

    //println!("population density: {}", config.pop_dens);
    let mut univ:Univ = Univ::init(config.n_rows, config.n_cols);
    //println!("before populate");

    /*
    for v in univ.iter() {
        for c in v{
            print!("celda: {:?}", *c);
        }
        println!();
    }
    */

    //univ.get_cell(&covid19_SEIRSF::Pos {r : 0, c : 0}).n_S = 1;

    let mut persons = univ.populate(&config);

    //fs::create_dir("tests");
    univ.export(0);

    //println!("{:#?}", univ.tess);
    //println!("{:#?}", persons);
    //println!("{:#?}", persons[0]);

    //covid19_SEIRSF::get_neigh(&univ,5,6, &config);

    total_iter::iter(&mut univ, &config, &mut persons);
    
}
