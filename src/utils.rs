use smash::lib::lua_const::*;
use smash::app::utility;
use smash::lib::{self, lua_const::*};
use smash::app::{self, lua_bind::*};
use smashline::L2CFighterCommon;
use smash::app::BattleObjectModuleAccessor;
use smash::app::BattleObject;
use crate::offsets;

//Aerial ECB Fixes, mainly for things like Link, Captain, Simon, and Richter (Credit to HDR)
extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;
}

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    smash::app::lua_bind::WorkModule::get_int(boma, *smash::lib::lua_const::FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

extern "C"{
    #[link_name = "\u{1}_ZN3app14sv_information11is_ready_goEv"]
    pub fn is_ready_go() -> bool;
}

extern "C"{
    #[link_name = "\u{1}_ZN3app7utility8get_kindEPKNS_26BattleObjectModuleAccessorE"]
    pub fn get_kind(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32;
}


pub fn in_range(num: f32, lower: f32, upper: f32) -> bool{
    num>lower && num<upper
}

pub unsafe fn clamp(x: f32, min: f32, max: f32) -> f32 {
    return if x < min { min } else if x < max { x } else { max };
}


pub unsafe fn clear_buffered_action(flag: i32, cmd: i32) -> i32 {
    return flag & !(flag & cmd);
}

pub unsafe fn add_buffered_action(flag: i32, cmd: i32) -> i32 {
    return flag | cmd;
}

pub unsafe fn compare_cat(cat: i32, fighter_pad_cmd_flag: i32) -> bool {
    return (cat & fighter_pad_cmd_flag) != 0;
}

pub fn get_category(boma: &mut app::BattleObjectModuleAccessor) -> i32 {
    get_category(boma) as i32
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(super::singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}
pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }

}

pub fn get_fighter_common_from_entry_id(entry_id: u32) -> Option<&'static mut L2CFighterCommon> {
    if let Some(object) = get_battle_object_from_entry_id(entry_id) {
        unsafe {
            Some(get_fighter_common_from_accessor(std::mem::transmute((*object).module_accessor)))
        }
    } else {
        None
    }
}

/// Utility function to compare two masks, such as a "cat flag"
/// Internally, its just a binary & operation. When possible, use 
/// fighter.is_cat_flag() or boma.is_cat_flag() instead, for clarity.
/// 
/// # Arguments:
/// * `mask1` : a bitmask, such as a cat flag mask (cat1, cat2, etc)\n
/// * `mask2` : another bitmask, such as `*FIGHTER_KIND_MARIO`
/// 
/// # Returns:
/// `bool` : whether the binary & of the masks is nonzero (aka "true")
pub fn compare_mask(mask1: i32, mask2: i32) -> bool {
    return (mask1 & mask2) != 0;
}