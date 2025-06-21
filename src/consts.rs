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

pub const FIGHTER_INSTANCE_WORK_ID_FLAG_UP_SPECIAL_CANCEL: smash::lib::LuaConst = smash::lib::LuaConst::new(0x200000ED);

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