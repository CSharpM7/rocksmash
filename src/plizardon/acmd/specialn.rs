use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    //frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK, false, -1);      
    }

    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }

    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ArticleModule::set_flag(agent.module_accessor, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK, true, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_FLAG_BREAK);

        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0,70,80,0,60, 7.8, 0.0, 9.0, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, 0x2, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
pub unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);

        macros::EFFECT(agent, Hash40::new("sys_misfire"), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent,0.5);
        LAST_EFFECT_SET_COLOR(agent,0.5,0.5,0.5);
    }
}
pub unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_plizardon_squat"));
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_plizardon_special_n01"));
    }
}
pub unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("plizardon")
        .acmd("game_specialn", game_specialn)
        .acmd("game_specialairn", game_specialn)
        .acmd("effect_specialn", effect_specialn)
        .acmd("effect_specialairn", effect_specialn)
        .acmd("sound_specialn", sound_specialn)
        .acmd("sound_specialairn", sound_specialn)
        .acmd("expression_specialn", expression_specialn)
        .acmd("expression_specialairn", expression_specialn)
        
    .install();
}