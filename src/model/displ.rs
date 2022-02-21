use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pos;

pub fn displace(univ: &mut Univ, pers: &mut Pers, config : &Config) {

    let cell =  univ.get_cell(&pers.origin_pos);
    //println!("{:?}", cell);

    /*match pers.state {
        State::S => cell.n_S -= 1,
        State::E => cell.n_E -= 1,
        State::I => cell.n_I -= 1,
        State::R => cell.n_R -= 1,
        State::F => cell.n_F -= 1,
    }*/
    cell.subs_state(&pers.state);
    //println!("{:?}", cell);
    
    let curr_pos : Pos = Pos::get_rand_pos(config);
    //println!("{:?}", curr_pos);
    pers.set_curr_pos(curr_pos);
    
    let cell = univ.get_cell(&curr_pos);
    cell.add_state(&pers.state);
    /*match pers.state {
        State::S => cell.n_S += 1,
        State::E => cell.n_E += 1,
        State::I => cell.n_I += 1,
        State::R => cell.n_R += 1,
        State::F => cell.n_F += 1,
    }*/

    pers.set_is_displ(true);
}


pub fn retrn(univ: &mut Univ, pers: &mut Pers) {
    let cell = univ.get_cell(&pers.curr_pos);
    cell.subs_state(&pers.p_state);

    pers.set_curr_pos(pers.origin_pos);
    pers.set_is_displ(false);

    let cell = univ.get_cell(&pers.origin_pos);
    cell.add_state(&pers.state);

}