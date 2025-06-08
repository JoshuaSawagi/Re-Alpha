/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */


#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
pub unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}




pub fn install() {
    skyline::install_hook!(
        is_valid_just_shield_reflector
    );
}
