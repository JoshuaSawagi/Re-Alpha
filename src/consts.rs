use smash::lib::LuaConst;

//COMMON
        //Ledge_const
pub const FIGHTER_STATUS_WORK_FLAG_DISABLE_CLIFF_CHECK: i32 = 0x20000000;

// Add this to your list of fighter instance work IDs
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_DOUBLE_TRACTION: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000F4);
pub const FIGHTER_STATUS_FLAG_SMASH_TURN: i32 = 1000;

pub const FIGHTER_INSTANCE_WORK_ID_FLAG_PIVOT: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000F5);

pub const FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000EE);
pub const FIGHTER_INSTANCE_WORK_ID_INT_TETHER_HOGGED: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000EF);


pub const FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH: smash::lib::LuaConst = smash::lib::LuaConst::new(0x2000012A);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_WAVELAND_PLATDROP: smash::lib::LuaConst = smash::lib::LuaConst::new(0x2000012B);

pub const FIGHTER_INSTANCE_WORK_FLOAT_GLIDE_TOSS_DIR: smash::lib::LuaConst = smash::lib::LuaConst::new(0x2000013C);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_LEDGE_OCCUPYING: smash::lib::LuaConst = smash::lib::LuaConst::new(0x2000013D);

pub const FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000F2);

pub const FIGHTER_INSTANCE_WORK_ID_FLAG_LEDGE_POS_X: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000F3);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_LEDGE_POS_Y: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000F4);


pub const FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL: i32 = 0x000E;
pub const FIGHTER_INSTANCE_WORK_ID_SPECIAL_STALL_USED: i32 = 0x000F;
pub const LEDGE_POS: i32 = 0x000A;


pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK: i32 = 0x20000116; //Indicates all fighters are on their last stock
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED: i32 = 0x20000117; //Tracks if the ball has bounced at least once since being thrown
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START: i32 = 0x20000118;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER: i32 = 496;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK: i32 = 497; //Tracks if a fighter used a certain special move in the air
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD: i32 = 498;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED: i32 = 499;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT: i32 = 500;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT: i32 = 501;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE: i32 = 502;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK: i32 = 503;
pub const FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT: i32 = 504; //Tracks if a player got hit during One-Hit mode
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW: i32 = 505;
pub const FIGHTER_INSTANCE_WORK_ID_INT_MASHING: i32 = 507;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 508;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER: i32 = 509;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO: i32 = 510; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER: i32 = 511;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_TIMER: i32 = 512;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL: i32 = 513;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_EFFECT: i32 = 514;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS: i32 = 515; //Flags when you just used a Final Smash in Special Smash
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE: i32 = 516;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_XLU_FRAME: i32 = 517;
pub const FIGHTER_INSTANCE_WORK_ID_INT_LANDING_FRAME: i32 = 518;

pub const FIGHTER_INSTANCE_WORK_ID_FLAG_MOONWALK: LuaConst = LuaConst::new(0x100000BD);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_MOONWALK_JUMP: LuaConst = LuaConst::new(0x100000BE);
pub const FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER: LuaConst = LuaConst::new(0x100000BF);
pub const FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER: LuaConst = LuaConst::new(0x100000C0);
pub const FIGHTER_INSTANCE_WORK_ID_INT_CRITICAL_COUNTER: LuaConst = LuaConst::new(0x100000C1);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_EFFECT_SCALE_MUL: LuaConst = LuaConst::new(0x4D);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME: LuaConst = LuaConst::new(0x4E);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION: LuaConst = LuaConst::new(0x4F);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_CSTICK_DIRECTION: LuaConst = LuaConst::new(0x50);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_CRITICAL_ON_FRAME: LuaConst = LuaConst::new(0x51);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FRAME: LuaConst = LuaConst::new(0x642);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x2000012C);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW: LuaConst = LuaConst::new(0x2000012D);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N: LuaConst = LuaConst::new(0x2000012E);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S: LuaConst = LuaConst::new(0x2000012F);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_FLOAT: LuaConst = LuaConst::new(0x20000130);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CRITICAL: LuaConst = LuaConst::new(0x20000131);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_OMNI_FLOAT: LuaConst = LuaConst::new(0x20000132);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_S4_IS_CHARGED: LuaConst = LuaConst::new(0x20000133);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_HI4_IS_CHARGED: LuaConst = LuaConst::new(0x20000134);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_LW4_IS_CHARGED: LuaConst = LuaConst::new(0x20000135);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES: LuaConst = LuaConst::new(0x20000136);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ULTRA_ARMOR: LuaConst = LuaConst::new(0x20000137);
pub const FIGHTER_STATUS_WORK_FLAG_ULTRA_ARMOR: LuaConst = LuaConst::new(0x21000030);
pub const FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL: LuaConst = LuaConst::new(0x2100002A);
pub const FIGHTER_STATUS_GLIDE_WORK_FLOAT_GLIDE_FRAME: LuaConst = LuaConst::new(0x100000B);
pub const FIGHTER_STATUS_GLIDE_WORK_INT_PITCH_SE: LuaConst = LuaConst::new(0x11000007);
pub const FIGHTER_STATUS_GLIDE_WORK_INT_STOP_SE: LuaConst = LuaConst::new(0x11000008);
pub const FIGHTER_STATUS_JUMP_FLAG_ASCENSION_REVALI_ENABLE: LuaConst = LuaConst::new(0x21000018);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_N: LuaConst = LuaConst::new(0x21000023);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_S: LuaConst = LuaConst::new(0x21000024);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x21000025);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_LW: LuaConst = LuaConst::new(0x21000026);
pub const FIGHTER_STATUS_FLOAT_WORK_FLAG_INHERIT_AERIAL: LuaConst = LuaConst::new(0x2100000D);
pub const FIGHTER_STATUS_FLOAT_WORK_FLAG_IS_FLOAT: LuaConst = LuaConst::new(0x2100000E);
pub const FIGHTER_STATUS_FLOAT_WORK_FLAG_TURN: LuaConst = LuaConst::new(0x2100000F);
pub const FIGHTER_STATUS_FLOAT_WORK_INT_TIME: LuaConst = LuaConst::new(0x11000006);
pub const FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ: LuaConst = LuaConst::new(0x11000007);
pub const FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS: LuaConst = LuaConst::new(0x11000008);
pub const FIGHTER_STATUS_FLOAT_WORK_FLOAT_ROT_Y: LuaConst = LuaConst::new(0x1000009);
pub const FIGHTER_STATUS_FLOAT_WORK_FLOAT_TURN_DIR: LuaConst = LuaConst::new(0x100000A);
pub const FIGHTER_STATUS_KIND_SPECIAL_GUARD: LuaConst = LuaConst::new(0x643);
pub const FIGHTER_STATUS_KIND_FLOAT: LuaConst = LuaConst::new(0x644);
pub const COLLISION_KIND_MASK_PARRY: LuaConst = LuaConst::new(0x80);

pub const FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST: smash::lib::LuaConst = smash::lib::LuaConst::new(0x200000ED);
pub const FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE: smash::lib::LuaConst = smash::lib::LuaConst::new(0x200000EE);
pub const FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST: smash::lib::LuaConst = smash::lib::LuaConst::new(0x200000EF);
pub const FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL: smash::lib::LuaConst = smash::lib::LuaConst::new(0x200000F0);
pub const FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE: smash::lib::LuaConst = smash::lib::LuaConst::new(0x50);

//WOLF
pub const FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH: LuaConst = LuaConst::new(0x1EA);
pub const FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END: LuaConst = LuaConst::new(0x1EB);


pub mod globals {
    //Vanilla Consts
    pub const UNK1: i32 = 0x0; //void value
    pub const UNK2: i32 = 0x1; //void value
    pub const FIGHTER_KIND: i32 = 0x2; //fighter kind, i32
    pub const OBJECT_ID: i32 = 0x3; //object id, i32
    pub const FIGHTER: i32 = 0x4; //ptr value, very similar to 0x6
    pub const MODULE_ACCESSOR: i32 = 0x5; //module accessor, ptr value
    pub const UNK4: i32 = 0x6; //void value
    pub const INIT_STATUS_FUNC: i32 = 0x7; //init status func, ptr value
    pub const IS_STOP: i32 = 0x8; //is stop, bool value
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9; //status kind interrupt, i32 value
    pub const PREV_STATUS_KIND: i32 = 0xA; //prev status kind, i32 value
    pub const STATUS_KIND: i32 = 0xB; //status kind, i32 value
    pub const STATUS_COUNT: i32 = 0xC; //status count, i32 value
    pub const UNK5: i32 = 0xD; //bool value
    pub const CURRENT_FRAME: i32 = 0xE; //current frame, f32 value
    pub const CURRENT_FRAME_NO_INTERP: i32 = 0xF; //current frame no interp, f32 value
    pub const UNK6: i32 = 0x10; //ptr value
    pub const UNK7: i32 = 0x11; //ptr value, equal to UNK8
    pub const UNK8: i32 = 0x12; //ptr value
    pub const SUB_STATUS3: i32 = 0x13; //sub status3, ptr/i32 value
    pub const PREV_SUB_STATUS: i32 = 0x14; //prev sub status, i32 value
    pub const SUB_STATUS: i32 = 0x15; //sub status, i32 value
    pub const SITUATION_KIND: i32 = 0x16; //situation kind, i32 value
    pub const PREV_SITUATION_KIND: i32 = 0x17; //prev situation kind, i32 value
    pub const PREV_STATUS_FRAME: i32 = 0x18; //prev status frame, f32 value
    pub const UNK9: i32 = 0x19; //i32 value, i32 value
    pub const STICK_X: i32 = 0x1A; //stick x, f32 value
    pub const STICK_Y: i32 = 0x1B; //stick y, f32 value
    pub const FLICK_X: i32 = 0x1C; //flick x, i32 value
    pub const FLICK_Y: i32 = 0x1D; //flick y, i32 value
    pub const FLICK_Y_DIR: i32 = 0x1E; //flick y dir, f32 value
    pub const PAD_FLAG: i32 = 0x1F; //pad flag, u64 value
    pub const CMD_CAT1: i32 = 0x20; //cmd cat1, u64 value
    pub const CMD_CAT2: i32 = 0x21; //cmd cat2, u64 value
    pub const CMD_CAT3: i32 = 0x22; //cmd cat3, u64 value
    pub const CMD_CAT4: i32 = 0x23; //cmd cat4, u64 value
    pub const UNK10: i32 = 0x24;
    pub const UNK11: i32 = 0x25;
    pub const CHECK_AIR_SPECIAL_UNIQ: i32 = 0x26; //check air special uniq
    pub const CHECK_GROUND_SPECIAL_UNIQ: i32 = 0x27; //check ground special uniq
    pub const CHECK_GROUND_ATTACK_UNIQ: i32 = 0x28; //check ground attack uniq
    pub const DASH_COMMON_UNIQ: i32 = 0x29; //dash common uniq
    pub const RUN_MAIN_UNIQ: i32 = 0x2A; //run main uniq
    pub const JUMP_SQUAT_MAIN_UNIQ: i32 = 0x2B; //jump squat main uniq
    pub const CHECK_AIR_LANDING_UNIQ: i32 = 0x2C; //check air landing uniq
    pub const CHECK_AIR_ITEM_THROW_UNIQ: i32 = 0x2D; //check air item throw uniq
    pub const CHECK_AIR_ATTACK_UNIQ: i32 = 0x2E; //check air attack uniq
    pub const CHECK_AIR_ESCAPE_UNIQ: i32 = 0x2F; //check air escape uniq
    pub const CHECK_AIR_TREAD_JUMP_UNIQ: i32 = 0x30; //check air tread jump uniq
    pub const CHECK_AIR_WALL_JUMP_UNIQ: i32 = 0x31; //check air wall jump uniq
    pub const CHECK_AIR_JUMP_UNIQ: i32 = 0x32; //check air jump uniq
    pub const CHECK_AIR_JUMP_AERIAL_UNIQ: i32 = 0x33; //check air jump aerial uniq
    pub const GUARD_CONT_UNIQ: i32 = 0x34; //guard cont uniq
    pub const TURN_UNIQ: i32 = 0x35; //turn uniq
    pub const CHECK_AIR_CLIFF_LASSO_UNIQ: i32 = 0x36; //check air cliff lasso uniq
    pub const LANDING_UNIQ_CHECK_STRANS_UNIQ: i32 = 0x37; //landing uniq check strans uniq
    pub const CHECK_SPECIAL_N_UNIQ: i32 = 0x38; //check special n uniq
    pub const CHECK_SPECIAL_S_UNIQ: i32 = 0x39; //check special s uniq
    pub const CHECK_SPECIAL_HI_UNIQ: i32 = 0x3A; //check special hi uniq
    pub const CHECK_SPECIAL_LW_UNIQ: i32 = 0x3B; //check special lw uniq
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C; //check special command
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D; //waza customize control
    pub const STATUS_END_CONTROL: i32 = 0x3E; //status end control
    pub const UNK12: i32 = 0x3F;
    pub const UNK13: i32 = 0x40;
    pub const UNK14: i32 = 0x41;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const SUB_UNIQ_DAMAGE_FLY_UNIQ: i32 = 0x43;
    pub const DOWN_DAMAGE_UNIQ: i32 = 0x44;
    pub const THROW_F_STATUS_KIND: i32 = 0x45;
    pub const THROW_B_STATUS_KIND: i32 = 0x46;
    pub const THROW_HI_STATUS_KIND: i32 = 0x47;
    pub const THROW_LW_STATUS_KIND: i32 = 0x48;
    pub const DAMAGE_STOP_MOTION_INTP_FRAME: i32 = 0x49;
    pub const SUB_REBIRTH_UNIQ_INIT_CORE_UNIQ: i32 = 0x4A;
    pub const SUB_REBIRTH_UNIQ_EXEC_UNIQ: i32 = 0x4B;
    pub const SUB_DEAD_UNIQ_INIT_UNIQ: i32 = 0x4C;
    pub const SUB_ROULETTE_SET_SETP_UNIQ: i32 = 0x4D;
    pub const FALL_BRAKE_UNIQ: i32 = 0x4E;
    pub const CHECK_GROUND_GUARD_UNIQ: i32 = 0x4F;
    pub const CHECK_GROUND_CATCH_UNIQ: i32 = 0x50;
    pub const CHECK_COMMAND_WALK_UNIQ: i32 = 0x51;
    pub const CHECK_GROUND_JUMP_MINI_ATTACK: i32 = 0x52;
    pub const CHECK_AIR_ITEM_THROW_POST: i32 = 0x53;
    pub const IS_ITEM_SHOOT_STATUS_UNIQ: i32 = 0x54;
    pub const CHECK_ATTACK_3_UNIQ: i32 = 0x55;
    pub const CHECK_ATTACK_N_UNIQ: i32 = 0x56;
    pub const CHECK_ATTACK_S4_UNIQ: i32 = 0x57;
    pub const CHECK_ATTACK_HI4_UNIQ: i32 = 0x58;
    pub const CHECK_ATTACK_LW4_UNIQ: i32 = 0x59;
    pub const SQUAT_COMMON_UNIQ: i32 = 0x5A;

    //Offsets
    pub const CONSTANT_OFFSET : usize = 0x3728030;
    pub const FLOAT_OFFSET: usize = 0x4E53E0;
}

pub mod vars {
    pub mod common {
        pub mod instance {
            pub const HITSTUN_START: i32 = 0x0000;
            pub const IS_IN_HITSTUN: i32 = 0x0001;
            pub const CSTICK_OVERRIDE: i32 = 0x0002;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0003;


            pub const IS_LATE_PIVOT: i32 = 0x004E;
            pub const CAN_PERFECT_PIVOT: i32 = 0x004F;
            pub const IS_SMASH_TURN: i32 = 0x0050;

            // pub const IS_TAP_JUMP: i32 = 0x0004;
            pub const OMNI_FLOAT: i32 = 0x0005;
            pub const AERIAL_NO_FLOAT: i32 = 0x0006;
            pub const FLOAT_PAUSE_AERIAL: i32 = 0x0007;
            pub const SIDE_SPECIAL_CANCEL: i32 = 0x0008;
            pub const UP_SPECIAL_CANCEL: i32 = 0x0009;
            pub const JAB_DA_CHECKS: i32 = 0x000A;
            pub const TILT_CHECKS: i32 = 0x000B;
            pub const AERIAL_CHECKS: i32 = 0x000C;
            pub const SMASH_CHECKS: i32 = 0x000D;
            pub const DITCIT_SLIDING: i32 = 0x0011;
            pub const FOOTSTOOL_AIRDODGE_LOCKOUT: i32 = 0x0012;
            pub const CAN_ESCAPE_TUMBLE: i32 = 0x0013;
            pub const SPECIAL_WALL_JUMP: i32 = 0x0014;
            pub const TETHER_HOGGED: i32 = 0x0015;
            pub const B_REVERSED: i32 = 0x0016; // Converted for now, but will likely get removed when B Reverse Reimplementation happens
            pub const TUMBLE_KB: i32 = 0x0017;
            pub const CAN_GLIDE_TOSS: i32 = 0x0019;
            pub const IS_MOTION_BASED_ATTACK: i32 = 0x001A;
            pub const PREV_FLAG_DISABLE_ESCAPE_AIR: i32 = 0x001B;
            pub const ENABLE_WAVELAND_PLATDROP: i32 = 0x001C;
            pub const IS_DACUS: i32 = 0x001D;
            pub const IS_STICKY_WALK: i32 = 0x001E;
            pub const ENABLE_BOOST_RUN: i32 = 0x001F;
            pub const PERFECT_WAVEDASH: i32 = 0x0020;
            pub const JUMP_NEXT: i32 = 0x0021;
            pub const SHOULD_TRUMP_TETHER: i32 = 0x0022;
            pub const UP_SPECIAL_INTERRUPT: i32 = 0x0023; // Ness and Lucas use this
            pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x0024; // Ness and Lucas use this
            pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x0025; // Luigi, Ivysaur, and Young Link use this
            pub const STALL_PREVENTION: i32 = 0x0027; //Ness and Lucas down b stall prevention
            pub const SPIN_ATTACK_LAND_CANCEL: i32 = 0x003E; // Link and Mii Sword use this
            pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 0x004D; // Used by Kazuya and Sora
            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x0051;
            pub const IS_KNOCKDOWN_THROW: i32 = 0x0052;
            pub const IS_HEAVY_ATTACK: i32 = 0x0053;
            pub const IS_CC_NON_TUMBLE: i32 = 0x0054;
            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x0055;
            pub const CHECK_CHANGE_MOTION_ONLY: i32 = 0x0056;
            pub const BEFORE_GROUND_COLLISION: i32 = 0x0057;
            pub const IS_IGNORED_STATUS_FRAME_0: i32 = 0x0058;
            pub const FLUSH_EFFECT_ACMD: i32 = 0x0059;
            pub const IS_PARRY_FOR_GUARD_OFF: i32 = 0x0060;
            pub const TEMPORARY_CLIFF_STOP: i32 = 0x0061;
            pub const ENABLE_FRAME_DATA_DEBUG: i32 = 0x0062;
            pub const IS_ATTACK_CANCEL: i32 = 0x0063;
            pub const DISABLE_CSTICK_BUFFER_ROLL_OOS: i32 = 0x0064;
            pub const IS_INIT: i32 = 0x0065;
            pub const IS_FLOAT: i32 = 0x0066;
            pub const WEIRD_ASS_TURN_RUN_ANIMATION: i32 = 0x0067;
            pub const ACMD_EFFECT: i32 = 0x0068;
            pub const WAS_PREV_STATUS_CANCELABLE: i32 = 0x0069;
            pub const IS_ENTER_DASH_CANCEL: i32 = 0x006A;
            pub const DOWN_DISABLE_PASSIVE: i32 = 0x006B;
            pub const DOWN_DISABLE_A_LAND: i32 = 0x006C;
            pub const IS_KILLING_BLOW: i32 = 0x006D;
            pub const JUMP_SQUAT_MAIN_UNIQ: i32 = 0x2B; //jump squat main uniq
            // ints
            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0000;
            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001; // Unironically why does this need to exist? We have WorkModule.
            pub const FLOAT_DURATION: i32 = 0x0002;
            pub const FLOAT_STATUS_KIND: i32 = 0x0003;
            pub const HITFALL_BUFFER: i32 = 0x0004;
            pub const FLY_NEXT_FRAME: i32 = 0x0005;

            // pub const JUMP_SQUAT_FRAME: i32 = 0x0005;
            pub const GIMMICK_TIMER: i32 = 0x0006;
            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x0007;
            pub const CSTICK_LIFE: i32 = 0x0008;
            pub const AGT_USED_COUNTER: i32 = 0x0009;
            pub const LAST_ATTACK_HITBOX_ID: i32 = 0x000B;
            pub const SHIELD_EFFECT_HANDLE: i32 = 0x000C;
            pub const FRAME_COUNTER: i32 = 0x000D;
            pub const LEFT_STICK_FLICK_X: i32 = 0x000E;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x000F;
            pub const RIGHT_STICK_FLICK_X: i32 = 0x0011;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0012;
            pub const PREV_STATUS_TRANSITION_FRAME: i32 = 0x0013;

            // floats
            pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0000;
            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            /// This const is set in a fighter reset because the params used to calculate change depending on situation
            pub const JUMP_SPEED_RATIO: i32 = 0x0003;
            pub const DOUBLE_JUMP_FRAME: i32 = 0x0004;
            pub const GROUND_VEL: i32 = 0x0005; // Only ever gets set, goes effectively unused.
            pub const RAR_LENIENCY: i32 = 0x0006; // Only ever gets set, goes effectively unused.
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0007;
            pub const DOUBLE_JUMP_TIMER: i32 = 0x0008; // Only used by Lucas, and it's commented out, goes unused.
            pub const ROLL_SPEED: i32 = 0x0009;
            pub const LAST_GROUNDED_POS: i32 = 0x000A;
            // pub const LAST_GROUNDED_POS_X: i32 = 0x000A;
            // pub const LAST_GROUNDED_POS_y: i32 = 0x000B;
            // pub const LAST_GROUNDED_POS_Z: i32 = 0x000C;
            pub const GET_DIST_TO_FLOOR: i32 = 0x000D;
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x000E;
            pub const CURR_DASH_SPEED: i32 = 0x000F;
            pub const ESCAPE_AIR_SLIDE_SPEED_X: i32 = 0x0011;
            pub const ESCAPE_AIR_SLIDE_SPEED_Y: i32 = 0x0012;
            pub const Y_POS: i32 = 0x0013;
            /// this multiplier can be set to a value between 0.1 and 3.0 to increase
            /// a character's jump speed max for momentum transfer (for meta quick, etc)
            pub const JUMP_SPEED_MAX_MUL: i32 = 0x0014;
            pub const ECB_TOP_Y_OFFSET: i32 = 0x0015;
            pub const LAST_ATTACK_HIT_LOCATION: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_X: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_Y: i32 = 0x0017;
            pub const LAST_ATTACK_HIT_LOCATION_Z: i32 = 0x0018;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0019;
            pub const DASH_HIP_OFFSET_X: i32 = 0x0020;
            pub const RUN_HIP_OFFSET_X: i32 = 0x0021;
            pub const DACUS_TRANSITION_SPEED: i32 = 0x0022;
            pub const ATTACK_S3_CSTICK_X: i32 = 0x0023;
        }
        pub mod status {

            // flags
            pub const FAF_REACHED: i32 = 0x10FD;
            pub const PREV_AUTOCANCEL_FLAG: i32 = 0x10FE;
            pub const DISABLE_ECB_SHIFT: i32 = 0x10FF;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1000;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1001;
            pub const APPLY_DASH_END_SPEED_MUL: i32 = 0x1002;
            pub const SHOULD_WAVELAND: i32 = 0x1000;
            pub const IS_JAB_LOCK_ROLL: i32 = 0x1000;
            pub const IS_SPIKE: i32 = 0x1001;
            pub const DAMAGE_FLY_RESET_TRIGGER: i32 = 0x1002;
            pub const SUICIDE_THROW_CAN_CLATTER: i32 = 0x1000;
            pub const ENABLE_UCF: i32 = 0x1000;
            pub const PUMMEL_OVERRIDE_GLOBAL_STATS: i32 = 0x1000;
            pub const CSTICK_IRAR: i32 = 0x1000;
            pub const FLOAT_INHERIT_AERIAL: i32 = 0x1000;
            pub const IS_TELEPORT_WALL_RIDE: i32 = 0x1000; // Mewtwo, Palutena, Sheik, and Zelda use this
            pub const ENABLE_SPECIAL_WALLJUMP: i32 = 0x1050;
            pub const HIT_EFFECT_DROP_ITEM: i32 = 0x1051;
            pub const SHOULD_HITFALL: i32 = 0x1006;
            pub const NO_POCKET: i32 = 0x1052;
            pub const IS_DASH_CANCEL: i32 = 0x1055;

            // ints
            pub const DOWN_STAND_FB_KIND: i32 = 0x1000;
            pub const FLOAT_FRAME: i32 = 0x1000;
            pub const FLOAT_ENABLE_UNIQ: i32 = 0x1001;
            pub const FLOAT_MTRANS: i32 = 0x1002;
            pub const WARP_EFF_HANDLER: i32 = 0x1000;

            // floats
            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1000;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1001;
            pub const RESTING_HIP_OFFSET_Y: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_X: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_Y: i32 = 0x1001;
        }
    }
}