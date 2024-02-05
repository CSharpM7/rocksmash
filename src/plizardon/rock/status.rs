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
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);

    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(rock_start_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn rock_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    snap_to_owner(weapon);
    if WorkModule::is_flag(weapon.module_accessor,WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_FLAG_BREAK) {
        VisibilityModule::set_whole(weapon.module_accessor, false);

        //spawn rock
        WorkModule::dec_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);
        if  WorkModule::get_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN) <= 0 {
            WorkModule::set_int(weapon.module_accessor, 1, WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN);

            let owner = get_owner_boma(weapon);
            ArticleModule::generate_article(owner, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCKSTONE, false, -1) as u32;
            WorkModule::inc_int(owner,*FIGHTER_PLIZARDON_STATUS_BREATH_WORK_INT_GENERATE_COUNT);

            if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
                smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
        /* 
        for i in 0..3 {
            let owner = get_owner_boma(weapon);
            ArticleModule::generate_article(owner, FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCKSTONE, false, -1) as u32;
            WorkModule::inc_int(owner,*FIGHTER_PLIZARDON_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
        }
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));*/
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

unsafe extern "C" fn snap_to_owner(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = get_owner_boma(weapon);
    let mut ownerPos = VECTOR_ZERO;
    let mut capPos = VECTOR_ZERO;
    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("throw")}, &mut ownerPos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut capPos);      
    let offset = Vector3f{x:-2.0*lr,y:2.0,z:0.0};
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + ownerPos.x - capPos.x + (offset.x), y: PostureModule::pos_y(owner) + ownerPos.y + offset.y, z: PostureModule::pos_z(owner) + ownerPos.z- capPos.z + offset.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    
    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(owner,Hash40::new("throw"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(
        weapon.module_accessor,
        &rot,
        0
    );
}

pub fn install() {    
    Agent::new("plizardon_rock")
        .status(Pre, ROCK_STATUS_KIND_START, rock_start_pre)
        .status(Main, ROCK_STATUS_KIND_START, rock_start_main)
        .status(End, ROCK_STATUS_KIND_START, rock_start_end)
        .install();
}
