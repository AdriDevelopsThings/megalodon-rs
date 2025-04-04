use super::{Account, Status};
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Notification {
    account: Account,
    created_at: DateTime<Utc>,
    id: String,
    status: Option<Status>,
    r#type: NotificationType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    Poll,
    Status,
    Update,
    #[serde(rename = "admin.sign_up")]
    AdminSignup,
    #[serde(rename = "admin.report")]
    AdminReport,
}

impl Into<MegalodonEntities::notification::NotificationType> for NotificationType {
    fn into(self) -> MegalodonEntities::notification::NotificationType {
        match self {
            NotificationType::Follow => MegalodonEntities::notification::NotificationType::Follow,
            NotificationType::FollowRequest => {
                MegalodonEntities::notification::NotificationType::FollowRequest
            }
            NotificationType::Mention => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Reblog => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Favourite => {
                MegalodonEntities::notification::NotificationType::Favourite
            }
            NotificationType::Poll => {
                MegalodonEntities::notification::NotificationType::PollExpired
            }
            NotificationType::Status => MegalodonEntities::notification::NotificationType::Status,
            NotificationType::Update => MegalodonEntities::notification::NotificationType::Update,
            NotificationType::AdminSignup => {
                MegalodonEntities::notification::NotificationType::AdminSignup
            }
            NotificationType::AdminReport => {
                MegalodonEntities::notification::NotificationType::AdminReport
            }
        }
    }
}

impl Into<MegalodonEntities::Notification> for Notification {
    fn into(self) -> MegalodonEntities::Notification {
        MegalodonEntities::Notification {
            account: Some(self.account.into()),
            created_at: self.created_at,
            id: self.id,
            status: self.status.map(|i| i.into()),
            emoji: None,
            reaction: None,
            target: None,
            r#type: self.r#type.into(),
        }
    }
}
