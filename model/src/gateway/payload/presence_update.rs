use crate::{
    gateway::presence::{Activity, ClientStatus, Status, UserOrId},
    id::{GuildId, RoleId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PresenceUpdate {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub game: Option<Activity>,
    pub guild_id: Option<GuildId>,
    pub roles: Option<Vec<RoleId>>,
    pub status: Status,
    pub user: UserOrId,
}
