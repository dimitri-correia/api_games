#[derive(Debug)]
pub enum ResponseCode {
    // General Responses
    Ok200, // 200: Successful operation
    ActionInProgress486, // 486: An action is already in progress for the user.
    CharacterNotFound498, // 498: Character not found.
    Cooldown499, // 499: Cooldown in progress.
    MapIncorrect598, // 598: Map incorrect.

    // Bank Responses
    ItemNotFound404, // 404: Item not found in your inventory.
    TransactionInProgress461, // 461: A transaction is already in progress with this item/your golds in your bank.
    BankFull462, // 462: Your bank is full.
    InsufficientQuantity478, // 478: Insufficient quantity of this item in your inventory.
}

impl ResponseCode {
    pub fn get_code(&self) -> u16 {
        match self {
            ResponseCode::Ok200 => 200,
            ResponseCode::ActionInProgress486 => 486,
            ResponseCode::CharacterNotFound498 => 498,
            ResponseCode::Cooldown499 => 499,
            ResponseCode::MapIncorrect598 => 598,
            ResponseCode::ItemNotFound404 => 404,
            ResponseCode::TransactionInProgress461 => 461,
            ResponseCode::BankFull462 => 462,
            ResponseCode::InsufficientQuantity478 => 478,
        }
    }
}