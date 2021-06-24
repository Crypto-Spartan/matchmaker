
use itertools::Itertools;
use good_lp::{constraint, default_solver, Solution, SolverModel, Expression, variable, ProblemVariables};
use good_lp::solvers::ResolutionError;
use std::{fmt, rc::Rc};

#[allow(non_camel_case_types)]
struct player<'a> {
    name: &'a str,
    mmr: i32,
}

impl<'a> fmt::Display for player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} - {}", self.name, self.mmr)
    }
} 



fn get_players<'a>() -> Vec<player<'a>> {

    let playerlist = vec![
        player{
            name: "Kipperacer",
            mmr: 2000
        },
        player{
            name: "Not Brad",
            mmr: 3900
        },
        player{
            name: "Aayan",
            mmr: 1900
        },
        player{
            name: "hans solo#6857",
            mmr: 2800
        },
        player{
            name: "V4lhallaRSL",
            mmr: 3000
        },
        player{
            name: "hans solo#6857",
            mmr: 2800
        },
        player{
            name: "King Size Ultra Krabby Supreme (Snake)",
            mmr: 1900
        },
        player{
            name: "r1sen__",
            mmr: 2400
        },
        player{
            name: "yogurthb",
            mmr: 2800
        },
        player{
            name: "Dexterity.none",
            mmr: 2300
        },
        player{
            name: "4000dpi",
            mmr: 1800
        },
        player{
            name: "crypto",
            mmr: 2600
        },
        player{
            name: "renaxla",
            mmr: 2400
        }
    ];
    
    /*for p in players.iter() {
        println!("{}", p);
    }*/
    
    playerlist
}

fn main() {
    let playerlist = get_players();
    let num_players = playerlist.len();

    let mut player_names_mut: Vec<&str> = Vec::with_capacity(num_players);
    let mut player_mmrs_mut: Vec<i32> = Vec::with_capacity(num_players);

    for p in playerlist.iter() {
        player_names_mut.push(p.name);
        player_mmrs_mut.push(p.mmr);
    }
    
    let _player_names = player_names_mut.into_boxed_slice();
    let player_mmrs = player_mmrs_mut.into_boxed_slice();


    let mut max_team_diff = 1000;
    let not_allowed_sameteam_idxs = get_not_allowed_sameteam_idxs(player_mmrs.clone(), max_team_diff);

    let max_match_diff = 0;

    let result = solver(num_players, player_mmrs, max_match_diff, not_allowed_sameteam_idxs);

    match result {
        Ok(teams) => {
            println!("team 1:");
            
            /*for i in teams.0.into_iter() {
                i.no();
                print!("{}", playerlist[i]);
            }*/

            for i in 0..5 {
                print!("{}", playerlist[teams.0[i]]);
            }

            println!("\nteam 2:");

            for i in 0..5 {
                print!("{}", playerlist[teams.1[i]]);
            }

        }
        Err(e) if e == ResolutionError::Infeasible => {
            println!("Infeasible!")
            /*max_team_diff = 1100;
            result = solver(players_by_idx, num_players, player_mmrs, max_match_diff, not_allowed_sameteam_idxs);*/
        }
        Err(e) => panic!(e)
    }

    //get_chosen_players(result, playerlist);

    //print_result(result, playerlist)

}



fn solver(num_players: usize, mmr_vals: Box<[i32]>, max_match_diff: i32, not_allowed_sameteam_idxs: Vec<(usize, usize)>) -> Result<([usize;5], [usize;5]), ResolutionError> {

    let mut vars = ProblemVariables::new();

    // VARIABLES

    let team1_bools = vars.add_vector(variable().binary(), num_players);
    let team2_bools = vars.add_vector(variable().binary(), num_players);

    let x: Expression = (0..num_players).map(|i: usize| mmr_vals[i] * team1_bools[i]).sum();
    let y: Expression = (0..num_players).map(|i: usize| mmr_vals[i] * team2_bools[i]).sum();

    let z: Expression = x - y;
    
    
    let mut model = vars.minimise(&z).using(default_solver);

    
    // CONSTRAINTS

    // same player can't be on both teams
    for i in 0..num_players {
        model = model.with(constraint!(team1_bools[i] + team2_bools[i] <= 1));
    }
    // preventing 2 players on the same team from a large rating difference
    for (i1, i2) in not_allowed_sameteam_idxs{
        model = model.with(constraint!(team1_bools[i1] + team1_bools[i2] <= 1))
                    .with(constraint!(team2_bools[i1] + team2_bools[i2] <= 1));
    }
    // each team must have 5 players
    model = model.with(constraint!(team1_bools.iter().sum::<Expression>() == 5))
                .with(constraint!(team2_bools.iter().sum::<Expression>() == 5))
    // setting maximum difference in rating between the teams
                .with(constraint!(z.clone() <= max_match_diff))
                .with(constraint!(z >= 0));

    let result = model.solve();
    

    //if result == CoinCbcSolution {
    match result {
        Ok(solution) => {
            let mut team1: [usize; 5] = [0; 5];
            let mut team2: [usize; 5] = [0; 5];
            let mut team_count = 0;

            for (i, v) in team1_bools.into_iter().map(|v| solution.value(v)).enumerate() {
                //println!("{}", v);
                if v as i32 == 1 {
                    team1[team_count] = i;
                    team_count += 1;
                }
            }

            println!("{:?}", team1);


            team_count = 0;
            
            for (i, v) in team2_bools.into_iter().map(|v| solution.value(v)).enumerate() {
                //println!("{}", v);
                if v as i32 == 1 {
                    team2[team_count] = i;
                    team_count += 1;
                }
            }

            println!("{:?}", team2);

            Ok((team1, team2))

        } 
        Err(e) => {
            //println!("{}", e);
            Err(e)
        }
    }
}


#[inline(always)]
fn get_not_allowed_sameteam_idxs(mmr_vals: Box<[i32]>, max_team_diff: i32) -> Vec<(usize, usize)> {

    let not_allowed_sameteam = mmr_vals
        .iter()
        .enumerate()
        .combinations(2)
        .filter(|x| (x[0].1 - x[1].1).abs() > max_team_diff)
        .map(|x|
            (x[0].0, x[1].0)
        )
        .collect::<Vec<(usize, usize)>>();
    
    //println!("not_allowed_sameteam: {:?}", not_allowed_sameteam);
    not_allowed_sameteam
}


//fn get_chosen_players() -> ([usize;5], [usize;5]) {
    
//}
