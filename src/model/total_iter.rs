use std::num::ParseIntError;

use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::Config;

use crate::displ;
use crate::model::one_iter;
use crate::model::exp_dec_data;

pub fn iter(univ: &mut Univ, config: &Config, persons: &mut Vec<Pers>){
    let mut n_dec : i32;
    let n_cycles: i32 = (*config).n_cycles;
    let mut records_dec : Vec<exp_dec_data::Record_Dec> = 
        Vec::with_capacity(n_cycles as usize);

    for i in 0..n_cycles {
        //println!("iteration: {}",i);
        n_dec = 0;
        for pers in &mut *persons {
            // function to determine if we have to displace the person
            if pers.will_be_displ() {
                displ::displace(univ, pers, config);
            }
        }

        one_iter::single_evo(univ, config, persons);
        n_dec = univ.get_n_dec();
        records_dec.push(exp_dec_data::Record_Dec::new(i, n_dec));

        for pers in &mut* persons {
            if pers.is_displ {
                displ::retrn(univ, pers);
            }
            println!("{:?}",pers);
        }
        univ.export(i+1);
    }

    exp_dec_data::write_results(records_dec);
}
