use crate::L2CFighterCommon;
use crate::L2CValue;
use smashline::Agent;


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sys_line_system_control_fighter)]
pub unsafe fn sys_line_system_control_fighter_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reserved for common OPFF to be placed on exec status
    // rather than main status (default behavior)
    moonwalk(fighter);

    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sys_line_system_control_fighter_hook
        );
    }
}

pub fn install() {
    Agent::new("fighter")
        .install();

    skyline::nro::add_hook(nro_hook);
}