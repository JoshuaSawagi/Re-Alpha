use crate::FIGHTER_INSTANCE_WORK_ID_INT_COLOR;
use smash2::app::WorkModule;

pub(crate) unsafe fn is_default(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) < 16  {
		return true 
	} else {
		return false
	}
}