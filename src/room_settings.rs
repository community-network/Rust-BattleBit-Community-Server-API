use crate::read_stream;

#[derive(Debug)]
pub(crate) struct RoomSettings {
    pub damage_multiplier: f32,
    pub bleeding_enabled: bool,
    pub stamine_enabled: bool,
    pub friendly_fire_enabled: bool,
    pub hide_map_votes: bool,
    pub only_winner_team_can_vote: bool,
    pub hit_markers_enabled: bool,
    pub point_log_enabled: bool,
    pub spectator_enabled: bool,

    pub medic_limit_per_squad: u8,
    pub engineer_limit_per_squad: u8,
    pub support_limit_per_squad: u8,
    pub recon_limit_per_squad: u8,
}

// 1
// True
// False
// False
// True
// False
// True
// True
// True
// 8
// 8
// 8
// 8

impl RoomSettings {
    pub fn new() -> Self {
        Self {
            damage_multiplier: 1.0,
            bleeding_enabled: true,
            stamine_enabled: false,
            friendly_fire_enabled: false,
            hide_map_votes: true,
            only_winner_team_can_vote: false,
            hit_markers_enabled: true,
            point_log_enabled: true,
            spectator_enabled: true,

            medic_limit_per_squad: 8,
            engineer_limit_per_squad: 8,
            support_limit_per_squad: 8,
            recon_limit_per_squad: 8,
        }
    }

    pub fn read(&mut self, mut stream: read_stream::Stream) {
        self.damage_multiplier = stream.read_float();
        self.bleeding_enabled = stream.read_bool();
        self.stamine_enabled = stream.read_bool();
        self.friendly_fire_enabled = stream.read_bool();
        self.hide_map_votes = stream.read_bool();
        self.only_winner_team_can_vote = stream.read_bool();
        self.hit_markers_enabled = stream.read_bool();
        self.point_log_enabled = stream.read_bool();
        self.spectator_enabled = stream.read_bool();

        self.medic_limit_per_squad = stream.read();
        self.engineer_limit_per_squad = stream.read();
        self.support_limit_per_squad = stream.read();
        self.recon_limit_per_squad = stream.read();
    }
}
