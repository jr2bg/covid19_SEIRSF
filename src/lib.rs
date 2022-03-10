use csv::Writer;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path;
use std::fs;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// Structure for the position of a cell
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub r: usize,
    pub c: usize,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Cell {
    pub n_S: i32,
    pub n_E: i32,
    pub n_I: i32,
    pub n_R: i32,
    pub n_F: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub n_rows: i32,
    pub n_cols: i32,
    pub radius: i32,
    pub pop_dens: f32,
    pub n_cycles: i32,
    pub R_0: f32,
    pub time_contagious: i32,
    pub case_fat_risk: f32,
    pub t_I: i32,
    pub p_R: f32,
    pub t_F: i32,
    pub t_L: i32,
    pub t_R: i32,
    pub p_S: f32,
    pub t_S: i32,
    pub E_in: i32,
    pub I_in: i32,
    pub p_displ: f32,
    pub max_people: i32,
    pub p_e: f32,
}

pub struct Univ {
    pub tess: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    S,
    E,
    I,
    R,
    F,
}

#[derive(Debug, Clone, Copy)]
pub struct Pers {
    pub origin_pos: Pos,
    pub curr_pos: Pos,
    pub p_state: State,
    pub state: State,
    pub t_state: i32,
    pub is_displ: bool,
}

impl Pers {
    pub fn new(origin_pos: Pos, state: State) -> Pers {
        Pers {
            origin_pos,
            curr_pos: origin_pos,
            p_state: state,
            state,
            t_state: 0,
            is_displ: false,
        }
    }

    pub fn set_origin_pos(&mut self, origin_pos: Pos) {
        (*self).origin_pos = origin_pos;
    }

    pub fn set_curr_pos(&mut self, curr_pos: Pos) {
        (*self).curr_pos = curr_pos;
    }

    pub fn set_state(&mut self, state: State) {
        (*self).state = state;
    }

    pub fn set_p_state(&mut self, state: State) {
        (*self).p_state = state;
    }

    pub fn set_is_displ(&mut self, is_displ: bool) {
        (*self).is_displ = is_displ;
    }

    pub fn set_t_state(&mut self, t_state: i32) {
        (*self).t_state = t_state;
    }

    pub fn add_time_state(&mut self, tm: i32) {
        self.set_t_state(self.t_state + tm);
    }

    // posteriormente cambiar esta parte
    // para que considere una probabilidad de desplazamiento
    pub fn will_be_displ(&self, config: &Config) -> bool {
        let rand_numb: f32 = thread_rng().gen();
        let res = {
            if rand_numb <= (*config).p_displ {
                true
            } else {
                false
            }
        };
        res
    }
}

impl Pos {
    pub fn new(r: &usize, c: &usize) -> Pos {
        Pos { r: *r, c: *c }
    }

    pub fn set_pos(&mut self, r: &usize, c: &usize) {
        (*self).r = *r;
        (*self).c = *c;
    }

    pub fn get_rand_pos(config: &Config) -> Pos {
        let r = thread_rng().gen_range(0, (*config).n_rows);
        let c = thread_rng().gen_range(0, (*config).n_cols);

        Pos {
            r: r as usize,
            c: c as usize,
        }
    }
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            n_S: 0,
            n_E: 0,
            n_I: 0,
            n_R: 0,
            n_F: 0,
        }
    }

    /// Function to add people in an state of the current cell
    pub fn add_state(&mut self, state: &State) {
        match state {
            State::S => self.n_S += 1,
            State::E => self.n_E += 1,
            State::I => self.n_I += 1,
            State::R => self.n_R += 1,
            State::F => self.n_F += 1,
        }
    }

    /// Function to substract people in an state of the current cell
    pub fn subs_state(&mut self, state: &State) {
        match state {
            State::S => self.n_S -= 1,
            State::E => self.n_E -= 1,
            State::I => self.n_I -= 1,
            State::R => self.n_R -= 1,
            State::F => self.n_F -= 1,
        }
    }

    // Function to get the number of people in the cell
    pub fn get_n_people(&self) -> i32 {
        self.n_S + self.n_E + self.n_I + self.n_R + self.n_F
    }
}

impl Config {
    pub fn default() -> Config {
        Config {
            n_rows: 0,
            n_cols: 0,
            radius: 0,
            pop_dens: 0.5,
            n_cycles: 0,
            R_0: 0.0,
            time_contagious: 0,
            case_fat_risk: 0.0,
            t_I: 0,
            p_R: 0.0,
            t_F: 0,
            t_L: 0,
            t_R: 0,
            p_S: 0.0,
            t_S: 0,
            E_in: 0,
            I_in: 0,
            p_displ: 0.5,
            max_people: 4,
            p_e: 0.0,
        }
    }

    pub fn get_p_e(&mut self) {
        let radius : f32 = self.radius as f32;
        let time_contagious: f32 = self.time_contagious as f32;
        // approximate number of people in the neighbourhood
        let n_pers_neigh: f32 = 
            self.pop_dens * ((2.0*radius + 1.0).powi(2) - 1.0);
            
        self.p_e = self.R_0 / (n_pers_neigh  * time_contagious);
    }

    pub fn export(&self, folder: &path::PathBuf) {
        let filename = folder.join("model_config.toml");
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        println!("{}", toml_string);
        fs::write(filename, toml_string).expect("Could not write to file!");
    }
}

impl Univ {
    pub fn init(n_rows: i32, n_cols: i32) -> Univ {
        let mut univ: Univ = Univ {
            tess: vec![vec![Cell::new()]],
        };
        let tess: Vec<Vec<Cell>> = vec![vec![Cell::new(); n_cols as usize]; n_rows as usize];
        univ.tess = tess;
        return univ;
    }

    pub fn get_cell(&mut self, pos: &Pos) -> &mut Cell {
        &mut self.tess[(*pos).r][(*pos).c]
    }

    pub fn set_cell(&mut self, pos: &Pos, cell: Cell) {
        self.tess[(*pos).r][(*pos).c] = cell;
    }

    pub fn get_n_dec(&self) -> i32 {
        let mut n_dec: i32 = 0;
        for row in &self.tess {
            for cell in row {
                n_dec += cell.n_F;
            }
        }
        n_dec
    }

    // populate universe with possibly more than one person per cell
    pub fn populate_poss_mult_pers_one_cell(&mut self, config: &Config) -> Vec<Pers> {
        let n_rows = (*config).n_rows;
        let n_cols = (*config).n_cols;
        let tot_cells: i32 = n_cols * n_rows;
        let tot_pop: i32 = ((*config).pop_dens * (tot_cells as f32)) as i32;
        // CHECK THIS LATER, MAYBE E_in WON'T BE USED
        let n_e_in: i32 = (*config).E_in;
        let n_i_in: i32 = (*config).I_in;

        let mut persons: Vec<Pers> = Vec::with_capacity(tot_pop as usize);

        let mut i: i32 = 0;
        let mut state: State;
        let mut pers: Pers;
        let mut pos: Pos;
        for _ in 0..tot_pop {
            pos = Pos::get_rand_pos(config);
            // at most config.max_people per cell
            while self.get_cell(&pos).get_n_people() > config.max_people {
                pos = Pos::get_rand_pos(config);
            }

            state = {
                if i <= n_e_in {
                    State::E
                } else if i <= n_e_in + n_i_in {
                    State::I
                } else {
                    State::S
                }
            };

            pers = Pers::new(pos, state);

            persons.push(pers);

            self.get_cell(&pos).add_state(&state);

            i += 1;
        }

        persons
    }

    pub fn populate(&mut self, config: &Config) -> Vec<Pers> {
        let tot_cells: i32 = (*config).n_cols * (*config).n_rows;
        let tot_pop: i32 = ((*config).pop_dens * (tot_cells as f32)) as i32;

        let mut persons: Vec<Pers> = Vec::with_capacity(tot_pop as usize);

        // CHECK THIS LATER, MAYBE E_in WON'T BE USED
        let n_E_in: i32 = (*config).E_in;
        let n_I_in: i32 = (*config).I_in;
        let s_pop: i32 = tot_pop - n_E_in - n_I_in;

        let mut pop: Vec<Pos> = vec![Pos::new(&0, &0); tot_cells as usize];

        for i in 0..(*config).n_rows as usize {
            for j in 0..(*config).n_cols as usize {
                pop[i * (*config).n_cols as usize + j].set_pos(&i, &j);
            }
        }

        // shuffle
        pop.shuffle(&mut thread_rng());

        // select the first E_in and I_in
        // later, s_pop will be susceptible
        let mut iterator_pop = pop.iter();

        let mut val_it: Pos;

        for _ in 0..n_E_in as usize {
            val_it = *iterator_pop.next().unwrap();
            persons.push(Pers::new(val_it, State::E));

            (*self).set_cell(
                &(val_it),
                Cell {
                    n_S: 0,
                    n_E: 1,
                    n_I: 0,
                    n_R: 0,
                    n_F: 0,
                },
            )
        }

        for _ in 0..n_I_in as usize {
            val_it = *iterator_pop.next().unwrap();
            persons.push(Pers::new(val_it, State::I));

            (*self).set_cell(
                &(val_it),
                Cell {
                    n_S: 0,
                    n_E: 0,
                    n_I: 1,
                    n_R: 0,
                    n_F: 0,
                },
            )
        }

        for _ in 0..s_pop as usize {
            val_it = *iterator_pop.next().unwrap();
            persons.push(Pers::new(val_it, State::S));

            (*self).set_cell(
                &(val_it),
                Cell {
                    n_S: 1,
                    n_E: 0,
                    n_I: 0,
                    n_R: 0,
                    n_F: 0,
                },
            )
        }

        persons
    }

    pub fn get_n_inf_ngbh(&mut self, pos: &Pos, config: &Config) -> i32 {
        let mut n_inf: i32 = 0;

        let n_rows = config.n_rows;
        let n_cols = config.n_cols;
        let radius = (*config).radius;

        for r in n_rows - radius..n_rows + radius + 1 {
            for c in n_cols - radius..n_rows + radius + 1 {
                if r < 0
                    || c < 0
                    || r >= n_rows
                    || c >= n_cols
                    || (r == pos.r as i32 && c == pos.c as i32)
                {
                    continue;
                }

                n_inf += (*self)
                    .get_cell(&Pos {
                        r: r as usize,
                        c: c as usize,
                    })
                    .n_E; // <-  CHECK IF WE CONSIDER Exposed

                n_inf += (*self)
                    .get_cell(&Pos {
                        r: r as usize,
                        c: c as usize,
                    })
                    .n_I;
            }
        }
        n_inf
    }

    pub fn export(&self, i: i32, folder: &path::PathBuf) -> Result<(), Box<dyn Error>> {
        let file = folder.join(format!("{}.csv", i));
        let mut wtr = Writer::from_path(file)?;

        for row in &self.tess {
            for cell in row {
                wtr.serialize(cell)?;
            }
        }
        wtr.flush()?;
        Ok(())
    }
}
