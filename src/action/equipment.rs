use crate::action::equipment::ErrorEquipment::{EmptySlot, ItemNotInInventory, NotEnoughPlaceInInventory};
use crate::action::{handle_action_with_cooldown, Action, ErrorAction};
use crate::character::CharacterData;
use crate::gameinfo::GameInfo;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum SlotType {
    Weapon,
    Shield,
    Helmet,
    BodyArmor,
    LegArmor,
    Boots,
    Ring1,
    Ring2,
    Amulet,
    Artifact1,
    Artifact2,
    Artifact3,
    Consumable1,
    Consumable2,
}

pub enum ErrorEquipment {
    ErrorAction(ErrorAction),
    NotEnoughPlaceInInventory,
    ItemNotInInventory,
    EmptySlot,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InventoryItemToEquip {
    pub slot: SlotType,
    pub code: String,
    pub quantity: u32,
}

impl InventoryItemToEquip {
    pub fn to_unequip(&self) -> InventoryItemToUnEquip {
        InventoryItemToUnEquip {
            slot,
            quantity,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InventoryItemToUnEquip {
    pub slot: SlotType,
    pub quantity: u32,
}

pub async fn equip(
    game_info: &Arc<GameInfo>,
    mut character: &mut CharacterData,
    item: InventoryItemToEquip,
) -> Result<(), ErrorEquipment> {
    // check that the item is in the inventory
    if !character.inventory
        .iter().map(|item| item.code).collect::<Vec<String>>()
        .contains(&item.code) {
        return Err(ItemNotInInventory);
    }

    // check if emplacement is empty and handle it if needed
    un_equip(game_info, &mut character, item.to_unequip()).await?;

    // equip the item
    handle_action_with_cooldown(
        game_info,
        Action::Equip,
        &mut character,
        Some(1),
        Some(&json!(item)),
    ).await.map_err(|e| ErrorEquipment::ErrorAction(e))
}

pub async fn un_equip(
    game_info: &Arc<GameInfo>,
    mut character: &mut CharacterData,
    item: InventoryItemToUnEquip,
) -> Result<(), ErrorEquipment> {
    // check that emplacement is not empty or don't have enough item
    if character.get_equipment(&item.slot).is_empty() {
        return Err(EmptySlot);
    }

    // check if enough place in inventory
    if character.get_inventory_count() + item.quantity > character.inventory_max_items {
        return Err(NotEnoughPlaceInInventory);
    }

    // unequip item
    handle_action_with_cooldown(
        game_info,
        Action::UnEquip,
        &mut character,
        Some(1),
        Some(&json!(item)),
    ).await.map_err(|e| ErrorEquipment::ErrorAction(e))
}

