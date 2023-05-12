use std::env;
use std::fs;

struct Strategy {
    opp_hand: char,
    winning_hand: char,
    equal_hand: char,
    point_for_equal: i32,
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

fn play_game(opp_pick: char, my_pick: char) -> i32 {
    let strats = vec![
        Strategy {
            opp_hand: 'A',
            winning_hand: 'Y',
            equal_hand: 'X',
            point_for_equal: 1,
            point_for_win: 2,
            point_for_loss: 3,
        },
        Strategy {
            opp_hand: 'B',
            winning_hand: 'Z',
            equal_hand: 'Y',
            point_for_equal: 2,
            point_for_win: 3,
            point_for_loss: 1,
        },
        Strategy {
            opp_hand: 'C',
            winning_hand: 'X',
            equal_hand: 'Z',
            point_for_equal: 3,
            point_for_win: 1,
            point_for_loss: 2,
        },
    ];

    for strat in strats {
        if opp_pick == strat.opp_hand {
            if my_pick == strat.winning_hand {
                return 6 + strat.point_for_win;
            } else if my_pick == strat.equal_hand {
                return 3 + strat.point_for_equal;
            } else {
                return strat.point_for_loss;
            }
        }
    }
    return 0;
}
