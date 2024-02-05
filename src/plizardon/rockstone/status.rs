use crate::imports::imports_acmd::*;


pub unsafe extern "C" fn rockstone_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    //Angle
    let num_rock = WorkModule::get_int(owner, *FIGHTER_PLIZARDON_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
    let rand_angle = sv_math::rand(hash40("fighter"), 30) as i32;
    let mut angle = (((num_rock) * 110)-45) + rand_angle;
    //prevent going behind too far
    while (angle > 100 && angle < 260) {
        angle+=25;
    }

    WorkModule::set_int(weapon.module_accessor, angle, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_INT_ANGLE);

    //Rot
    let rand_rot = sv_math::rand(hash40("fighter"), 360) as i32;
    WorkModule::set_int(weapon.module_accessor, angle, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_INT_ROT);
    
    //Snap to throw position
    let mut owner_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut article_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut offset_add = Vector3f{x:0.0,y:0.0,z:0.0};
    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("throw")}, &mut owner_pos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut article_pos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + owner_pos.x - article_pos.x + (offset_add.x*lr), y: PostureModule::pos_y(owner) + owner_pos.y - (article_pos.y)+ offset_add.y, z: PostureModule::pos_z(owner) + owner_pos.z - article_pos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    0.into()
}
pub unsafe extern "C" fn rockstone_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    0.into()
}
pub unsafe extern "C" fn rockstone_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Life
    let life = 1;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    HitModule::set_status_all(weapon.module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stay"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(rockstone_start_main_status_loop as *const () as _))
}

unsafe extern "C" fn rockstone_start_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn rockstone_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        StatusModule::change_status_force(weapon.module_accessor, ROCKSTONE_STATUS_KIND_MOVE, false);
        return 0.into();
    }
    0.into()
}

pub unsafe extern "C" fn rockstone_move_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    0.into()
}

pub unsafe extern "C" fn rockstone_move_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let angle = WorkModule::get_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_INT_ANGLE) as f32;

    //let angle = (sv_math::rand(hash40("fighter"), 360) as f32);
    //WorkModule::set_int(weapon.module_accessor, angle as i32, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_INT_ANGLE);

    //Kinetics
    let lr = PostureModule::lr(weapon.module_accessor);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let speed = 2.0;
    let speed_x = (angle.to_radians()).cos()*speed;
    let speed_y = (angle.to_radians()).sin()*speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -speed_x*lr*0.05,
        -speed_y*0.05
    );


    0.into()
}

pub unsafe extern "C" fn rockstone_move_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Init life
    let life = 16; //WorkModule::get_param_int(weapon.module_accessor, hash40("param_rockstone"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    HitModule::set_status_all(weapon.module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    
    if StopModule::is_stop(weapon.module_accessor){
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);

    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(rockstone_move_main_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(rockstone_move_main_status_loop as *const () as _))
}

unsafe extern "C" fn rockstone_move_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {

    0.into()
}

unsafe extern "C" fn rockstone_move_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let rot = WorkModule::get_int(weapon.module_accessor, WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_INT_ROT) as f32;
    
    let rotation = Vector3f{x: rot, y: rot, z: rot};
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("needle"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    //Life
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        rockstone_remove(weapon);
        return 0.into();
    }

    //Change LR
    let time_active = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE)-life;
    if time_active >= 4 {
        let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        PostureModule::set_lr(weapon.module_accessor, speed_x.signum());
    }

    //Check for reflect
    let reflected = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR);
    let was_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_SHEIK_NEEDLE_STATUS_WORK_FLAG_INFLICT);
    if (reflected && !was_reflected) {
        //Reflect upwards (remove to have it not reflect)
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.75, y: 0.75, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        
        let new_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, new_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

        WorkModule::on_flag(weapon.module_accessor, *WEAPON_SHEIK_NEEDLE_STATUS_WORK_FLAG_INFLICT);
        return 0.into();
    } 
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    {
        rockstone_remove(weapon);
    }

    0.into()
}
pub unsafe extern "C" fn rockstone_move_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn rockstone_move_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn rockstone_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}
pub fn install() {    
    Agent::new("plizardon_rockstone")
        .status(Init, ROCKSTONE_STATUS_KIND_START, rockstone_start_init)
        .status(Pre, ROCKSTONE_STATUS_KIND_START, rockstone_start_pre)
        .status(Main, ROCKSTONE_STATUS_KIND_START, rockstone_start_main)
        .status(End, ROCKSTONE_STATUS_KIND_START, rockstone_move_end)

        .status(Init, ROCKSTONE_STATUS_KIND_MOVE, rockstone_move_init)
        .status(Pre, ROCKSTONE_STATUS_KIND_MOVE, rockstone_move_pre)
        .status(Main, ROCKSTONE_STATUS_KIND_MOVE, rockstone_move_main)
        .status(Exec, ROCKSTONE_STATUS_KIND_MOVE, rockstone_move_exec)
        .status(End, ROCKSTONE_STATUS_KIND_MOVE, rockstone_move_end)
        .install();
}
