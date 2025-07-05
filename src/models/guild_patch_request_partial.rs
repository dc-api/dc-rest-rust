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
pub struct GuildPatchRequestPartial {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "verification_level", skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<i32>,
    #[serde(rename = "default_message_notifications", skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<i32>,
    #[serde(rename = "explicit_content_filter", skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<i32>,
    #[serde(rename = "preferred_locale", skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
    #[serde(rename = "afk_timeout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<Option<i32>>,
    #[serde(rename = "afk_channel_id", skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<String>,
    #[serde(rename = "system_channel_id", skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<String>,
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "splash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub splash: Option<Option<String>>,
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    #[serde(rename = "system_channel_flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<Option<i32>>,
    #[serde(rename = "features", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub features: Option<Option<Vec<String>>>,
    #[serde(rename = "discovery_splash", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<Option<String>>,
    #[serde(rename = "home_header", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub home_header: Option<Option<String>>,
    #[serde(rename = "rules_channel_id", skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<String>,
    #[serde(rename = "safety_alerts_channel_id", skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<String>,
    #[serde(rename = "public_updates_channel_id", skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<String>,
    #[serde(rename = "premium_progress_bar_enabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<Option<bool>>,
}

impl GuildPatchRequestPartial {
    pub fn new() -> GuildPatchRequestPartial {
        GuildPatchRequestPartial {
            name: None,
            description: None,
            region: None,
            icon: None,
            verification_level: None,
            default_message_notifications: None,
            explicit_content_filter: None,
            preferred_locale: None,
            afk_timeout: None,
            afk_channel_id: None,
            system_channel_id: None,
            owner_id: None,
            splash: None,
            banner: None,
            system_channel_flags: None,
            features: None,
            discovery_splash: None,
            home_header: None,
            rules_channel_id: None,
            safety_alerts_channel_id: None,
            public_updates_channel_id: None,
            premium_progress_bar_enabled: None,
        }
    }
}

