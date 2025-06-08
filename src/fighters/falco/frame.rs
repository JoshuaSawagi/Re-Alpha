use smash::lib::lua_const::*;
use smash::phx::{Vector2f, Vector3f};
use smash::app::sv_system;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use crate::ext::Vec3Ext;
use crate::ext::Vec2Ext;
use smash::app::GroundCorrectKind;
use smash::hash40;
use smash::app::sv_battle_object::pos;

pub static mut SPEED_Y : [f32; 8] = [0.0; 8];

pub(crate) unsafe fn is_default(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) < 16  {
		return true 
	} else {
		return false
	}
}

pub(crate) unsafe fn get_speed_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return SPEED_Y[ENTRY_ID]
}

unsafe extern "C" fn falco_laser_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if fighter_kind == *FIGHTER_KIND_FALCO {
				if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						let cat2 = ControlModule::get_command_flag_cat(boma, 1);
						if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
							WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
						}
					};
				};
			};
		}
    }
}

pub(crate) fn is_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}

static mut CAN_DOWNB: [i32; 8] = [0; 8];
static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];

unsafe extern "C" fn shine(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        if is_default(boma) {
            let status_kind = StatusModule::status_kind(boma);
            let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let motion_kind = MotionModule::motion_kind(boma);
            let frame = MotionModule::frame(boma);

            if [
                hash40("special_lw"), hash40("special_lw_r"), hash40("special_lw_l"),
                hash40("special_air_lw"), hash40("special_air_lw_r"), hash40("special_air_lw_l")
            ].contains(&motion_kind) {

                if !HAS_DOWNB[ENTRY_ID] && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                    HAS_DOWNB[ENTRY_ID] = true;
                    DO_STALL[ENTRY_ID] = true;
                }

                if frame > 32.0 {
                    DO_STALL[ENTRY_ID] = false;
                    KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    CancelModule::enable_cancel(boma);
                } else if DO_STALL[ENTRY_ID] {
                    // Stall gravity and control
                    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    macros::SET_SPEED_EX(fighter, 0.0, -0.227, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

                    // Ground snap attempt
					let pos = *PostureModule::pos(fighter.module_accessor); // Properly dereferenced Vector3f

					let lower_bound = Vector2f { x: pos.x, y: pos.y - 3.0 };
					let upper_bound = Vector2f { x: pos.x, y: pos.y + 3.0 };
					let zero_vec = Vector2f { x: 0.0, y: 0.0 };
					let mut ground_pos_any = Vector2f { x: 0.0, y: 0.0 };

					let is_touch_any = GroundModule::line_segment_check(
						fighter.module_accessor,
						&upper_bound,
						&lower_bound,
						&zero_vec,
						&mut ground_pos_any,
						true,
					);

					let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
					let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
					let is_diagonal_down = stick_y < -0.2 && stick_x.abs() > 0.2;

					let close_enough_to_ground = is_touch_any != std::ptr::null() && (pos.y - ground_pos_any.y).abs() < 2.0;

					if is_diagonal_down && close_enough_to_ground {
						GroundModule::attach_ground(fighter.module_accessor, true);
						GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
						PostureModule::set_pos(
							fighter.module_accessor,
							&Vector3f { x: pos.x, y: ground_pos_any.y + 0.3, z: pos.z },
						);
						fighter.set_situation(SITUATION_KIND_GROUND.into());
						fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
						return;
                    }
                }

            } else {
                DO_STALL[ENTRY_ID] = false;
            }

            if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
                HAS_DOWNB[ENTRY_ID] = false;
                DO_STALL[ENTRY_ID] = false;
            }
        }
    }
}


pub fn install() {
	Agent::new("falco")
	.on_line(Main, falco_laser_cancel)
	.on_line(Main, shine)
	.install();
}