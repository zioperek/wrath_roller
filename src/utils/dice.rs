use rand::Rng;

fn gen_random_number(max: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=max)
}

fn roll_a_dice() -> u8 {
    gen_random_number(6)
}

fn roll_dices(num_of_dice: u8) -> Vec<u8> {
    let mut dices: Vec<u8> = vec![];
    for _ in 0..num_of_dice {
        dices.push(roll_a_dice())
    }
    dices
}

fn determine_fury_dice(rolled_hand: &mut Vec<u8>) -> u8 {
    let special_die_pos = gen_random_number(rolled_hand.len() as u8) - 1;
    rolled_hand.remove(special_die_pos as usize)
}

pub fn get_full_roll(dices: u8) -> (Vec<u8>, u8, u8) {
    let mut rolls: Vec<u8> = roll_dices(dices);
    let mut number_of_successes = 0;
    for dice in rolls.iter() {
        if *dice == 6 {
            number_of_successes += 2;
        } else if *dice <= 3 {
        } else {
            number_of_successes += 1
        }
    }
    let fury_dice = determine_fury_dice(&mut rolls);

    (rolls, number_of_successes, fury_dice)
}
