/// Actual logic to run a complete game
use super::{GameConfig, TableState};
use crate::defs;
use crate::defs::{OperatorType, NO_HACKER};
use crate::game::ChoiceState::ChooseAction;
use crate::game::Difficulty::Easy;
use crate::game::{
    Choice, Difficulty, HackerCard, HackerDeck, OperatorID, OperatorState, TableEvent,
};
use arrayvec::ArrayVec;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;
use std::process::id;
use TableEvent::*;

// TODO: Convert to impl
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

impl TableState {
    /// Returns a tablestate fully setup in accordance with
    /// the provided game config, ready for the first operator to perform their turn.
    pub fn setup_game(config: &GameConfig) -> TableState {
        let (firewall_mod, hacker_mult) = difficulty_mod(&config.difficulty);
        TableState {
            firewalls: (config.operators.len() + firewall_mod) as u8,
            databases: [true; 3],
            webservices: [true; 6],
            hackers: shuffle(config.operators.len() * hacker_mult),
            breach: HackerDeck::new(),
            discard: HackerDeck::new(),
            round: 0,
            facing: NO_HACKER,
            active_operator: 0,
            operators: init_operators(&config.operators),
            choice_state: ChooseAction(0),
        }
    }

    /// Returns the valid choices that can be performed based on current game state
    pub fn valid_choices(&self) -> Vec<Choice> {
        match self.choice_state {
            ChooseAction(operator) => {
                let mut choices = vec![Choice::Idle];
                if !self.hackers.is_empty() {
                    choices.push(Choice::Face);
                }
                for i in 0..self.operators.len() {
                    if i != operator as usize {
                        choices.push(Choice::Assist(i as OperatorID));
                    }
                }
                choices
            }
            _ => panic!("choice state not implemented"),
        }
    }

    /// Perform the indicated action. TableState will be updated until next choice state is
    /// reached. Returns a vec consisting of events that occurred during the updates, in the
    /// order they happened.
    pub fn choose(&self, choice: Choice) -> Vec<TableEvent> {
        match choice {
            _ => panic!("choice not implemented"),
        }
    }

    /// Update TableState corresponding with what the event says to do.
    fn perform(&mut self, event: TableEvent) {
        match event {
            FirewallDelta(delta) => {
                let result = (self.firewalls as i8) + delta;
                if !(0..=3).contains(&result) {
                    panic!(
                        "delta out of range - firewalls must remain between 0..=3, cur {} delta {}",
                        self.firewalls, delta
                    );
                }
                self.firewalls = result as u8;
            }
            DatabaseRemove(idx) => {
                if !(0..3).contains(&idx) {
                    panic!("database index out of range, must be 0..=2, was {}", idx);
                }
                let idx = idx as usize;
                if !self.databases[idx] {
                    panic!("database index {} already removed", idx);
                }
                self.databases[idx] = false;
            }
            WebserviceRemove(idx) => {
                if !(0..5).contains(&idx) {
                    panic!("webservice index out of range, must be 0..=5, was {}", idx);
                }
                let idx = idx as usize;
                if !self.webservices[idx] {
                    panic!("webservice index {} already removed", idx);
                }
                self.webservices[idx] = false;
            }
            Face => {
                if self.facing != NO_HACKER {
                    panic!("cannot face, already facing HackerID {}", self.facing);
                }
                if self.hackers.is_empty() {}
                match self.hackers.pop() {
                    Some(x) => self.facing = x.hacker,
                    None => panic!("cannot face, hacker deck is empty"),
                }
            }
            Idle => {
                if self.active_operator().idle {
                    panic!(
                        "cannot idle, operator {} already idle",
                        self.active_operator
                    );
                }
                self.active_operator().idle = true;
            }
            ChoiceState(x) => {
                self.choice_state = x;
            }
            _ => panic!("event not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Difficulty, GameConfig};
    use super::*;
    use crate::defs;
    use crate::defs::{OperatorType, NO_HACKER};
    use crate::game::{ChoiceState, OperatorID};
    use arrayvec::ArrayVec;
    use spectral::prelude::*;
    use test_case::test_case;

    static OPERATORS: [OperatorType; 7] = [
        OperatorType::Stone,
        OperatorType::Sniper,
        OperatorType::Rogue,
        OperatorType::Biggs,
        OperatorType::Admin,
        OperatorType::Charm,
        OperatorType::Rich,
    ];

    fn get_operators(operators: usize) -> ArrayVec<OperatorType, 7> {
        let chosen_operators = &OPERATORS[0..operators];
        ArrayVec::from_iter(chosen_operators.iter().copied())
    }

    /// Basic initial state with easy difficulty and 2 operators
    fn initial_state(difficulty: Difficulty) -> TableState {
        TableState::setup_game(&GameConfig::new(difficulty, get_operators(2)).unwrap())
    }

    fn initial_state_easy() -> TableState {
        initial_state(Difficulty::Easy)
    }

    #[test_case(1, Difficulty::Easy)]
    #[test_case(2, Difficulty::Easy)]
    #[test_case(3, Difficulty::Hard)]
    #[test_case(4, Difficulty::Heroic)]
    #[test_case(7, Difficulty::Heroic)]
    #[test_case(7, Difficulty::Easy)]
    #[test_case(7, Difficulty::Normal)]
    fn sets_up_game(operators: usize, difficulty: Difficulty) {
        let (firewall_mod, hacker_mult) = difficulty_mod(&difficulty);
        let config = GameConfig::new(difficulty, get_operators(operators)).unwrap();

        let state = TableState::setup_game(&config);
        assert_eq!(
            state.firewalls,
            (operators + firewall_mod) as u8,
            "firewalls = operators + 2"
        );
        assert_eq!(state.databases, [true; 3]);
        assert_eq!(state.webservices, [true; 6]);

        assert_eq!(
            state.hackers.len(),
            operators * hacker_mult,
            "hackers = operators * 7"
        );
        for hacker in state.hackers.iter() {
            assert!(!hacker.face_up, "face down");
            let hacker = defs::hacker(hacker.hacker);
            assert!(hacker.value() <= 4, "not lieutenant or boss")
        }

        assert!(state.discard.is_empty());
        assert!(state.breach.is_empty());
        assert_eq!(state.round, 0);
        assert_eq!(state.active_operator, 0);

        assert_eq!(state.operators.len(), operators);
        for (i, operator) in state.operators.iter().enumerate() {
            assert_eq!(operator.secure_slots, [NO_HACKER; 3]);
            assert!(operator.backtrace_list.is_empty());
            assert_that(&operator.idle).is_false();
            assert_eq!(operator.skills.len(), 1);
            assert_eq!(operator.skills[0], OPERATORS[i]);
        }

        assert!(matches!(state.choice_state, ChooseAction(0)));
    }

    #[test_case(1, false)]
    #[test_case(1, true)]
    #[test_case(7, true)]
    #[test_case(7, false)]
    fn valid_choice_choose_action(operators: usize, has_hackers: bool) {
        let mut state =
            TableState::setup_game(&GameConfig::new(Easy, get_operators(operators)).unwrap());
        if !has_hackers {
            state.hackers.clear();
        }
        let choices = state.valid_choices();
        let mut expected_choices = vec![Choice::Idle];
        if has_hackers {
            expected_choices.push(Choice::Face);
        }
        for i in 0..operators {
            if i != state.active_operator as usize {
                expected_choices.push(Choice::Assist(i as OperatorID));
            }
        }
        assert_that(&choices.iter()).equals_iterator(&expected_choices.iter());
    }

    #[test_case(2, 1, 3)]
    #[test_case(1, - 1, 0)]
    #[test_case(3, - 2, 1)]
    #[test_case(3, - 3, 0)]
    #[test_case(0, 3, 3)]
    fn perform_firewall_delta_valid(initial: u8, delta: i8, expected: u8) {
        let state = firewall_delta(initial, delta);
        assert_that(&state.firewalls).is_equal_to(expected);
    }

    #[test]
    #[should_panic(
        expected = "delta out of range - firewalls must remain between 0..=3, cur 0 delta -1"
    )]
    fn perform_firewall_delta_invalid() {
        firewall_delta(0, -1);
    }

    #[test]
    #[should_panic(
        expected = "delta out of range - firewalls must remain between 0..=3, cur 2 delta 2"
    )]
    fn perform_firewall_delta_invalid_2() {
        firewall_delta(2, 2);
    }

    fn firewall_delta(initial: u8, delta: i8) -> TableState {
        let mut state = initial_state_easy();
        state.firewalls = initial;
        state.perform(FirewallDelta(delta));
        state
    }

    #[test_case([false, true, false], 1, [false, false, false])]
    #[test_case([true, true, false], 0, [false, true, false])]
    #[test_case([true, true, true], 2, [true, true, false])]
    fn perform_database_remove_valid(initial: [bool; 3], delta: u8, expected: [bool; 3]) {
        let state = database_remove(initial, delta);
        assert_that(&state.databases).is_equal_to(&expected);
    }

    #[test]
    #[should_panic(expected = "database index out of range, must be 0..=2, was 3")]
    fn perform_database_remove_invalid() {
        database_remove([true, false, true], 3);
    }

    #[test]
    #[should_panic(expected = "database index 1 already removed")]
    fn perform_database_remove_invalid_2() {
        database_remove([true, false, true], 1);
    }

    fn database_remove(initial: [bool; 3], idx: u8) -> TableState {
        let mut state = initial_state_easy();
        state.databases = initial;
        state.perform(DatabaseRemove(idx));
        state
    }

    #[test_case([false, true, false, false, false, false], 1, [false, false, false, false, false, false])]
    #[test_case([true, true, false, false, false, false], 0, [false, true, false, false, false, false])]
    #[test_case([true, true, true, false, false, false], 2, [true, true, false, false, false, false])]
    fn perform_webservice_remove_valid(initial: [bool; 6], delta: u8, expected: [bool; 6]) {
        let state = webservice_remove(initial, delta);
        assert_that(&state.webservices).is_equal_to(&expected);
    }

    #[test]
    #[should_panic(expected = "webservice index out of range, must be 0..=5, was 6")]
    fn perform_webservice_remove_invalid() {
        webservice_remove([true, false, true, false, false, false], 6);
    }

    #[test]
    #[should_panic(expected = "webservice index 1 already removed")]
    fn perform_webservice_remove_invalid_2() {
        webservice_remove([true, false, true, false, false, false], 1);
    }

    fn webservice_remove(initial: [bool; 6], idx: u8) -> TableState {
        let mut state = initial_state_easy();
        state.webservices = initial;
        state.perform(WebserviceRemove(idx));
        state
    }

    #[test]
    fn perform_face() {
        let mut state = initial_state_easy();
        let mut expected_hackers = state.hackers.clone();
        let expected_face = expected_hackers.pop().unwrap();
        state.perform(Face);
        assert_that(&state.hackers.iter()).equals_iterator(&expected_hackers.iter());
        assert_that(&state.facing).is_equal_to(&expected_face.hacker);
    }

    #[test]
    #[should_panic(expected = "cannot face, hacker deck is empty")]
    fn perform_face_invalid_deck() {
        let mut state = initial_state_easy();
        state.hackers.clear();
        state.perform(Face);
    }

    #[test]
    #[should_panic(expected = "cannot face, already facing HackerID 3")]
    fn perform_face_already_facing() {
        let mut state = initial_state_easy();
        state.facing = 3;
        state.perform(Face);
    }

    #[test]
    fn perform_idle() {
        let mut state = initial_state_easy();
        state.perform(Idle);
        assert_that(&state.operators[0].idle).is_true();
    }

    #[test]
    #[should_panic(expected = "cannot idle, operator 0 already idle")]
    fn perform_idle_invalid() {
        let mut state = initial_state_easy();
        state.operators[0].idle = true;
        state.perform(Idle);
    }

    #[test]
    fn perform_choice_state() {
        let mut state = initial_state_easy();
        state.perform(ChoiceState(ChoiceState::Face(3)));
        assert_that(&state.choice_state).is_equal_to(ChoiceState::Face(3));
    }
}
