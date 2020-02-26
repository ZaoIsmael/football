use crate::core::context::SimulationContext;
use crate::people::Player;
pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct PlayerClubContract {
    pub player: Player,
    pub salary: f64,
    pub expired: NaiveDate,
    pub additional_options: AdditionalOptions,
}

#[derive(Debug, Clone)]
pub struct AdditionalOptions {
    pub yearly_increase_wage: u16,
}

impl PlayerClubContract {
    pub fn new(player: Player, expired: NaiveDate) -> Self {
        PlayerClubContract {
            player,
            salary: 100_000.0,
            expired,
            additional_options: AdditionalOptions {
                yearly_increase_wage: 15,
            },
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();

        let naive_now = NaiveDate::from_ymd(now.year(), now.month(), now.day());

        self.expired >= naive_now
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if context.check_contract_expiration() && self.is_expired() {}
    }
}

#[derive(Debug, Clone)]
pub struct PlayerCollection {
    pub contracts: Vec<PlayerClubContract>,
}

impl PlayerCollection {
    pub fn new(contracts: Vec<PlayerClubContract>) -> Self {
        PlayerCollection { contracts }
    }

    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        for player_contract in &mut self.contracts {
            player_contract.simulate(context)
        }
    }
}