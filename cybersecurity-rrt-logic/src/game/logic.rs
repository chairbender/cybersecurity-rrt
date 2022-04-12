use super::{GameConfig, TableState};
use crate::game::ChoiceState::ChooseAction;
use crate::game::HackerDeck;
use arrayvec::ArrayVec;

/// Actual logic to run a complete game

/// Returns a tablestate fully setup in accordance with
/// the provided game config, ready for the first operator to perform their turn.
pub fn setup_game(config: &GameConfig) -> TableState {
    TableState {
        firewalls: 0,
        databases: [false; 3],
        webservices: [false; 6],
        hackers: HackerDeck::new(),
        breach: HackerDeck::new(),
        discard: HackerDeck::new(),
        round: 0,
        active_operator: 0,
        operators: ArrayVec::new(),
        choice_state: ChooseAction(0),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Difficulty, GameConfig};
    use super::*;
    use crate::defs;
    use crate::defs::{OperatorType, NO_HACKER};
    use arrayvec::ArrayVec;
    // TODO: use test_case::test_case;

    #[test]
    fn sets_up_game() {
        let operator_order = [OperatorType::Biggs, OperatorType::Sniper];
        // TODO: use test_case, Parameterize num of operators + difficulty
        let config =
            GameConfig::new(Difficulty::Easy, ArrayVec::from_iter(operator_order)).unwrap();

        let state = setup_game(&config);
        assert_eq!(state.firewalls(), 4, "firewalls = operators + 2");
        assert_eq!(state.databases(), [true; 3]);
        assert_eq!(state.webservices(), [true; 6]);

        assert_eq!(state.hackers().len(), 14, "hackers = operators * 7");
        for hacker in state.hackers().iter() {
            assert!(!hacker.face_up, "face down");
            let hacker = defs::hacker(hacker.hacker);
            assert!(hacker.value() <= 4, "not lieutenant or boss")
        }

        assert!(state.discard().is_empty());
        assert!(state.breach().is_empty());
        assert_eq!(state.round(), 0);
        assert_eq!(state.active_operator(), 0);

        assert_eq!(state.operators().len(), 2);
        for (i, operator) in state.operators().iter().enumerate() {
            assert_eq!(operator.secure_slots(), [NO_HACKER; 3]);
            assert!(operator.backtrace_list().is_empty());
            assert_eq!(operator.skills().len(), 1);
            assert_eq!(operator.skills()[0], operator_order[i]);
        }
    }
}
