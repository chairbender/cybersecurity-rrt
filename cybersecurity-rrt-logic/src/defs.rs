/// Definitions of all the game elements.
/// TODO: We could possibly encapsulate some of these things better rather
/// than relying on convention so much.
use crate::defs::OperatorType::*;
use crate::defs::Penalty::*;
use crate::defs::Symbol::*;

/// Definition of a operator's stats and type
pub struct Operator {
    operator: OperatorType,
    normal_track: u8,
    desperation_track: u8,
}

impl Operator {
    const STONE: Operator = Operator {
        operator: Stone,
        normal_track: 9,
        desperation_track: 12,
    };
    const SNIPER: Operator = Operator {
        operator: Sniper,
        normal_track: 9,
        desperation_track: 12,
    };
    const ROGUE: Operator = Operator {
        operator: Rogue,
        normal_track: 10,
        desperation_track: 13,
    };
    const BIGGS: Operator = Operator {
        operator: Biggs,
        normal_track: 8,
        desperation_track: 11,
    };
    const RICH: Operator = Operator {
        operator: Rich,
        normal_track: 10,
        desperation_track: 13,
    };
    const CHARM: Operator = Operator {
        operator: Charm,
        normal_track: 9,
        desperation_track: 11,
    };
    const ADMIN: Operator = Operator {
        operator: Admin,
        normal_track: 9,
        desperation_track: 12,
    };
}

/// The different unique operators (each operator has unique abilities, so
/// we only distinguish them by name)
#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
pub enum OperatorType {
    /// Skill: when facing attacker with value identical to one already in
    /// their backtrace list, can discard the attacker.
    ///
    /// Flow: Any operator can give their assist token to
    /// another operator. Desperation: can additionally add a firewall.
    Stone,
    /// Skill: Ignore penalties on attackers that have an even value
    ///
    /// Flow: Discard top 2 cards from Hacker stack. Desperation: top 3 cards.
    Sniper,
    /// Skill: Can operate a second time in a turn.
    ///
    /// Flow: Discard last card from any operator's backtrace list. Desperation: can use twice.
    Rogue,
    /// Skill: Can pass odd valued Hackers to a neighbor, who must then face it.
    ///
    /// Flow: Can take a Hacker from any Operator's backtrace list and give it to
    /// any other Operator, who can then put it in their backtrace list or
    /// secure slots. Desperation: Can do it twice.
    Biggs,
    /// Skill: When facing a Hacker, can put it on the bottom of the Hacker stack and
    /// face a new Hacker, who must then be faced.
    ///
    /// Flow: Turn the top 2 cards of Hacker stack face up and reorder them if desired.
    /// Desperation: top 3 cards.
    Rich,
    /// Skill: Can pass even valued Hackers to a neighbor, who must then face it.
    ///
    /// Flow: Can add one firewall. Desperation: Can add one firewall and remove
    /// one burnout token from any Operator. Note number of firewalls can never exceed the
    /// original amount as determined by operator count.
    Charm,
    /// Skill: Ignore penalties on attackers that have an odd value
    ///
    /// Flow: Discard top 2 cards from Breach stack. Desperation: top 3 cards.
    Admin,
}

pub fn operator(operator: &OperatorType) -> Operator {
    match operator {
        Stone => Operator::STONE,
        Sniper => Operator::SNIPER,
        Rogue => Operator::ROGUE,
        Rich => Operator::RICH,
        Biggs => Operator::BIGGS,
        Charm => Operator::CHARM,
        Admin => Operator::ADMIN,
    }
}

/// Definition of a particular hacker
pub struct Hacker {
    value: u8,
    virus: bool,
    penalty: Penalty,
    symbol: Symbol,
}

impl Hacker {
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn virus(&self) -> bool {
        self.virus
    }
    pub fn penalty(&self) -> &Penalty {
        &self.penalty
    }
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

/// Symbol on top right of hackers, which operators
/// need to secure one of each by end of turn in order
/// to not suffer consequences
pub enum Symbol {
    NoSymbol,
    Keyboard,
    Webservice,
    Database,
}
/// index in defs::SYMBOLS
pub type SymbolID = u8;
pub static SYMBOLS: [Symbol; 4] = [NoSymbol, Keyboard, Webservice, Database];

/// Penalties which enemies can inflict
pub enum Penalty {
    NoPenalty,
    /// Compromise a firewall, or webservice if no firewalls left.
    Compromise,
    /// inflict burnout on the operator
    Burnout,
    /// Place top card of Hacker stack on Breach stack, unseen
    Ninja,
    /// operator cannot Secure (place to left of their board) during their turn
    NoSecure,
    /// operator may not perform Assist action, but can receive it.
    NoGiveAssist,
    /// operator to left must draw a hacker and add to their backtrace list.
    DrawLeft,
    /// operator to right must draw a hacker and add to their backtrace list.
    DrawRight,
    /// Compromise x2
    DoubleCompromise,
    /// NoSecure + shuffle a random hacker from discard into hacker stack
    NoSecureAndHackerRevive,
    /// NoGiveAssist + Burnout
    NoGiveAssistAndBurnout,
    /// Discard a card from left side of operator's board
    DiscardSecure,
    /// Burnout + operator may not use their skill nor any assist tokens they have
    NoTalentAndBurnout,
    /// Ninja x2
    DoubleNinja,
    /// operator must choose to idle
    Idle,
}

/// TODO: Could encapsulate this stuff better so we avoid out of bounds indexing.
/// Index in defs::HACKERS; Note that NO_HACKER indicates no hacker and should not
/// be used to index into HACKERS
pub type HackerID = u8;
/// Indicates no hacker is there.
pub const NO_HACKER: HackerID = 66;
/// Indexable array of all the hackers in the game. ID = index in array. Note that
/// no 2 hackers are alike.
pub static HACKERS: [Hacker; 66] = [
    Hacker {
        value: 1,
        virus: true,
        symbol: Database,
        penalty: Burnout,
    },
    Hacker {
        value: 1,
        virus: true,
        symbol: Database,
        penalty: Compromise,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: Database,
        penalty: Ninja,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: Database,
        penalty: NoPenalty,
    },
    Hacker {
        value: 1,
        virus: true,
        symbol: Webservice,
        penalty: Compromise,
    },
    Hacker {
        value: 1,
        virus: true,
        symbol: Webservice,
        penalty: Burnout,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: Webservice,
        penalty: NoGiveAssist,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: Webservice,
        penalty: Ninja,
    },
    Hacker {
        value: 1,
        virus: true,
        symbol: Keyboard,
        penalty: Burnout,
    },
    Hacker {
        value: 1,
        virus: true,
        symbol: Keyboard,
        penalty: NoPenalty,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: Keyboard,
        penalty: Ninja,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: Keyboard,
        penalty: NoSecure,
    },
    Hacker {
        value: 1,
        virus: false,
        symbol: NoSymbol,
        penalty: Compromise,
    },
    Hacker {
        value: 2,
        virus: true,
        symbol: Database,
        penalty: Compromise,
    },
    Hacker {
        value: 2,
        virus: true,
        symbol: Database,
        penalty: Burnout,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: Database,
        penalty: Ninja,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: Database,
        penalty: NoPenalty,
    },
    Hacker {
        value: 2,
        virus: true,
        symbol: Webservice,
        penalty: Burnout,
    },
    Hacker {
        value: 2,
        virus: true,
        symbol: Webservice,
        penalty: Compromise,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: Webservice,
        penalty: Ninja,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: Webservice,
        penalty: NoGiveAssist,
    },
    Hacker {
        value: 2,
        virus: true,
        symbol: Keyboard,
        penalty: NoPenalty,
    },
    Hacker {
        value: 2,
        virus: true,
        symbol: Keyboard,
        penalty: Burnout,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: Keyboard,
        penalty: Ninja,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: Keyboard,
        penalty: NoSecure,
    },
    Hacker {
        value: 2,
        virus: false,
        symbol: NoSymbol,
        penalty: Compromise,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: Database,
        penalty: NoPenalty,
    },
    Hacker {
        value: 3,
        virus: true,
        symbol: Database,
        penalty: Compromise,
    },
    Hacker {
        value: 3,
        virus: true,
        symbol: Database,
        penalty: Burnout,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: Database,
        penalty: DrawLeft,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: Database,
        penalty: NoSecure,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: Webservice,
        penalty: NoGiveAssist,
    },
    Hacker {
        value: 3,
        virus: true,
        symbol: Webservice,
        penalty: Burnout,
    },
    Hacker {
        value: 3,
        virus: true,
        symbol: Webservice,
        penalty: Compromise,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: Webservice,
        penalty: DrawLeft,
    },
    Hacker {
        value: 3,
        virus: true,
        symbol: Keyboard,
        penalty: Compromise,
    },
    Hacker {
        value: 3,
        virus: true,
        symbol: Keyboard,
        penalty: NoPenalty,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: Keyboard,
        penalty: DrawLeft,
    },
    Hacker {
        value: 3,
        virus: false,
        symbol: NoSymbol,
        penalty: Burnout,
    },
    Hacker {
        value: 4,
        virus: true,
        symbol: Database,
        penalty: Compromise,
    },
    Hacker {
        value: 4,
        virus: true,
        symbol: Database,
        penalty: Burnout,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: Database,
        penalty: DrawRight,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: Database,
        penalty: NoSecure,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: Database,
        penalty: NoPenalty,
    },
    Hacker {
        value: 4,
        virus: true,
        symbol: Webservice,
        penalty: Burnout,
    },
    Hacker {
        value: 4,
        virus: true,
        symbol: Webservice,
        penalty: Compromise,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: Webservice,
        penalty: DrawRight,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: Webservice,
        penalty: NoGiveAssist,
    },
    Hacker {
        value: 4,
        virus: true,
        symbol: Keyboard,
        penalty: DrawRight,
    },
    Hacker {
        value: 4,
        virus: true,
        symbol: Keyboard,
        penalty: Compromise,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: Keyboard,
        penalty: NoPenalty,
    },
    Hacker {
        value: 4,
        virus: false,
        symbol: NoSymbol,
        penalty: Burnout,
    },
    Hacker {
        value: 5,
        virus: false,
        symbol: Database,
        penalty: Compromise,
    },
    Hacker {
        value: 5,
        virus: false,
        symbol: Webservice,
        penalty: Ninja,
    },
    Hacker {
        value: 5,
        virus: false,
        symbol: Keyboard,
        penalty: Burnout,
    },
    Hacker {
        value: 5,
        virus: true,
        symbol: NoSymbol,
        penalty: Compromise,
    },
    Hacker {
        value: 5,
        virus: true,
        symbol: NoSymbol,
        penalty: Ninja,
    },
    Hacker {
        value: 5,
        virus: true,
        symbol: NoSymbol,
        penalty: Burnout,
    },
    Hacker {
        value: 5,
        virus: true,
        symbol: NoSymbol,
        penalty: NoGiveAssist,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: Database,
        penalty: Idle,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: Keyboard,
        penalty: DoubleNinja,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: NoSymbol,
        penalty: NoTalentAndBurnout,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: NoSymbol,
        penalty: DiscardSecure,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: NoSymbol,
        penalty: NoGiveAssistAndBurnout,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: NoSymbol,
        penalty: NoSecureAndHackerRevive,
    },
    Hacker {
        value: 6,
        virus: true,
        symbol: Webservice,
        penalty: DoubleCompromise,
    },
];

/// panic if defs::NO_HACKER passed
pub fn hacker(id: HackerID) -> &'static Hacker {
    &HACKERS[id as usize]
}
