use crate::people::Player;
use crate::simulator::SimulatorData;

pub struct PlayerCollectionResult {
    pub players: Vec<PlayerResult>,
    pub outgoing_players: Vec<Player>
}

impl PlayerCollectionResult{
    pub fn new(players: Vec<PlayerResult>, outgoing_players: Vec<Player>) -> Self {
        PlayerCollectionResult {
            players,
            outgoing_players
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        
    }
}

pub struct PlayerResult {
    pub transfer_requests: Vec<u32>
}

impl PlayerResult{
    pub fn new() -> Self {
        PlayerResult {
            transfer_requests: Vec::new()
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}