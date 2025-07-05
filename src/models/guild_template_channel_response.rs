//! # Discord HTTP API (Preview) - REST API Client
//! 
//! Preview of the Discord v10 HTTP API specification. See https://discord.com/developers/docs for more details.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 10
//! - **Modified**: 2025-07-05T02:42:20.508163788Z[Etc/UTC]
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
pub struct GuildTemplateChannelResponse {
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<i32>>,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<i32>>,
    #[serde(rename = "topic", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub topic: Option<Option<String>>,
    #[serde(rename = "bitrate")]
    pub bitrate: i32,
    #[serde(rename = "user_limit")]
    pub user_limit: i32,
    #[serde(rename = "nsfw")]
    pub nsfw: bool,
    #[serde(rename = "rate_limit_per_user")]
    pub rate_limit_per_user: i32,
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "default_auto_archive_duration", skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<i32>,
    #[serde(rename = "permission_overwrites")]
    pub permission_overwrites: Vec<models::ChannelPermissionOverwriteResponse>,
    #[serde(rename = "available_tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Option<Vec<models::GuildTemplateChannelTags>>>,
    #[serde(rename = "template")]
    pub template: String,
    #[serde(rename = "default_reaction_emoji", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<Option<Box<models::DefaultReactionEmojiResponse>>>,
    #[serde(rename = "default_thread_rate_limit_per_user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<Option<i32>>,
    #[serde(rename = "default_sort_order", skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<i32>,
    #[serde(rename = "default_forum_layout", skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<i32>,
    #[serde(rename = "default_tag_setting", skip_serializing_if = "Option::is_none")]
    pub default_tag_setting: Option<String>,
    #[serde(rename = "icon_emoji", skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<serde_json::Value>,
    #[serde(rename = "theme_color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub theme_color: Option<Option<i32>>,
}

impl GuildTemplateChannelResponse {
    pub fn new(r#type: i32, bitrate: i32, user_limit: i32, nsfw: bool, rate_limit_per_user: i32, permission_overwrites: Vec<models::ChannelPermissionOverwriteResponse>, template: String) -> GuildTemplateChannelResponse {
        GuildTemplateChannelResponse {
            id: None,
            r#type,
            name: None,
            position: None,
            topic: None,
            bitrate,
            user_limit,
            nsfw,
            rate_limit_per_user,
            parent_id: None,
            default_auto_archive_duration: None,
            permission_overwrites,
            available_tags: None,
            template,
            default_reaction_emoji: None,
            default_thread_rate_limit_per_user: None,
            default_sort_order: None,
            default_forum_layout: None,
            default_tag_setting: None,
            icon_emoji: None,
            theme_color: None,
        }
    }
}

