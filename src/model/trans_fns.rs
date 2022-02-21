// Module for the transition functions of the CA
use rand::{thread_rng, Rng};

use covid19_SEIRSF::Univ;
use covid19_SEIRSF::Pers;
use covid19_SEIRSF::Config;
use covid19_SEIRSF::State;

// Multiply p_E times the number of elements in E or I in its ngh
// chech with random number
pub fn s2e(pers: &mut Pers, univ : &mut Univ, config: &Config){
    let n_inf_ngbh = univ.get_n_inf_ngbh(&pers.curr_pos, config);
    let rand_numb : f32 = thread_rng().gen::<f32>();

    // falta considerar el número total de personas
    // usar dens_pob*n_rows*n_cols
    //if rand_numb <= n_inf_ngbh as f32 * config.R_0 / ( config.time_contagious as f32) {
    
    //if rand_numb <= n_inf_ngbh as f32 * config.R_0 {
    if true {
        pers.set_state(State::E);
        pers.set_t_state(0);
    }
}


pub fn e2i(pers: &mut Pers, p_Is: Vec<f32>){
    // no consideramos un tiempo
    let rand_numb : f32 = thread_rng().gen::<f32>();

    //if rand_numb <= p_Is[pers.t_state as usize] {
        if rand_numb <= 1.0 {
        // cambiar a cero o 1 el t_state
        pers.set_state(State::I);
        pers.set_t_state(0);
    } else {
        pers.add_time_state(1);
    }
    pers.set_p_state(State::E);
}


pub fn i2rf(pers: &mut Pers, config: &Config) {
    let rand_numb : f32 = thread_rng().gen::<f32>();
    if rand_numb <= config.p_R && pers.t_state >= config.t_R {
        // cambiar a cero o 1 el t_state
        pers.set_state(State::R);
        pers.set_t_state(0);
    } else if rand_numb <= config.case_fat_risk && pers.t_state >= config.t_F {
        pers.set_state(State::F);
        pers.set_t_state(0); 
    } else {
        pers.add_time_state(1);
    }
    pers.set_p_state(State::I);
}


pub fn r2s(pers: &mut Pers, config: &Config) {
    let rand_numb : f32 = thread_rng().gen();
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
pub fn get_p_Is() -> Vec<f32>{
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
        1.0
    ]
}


pub fn get_p_R(t: i32) -> f32 {

    if t < 10 {
        return 0.0 ;
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
        return 0.477419
    } else if t < 27 {
        return 0.534884;
    } else if t < 37 {
        return 0.557634;
    }
    return 0.557634;
}
