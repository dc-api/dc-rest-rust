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
pub struct WidgetMember {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "discriminator", deserialize_with = "Option::deserialize")]
    pub discriminator: Option<String>,
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<serde_json::Value>>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "activity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub activity: Option<Option<Box<models::WidgetActivity>>>,
    #[serde(rename = "deaf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deaf: Option<Option<bool>>,
    #[serde(rename = "mute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mute: Option<Option<bool>>,
    #[serde(rename = "self_deaf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub self_deaf: Option<Option<bool>>,
    #[serde(rename = "self_mute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub self_mute: Option<Option<bool>>,
    #[serde(rename = "suppress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suppress: Option<Option<bool>>,
    #[serde(rename = "channel_id", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl WidgetMember {
    pub fn new(id: String, username: String, discriminator: Option<String>, status: String, avatar_url: String) -> WidgetMember {
        WidgetMember {
            id,
            username,
            discriminator,
            avatar: None,
            status,
            avatar_url,
            activity: None,
            deaf: None,
            mute: None,
            self_deaf: None,
            self_mute: None,
            suppress: None,
            channel_id: None,
        }
    }
}

