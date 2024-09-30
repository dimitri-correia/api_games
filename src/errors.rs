#[derive(Debug)]
pub enum ErrorAction {
    ErrorParsingResponse,
    ErrorRequest(ErrorRequest),
}

#[derive(Debug)]
pub enum ErrorRequest {
    ErrorSendingRequest,
}

#[derive(Debug)]
pub enum ErrorBank {
    NotEnoughGoldInBank,
    NotEnoughPlaceInBank,
    NotEnoughPlaceInInventory,
    NotInInventory,
    NotEnoughItemInBank,
    ItemNotFoundInBank,
    ErrorAction(ErrorAction),
}

#[derive(Debug)]
pub enum ErrorEquipment {
    ErrorAction(ErrorAction),
    NotEnoughPlaceInInventory,
    ItemNotInInventory,
    EmptySlot,
}

#[derive(Debug)]
pub enum ErrorMovement {
    NoPositionFound,
    Action(ErrorAction),
}
