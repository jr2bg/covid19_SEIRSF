use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pos;

pub fn displace(univ: &mut Univ, pers: &mut Pers, config : &Config) {
    let mut curr_pos : Pos = Pos::get_rand_pos(config);
    /*
    while univ.get_cell(&curr_pos).get_n_people() > config.max_people{
        curr_pos = Pos::get_rand_pos(config);
    }
    */
    let cell =  univ.get_cell(&pers.origin_pos);
    cell.subs_state(&pers.state);

    pers.set_curr_pos(curr_pos);
    
    let cell = univ.get_cell(&curr_pos);
    cell.add_state(&pers.state);

    pers.set_is_displ(true);
}


pub fn retrn(univ: &mut Univ, pers: &mut Pers) {
    let cell = univ.get_cell(&pers.curr_pos);
    cell.subs_state(&pers.state);

    pers.set_curr_pos(pers.origin_pos);
    pers.set_is_displ(false);

    let cell = univ.get_cell(&pers.origin_pos);
    cell.add_state(&pers.state);

}