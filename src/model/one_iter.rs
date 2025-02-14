use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::State;
use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Model;

use crate::model::trans_fns;

// evolution from seiqsf
pub fn evo_seiqsf(
    univ: &mut Univ,
    config: &Config,
    persons: &mut Vec<Pers>,
    dec_index: &mut Vec<usize>
){
    // consider all the persons in the model
    for (i, pers) in persons.iter_mut().enumerate() {
        match pers.state {
            State::S => trans_fns::s2e(pers, univ, config),
            State::E => trans_fns::e2i(pers),
            State::I => trans_fns::i2qrf(pers, config),
            State::Q => trans_fns::q2rf(pers, config),
            State::R => trans_fns::r2s(pers, config),
            // in F means that in the previous interation the individual died
            // so from now on it can not aggregate value to the simulation as
            // the state won't change
            State::F => {
                // remove from the list of current deceased
                univ.get_cell(&pers.curr_pos).subs_state(&State::F);
                // add the persons' index to be removed
                dec_indexes.push(i);
            },
        }
    }
}

// evolution from seisf
pub fn evo_seisf(
    univ: &mut Univ,
    config: &Config,
    persons: &mut Vec<Pers>,
    dec_index: &mut Vec<usize>
){
    for (i, pers) in persons.iter_mut().enumerate() {
        match pers.state {
            State::S => trans_fns::s2e(pers, univ, config),
            State::E => trans_fns::e2i(pers),
            State::I => trans_fns::i2sf(pers, config),
            // in F means that in the previous interation the individual died
            // so from now on it can not aggregate value to the simulation as
            // the state won't change
            State::F => {
                // remove from the list of current deceased
                univ.get_cell(&pers.curr_pos).subs_state(&State::F);
                // add the persons' index to be removed
                dec_indexes.push(i);
            },
            _ => (),
        }
    }
}

/// One iteration of the CA
pub fn single_evo(
    model: Model,
    univ: &mut Univ,
    config: &Config,
    persons: &mut Vec<Pers>
) {
    // contains the indexes of all deceased people in the current step
    // to discard them in future evolutions
    let mut dec_indexes: Vec<usize> = vec![];

    // apply the transition functions according to the passed model
    match model {
        Model::SEIQSF => evo_seiqsf(univ, config, persons, dec_index),
        Model::SEISF => evo_seisf(univ, config, persons, dec_index),
    }
    
    // remove deceased persons for the next iterations
    let mut j = 0;
    for i in dec_indexes {
        persons.remove(i - j);
        j += 1;
    }

    // Consider the displacement of a person
    for pers in persons {
        univ.get_cell(&pers.curr_pos).subs_state(&pers.p_state);
        univ.get_cell(&pers.curr_pos).add_state(&pers.state);
    }
}
