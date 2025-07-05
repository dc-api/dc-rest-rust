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
pub struct VoiceStateResponse {
    #[serde(rename = "channel_id", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "deaf")]
    pub deaf: bool,
    #[serde(rename = "guild_id", skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(rename = "member", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub member: Option<Option<Box<models::GuildMemberResponse>>>,
    #[serde(rename = "mute")]
    pub mute: bool,
    #[serde(rename = "request_to_speak_timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub request_to_speak_timestamp: Option<Option<String>>,
    #[serde(rename = "suppress")]
    pub suppress: bool,
    #[serde(rename = "self_stream", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub self_stream: Option<Option<bool>>,
    #[serde(rename = "self_deaf")]
    pub self_deaf: bool,
    #[serde(rename = "self_mute")]
    pub self_mute: bool,
    #[serde(rename = "self_video")]
    pub self_video: bool,
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl VoiceStateResponse {
    pub fn new(deaf: bool, mute: bool, suppress: bool, self_deaf: bool, self_mute: bool, self_video: bool, session_id: String, user_id: String) -> VoiceStateResponse {
        VoiceStateResponse {
            channel_id: None,
            deaf,
            guild_id: None,
            member: None,
            mute,
            request_to_speak_timestamp: None,
            suppress,
            self_stream: None,
            self_deaf,
            self_mute,
            self_video,
            session_id,
            user_id,
        }
    }
}

