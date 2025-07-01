//! # Discord HTTP API (Preview) - REST API Client
//! 
//! Preview of the Discord v10 HTTP API specification. See https://discord.com/developers/docs for more details.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 10
//! - **Modified**: 2025-07-01T10:17:22.441711403Z[Etc/UTC]
//! - **Generator Version**: 7.14.0
//!
//! <details>
//! <summary><strong>⚠️ Important Disclaimer & Limitation of Liability</strong></summary>
//! <br>
//! > **IMPORTANT**: This software is provided "as is" without any warranties, express or implied, including but not limited
//! > to warranties of merchantability, fitness for a particular purpose, or non-infringement. The developers, contributors,
//! > and licensors (collectively, "Developers") make no representations regarding the accuracy, completeness, or reliability
//! > of this software or its outputs.
//! >
//! > This client is not intended to provide financial, investment, tax, or legal advice. It facilitates interaction with the
//! > Discord HTTP API (Preview) service but does not endorse or recommend any financial actions, including the purchase, sale, or holding of
//! > financial instruments (e.g., stocks, bonds, derivatives, cryptocurrencies). Users must consult qualified financial or
//! > legal professionals before making decisions based on this software's outputs.
//! >
//! > Financial markets are inherently speculative and carry significant risks. Using this software in trading, analysis, or
//! > other financial activities may result in substantial losses, including total loss of capital. The Developers are not
//! > liable for any losses or damages arising from such use. Users assume full responsibility for validating the software's
//! > outputs and ensuring their suitability for intended purposes.
//! >
//! > This client may rely on third-party data or services (e.g., market feeds, APIs). The Developers do not control or verify
//! > the accuracy of these services and are not liable for any errors, delays, or losses resulting from their use. Users must
//! > comply with third-party terms and conditions.
//! >
//! > Users are solely responsible for ensuring compliance with all applicable financial, tax, and regulatory requirements in
//! > their jurisdiction. This includes obtaining necessary licenses or approvals for trading or investment activities. The
//! > Developers disclaim liability for any legal consequences arising from non-compliance.
//! >
//! > To the fullest extent permitted by law, the Developers shall not be liable for any direct, indirect, incidental,
//! > consequential, or punitive damages arising from the use or inability to use this software, including but not limited to
//! > loss of profits, data, or business opportunities.
//!
//! </details>
use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuildInviteResponse {
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<i32>>,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "inviter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub inviter: Option<Option<Box<models::UserResponse>>>,
    #[serde(rename = "max_age", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<Option<i32>>,
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    #[serde(rename = "expires_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Option<String>>,
    #[serde(rename = "is_contact", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_contact: Option<Option<bool>>,
    #[serde(rename = "flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
    #[serde(rename = "guild", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub guild: Option<Option<Box<models::InviteGuildResponse>>>,
    #[serde(rename = "guild_id", skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(rename = "channel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Option<Box<models::InviteChannelResponse>>>,
    #[serde(rename = "target_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<Option<i32>>,
    #[serde(rename = "target_user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_user: Option<Option<Box<models::UserResponse>>>,
    #[serde(rename = "target_application", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_application: Option<Option<Box<models::InviteApplicationResponse>>>,
    #[serde(rename = "guild_scheduled_event", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event: Option<Option<Box<models::ScheduledEventResponse>>>,
    #[serde(rename = "uses", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uses: Option<Option<i32>>,
    #[serde(rename = "max_uses", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<Option<i32>>,
    #[serde(rename = "temporary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temporary: Option<Option<bool>>,
    #[serde(rename = "approximate_member_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<Option<i32>>,
    #[serde(rename = "approximate_presence_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<Option<i32>>,
    #[serde(rename = "is_nickname_changeable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_nickname_changeable: Option<Option<bool>>,
}

impl GuildInviteResponse {
    pub fn new(code: String) -> GuildInviteResponse {
        GuildInviteResponse {
            r#type: None,
            code,
            inviter: None,
            max_age: None,
            created_at: None,
            expires_at: None,
            is_contact: None,
            flags: None,
            guild: None,
            guild_id: None,
            channel: None,
            target_type: None,
            target_user: None,
            target_application: None,
            guild_scheduled_event: None,
            uses: None,
            max_uses: None,
            temporary: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            is_nickname_changeable: None,
        }
    }
}

