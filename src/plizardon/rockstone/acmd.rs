use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("have"), 3.0,70,50,0,70, 3.6, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 0x2, *ATTACK_REGION_OBJECT);  
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("plizardon_rockstone")
        .game_acmd("game_move", game_move)
        
    .install();
}