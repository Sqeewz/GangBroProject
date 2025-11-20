use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::{brawlers::Brawler, missions::MissionEntity},
    infrastructure::database::schema::crew_memberships,
};

pub const MAX_CREW_PER_MISSION: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Insertable, Queryable, Associations)]
#[diesel(belongs_to(Brawler, foreign_key = brawler_id))]
#[diesel(belongs_to(MissionEntity, foreign_key = mission_id))]
#[diesel(table_name = crew_memberships)]
pub struct CrewMembership {
    pub mission_id: i32,
    pub brawler_id: i32,
}

