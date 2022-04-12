use crate::defs;
use crate::defs::*;
use arrayvec::ArrayVec;
use std::collections::HashSet;

pub mod logic;

/// Game state and configuration
/// TODO: Using ArrayVec here to see if we can keep everything on the stack.
/// Could experiment with using Vec as an alternative.

/// Configuration of a specific game (number of operators, difficulty, etc...)
/// Does not change for the duration of an entire game.
#[derive(Debug)]
pub struct GameConfig {
    /// Operators selected to be in this game in clockwise order.
    /// Max 7, and all must be unique.
    operators: ArrayVec<OperatorType, 7>,
    difficulty: Difficulty,
}

impl GameConfig {
    /// Gets a new, validated gameconfig.
    pub fn new(
        difficulty: Difficulty,
        operators: ArrayVec<OperatorType, 7>,
    ) -> Result<GameConfig, GameConfigError> {
        if operators.is_empty() {
            return Result::Err(GameConfigError::NoOperators);
        }

        let mut uniq = HashSet::new();
        for operator in operators.iter() {
            if uniq.contains(operator) {
                return Result::Err(GameConfigError::DuplicateOperator(*operator));
            }
            uniq.insert(operator);
        }

        Result::Ok(GameConfig {
            difficulty,
            operators,
        })
    }

    pub fn operators(&self) -> &ArrayVec<OperatorType, 7> {
        &self.operators
    }
    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }
}

#[derive(Debug)]
pub enum GameConfigError {
    /// duplicate operator in list
    DuplicateOperator(OperatorType),
    /// no operators provided
    NoOperators,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Heroic,
}

/// Entire state of an ongoing game. This + a GameConfig should contain EVERYTHING needed
/// to fully describe a state of the game (i.e., a snapshot of this would allow
/// saving / resuming the game).
pub struct TableState {
    /// amount of firewalls still standing
    firewalls: u8,
    /// remaining databases: rest, firewall, discard
    databases: [bool; 3],
    /// remaining webservices: compromise, compromise, burnout, burnout, compromise webservice, database
    webservices: [bool; 6],
    /// hacker stack - hackers randomly selected to be in this game
    hackers: HackerDeck,
    /// hackers let through this round
    breach: HackerDeck,
    /// discarded hackers
    discard: HackerDeck,
    /// round 0, 1, or 2
    round: u8,
    active_operator: OperatorID,
    /// Status of each operator, corresponds with GameConfig.operators
    operators: ArrayVec<OperatorState, 7>,
    /// current decision that needs to be made by a operator
    choice_state: ChoiceState,
}

impl TableState {
    pub fn firewalls(&self) -> u8 {
        self.firewalls
    }
    pub fn databases(&self) -> [bool; 3] {
        self.databases
    }
    pub fn webservices(&self) -> [bool; 6] {
        self.webservices
    }
    pub fn hackers(&self) -> &HackerDeck {
        &self.hackers
    }
    pub fn breach(&self) -> &HackerDeck {
        &self.breach
    }
    pub fn discard(&self) -> &HackerDeck {
        &self.discard
    }
    pub fn round(&self) -> u8 {
        self.round
    }
    pub fn active_operator(&self) -> OperatorID {
        self.active_operator
    }
    pub fn operators(&self) -> &ArrayVec<OperatorState, 7> {
        &self.operators
    }
    pub fn choice_state(&self) -> &ChoiceState {
        &self.choice_state
    }
}

/// Operator in current game. Index in TableState.operators and GameConfig.operators
/// NOT a OperatorTYPEId.
type OperatorID = u8;

/// a deck of hacker cards. The top is the end of the vec, bottom is the start.
type HackerDeck = ArrayVec<HackerCard, 66>;

#[derive(Copy, Clone)]
pub struct HackerCard {
    hacker: HackerID,
    /// true if faceup (visible to players), otherwise facedown
    face_up: bool,
}

impl HackerCard {
    pub fn new(hacker: HackerID) -> HackerCard {
        HackerCard {
            hacker,
            face_up: false,
        }
    }
}

pub struct OperatorState {
    /// hackers on left side of the operator board,
    /// in the Secure slots.
    /// index in array: index in defs::SYMBOLS
    /// value: hacker placed there, or defs::NO_HACKER
    secure_slots: [HackerID; 3],
    /// hackers on right side of operator board - backtrace list - end of array = bottom (i.e. most recently placed)
    /// start = top
    backtrace_list: ArrayVec<HackerID, 13>,
    /// whether there is a burnout token
    burnout: bool,
    /// whether they are in desperation mode
    desperation: bool,
    /// which skills the operator currently has, including their own + any assist
    skills: ArrayVec<OperatorType, 7>,
}

impl OperatorState {
    /// New operator in initial state they should be in at start of a game
    pub fn new(operator: &OperatorType) -> OperatorState {
        return OperatorState {
            secure_slots: [NO_HACKER; 3],
            backtrace_list: ArrayVec::new(),
            burnout: false,
            desperation: false,
            skills: ArrayVec::from_iter([*operator]),
        };
    }

    pub fn secure_slots(&self) -> [HackerID; 3] {
        self.secure_slots
    }
    pub fn backtrace_list(&self) -> &ArrayVec<HackerID, 13> {
        &self.backtrace_list
    }
    pub fn skills(&self) -> &ArrayVec<OperatorType, 7> {
        &self.skills
    }

    pub fn burnout(&self) -> bool {
        self.burnout
    }
    pub fn desperation(&self) -> bool {
        self.desperation
    }
}

/// Discrete states of the game where player input is required. Each state has associated actions
/// which can be performed by players (via their operators). All states
/// must represent situations where a player has some choice to make
/// (including cases where they normally have a choice, but there is only one valid choice).
///
/// We do NOT encode "intermediate" states in here which might represent
/// some internal step of processing - these are handled internally and
/// should be invisible to the client.
///
/// Note we have active_operator in the game state, but some of these enums
/// still have a OperatorID - this is because sometimes choices need to be
/// made by operators other than the active operator.
pub enum ChoiceState {
    /// Specific operator must decide whether to use their Flow or not
    Flow(OperatorID),
    /// Charm (desperation mode) must choose who to heal with their flow
    CharmDesperationFlow,
    /// Biggs must choose whose backtrace line to take
    /// a card from and who should receive it. The choice of
    /// where to place it will be a separate ChoiceState (Face or Skill if applicable)
    BiggsFlow,
    /// Biggs must choost whether to use their flow a second time
    BiggsDesperationFlow,
    /// Indicated operator must choose whether to place card to left or right
    Face(OperatorID),
    /// Indicated operator must choose whether to use one of their applicable skills.
    Skill(OperatorID),
    /// Indicated operator must choose a card to discard from the left of their board
    DiscardLeft(OperatorID),
    /// Indicated operator must choose to Face, Assist, or Idle
    ChooseAction(OperatorID),

    /// Game is over, only action is to quit or start a new one.
    GameOver,
}

#[cfg(test)]
mod tests {
    use super::*;
    use OperatorType::*;

    #[test]
    fn valid_game_config() {
        let operators = [Biggs, Charm, Sniper];
        let config =
            GameConfig::new(Difficulty::Easy, ArrayVec::from_iter(operators.clone())).unwrap();

        assert!(matches!(config.difficulty(), Difficulty::Easy));
        assert!(config.operators.iter().eq(operators.iter()));
    }

    #[test]
    fn requires_operators() {
        let operators = [Biggs, Charm, Sniper];
        let config = GameConfig::new(Difficulty::Easy, ArrayVec::new()).unwrap_err();
        assert!(matches!(config, GameConfigError::NoOperators));
    }

    #[test]
    fn requires_unique_operators() {
        validate_unique_operators(3, vec![Biggs, Charm, Sniper, Charm]);
        validate_unique_operators(1, vec![Biggs, Biggs, Sniper, Charm]);
        validate_unique_operators(3, vec![Biggs, Sniper, Charm, Charm]);
    }

    fn validate_unique_operators(dupe_idx: u8, operators: Vec<OperatorType>) {
        let config = GameConfig::new(Difficulty::Easy, ArrayVec::from_iter(operators)).unwrap_err();
        assert!(matches!(
            config,
            GameConfigError::DuplicateOperator(dupe_idx)
        ));
    }
}
