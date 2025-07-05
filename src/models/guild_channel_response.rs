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
pub struct GuildChannelResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "last_message_id", skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    #[serde(rename = "flags")]
    pub flags: i32,
    #[serde(rename = "last_pin_timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Option<String>>,
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "rate_limit_per_user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<Option<i32>>,
    #[serde(rename = "bitrate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<Option<i32>>,
    #[serde(rename = "user_limit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<Option<i32>>,
    #[serde(rename = "rtc_region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<Option<String>>,
    #[serde(rename = "video_quality_mode", skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<i32>,
    #[serde(rename = "permissions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<String>>,
    #[serde(rename = "topic", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub topic: Option<Option<String>>,
    #[serde(rename = "default_auto_archive_duration", skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<i32>,
    #[serde(rename = "default_thread_rate_limit_per_user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<Option<i32>>,
    #[serde(rename = "position")]
    pub position: i32,
    #[serde(rename = "permission_overwrites", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Option<Vec<models::ChannelPermissionOverwriteResponse>>>,
    #[serde(rename = "nsfw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<Option<bool>>,
    #[serde(rename = "available_tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Option<Vec<models::ForumTagResponse>>>,
    #[serde(rename = "default_reaction_emoji", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<Option<Box<models::DefaultReactionEmojiResponse>>>,
    #[serde(rename = "default_sort_order", skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<i32>,
    #[serde(rename = "default_forum_layout", skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<i32>,
    #[serde(rename = "default_tag_setting", skip_serializing_if = "Option::is_none")]
    pub default_tag_setting: Option<String>,
    #[serde(rename = "hd_streaming_until", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hd_streaming_until: Option<Option<String>>,
    #[serde(rename = "hd_streaming_buyer_id", skip_serializing_if = "Option::is_none")]
    pub hd_streaming_buyer_id: Option<String>,
}

impl GuildChannelResponse {
    pub fn new(id: String, r#type: i32, flags: i32, guild_id: String, name: String, position: i32) -> GuildChannelResponse {
        GuildChannelResponse {
            id,
            r#type,
            last_message_id: None,
            flags,
            last_pin_timestamp: None,
            guild_id,
            name,
            parent_id: None,
            rate_limit_per_user: None,
            bitrate: None,
            user_limit: None,
            rtc_region: None,
            video_quality_mode: None,
            permissions: None,
            topic: None,
            default_auto_archive_duration: None,
            default_thread_rate_limit_per_user: None,
            position,
            permission_overwrites: None,
            nsfw: None,
            available_tags: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_forum_layout: None,
            default_tag_setting: None,
            hd_streaming_until: None,
            hd_streaming_buyer_id: None,
        }
    }
}

