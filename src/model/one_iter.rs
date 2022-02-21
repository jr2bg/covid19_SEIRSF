
use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::Config;
use covid19_SEIRSF::State;

use crate::model::trans_fns;

pub fn single_evo(univ:&mut Univ, config: &Config, persons:&mut Vec<Pers>){
    for pers in &mut *persons{
        match pers.state {
            State::S => trans_fns::s2e(pers, univ, config),
            State::E => trans_fns::e2i(pers, trans_fns::get_p_Is()),
            State::I => trans_fns::i2rf(pers, config),
            State::R => trans_fns::r2s(pers, config),
            State::F => trans_fns::f2f(pers),
        }
    }

    // update pers positions in univ
    for pers in persons{
        univ.get_cell(&pers.curr_pos).subs_state(&pers.p_state);
        univ.get_cell(&pers.curr_pos).add_state(&pers.state);
    }
}