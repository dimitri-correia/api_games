use crate::character::CharacterData;
use crate::gameinfo::GameInfo;
use std::sync::Arc;

pub struct AttackStats {
    pub hp: u32,
    pub attack_fire: u32,
    pub attack_earth: u32,
    pub attack_water: u32,
    pub attack_air: u32,
    pub res_fire: i32,
    pub res_earth: i32,
    pub res_water: i32,
    pub res_air: i32,
    pub dmg_fire: Option<u32>,
    pub dmg_earth: Option<u32>,
    pub dmg_water: Option<u32>,
    pub dmg_air: Option<u32>,
}

pub async fn can_win_fight(
    game_info: &Arc<GameInfo>,
    char: &CharacterData,
    enemy_id: &str,
) -> bool {
    let monster_stats = game_info
        .monsters
        .iter()
        .find(|&monster| monster.code == enemy_id)
        .expect("Monster not found")
        .get_attack_stats();

    let char_stats = char.get_attack_stats();
}

fn calculate_hits_to_kill(can_win: AttackStats, against: AttackStats) -> u32 {
    against.hp / damage_given(can_win, against)
}

fn damage_given(by: AttackStats, to: AttackStats) -> u32 {
    let mut damage = 0;

    damage += by.attack_fire + (by.dmg_fire.unwrap_or(0) as u32) - to.res_fire as u32;

    damage
}
