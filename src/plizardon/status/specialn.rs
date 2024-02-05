use crate::imports::imports_agent::*;

unsafe extern "C" fn specialn_init(agent: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
        let sum_speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mul_x = 1.0;
        let start_accel_x = 0.0025;
        let accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_y"), 0);
        let speed_x = sum_speed_x*mul_x;
        
        sv_kinetic_energy!(
            reset_energy,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        /*
        sv_kinetic_energy!(
            set_brake,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            start_accel_x,
            0.0
        ); */
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        /*
        let speed_y = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_s"), hash40("special_s_attack_spd_y"));

        sv_kinetic_energy!(
            reset_energy,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel_y
        ); */
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    return 0.into();
}

unsafe extern "C" fn specialn_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn specialn_end(agent: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}
unsafe extern "C" fn specialn_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}
unsafe extern "C" fn specialn_exit(agent: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(agent.module_accessor, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into();
}


unsafe extern "C" fn specialn_main(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), false.into());
    agent.sub_set_ground_correct_by_situation(true.into());
    agent.main_shift(specialn_main_loop)
}

unsafe extern "C" fn specialn_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(agent.module_accessor)
    && StatusModule::is_situation_changed(agent.module_accessor) {
        agent.sub_set_ground_correct_by_situation(false.into());
        agent.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        agent.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), true.into());
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    
    if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
        let speed_x = {
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy::get_speed_x(agent.lua_state_agent)
        };
        if speed_x.abs() < 0.1 {
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, agent.module_accessor);
        }
    }

    0.into()
} 

pub fn install() {
    Agent::new("plizardon")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_init)
        //.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_main)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_exec)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_end)
        .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_exit)
    .install();
}