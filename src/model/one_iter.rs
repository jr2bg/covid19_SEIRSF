use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::State;
use covid19_SEIRSF::Univ;

use crate::model::trans_fns;

pub fn single_evo(univ: &mut Univ, config: &Config, persons: &mut Vec<Pers>) {
    let mut dec_indexes: Vec<usize> = vec![];

    //for pers in &mut *persons {
    for (i, pers) in persons.iter_mut().enumerate() {
        match pers.state {
            State::S => trans_fns::s2e(pers, univ, config),
            State::E => trans_fns::e2i(pers),
            State::I => trans_fns::i2qrf(pers, config),
            State::Q => trans_fns::q2rf(pers, config),
            State::R => trans_fns::r2s(pers, config),
            State::F => {
                univ.get_cell(&pers.curr_pos).subs_state(&State::F);
                dec_indexes.push(i);
            },
        }
    }
    
    let mut j = 0;
    for i in dec_indexes {
        persons.remove(i - j);
        j += 1;
    }

    // update pers positions in univ
    for pers in persons {
        univ.get_cell(&pers.curr_pos).subs_state(&pers.p_state);
        univ.get_cell(&pers.curr_pos).add_state(&pers.state);
    }
}
