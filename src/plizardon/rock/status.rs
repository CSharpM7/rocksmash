use crate::imports::imports_acmd::*;

pub unsafe extern "C" fn rock_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn rock_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let life = 3;//WorkModule::get_param_int(weapon.module_accessor, hash40("param_rock"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_MAX);
    WorkModule::set_int(weapon.module_accessor, life, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_REMAINING);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);

    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    
    //HAVE constraint. Tie the Rock's "have" bone to Zard's "throw" bone
    //Pretty sure most things until set model constraint arent necessary...
    LinkModule::remove_model_constraint(weapon.module_accessor,true);
    if LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink_all(weapon.module_accessor);
    }
    if LinkModule::is_link(weapon.module_accessor,*ITEM_LINK_NO_HAVE) == false {
        LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("have"),Hash40::new("throw"),(*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION) as u32,true);
    }

    weapon.fastshift(L2CValue::Ptr(rock_start_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn rock_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if WorkModule::is_flag(weapon.module_accessor,WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_FLAG_BREAK) {
        VisibilityModule::set_whole(weapon.module_accessor, false);

        //spawn stones
        WorkModule::dec_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);
        if  WorkModule::get_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN) <= 0 {
            WorkModule::set_int(weapon.module_accessor, 1, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);

            let owner = get_owner_boma(weapon);
            ArticleModule::generate_article(owner, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCKSTONE, false, -1) as u32;
            WorkModule::inc_int(owner,*FIGHTER_PLIZARDON_STATUS_BREATH_WORK_INT_GENERATE_COUNT);

            if WorkModule::count_down_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_REMAINING, 0) {
                smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
    }
    0.into()
}

pub unsafe extern "C" fn rock_start_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    delete_if_orphaned(weapon);
    0.into()
}
pub unsafe extern "C" fn rock_start_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}


unsafe extern "C" fn delete_if_orphaned(weapon: &mut smashline::L2CWeaponCommon){
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let mut should_delete = false;
    if !sv_battle_object::is_active(owner_id) {
        should_delete = true;
    }
    else{
        let owner_boma = get_owner_boma(weapon);
        let status = StatusModule::status_kind(owner_boma);
        if [*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH].contains(&status) {
            should_delete = true;
        }
    }
    if should_delete == true {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install() {    
    Agent::new("plizardon_rock")
        .status(Pre, ROCK_STATUS_KIND_START, rock_start_pre)
        .status(Main, ROCK_STATUS_KIND_START, rock_start_main)
        .status(End, ROCK_STATUS_KIND_START, rock_start_end)
        .install();
}
