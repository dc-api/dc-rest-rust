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
pub struct BasicMessageResponse {
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "mentions")]
    pub mentions: Vec<models::UserResponse>,
    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<String>,
    #[serde(rename = "attachments")]
    pub attachments: Vec<models::MessageAttachmentResponse>,
    #[serde(rename = "embeds")]
    pub embeds: Vec<models::MessageEmbedResponse>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "edited_timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Option<String>>,
    #[serde(rename = "flags")]
    pub flags: i32,
    #[serde(rename = "components")]
    pub components: Vec<models::BasicMessageResponseComponentsInner>,
    #[serde(rename = "resolved", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Option<Box<models::ResolvedObjectsResponse>>>,
    #[serde(rename = "stickers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Option<Vec<models::GetSticker200Response>>>,
    #[serde(rename = "sticker_items", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sticker_items: Option<Option<Vec<models::MessageStickerItemResponse>>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "author")]
    pub author: Box<models::UserResponse>,
    #[serde(rename = "pinned")]
    pub pinned: bool,
    #[serde(rename = "mention_everyone")]
    pub mention_everyone: bool,
    #[serde(rename = "tts")]
    pub tts: bool,
    #[serde(rename = "call", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call: Option<Option<Box<models::MessageCallResponse>>>,
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<serde_json::Value>,
    #[serde(rename = "application", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub application: Option<Option<Box<models::BasicApplicationResponse>>>,
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "interaction", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interaction: Option<Option<Box<models::MessageInteractionResponse>>>,
    #[serde(rename = "nonce", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<Box<models::BasicMessageResponseNonce>>>,
    #[serde(rename = "webhook_id", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    #[serde(rename = "message_reference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Option<Box<models::MessageReferenceResponse>>>,
    #[serde(rename = "thread", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub thread: Option<Option<Box<models::ThreadResponse>>>,
    #[serde(rename = "mention_channels", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mention_channels: Option<Option<Vec<models::MessageMentionChannelResponse>>>,
    #[serde(rename = "role_subscription_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role_subscription_data: Option<Option<Box<models::MessageRoleSubscriptionDataResponse>>>,
    #[serde(rename = "purchase_notification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub purchase_notification: Option<Option<Box<models::PurchaseNotificationResponse>>>,
    #[serde(rename = "position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<i32>>,
    #[serde(rename = "poll", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub poll: Option<Option<Box<models::PollResponse>>>,
    #[serde(rename = "interaction_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interaction_metadata: Option<Option<Box<models::BasicMessageResponseInteractionMetadata>>>,
    #[serde(rename = "message_snapshots", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_snapshots: Option<Option<Vec<models::MessageSnapshotResponse>>>,
}

impl BasicMessageResponse {
    pub fn new(r#type: i32, content: String, mentions: Vec<models::UserResponse>, mention_roles: Vec<String>, attachments: Vec<models::MessageAttachmentResponse>, embeds: Vec<models::MessageEmbedResponse>, timestamp: String, flags: i32, components: Vec<models::BasicMessageResponseComponentsInner>, id: String, channel_id: String, author: models::UserResponse, pinned: bool, mention_everyone: bool, tts: bool) -> BasicMessageResponse {
        BasicMessageResponse {
            r#type,
            content,
            mentions,
            mention_roles,
            attachments,
            embeds,
            timestamp,
            edited_timestamp: None,
            flags,
            components,
            resolved: None,
            stickers: None,
            sticker_items: None,
            id,
            channel_id,
            author: Box::new(author),
            pinned,
            mention_everyone,
            tts,
            call: None,
            activity: None,
            application: None,
            application_id: None,
            interaction: None,
            nonce: None,
            webhook_id: None,
            message_reference: None,
            thread: None,
            mention_channels: None,
            role_subscription_data: None,
            purchase_notification: None,
            position: None,
            poll: None,
            interaction_metadata: None,
            message_snapshots: None,
        }
    }
}

