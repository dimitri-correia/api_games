pub enum Action {
    Move,
    Fight,
    Gathering,
    Unequip,
    Equip,
    Craft,
}

pub fn get_action_name(action: Action) -> &'static str {
    match action {
        Action::Fight => "fight",
        Action::Gathering => "gathering",
        Action::Move => "move",
        Action::Unequip => "unequip",
        Action::Equip => "equip",
        Action::Craft => "crafting",
    }
}