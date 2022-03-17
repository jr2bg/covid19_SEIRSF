use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::State;
use covid19_SEIRSF::Univ;

use crate::model::trans_fns;

pub fn single_evo(univ: &mut Univ, config: &Config, persons: &mut Vec<Pers>) {
    for pers in &mut *persons {
        match pers.state {
            State::S => trans_fns::s2e(pers, univ, config),
            State::E => trans_fns::e2i(pers),
            State::I => trans_fns::i2sf(pers, config),
            State::F => trans_fns::f2f(pers),
            _ => (),
        }
    }

    // update state counter for each person's cell
    for pers in persons {
        univ.get_cell(&pers.curr_pos).subs_state(&pers.p_state);
        univ.get_cell(&pers.curr_pos).add_state(&pers.state);
    }
}
