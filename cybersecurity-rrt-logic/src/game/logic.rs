use super::{GameConfig, TableState};
use crate::defs;
use crate::defs::OperatorType;
use crate::game::ChoiceState::ChooseAction;
use crate::game::Difficulty::Easy;
use crate::game::{Difficulty, HackerCard, HackerDeck, OperatorState};
use arrayvec::ArrayVec;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

/// Actual logic to run a complete game

/// Returns a tablestate fully setup in accordance with
/// the provided game config, ready for the first operator to perform their turn.
pub fn setup_game(config: &GameConfig) -> TableState {
    let (firewall_mod, hacker_mult) = difficulty_mod(config.difficulty());
    TableState {
        firewalls: (config.operators().len() + firewall_mod) as u8,
        databases: [true; 3],
        webservices: [true; 6],
        hackers: shuffle(config.operators().len() * hacker_mult),
        breach: HackerDeck::new(),
        discard: HackerDeck::new(),
        round: 0,
        active_operator: 0,
        operators: init_operators(config.operators()),
        choice_state: ChooseAction(0),
    }
}
/// Gets (firewall mod, hacker_multiplier) depending on difficulty
fn difficulty_mod(difficulty: &Difficulty) -> (usize, usize) {
    match difficulty {
        Easy => (3, 6),
        Normal => (2, 7),
        Hard => (1, 7),
        Heroic => (0, 7),
    }
}

fn init_operators(operators: &ArrayVec<OperatorType, 7>) -> ArrayVec<OperatorState, 7> {
    ArrayVec::from_iter(operators.iter().map(|x| OperatorState::new(x)))
}

/// Shuffle initial hacker deck, with `hackers` number of hacker
/// cards, chosen randomly without replacement from 1-4 value range
fn shuffle(hackers: usize) -> HackerDeck {
    // TODO: Is there a more efficient way?
    let mut rng = rand::thread_rng();
    let mut valid_hackers: Vec<HackerCard> = defs::HACKERS
        .iter()
        .enumerate()
        .filter(|(_, x)| x.value() <= 4)
        .map(|(x, _)| HackerCard::new(x as u8))
        .collect();
    valid_hackers.shuffle(&mut rng);

    return HackerDeck::from_iter(valid_hackers.iter().take(hackers).map(|x| *x));
}

#[cfg(test)]
mod tests {
    use super::super::{Difficulty, GameConfig};
    use super::*;
    use crate::defs;
    use crate::defs::{OperatorType, NO_HACKER};
    use arrayvec::ArrayVec;
    use test_case::test_case;

    #[test_case(1, Difficulty::Easy)]
    #[test_case(2, Difficulty::Easy)]
    #[test_case(3, Difficulty::Hard)]
    #[test_case(4, Difficulty::Heroic)]
    #[test_case(7, Difficulty::Heroic)]
    #[test_case(7, Difficulty::Easy)]
    #[test_case(7, Difficulty::Normal)]
    fn sets_up_game(operators: usize, difficulty: Difficulty) {
        let operator_order = [
            OperatorType::Stone,
            OperatorType::Sniper,
            OperatorType::Rogue,
            OperatorType::Biggs,
            OperatorType::Admin,
            OperatorType::Charm,
            OperatorType::Rich,
        ];
        let chosen_operators = &operator_order[0..operators];
        let (firewall_mod, hacker_mult) = difficulty_mod(&difficulty);

        let config = GameConfig::new(
            difficulty,
            ArrayVec::from_iter(chosen_operators.iter().map(|x| *x)),
        )
        .unwrap();

        let state = setup_game(&config);
        assert_eq!(
            state.firewalls(),
            (operators + firewall_mod) as u8,
            "firewalls = operators + 2"
        );
        assert_eq!(state.databases(), [true; 3]);
        assert_eq!(state.webservices(), [true; 6]);

        assert_eq!(
            state.hackers().len(),
            operators * hacker_mult,
            "hackers = operators * 7"
        );
        for hacker in state.hackers().iter() {
            assert!(!hacker.face_up, "face down");
            let hacker = defs::hacker(hacker.hacker);
            assert!(hacker.value() <= 4, "not lieutenant or boss")
        }

        assert!(state.discard().is_empty());
        assert!(state.breach().is_empty());
        assert_eq!(state.round(), 0);
        assert_eq!(state.active_operator(), 0);

        assert_eq!(state.operators().len(), operators);
        for (i, operator) in state.operators().iter().enumerate() {
            assert_eq!(operator.secure_slots(), [NO_HACKER; 3]);
            assert!(operator.backtrace_list().is_empty());
            assert_eq!(operator.skills().len(), 1);
            assert_eq!(operator.skills()[0], operator_order[i]);
        }

        assert!(matches!(state.choice_state(), ChooseAction(0)));
    }
}
