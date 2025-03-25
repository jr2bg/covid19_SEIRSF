use std::{fs, path};

use covid19_spatiotemp_simulator::Config;
use covid19_spatiotemp_simulator::Pers;
use covid19_spatiotemp_simulator::Univ;
use covid19_spatiotemp_simulator::State;
use covid19_spatiotemp_simulator::Model;

use crate::displ;
use crate::model::exp_dec_data;
use crate::model::one_iter;

pub fn iter(model: &Model ,univ: &mut Univ, config: &Config, persons: &mut Vec<Pers>, folder: &path::PathBuf) {
    let mut n_dec: i32 = 0;
    let n_cycles: i32 = (*config).n_cycles;
    let mut records_dec: Vec<exp_dec_data::RecordDec> = Vec::with_capacity(n_cycles as usize);

    for i in 0..n_cycles {

        // randomly displace persons
        for pers in &mut *persons {
            // function to determine if we have to displace the person
            if pers.state != State::F && pers.will_be_displ(config) {
                displ::displace(univ, pers, config);
            }
        }

        // one iteration of the CA
        one_iter::single_evo(model,univ, config, persons);

        // update number of deceased
        n_dec += univ.get_n_dec();

        // update the file of deceased
        records_dec.push(exp_dec_data::RecordDec::new(i, n_dec));

        // return person to its original position
        for pers in &mut *persons {
            if pers.is_displ {
                displ::retrn(univ, pers);
            }
        }
        /*match univ.export(i + 1, folder) {
            Ok(_) => (),
            Err(_) => println!("couldn't export universe data"),
        };*/
    }

    // export deceased data
    match exp_dec_data::write_results(records_dec, &folder) {
        Ok(_) => (),
        Err(_) => println!("couldn't export decease time series"),
    };
    /*
    match exp_dec_data::copy_config(&folder) {
        Ok(_) => (),
        Err(_) => println!("couldnt export configuration"),
    };
    */
}

/// Creates the folder where the data will be stored
/// If in_cloud, a folder in `G will be created`
/// Else, the folder will be stored in a external hard disk
pub fn create_folder(in_cloud: bool,model: &Model) -> path::PathBuf {
    let model_prefix = match model {
        Model::SEIQSF => "seiqsf_",
        Model::SEISF => "seisf_",
    };
    // selects the location depending on the passed value
    let location = if in_cloud { 
        r"G:\Mi unidad\Tesis\A22\models\covid19_SEIRSF\data_runs" 
    } else { 
        r"D:\rust\thesis\data_runs"
    };

    // create the path for the corresponding input
    let folderpath = path::PathBuf::from(location);
    let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    let folder = format!(
        "{}{}",
        model_prefix,
        now.format("%Y%m%d_%H%M%S"));
    let folder = folderpath.join(&folder);
    //let folder = path::PathBuf::from(&folder);

    // create the folder
    fs::create_dir(&folder).unwrap();

    return folder;
}
