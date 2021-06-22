
use itertools::Itertools;
use good_lp::{constraint, default_solver, Solution, SolverModel, Expression, variable, ProblemVariables};
use good_lp::solvers::coin_cbc::CoinCbcSolution;
use good_lp::solvers::ResolutionError;
use std::fmt;

#[allow(non_camel_case_types)]
struct player {
    name: String,
    mmr: i32,
}

impl fmt::Display for player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}\nmmr: {}\n", self.name, self.mmr)
    }
} 



fn get_players() -> Vec<player> {
    let players = vec![
        player{
            name: "Kipperacer".to_owned(),
            mmr: 2000
        },
        player{
            name: "Not Brad".to_owned(),
            mmr: 3900
        },
        player{
            name: "Aayan".to_owned(),
            mmr: 1900
        },
        player{
            name: "hans solo#6857".to_owned(),
            mmr: 2800
        },
        player{
            name: "V4lhallaRSL".to_owned(),
            mmr: 3000
        },
        player{
            name: "hans solo#6857".to_owned(),
            mmr: 2800
        },
        player{
            name: "King Size Ultra Krabby Supreme (Snake)".to_owned(),
            mmr: 1900
        },
        player{
            name: "r1sen__".to_owned(),
            mmr: 2400
        },
        player{
            name: "yogurthb".to_owned(),
            mmr: 2800
        },
        player{
            name: "Dexterity.none".to_owned(),
            mmr: 2300
        },
        player{
            name: "4000dpi".to_owned(),
            mmr: 1800
        },
        player{
            name: "crypto".to_owned(),
            mmr: 2600
        },
        player{
            name: "renaxla".to_owned(),
            mmr: 2400
        }
    ];
    
    /*for p in players.iter() {
        println!("{}", p);
    }*/
    
    players
}

fn main() {
    let playerlist = get_players();
    let num_players = playerlist.len();

    let mut player_names_mut: Vec<String> = Vec::with_capacity(num_players);
    let mut player_mmrs_mut: Vec<i32> = Vec::with_capacity(num_players);

    for p in playerlist.iter() {
        player_names_mut.push(p.name.clone());
        player_mmrs_mut.push(p.mmr);
    }
    
    let _player_names = player_names_mut.into_boxed_slice();
    let player_mmrs = player_mmrs_mut.into_boxed_slice();


    let mut max_team_diff = 1000;
    let not_allowed_sameteam_idxs = get_not_allowed_sameteam_idxs(player_mmrs.clone(), max_team_diff);

    let players_by_idx: Vec<usize> = (0..num_players).collect();
    let max_match_diff = 0;

    let result = solver(players_by_idx, num_players, player_mmrs, max_match_diff, not_allowed_sameteam_idxs);

    //assert_eq!(&result.err(), &Some(ResolutionError::Infeasible));

    match result {
        Ok(teams) => {
            //println!("team1: {:?}, team2: {:?}", teams.0, teams.1)
            println!("team 1:");
            for &i in teams.0.into_iter() {
                print!("{}", playerlist[i]);
            }

            println!("\nteam 2:");

            for &i in teams.1.into_iter() {
                print!("{}", playerlist[i]);
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



fn solver(players_by_idx: Vec<usize>, num_players: usize, mmr_vals: Box<[i32]>, max_match_diff: i32, not_allowed_sameteam_idxs: Vec<(usize, usize)>) -> Result<([usize;5], [usize;5]), ResolutionError> {

    let mut vars = ProblemVariables::new();

    // VARIABLES

    let team1_bools = vars.add_vector(variable().binary(), num_players);
    let team2_bools = vars.add_vector(variable().binary(), num_players);

    let x: Expression = players_by_idx.iter().map(|i: &usize| mmr_vals[*i] * team1_bools[*i]).sum();
    let y: Expression = players_by_idx.iter().map(|i: &usize| mmr_vals[*i] * team2_bools[*i]).sum();

    let z: Expression = x - y;
    
    //let max_team_members = vars.add(variable().min(5).max(5));
    //let binary_true = vars.add(variable().binary().min(1));
    
    let mut model = vars.minimise(&z).using(default_solver);

    
    // CONSTRAINTS

    // same player can't be on both teams
    for i in players_by_idx {
        model = model.with(constraint!(team1_bools[i] + team2_bools[i] <= 1));
    }
    // each team must have 5 players
    model = model.with(constraint!(team1_bools.iter().sum::<Expression>() == 5));
    model = model.with(constraint!(team2_bools.iter().sum::<Expression>() == 5));
    // setting maximum difference in rating between the teams
    model = model.with(constraint!(z.clone() <= max_match_diff));
    model = model.with(constraint!(z.clone() >= 0));
    // preventing 2 players on the same team from a large rating difference
    for (i1, i2) in not_allowed_sameteam_idxs{
        model = model.with(constraint!(team1_bools[i1] + team1_bools[i2] <= 1));
        model = model.with(constraint!(team2_bools[i1] + team2_bools[i2] <= 1));
    }

    let result = model.solve();
    
    //result    

    //if result == CoinCbcSolution {
    match result {
        Ok(Solution) => {
            let mut team1: [usize; 5] = [0; 5];
            let mut team2: [usize; 5] = [0; 5];
            let mut team_count = 0;

            for (i, v) in team1_bools.into_iter().map(|v| Solution.value(v)).enumerate() {
                //println!("{}", v);
                if v == 1.0 {
                    team1[team_count] = i;
                    team_count += 1;
                }
            }

            println!("{:?}", team1);


            team_count = 0;
            
            for (i, v) in team2_bools.into_iter().map(|v| Solution.value(v)).enumerate() {
                //println!("{}", v);
                if v == 1.0 {
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



fn get_not_allowed_sameteam_idxs(mmr_vals: Box<[i32]>, max_team_diff: i32) -> Vec<(usize, usize)> {
    
    let mut not_allowed_sameteam: Vec<(usize, usize)> = Vec::new();

    let pairs_diff_too_big = mmr_vals
        .iter()
        .combinations(2)
        .filter(|x| (x[0] - x[1]).abs() > max_team_diff);


    for invalid_pair in pairs_diff_too_big {

        let (p1, p2) = (invalid_pair[0], invalid_pair[1]);
        let mut p1_idxs: Vec<usize> = Vec::with_capacity(4);
        let mut p2_idxs: Vec<usize> = Vec::with_capacity(4);

        for (i, x) in mmr_vals.iter().enumerate() {
            if x == p1 {
                p1_idxs.push(i);
            } else if x == p2 {
                p2_idxs.push(i);
            }
        }

        for i1 in p1_idxs {
            for &i2 in p2_idxs.iter() {
                not_allowed_sameteam.push((i1, i2))
            }
        }
    }

    //println!("{:?}", not_allowed_sameteam)
    not_allowed_sameteam
}


//fn get_chosen_players() -> ([usize;5], [usize;5]) {
    
//}
