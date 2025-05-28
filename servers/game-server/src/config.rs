use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub cluster: ClusterConfig,
    pub resources: ResourceConfig,
}

#[derive(Deserialize)]
pub struct ClusterConfig {
    pub num_clusters: u32,
}

#[derive(Deserialize)]
pub struct ResourceConfig {
    pub fileconfig_directory: String,
    pub level_process_directory: String,
    pub usm_keys_path: String,
}

#[derive(Deserialize)]
pub struct GachaScheduleConfig {
    #[serde(rename = "schedule")]
    pub gacha_schedule_list: Vec<GachaSchedule>,
}

#[derive(Debug, Deserialize)]
pub struct GachaSchedule {
    pub gacha_id: u32,
    pub gacha_type: u32,
    pub gacha_schedule_id: u32,
    #[serde(default)]
    pub up_item_id_list: Vec<u32>,
    #[serde(default)]
    pub optional_up_item_id_list: Vec<u32>,
    pub begin_time: i64,
    pub end_time: i64,
    pub gacha_materials: Vec<GachaMaterialConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GachaMaterialConfig {
    pub id: u32,
    pub count: u32,
}

#[derive(Deserialize)]
pub struct FirstLoginConfig {
    pub interknot_level: u32,
    pub control_avatar_id: u32,
    pub control_guise_avatar_id: u32,
    pub day_of_week: u32,
    pub start_main_quest: bool,
    pub default_section_id: u32,
    pub avatar: FirstLoginAvatarConfig,
    pub weapon: FirstLoginWeaponConfig,
}

#[derive(Deserialize)]
pub struct FirstLoginAvatarConfig {
    pub unlock_all: bool,
    #[serde(default)]
    pub unlock_id_list: Vec<u32>,
    pub level: u32,
    pub rank: u32,
    pub unlocked_talent_num: u32,
    pub talent_switch: Vec<bool>,
    pub passive_skill_level: u32,
    pub skill_level_map: Vec<u32>,
}

#[derive(Deserialize)]
pub struct FirstLoginWeaponConfig {
    pub unlock_all: bool,
    #[serde(default)]
    pub unlock_id_list: Vec<u32>,
    pub level: u32,
    pub star: u32,
    pub refine_level: u32,
}
