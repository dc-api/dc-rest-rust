//! # Discord HTTP API (Preview) - REST API Client
//! 
//! Preview of the Discord v10 HTTP API specification. See https://discord.com/developers/docs for more details.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 10
//! - **Modified**: 2025-07-01T10:27:33.009959252Z[Etc/UTC]
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
pub struct GuildAuditLogResponse {
    #[serde(rename = "audit_log_entries")]
    pub audit_log_entries: Vec<models::AuditLogEntryResponse>,
    #[serde(rename = "users")]
    pub users: Vec<models::UserResponse>,
    #[serde(rename = "integrations")]
    pub integrations: Vec<models::GuildAuditLogResponseIntegrationsInner>,
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<models::ListChannelWebhooks200ResponseInner>,
    #[serde(rename = "guild_scheduled_events")]
    pub guild_scheduled_events: Vec<models::ListGuildScheduledEvents200ResponseInner>,
    #[serde(rename = "threads")]
    pub threads: Vec<models::ThreadResponse>,
    #[serde(rename = "application_commands")]
    pub application_commands: Vec<models::ApplicationCommandResponse>,
    #[serde(rename = "auto_moderation_rules")]
    pub auto_moderation_rules: Vec<models::ListAutoModerationRules200ResponseInner>,
}

impl GuildAuditLogResponse {
    pub fn new(audit_log_entries: Vec<models::AuditLogEntryResponse>, users: Vec<models::UserResponse>, integrations: Vec<models::GuildAuditLogResponseIntegrationsInner>, webhooks: Vec<models::ListChannelWebhooks200ResponseInner>, guild_scheduled_events: Vec<models::ListGuildScheduledEvents200ResponseInner>, threads: Vec<models::ThreadResponse>, application_commands: Vec<models::ApplicationCommandResponse>, auto_moderation_rules: Vec<models::ListAutoModerationRules200ResponseInner>) -> GuildAuditLogResponse {
        GuildAuditLogResponse {
            audit_log_entries,
            users,
            integrations,
            webhooks,
            guild_scheduled_events,
            threads,
            application_commands,
            auto_moderation_rules,
        }
    }
}

