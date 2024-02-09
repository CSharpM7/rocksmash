use crate::imports::imports_agent::*;

unsafe extern "C" fn attacks4_damage(agent: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ArticleModule::remove_exist(agent.module_accessor, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}

pub fn install() {
    Agent::new("plizardon")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attacks4_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attacks4_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4, attacks4_damage)
    .install();
}