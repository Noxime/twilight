mod member;
mod membership_state;

pub use self::{member::TeamMember, membership_state::TeamMembershipState};

use crate::{id::UserId, oauth::id::TeamId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Team {
    pub id: TeamId,
    pub icon: Option<String>,
    pub members: Vec<TeamMember>,
    pub owner_user_id: UserId,
}
