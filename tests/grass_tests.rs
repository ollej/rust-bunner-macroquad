use bunner_macroquad::grass::Grass;
use bunner_macroquad::hedge_mask::HedgeMask;
use macroquad::rand::gen_range;
use macroquad::rand::srand;

#[test]
fn generate_hedge_mask_length() {
    let mask = Grass::generate_hedge_mask();
    assert_eq!(mask.len(), 14);
}

#[test]
fn generate_hedge_mask_beginning_hole() {
    srand(42);
    let mask = Grass::generate_hedge_mask();
    assert_eq!(
        mask,
        vec![
            HedgeMask::Empty,
            HedgeMask::Empty,
            HedgeMask::Empty,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
        ]
    );
}

#[test]
fn generate_hedge_mask_middle_hole() {
    srand(1);
    let mask = Grass::generate_hedge_mask();
    assert_eq!(
        mask,
        vec![
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Empty,
            HedgeMask::Empty,
            HedgeMask::Empty,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
            HedgeMask::Hedge,
        ]
    );
}
