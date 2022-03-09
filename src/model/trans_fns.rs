// Module for the transition functions of the CA
use rand::{thread_rng, Rng};

use covid19_SEIRSF::Config;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::State;
use covid19_SEIRSF::Univ;

// Multiply p_E times the number of elements in E or I in its ngh
// chech with random number
pub fn s2e(pers: &mut Pers, univ: &mut Univ, config: &Config) {
    let n_inf_ngbh = univ.get_n_inf_ngbh(&pers.curr_pos, config);
    // CONSIDER IF WE ONLY TAKE I OR BOTH I AND E
    let n_inf_cell = univ.get_cell(&pers.curr_pos).n_E + univ.get_cell(&pers.curr_pos).n_I;

    let rand_numb: f32 = thread_rng().gen::<f32>();
    // población total??
    //let tot_pop: f32 = (config.n_cols * config.n_cols) as f32 * config.pop_dens;
    //let p_e: f32 = config.R_0 / (tot_pop * config.time_contagious as f32);
    //let radius : i32 = config.radius;
    //let n_pers_neigh: i32 = (config.pop_dens * ((2.0*radius as f32 + 1.0).powi(2) - 1.0)) as i32;
    //let p_e: f32 = config.R_0 / (n_pers_neigh as f32 * config.time_contagious as f32);
    let p_e: f32 = config.p_e;
    //let p_e = 0.5;
    let p_e_cell: f32 = get_cum_p_e_cell(p_e, n_inf_cell);
    //let p_e_neigh: f32 = n_inf_ngbh as f32 *p_e;
    let p_e_neigh: f32 = get_cum_geo_distr(p_e, n_inf_ngbh);

    // union of independent events
    let tot_p_e: f32 = p_e_neigh + p_e_cell - p_e_cell * p_e_neigh;

    // falta considerar el número total de personas
    // usar dens_pob*n_rows*n_cols
    if rand_numb <= tot_p_e {
        pers.set_state(State::E);
        pers.set_t_state(0);
    }
    pers.set_p_state(State::S);
}

pub fn e2i(pers: &mut Pers) {
    let ps_i: Vec<f32> = get_ps_i();
    // no consideramos un tiempo
    let rand_numb: f32 = thread_rng().gen::<f32>();

    if rand_numb <= ps_i[pers.t_state as usize] {
        // cambiar a cero o 1 el t_state
        pers.set_state(State::I);
        pers.set_t_state(0);
    } else {
        pers.add_time_state(1);
    }
    pers.set_p_state(State::E);
}

pub fn i2rf(pers: &mut Pers, config: &Config) {
    let rand_numb: f32 = thread_rng().gen::<f32>();

    //} else if rand_numb <= config.case_fat_risk && pers.t_state >= config.t_F {
    if rand_numb <= config.case_fat_risk {
        pers.set_state(State::F);
        pers.set_t_state(0);
    //if rand_numb <= config.p_R && pers.t_state >= config.t_R {
    } else if rand_numb <= get_p_r(pers.t_state) + config.case_fat_risk {
        // cambiar a cero o 1 el t_state
        pers.set_state(State::R);
        pers.set_t_state(0);
    } else {
        pers.add_time_state(1);
    }
    pers.set_p_state(State::I);
}

pub fn r2s(pers: &mut Pers, config: &Config) {
    let rand_numb: f32 = thread_rng().gen();
    if rand_numb <= config.p_S && pers.t_state >= config.t_S {
        pers.set_state(State::S);
    }
    pers.set_p_state(State::R);
}

pub fn f2f(pers: &mut Pers) {
    pers.set_p_state(State::F);
}

// list with the information to get the cdf of the normal distribution specified
// in the article Lauer et al 2020
pub fn get_ps_i() -> Vec<f32> {
    vec![
        9.999999999999995e-05,
        0.005069057888351579,
        0.04697281138408274,
        0.1524326835113221,
        0.32668358459673175,
        0.5030663485714983,
        0.6311817405684047,
        0.7456153473400083,
        0.8284751117617429,
        0.8781121051932557,
        0.9180773502240464,
        0.9453219480606242,
        0.9611146387137549,
        0.972254390405818,
        0.9811215095713122,
        0.9871710514697591,
        0.9907105095201212,
        0.9935645408907046,
        0.9955415666031736,
        0.9967180004675154,
        0.9976829858286643,
        0.9983634255871315,
        0.9987757742587142,
        0.9990799978812611,
        0.9993352602947051,
        0.9995193858705637,
        0.9996335281494982,
        0.9997308776387785,
        0.9998022415547547,
        0.9998471946417632,
        0.9998814873225667,
        1.0,
    ]
}

pub fn get_p_r(t: i32) -> f32 {
    if t < 10 {
        return 0.0;
    } else if t < 15 {
        return 0.046512;
    } else if t < 18 {
        return 0.293023;
    } else if t < 20 {
        return 0.395349;
    } else if t < 21 {
        return 0.465116;
    } else if t < 23 {
        return 0.465116;
    } else if t < 25 {
        return 0.477419;
    } else if t < 27 {
        return 0.534884;
    } else if t < 37 {
        return 0.557634;
    }
    return 0.557634;
}

// iid events inside cell of person of interest
pub fn get_cum_p_e_cell(p_e: f32, n_inf_cell: i32) -> f32 {
    1.0 - (1.0 - p_e).powi(n_inf_cell)
}

// iid events for _external_ neighbourhood
pub fn get_geo_distr(p: f32, k: i32) -> f32 {
    (1.0 - p).powi(k - 1) * p
}

pub fn get_cum_geo_distr(p: f32, n: i32) -> f32 {
    let mut p_tot: f32 = 0.0;
    for k in 0..n {
        p_tot += get_geo_distr(p, k)
    }
    return p_tot;
}
