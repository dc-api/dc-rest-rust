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
pub struct GuildWithCountsResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "home_header", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub home_header: Option<Option<String>>,
    #[serde(rename = "splash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub splash: Option<Option<String>>,
    #[serde(rename = "discovery_splash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<Option<String>>,
    #[serde(rename = "features")]
    pub features: Vec<String>,
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "afk_channel_id", skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<String>,
    #[serde(rename = "afk_timeout", deserialize_with = "Option::deserialize")]
    pub afk_timeout: Option<i32>,
    #[serde(rename = "system_channel_id", skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<String>,
    #[serde(rename = "system_channel_flags")]
    pub system_channel_flags: i32,
    #[serde(rename = "widget_enabled")]
    pub widget_enabled: bool,
    #[serde(rename = "widget_channel_id", skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<String>,
    #[serde(rename = "verification_level")]
    pub verification_level: i32,
    #[serde(rename = "roles")]
    pub roles: Vec<models::GuildRoleResponse>,
    #[serde(rename = "default_message_notifications")]
    pub default_message_notifications: i32,
    #[serde(rename = "mfa_level")]
    pub mfa_level: i32,
    #[serde(rename = "explicit_content_filter")]
    pub explicit_content_filter: i32,
    #[serde(rename = "max_presences", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<Option<i32>>,
    #[serde(rename = "max_members", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_members: Option<Option<i32>>,
    #[serde(rename = "max_stage_video_channel_users", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<Option<i32>>,
    #[serde(rename = "max_video_channel_users", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<Option<i32>>,
    #[serde(rename = "vanity_url_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<Option<String>>,
    #[serde(rename = "premium_tier")]
    pub premium_tier: i32,
    #[serde(rename = "premium_subscription_count")]
    pub premium_subscription_count: i32,
    #[serde(rename = "preferred_locale")]
    pub preferred_locale: String,
    #[serde(rename = "rules_channel_id", skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<String>,
    #[serde(rename = "safety_alerts_channel_id", skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<String>,
    #[serde(rename = "public_updates_channel_id", skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<String>,
    #[serde(rename = "premium_progress_bar_enabled")]
    pub premium_progress_bar_enabled: bool,
    #[serde(rename = "nsfw")]
    pub nsfw: bool,
    #[serde(rename = "nsfw_level", deserialize_with = "Option::deserialize")]
    pub nsfw_level: Option<i32>,
    #[serde(rename = "emojis")]
    pub emojis: Vec<models::EmojiResponse>,
    #[serde(rename = "stickers")]
    pub stickers: Vec<models::GuildStickerResponse>,
    #[serde(rename = "approximate_member_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<Option<i32>>,
    #[serde(rename = "approximate_presence_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<Option<i32>>,
}

impl GuildWithCountsResponse {
    pub fn new(id: String, name: String, features: Vec<String>, owner_id: String, region: String, afk_timeout: Option<i32>, system_channel_flags: i32, widget_enabled: bool, verification_level: i32, roles: Vec<models::GuildRoleResponse>, default_message_notifications: i32, mfa_level: i32, explicit_content_filter: i32, premium_tier: i32, premium_subscription_count: i32, preferred_locale: String, premium_progress_bar_enabled: bool, nsfw: bool, nsfw_level: Option<i32>, emojis: Vec<models::EmojiResponse>, stickers: Vec<models::GuildStickerResponse>) -> GuildWithCountsResponse {
        GuildWithCountsResponse {
            id,
            name,
            icon: None,
            description: None,
            home_header: None,
            splash: None,
            discovery_splash: None,
            features,
            banner: None,
            owner_id,
            application_id: None,
            region,
            afk_channel_id: None,
            afk_timeout,
            system_channel_id: None,
            system_channel_flags,
            widget_enabled,
            widget_channel_id: None,
            verification_level,
            roles,
            default_message_notifications,
            mfa_level,
            explicit_content_filter,
            max_presences: None,
            max_members: None,
            max_stage_video_channel_users: None,
            max_video_channel_users: None,
            vanity_url_code: None,
            premium_tier,
            premium_subscription_count,
            preferred_locale,
            rules_channel_id: None,
            safety_alerts_channel_id: None,
            public_updates_channel_id: None,
            premium_progress_bar_enabled,
            nsfw,
            nsfw_level,
            emojis,
            stickers,
            approximate_member_count: None,
            approximate_presence_count: None,
        }
    }
}

