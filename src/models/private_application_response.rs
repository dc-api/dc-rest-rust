//! # Discord HTTP API (Preview) - REST API Client
//! 
//! Preview of the Discord v10 HTTP API specification. See https://discord.com/developers/docs for more details.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 10
//! - **Modified**: 2025-07-01T06:33:04.448935044Z[Etc/UTC]
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
pub struct PrivateApplicationResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<i32>>,
    #[serde(rename = "cover_image", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<Option<String>>,
    #[serde(rename = "primary_sku_id", skip_serializing_if = "Option::is_none")]
    pub primary_sku_id: Option<String>,
    #[serde(rename = "bot", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bot: Option<Option<Box<models::UserResponse>>>,
    #[serde(rename = "slug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slug: Option<Option<String>>,
    #[serde(rename = "guild_id", skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(rename = "rpc_origins", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rpc_origins: Option<Option<Vec<String>>>,
    #[serde(rename = "bot_public", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bot_public: Option<Option<bool>>,
    #[serde(rename = "bot_require_code_grant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bot_require_code_grant: Option<Option<bool>>,
    #[serde(rename = "terms_of_service_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<Option<String>>,
    #[serde(rename = "privacy_policy_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<Option<String>>,
    #[serde(rename = "custom_install_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<Option<String>>,
    #[serde(rename = "install_params", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub install_params: Option<Option<Box<models::ApplicationOAuth2InstallParamsResponse>>>,
    #[serde(rename = "integration_types_config", skip_serializing_if = "Option::is_none")]
    pub integration_types_config: Option<std::collections::HashMap<String, models::ApplicationIntegrationTypeConfigurationResponse>>,
    #[serde(rename = "verify_key")]
    pub verify_key: String,
    #[serde(rename = "flags")]
    pub flags: i32,
    #[serde(rename = "max_participants", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<Option<i32>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<String>>>,
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<String>,
    #[serde(rename = "interactions_endpoint_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<Option<String>>,
    #[serde(rename = "role_connections_verification_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<Option<String>>,
    #[serde(rename = "owner")]
    pub owner: Box<models::UserResponse>,
    #[serde(rename = "approximate_guild_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub approximate_guild_count: Option<Option<i32>>,
    #[serde(rename = "approximate_user_install_count")]
    pub approximate_user_install_count: i32,
    #[serde(rename = "approximate_user_authorization_count")]
    pub approximate_user_authorization_count: i32,
    #[serde(rename = "explicit_content_filter")]
    pub explicit_content_filter: i32,
    #[serde(rename = "team", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub team: Option<Option<Box<models::TeamResponse>>>,
}

impl PrivateApplicationResponse {
    pub fn new(id: String, name: String, description: String, verify_key: String, flags: i32, redirect_uris: Vec<String>, owner: models::UserResponse, approximate_user_install_count: i32, approximate_user_authorization_count: i32, explicit_content_filter: i32) -> PrivateApplicationResponse {
        PrivateApplicationResponse {
            id,
            name,
            icon: None,
            description,
            r#type: None,
            cover_image: None,
            primary_sku_id: None,
            bot: None,
            slug: None,
            guild_id: None,
            rpc_origins: None,
            bot_public: None,
            bot_require_code_grant: None,
            terms_of_service_url: None,
            privacy_policy_url: None,
            custom_install_url: None,
            install_params: None,
            integration_types_config: None,
            verify_key,
            flags,
            max_participants: None,
            tags: None,
            redirect_uris,
            interactions_endpoint_url: None,
            role_connections_verification_url: None,
            owner: Box::new(owner),
            approximate_guild_count: None,
            approximate_user_install_count,
            approximate_user_authorization_count,
            explicit_content_filter,
            team: None,
        }
    }
}

