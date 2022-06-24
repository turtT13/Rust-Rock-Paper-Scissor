extern crate rand;

use rand::Rng;
use std::io;
fn main() {
    let mut running = true;
    let mut set_score1: i32 = 0;
    let mut set_score2: i32 = 0;
    let mut round_score1: i8 = 0;
    let mut round_score2: i8 = 0;



    while running {
        println!("Enter (R)ock (P)aper or (S)cissors or (Q)uit");
        let mut user = String::new();
        io::stdin()
            .read_line(&mut user)
            .expect("Failed to read line");
        
        let input = String::from(user);
        let input = input.trim_end();

        let comp_move = get_computer();


        if input.eq("Q") {
            running = false
        } else if input.eq("R") {
            if comp_move == 'R' {
                print!("Rock vs Rock ");
                println!("Tie");
                print!("Round: {} - {}" , round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
            if comp_move == 'P' {
                round_score2 = round_score2 + 1;
                print!("Rock vs paper ");
                println!("Lose");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
            if comp_move == 'S' {
                round_score1 = round_score1 + 1;
                print!("Rock vs Scissors ");
                println!("Win");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
        } else if input.eq("P") {
            if comp_move == 'R' {
                round_score1 = round_score1 + 1;
                println!("Win");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
            if comp_move == 'P' {
                println!("Tie");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
            if comp_move == 'S' {
                round_score2 = round_score2 + 1;
                println!("Lose");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
        } else if input.eq("S") {
            if comp_move == 'R' {
                round_score2 = round_score2 + 1;
                println!("Lose");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
            if comp_move == 'P' {
                round_score1 = round_score1 + 1;
                println!("Win");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
            if comp_move == 'S' {
                println!("Tie");
                print!("Round: {} - {}", round_score1, round_score2);
                println!("Set: {} - {}", set_score1, set_score2);
            }
        
        } else {
            println!("Invalid please try again")
        }
        if round_score1 == 2{
            set_score1 = set_score1 + 1;
            round_score1 = 0;
            round_score2 = 0;
            print!("Round: {} - {} ", round_score1, round_score2);
            println!("Set: {} - {} ", set_score1, set_score2);
        }
        if round_score2 == 2{
            set_score2 = set_score2 + 1;
            round_score1 = 0;
            round_score2 = 0;
            print!("Round: {} - {} ", round_score1, round_score2);
            println!("Set: {} - {} ", set_score1, set_score2);
        }
    }

    println!("Bye bye");
}

fn get_computer() -> char {
    let random = rand::thread_rng().gen_range(0, 3);
    const CHOICES: [char; 3] = ['R', 'P', 'S'];
    let rsp = CHOICES[random];
    return rsp;
}
