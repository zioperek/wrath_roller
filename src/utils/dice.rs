use rand::Rng;

fn gen_random_number(max: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=max)
}

fn roll_a_dice() -> u32 {
    gen_random_number(6)
}

fn roll_dices(num_of_dice: u32) -> Vec<u32> {
    let mut dices: Vec<u32> = vec![];
    for _ in 0..num_of_dice {
        dices.push(roll_a_dice())
    }
    dices
}

fn determine_fury_dice(rolled_hand: &Vec<u32>) -> Option<&u32> {
    let special_die_pos = gen_random_number(rolled_hand.len() as u32) - 1;
    rolled_hand.get(special_die_pos as usize)
}

#[derive(Debug)]
pub enum FuryDice {
    CriticalFailure,
    CriticalSuccess,
    Normal(u32),
}

enum Dice {
    Failure(u32),
    Normal(u32),
    Double,
}
pub fn get_full_roll(dices: u32) -> (Vec<u32>, u32, FuryDice) {
    let mut rolls: Vec<u32> = roll_dices(dices);
    let mut number_of_successes = 0;
    for dice in rolls.iter() {
        if *dice == 6 {
            number_of_successes += 2;
        } else if *dice <= 3 {
        } else {
            number_of_successes += 1
        }
    }
    let fury_dice = match determine_fury_dice(&mut rolls).unwrap() {
        6 => FuryDice::CriticalSuccess,
        1 => FuryDice::CriticalFailure,
        i => FuryDice::Normal(*i),
    };

    (rolls, number_of_successes, fury_dice)
}
