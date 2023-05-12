use std::env;
use std::fs;

struct Strategy {
    opp_hand: char,
    point_for_tie: i32,
    point_for_win: i32,
    point_for_loss: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Failed to read content from input file");

    let inputs_list: Vec<&str> = input.split('\n').collect();

    let mut total_points = 0;

    for i in inputs_list {
        let game_hands: Vec<char> = i.chars().filter(|n| n != &' ').collect();

        if game_hands.len() > 1 {
            total_points += play_game(game_hands[0], game_hands[1]);
        }
    }

    println!("Total: {}", total_points);
}

fn play_game(opp_pick: char, prediction: char) -> i32 {
    //points per use + win/loss/tie
    let strats = vec![
        Strategy {
            opp_hand: 'A',
            point_for_tie: 1 + 3,
            point_for_win: 2 + 6,
            point_for_loss: 3,
        },
        Strategy {
            opp_hand: 'B',
            point_for_tie: 2 + 3,
            point_for_win: 3 + 6,
            point_for_loss: 1,
        },
        Strategy {
            opp_hand: 'C',
            point_for_tie: 3 + 3,
            point_for_win: 1 + 6,
            point_for_loss: 2,
        },
    ];

    for strat in strats {
        if opp_pick == strat.opp_hand {
            if prediction == 'X' {
                return strat.point_for_loss;
            } else if prediction == 'Y' {
                return strat.point_for_tie;
            } else {
                return strat.point_for_win;
            }
        }
    }
    return 0;
}

