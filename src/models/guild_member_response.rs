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
pub struct GuildMemberResponse {
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    #[serde(rename = "avatar_decoration_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Option<Box<models::UserAvatarDecorationResponse>>>,
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    #[serde(rename = "communication_disabled_until", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Option<String>>,
    #[serde(rename = "flags")]
    pub flags: i32,
    #[serde(rename = "joined_at")]
    pub joined_at: String,
    #[serde(rename = "nick", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nick: Option<Option<String>>,
    #[serde(rename = "pending")]
    pub pending: bool,
    #[serde(rename = "premium_since", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Option<String>>,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "user")]
    pub user: Box<models::UserResponse>,
    #[serde(rename = "mute")]
    pub mute: bool,
    #[serde(rename = "deaf")]
    pub deaf: bool,
}

impl GuildMemberResponse {
    pub fn new(flags: i32, joined_at: String, pending: bool, roles: Vec<String>, user: models::UserResponse, mute: bool, deaf: bool) -> GuildMemberResponse {
        GuildMemberResponse {
            avatar: None,
            avatar_decoration_data: None,
            banner: None,
            communication_disabled_until: None,
            flags,
            joined_at,
            nick: None,
            pending,
            premium_since: None,
            roles,
            user: Box::new(user),
            mute,
            deaf,
        }
    }
}

