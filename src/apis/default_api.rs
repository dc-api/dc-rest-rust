//! # Discord HTTP API (Preview) - REST API Client
//! 
//! Preview of the Discord v10 HTTP API specification. See https://discord.com/developers/docs for more details.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 10
//! - **Modified**: 2025-11-02T04:40:22.540540420Z[Etc/UTC]
//! - **Generator Version**: 7.17.0
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

use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed successes of method [`add_group_dm_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGroupDmUserSuccess {
    Status201(models::AddGroupDmUser201Response),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`add_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGuildMemberSuccess {
    Status201(models::GuildMemberResponse),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`add_guild_member_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGuildMemberRoleSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`add_lobby_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddLobbyMemberSuccess {
    Status200(models::LobbyMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`add_my_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddMyMessageReactionSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`add_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddThreadMemberSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`applications_get_activity_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationsGetActivityInstanceSuccess {
    Status200(models::EmbeddedActivityInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`ban_user_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanUserFromGuildSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bot_partner_sdk_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotPartnerSdkTokenSuccess {
    Status200(models::ProvisionalTokenResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bot_partner_sdk_unmerge_provisional_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotPartnerSdkUnmergeProvisionalAccountSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_ban_users_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkBanUsersFromGuildSuccess {
    Status200(models::BulkBanUsersResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_delete_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkDeleteMessagesSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_set_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkSetApplicationCommandsSuccess {
    Status200(Vec<models::ApplicationCommandResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_set_guild_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkSetGuildApplicationCommandsSuccess {
    Status200(Vec<models::ApplicationCommandResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_update_guild_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateGuildChannelsSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_update_guild_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateGuildRolesSuccess {
    Status200(Vec<models::GuildRoleResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`bulk_update_lobby_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateLobbyMembersSuccess {
    Status200(Vec<models::LobbyMemberResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`consume_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConsumeEntitlementSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationCommandSuccess {
    Status200(models::ApplicationCommandResponse),
    Status201(models::ApplicationCommandResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationEmojiSuccess {
    Status201(models::EmojiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAutoModerationRuleSuccess {
    Status200(models::CreateAutoModerationRule200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_channel_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChannelInviteSuccess {
    Status200(models::CreateChannelInvite200Response),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_dm`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmSuccess {
    Status200(models::AddGroupDmUser201Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEntitlementSuccess {
    Status200(models::EntitlementResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildApplicationCommandSuccess {
    Status200(models::ApplicationCommandResponse),
    Status201(models::ApplicationCommandResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildChannelSuccess {
    Status201(models::GuildChannelResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildEmojiSuccess {
    Status201(models::EmojiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildRoleSuccess {
    Status200(models::GuildRoleResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildScheduledEventSuccess {
    Status200(models::ListGuildScheduledEvents200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildSoundboardSoundSuccess {
    Status201(models::SoundboardSoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildStickerSuccess {
    Status201(models::GuildStickerResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildTemplateSuccess {
    Status200(models::GuildTemplateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_interaction_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInteractionResponseSuccess {
    Status200(models::InteractionCallbackResponse),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_linked_lobby_guild_invite_for_self`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLinkedLobbyGuildInviteForSelfSuccess {
    Status200(models::LobbyGuildInviteResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_linked_lobby_guild_invite_for_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLinkedLobbyGuildInviteForUserSuccess {
    Status200(models::LobbyGuildInviteResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLobbySuccess {
    Status201(models::LobbyResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_lobby_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLobbyMessageSuccess {
    Status201(models::LobbyMessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_or_join_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrJoinLobbySuccess {
    Status200(models::LobbyResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePinSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateStageInstanceSuccess {
    Status200(models::StageInstanceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadSuccess {
    Status201(models::CreatedThreadResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_thread_from_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadFromMessageSuccess {
    Status201(models::ThreadResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`create_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhookSuccess {
    Status200(models::GuildIncomingWebhookResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`crosspost_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrosspostMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_all_message_reactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAllMessageReactionsSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_all_message_reactions_by_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAllMessageReactionsByEmojiSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationCommandSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationEmojiSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationUserRoleConnectionSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAutoModerationRuleSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChannelSuccess {
    Status200(models::GetChannel200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_channel_permission_overwrite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChannelPermissionOverwriteSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEntitlementSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_group_dm_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGroupDmUserSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildApplicationCommandSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildEmojiSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildIntegrationSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildMemberSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_member_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildMemberRoleSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildRoleSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildScheduledEventSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildSoundboardSoundSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildStickerSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildTemplateSuccess {
    Status200(models::GuildTemplateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_lobby_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLobbyMemberSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessageSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_my_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMyMessageReactionSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOriginalWebhookMessageSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePinSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStageInstanceSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteThreadMemberSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_user_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserMessageReactionSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookByTokenSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookMessageSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`deprecated_create_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedCreatePinSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`deprecated_delete_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedDeletePinSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`deprecated_list_pins`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedListPinsSuccess {
    Status200(Vec<models::MessageResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditLobbySuccess {
    Status200(models::LobbyResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`edit_lobby_channel_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditLobbyChannelLinkSuccess {
    Status200(models::LobbyResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`execute_github_compatible_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteGithubCompatibleWebhookSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`execute_slack_compatible_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteSlackCompatibleWebhookSuccess {
    Status200(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`execute_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteWebhookSuccess {
    Status200(models::MessageResponse),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`follow_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowChannelSuccess {
    Status200(models::ChannelFollowerResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_active_guild_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveGuildThreadsSuccess {
    Status200(models::ThreadsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_answer_voters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAnswerVotersSuccess {
    Status200(models::PollAnswerDetailsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationSuccess {
    Status200(models::PrivateApplicationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationCommandSuccess {
    Status200(models::ApplicationCommandResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationEmojiSuccess {
    Status200(models::EmojiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_application_role_connections_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationRoleConnectionsMetadataSuccess {
    Status200(Vec<models::ApplicationRoleConnectionsMetadataItemResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationUserRoleConnectionSuccess {
    Status200(models::ApplicationUserRoleConnectionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAutoModerationRuleSuccess {
    Status200(models::CreateAutoModerationRule200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_bot_gateway`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBotGatewaySuccess {
    Status200(models::GatewayBotResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChannelSuccess {
    Status200(models::GetChannel200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEntitlementSuccess {
    Status200(models::EntitlementResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_entitlements`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEntitlementsSuccess {
    Status200(Vec<models::EntitlementResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_gateway`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGatewaySuccess {
    Status200(models::GatewayResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildSuccess {
    Status200(models::GuildWithCountsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildApplicationCommandSuccess {
    Status200(models::ApplicationCommandResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildApplicationCommandPermissionsSuccess {
    Status200(models::CommandPermissionsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_ban`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildBanSuccess {
    Status200(models::GuildBanResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildEmojiSuccess {
    Status200(models::EmojiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildMemberSuccess {
    Status200(models::GuildMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_new_member_welcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildNewMemberWelcomeSuccess {
    Status200(models::GuildHomeSettingsResponse),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_preview`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildPreviewSuccess {
    Status200(models::GuildPreviewResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildRoleSuccess {
    Status200(models::GuildRoleResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildScheduledEventSuccess {
    Status200(models::ListGuildScheduledEvents200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildSoundboardSoundSuccess {
    Status200(models::SoundboardSoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildStickerSuccess {
    Status200(models::GuildStickerResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildTemplateSuccess {
    Status200(models::GuildTemplateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_vanity_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildVanityUrlSuccess {
    Status200(models::VanityUrlResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWebhooksSuccess {
    Status200(Vec<models::ListChannelWebhooks200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_welcome_screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWelcomeScreenSuccess {
    Status200(models::GuildWelcomeScreenResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetSuccess {
    Status200(models::WidgetResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_widget_png`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetPngSuccess {
    Status200(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guild_widget_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetSettingsSuccess {
    Status200(models::WidgetSettingsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_guilds_onboarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildsOnboardingSuccess {
    Status200(models::UserGuildOnboardingResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLobbySuccess {
    Status200(models::LobbyResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_lobby_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLobbyMessagesSuccess {
    Status200(Vec<models::LobbyMessageResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyApplicationSuccess {
    Status200(models::PrivateApplicationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyGuildMemberSuccess {
    Status200(models::PrivateGuildMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_oauth2_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyOauth2ApplicationSuccess {
    Status200(models::PrivateApplicationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_oauth2_authorization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyOauth2AuthorizationSuccess {
    Status200(models::OAuth2GetAuthorizationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_my_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyUserSuccess {
    Status200(models::UserPiiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_openid_connect_userinfo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOpenidConnectUserinfoSuccess {
    Status200(models::OAuth2GetOpenIdConnectUserInfoResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOriginalWebhookMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_public_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicKeysSuccess {
    Status200(models::OAuth2GetKeys),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_self_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSelfVoiceStateSuccess {
    Status200(models::VoiceStateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_soundboard_default_sounds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSoundboardDefaultSoundsSuccess {
    Status200(Vec<models::SoundboardSoundResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStageInstanceSuccess {
    Status200(models::StageInstanceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerSuccess {
    Status200(models::GetSticker200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_sticker_pack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerPackSuccess {
    Status200(models::StickerPackResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetThreadMemberSuccess {
    Status200(models::ThreadMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserSuccess {
    Status200(models::UserResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVoiceStateSuccess {
    Status200(models::VoiceStateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookSuccess {
    Status200(models::ListChannelWebhooks200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookByTokenSuccess {
    Status200(models::ListChannelWebhooks200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`invite_resolve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteResolveSuccess {
    Status200(models::CreateChannelInvite200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`invite_revoke`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRevokeSuccess {
    Status200(models::CreateChannelInvite200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`join_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JoinThreadSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`leave_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveGuildSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`leave_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveLobbySuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`leave_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveThreadSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationCommandsSuccess {
    Status200(Vec<models::ApplicationCommandResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_application_emojis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationEmojisSuccess {
    Status200(models::ListApplicationEmojisResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_auto_moderation_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAutoModerationRulesSuccess {
    Status200(Vec<models::ListAutoModerationRules200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_channel_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListChannelInvitesSuccess {
    Status200(Vec<models::ListChannelInvites200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_channel_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListChannelWebhooksSuccess {
    Status200(Vec<models::ListChannelWebhooks200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildApplicationCommandPermissionsSuccess {
    Status200(Vec<models::CommandPermissionsResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildApplicationCommandsSuccess {
    Status200(Vec<models::ApplicationCommandResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_audit_log_entries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildAuditLogEntriesSuccess {
    Status200(models::GuildAuditLogResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_bans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildBansSuccess {
    Status200(Vec<models::GuildBanResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildChannelsSuccess {
    Status200(Vec<models::GetChannel200Response>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_emojis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildEmojisSuccess {
    Status200(Vec<models::EmojiResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_integrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildIntegrationsSuccess {
    Status200(Vec<models::ListGuildIntegrations200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildInvitesSuccess {
    Status200(Vec<models::ListChannelInvites200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildMembersSuccess {
    Status200(Vec<models::GuildMemberResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildRolesSuccess {
    Status200(Vec<models::GuildRoleResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_scheduled_event_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildScheduledEventUsersSuccess {
    Status200(Vec<models::ScheduledEventUserResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_scheduled_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildScheduledEventsSuccess {
    Status200(Vec<models::ListGuildScheduledEvents200ResponseInner>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_soundboard_sounds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildSoundboardSoundsSuccess {
    Status200(models::ListGuildSoundboardSoundsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildStickersSuccess {
    Status200(Vec<models::GuildStickerResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildTemplatesSuccess {
    Status200(Vec<models::GuildTemplateResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_guild_voice_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildVoiceRegionsSuccess {
    Status200(Vec<models::VoiceRegionResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_message_reactions_by_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessageReactionsByEmojiSuccess {
    Status200(Vec<models::UserResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessagesSuccess {
    Status200(Vec<models::MessageResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_my_connections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyConnectionsSuccess {
    Status200(Vec<models::ConnectedAccountResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_my_guilds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyGuildsSuccess {
    Status200(Vec<models::MyGuildResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_my_private_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyPrivateArchivedThreadsSuccess {
    Status200(models::ThreadsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_pins`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPinsSuccess {
    Status200(models::PinnedMessagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_private_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrivateArchivedThreadsSuccess {
    Status200(models::ThreadsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_public_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPublicArchivedThreadsSuccess {
    Status200(models::ThreadsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_sticker_packs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStickerPacksSuccess {
    Status200(models::StickerPackCollectionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_thread_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListThreadMembersSuccess {
    Status200(Vec<models::ThreadMemberResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_voice_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceRegionsSuccess {
    Status200(Vec<models::VoiceRegionResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`partner_sdk_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartnerSdkTokenSuccess {
    Status200(models::ProvisionalTokenResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`partner_sdk_unmerge_provisional_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartnerSdkUnmergeProvisionalAccountSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`poll_expire`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollExpireSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`preview_prune_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreviewPruneGuildSuccess {
    Status200(models::GuildPruneResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`prune_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PruneGuildSuccess {
    Status200(models::GuildPruneResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`put_guilds_onboarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuildsOnboardingSuccess {
    Status200(models::GuildOnboardingResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`search_guild_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchGuildMembersSuccess {
    Status200(Vec<models::GuildMemberResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`send_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendSoundboardSoundSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_channel_permission_overwrite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChannelPermissionOverwriteSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`set_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGuildApplicationCommandPermissionsSuccess {
    Status200(models::CommandPermissionsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`sync_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncGuildTemplateSuccess {
    Status200(models::GuildTemplateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`thread_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThreadSearchSuccess {
    Status200(models::ThreadSearchResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`trigger_typing_indicator`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerTypingIndicatorSuccess {
    Status200(serde_json::Value),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`unban_user_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanUserFromGuildSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationSuccess {
    Status200(models::PrivateApplicationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationCommandSuccess {
    Status200(models::ApplicationCommandResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationEmojiSuccess {
    Status200(models::EmojiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_application_role_connections_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationRoleConnectionsMetadataSuccess {
    Status200(Vec<models::ApplicationRoleConnectionsMetadataItemResponse>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationUserRoleConnectionSuccess {
    Status200(models::ApplicationUserRoleConnectionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAutoModerationRuleSuccess {
    Status200(models::CreateAutoModerationRule200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateChannelSuccess {
    Status200(models::GetChannel200Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildSuccess {
    Status200(models::GuildResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildApplicationCommandSuccess {
    Status200(models::ApplicationCommandResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildEmojiSuccess {
    Status200(models::EmojiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildMemberSuccess {
    Status200(models::GuildMemberResponse),
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildRoleSuccess {
    Status200(models::GuildRoleResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildScheduledEventSuccess {
    Status200(models::ListGuildScheduledEvents200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildSoundboardSoundSuccess {
    Status200(models::SoundboardSoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildStickerSuccess {
    Status200(models::GuildStickerResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildTemplateSuccess {
    Status200(models::GuildTemplateResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_welcome_screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildWelcomeScreenSuccess {
    Status200(models::GuildWelcomeScreenResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_guild_widget_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildWidgetSettingsSuccess {
    Status200(models::WidgetSettingsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_my_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyApplicationSuccess {
    Status200(models::PrivateApplicationResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_my_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyGuildMemberSuccess {
    Status200(models::PrivateGuildMemberResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_my_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyUserSuccess {
    Status200(models::UserPiiResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOriginalWebhookMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_self_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSelfVoiceStateSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateStageInstanceSuccess {
    Status200(models::StageInstanceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateVoiceStateSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookSuccess {
    Status200(models::ListChannelWebhooks200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookByTokenSuccess {
    Status200(models::ListChannelWebhooks200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookMessageSuccess {
    Status200(models::MessageResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`upload_application_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadApplicationAttachmentSuccess {
    Status200(models::ActivitiesAttachmentResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_group_dm_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGroupDmUserError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGuildMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_guild_member_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGuildMemberRoleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_lobby_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddLobbyMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_my_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddMyMessageReactionError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddThreadMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`applications_get_activity_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationsGetActivityInstanceError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ban_user_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanUserFromGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_partner_sdk_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotPartnerSdkTokenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_partner_sdk_unmerge_provisional_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotPartnerSdkUnmergeProvisionalAccountError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_ban_users_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkBanUsersFromGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_delete_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkDeleteMessagesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_set_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkSetApplicationCommandsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_set_guild_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkSetGuildApplicationCommandsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_guild_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateGuildChannelsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_guild_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateGuildRolesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_lobby_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateLobbyMembersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`consume_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConsumeEntitlementError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAutoModerationRuleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_channel_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChannelInviteError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dm`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEntitlementError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildChannelError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildRoleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildScheduledEventError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildSoundboardSoundError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildStickerError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildTemplateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_interaction_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInteractionResponseError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_linked_lobby_guild_invite_for_self`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLinkedLobbyGuildInviteForSelfError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_linked_lobby_guild_invite_for_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLinkedLobbyGuildInviteForUserError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLobbyError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_lobby_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLobbyMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_or_join_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrJoinLobbyError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePinError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateStageInstanceError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_thread_from_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadFromMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crosspost_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrosspostMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_all_message_reactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAllMessageReactionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_all_message_reactions_by_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAllMessageReactionsByEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationUserRoleConnectionError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAutoModerationRuleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChannelError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_channel_permission_overwrite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChannelPermissionOverwriteError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEntitlementError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_group_dm_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGroupDmUserError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildIntegrationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_member_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildMemberRoleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildRoleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildScheduledEventError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildSoundboardSoundError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildStickerError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildTemplateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_lobby_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLobbyMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_my_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMyMessageReactionError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOriginalWebhookMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePinError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStageInstanceError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteThreadMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserMessageReactionError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookByTokenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deprecated_create_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedCreatePinError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deprecated_delete_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedDeletePinError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deprecated_list_pins`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedListPinsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditLobbyError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_lobby_channel_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditLobbyChannelLinkError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`execute_github_compatible_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteGithubCompatibleWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`execute_slack_compatible_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteSlackCompatibleWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`execute_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`follow_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowChannelError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_active_guild_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveGuildThreadsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_answer_voters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAnswerVotersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_role_connections_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationRoleConnectionsMetadataError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationUserRoleConnectionError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAutoModerationRuleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_bot_gateway`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBotGatewayError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChannelError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEntitlementError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_entitlements`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEntitlementsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_gateway`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGatewayError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildApplicationCommandPermissionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_ban`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildBanError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_new_member_welcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildNewMemberWelcomeError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_preview`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildPreviewError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildRoleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildScheduledEventError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildSoundboardSoundError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildStickerError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildTemplateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_vanity_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildVanityUrlError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWebhooksError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_welcome_screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWelcomeScreenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_widget_png`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetPngError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_widget_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetSettingsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guilds_onboarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildsOnboardingError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLobbyError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lobby_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLobbyMessagesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyApplicationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyGuildMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_oauth2_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyOauth2ApplicationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_oauth2_authorization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyOauth2AuthorizationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyUserError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_openid_connect_userinfo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOpenidConnectUserinfoError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOriginalWebhookMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_public_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicKeysError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_self_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSelfVoiceStateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_soundboard_default_sounds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSoundboardDefaultSoundsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStageInstanceError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sticker_pack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerPackError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetThreadMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVoiceStateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookByTokenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`invite_resolve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteResolveError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`invite_revoke`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRevokeError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`join_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JoinThreadError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveLobbyError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveThreadError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationCommandsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_application_emojis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationEmojisError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_auto_moderation_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAutoModerationRulesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_channel_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListChannelInvitesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_channel_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListChannelWebhooksError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildApplicationCommandPermissionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildApplicationCommandsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_audit_log_entries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildAuditLogEntriesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_bans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildBansError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildChannelsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_emojis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildEmojisError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_integrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildIntegrationsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildInvitesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildMembersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildRolesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_scheduled_event_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildScheduledEventUsersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_scheduled_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildScheduledEventsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_soundboard_sounds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildSoundboardSoundsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildStickersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildTemplatesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_voice_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildVoiceRegionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_message_reactions_by_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessageReactionsByEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessagesError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_my_connections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyConnectionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_my_guilds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyGuildsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_my_private_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyPrivateArchivedThreadsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_pins`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPinsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_private_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrivateArchivedThreadsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_public_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPublicArchivedThreadsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sticker_packs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStickerPacksError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_thread_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListThreadMembersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_voice_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceRegionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`partner_sdk_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartnerSdkTokenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`partner_sdk_unmerge_provisional_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartnerSdkUnmergeProvisionalAccountError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`poll_expire`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollExpireError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`preview_prune_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreviewPruneGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`prune_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PruneGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_guilds_onboarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuildsOnboardingError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_guild_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchGuildMembersError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendSoundboardSoundError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_channel_permission_overwrite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChannelPermissionOverwriteError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGuildApplicationCommandPermissionsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sync_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncGuildTemplateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`thread_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThreadSearchError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`trigger_typing_indicator`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerTypingIndicatorError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unban_user_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanUserFromGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_role_connections_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationRoleConnectionsMetadataError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationUserRoleConnectionError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAutoModerationRuleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateChannelError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildApplicationCommandError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildEmojiError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildRoleError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildScheduledEventError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildSoundboardSoundError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildStickerError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildTemplateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_welcome_screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildWelcomeScreenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_widget_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildWidgetSettingsError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyApplicationError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyGuildMemberError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyUserError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOriginalWebhookMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_self_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSelfVoiceStateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateStageInstanceError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateVoiceStateError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookByTokenError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookMessageError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_application_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadApplicationAttachmentError {
    Status429(models::RatelimitedResponse),
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn add_group_dm_user(configuration: &configuration::Configuration, channel_id: &str, user_id: &str, add_group_dm_user_request: models::AddGroupDmUserRequest) -> Result<ResponseContent<AddGroupDmUserSuccess>, Error<AddGroupDmUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_user_id = user_id;
    let p_body_add_group_dm_user_request = add_group_dm_user_request;

    let uri_str = format!("{}/channels/{channel_id}/recipients/{user_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_add_group_dm_user_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<AddGroupDmUserSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<AddGroupDmUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn add_guild_member(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, bot_add_guild_member_request: models::BotAddGuildMemberRequest) -> Result<ResponseContent<AddGuildMemberSuccess>, Error<AddGuildMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_body_bot_add_guild_member_request = bot_add_guild_member_request;

    let uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bot_add_guild_member_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<AddGuildMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<AddGuildMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn add_guild_member_role(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, role_id: &str) -> Result<ResponseContent<AddGuildMemberRoleSuccess>, Error<AddGuildMemberRoleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_path_role_id = role_id;

    let uri_str = format!("{}/guilds/{guild_id}/members/{user_id}/roles/{role_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id), role_id=crate::apis::urlencode(p_path_role_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<AddGuildMemberRoleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<AddGuildMemberRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn add_lobby_member(configuration: &configuration::Configuration, lobby_id: &str, user_id: &str, add_lobby_member_request: models::AddLobbyMemberRequest) -> Result<ResponseContent<AddLobbyMemberSuccess>, Error<AddLobbyMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_path_user_id = user_id;
    let p_body_add_lobby_member_request = add_lobby_member_request;

    let uri_str = format!("{}/lobbies/{lobby_id}/members/{user_id}", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_add_lobby_member_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<AddLobbyMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<AddLobbyMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn add_my_message_reaction(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, emoji_name: &str) -> Result<ResponseContent<AddMyMessageReactionSuccess>, Error<AddMyMessageReactionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_path_emoji_name = emoji_name;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id), emoji_name=crate::apis::urlencode(p_path_emoji_name));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<AddMyMessageReactionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<AddMyMessageReactionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn add_thread_member(configuration: &configuration::Configuration, channel_id: &str, user_id: &str) -> Result<ResponseContent<AddThreadMemberSuccess>, Error<AddThreadMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/channels/{channel_id}/thread-members/{user_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<AddThreadMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<AddThreadMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn applications_get_activity_instance(configuration: &configuration::Configuration, application_id: &str, instance_id: &str) -> Result<ResponseContent<ApplicationsGetActivityInstanceSuccess>, Error<ApplicationsGetActivityInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_instance_id = instance_id;

    let uri_str = format!("{}/applications/{application_id}/activity-instances/{instance_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), instance_id=crate::apis::urlencode(p_path_instance_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ApplicationsGetActivityInstanceSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ApplicationsGetActivityInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn ban_user_from_guild(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, ban_user_from_guild_request: models::BanUserFromGuildRequest) -> Result<ResponseContent<BanUserFromGuildSuccess>, Error<BanUserFromGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_body_ban_user_from_guild_request = ban_user_from_guild_request;

    let uri_str = format!("{}/guilds/{guild_id}/bans/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_ban_user_from_guild_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BanUserFromGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BanUserFromGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bot_partner_sdk_token(configuration: &configuration::Configuration, bot_partner_sdk_token_request: models::BotPartnerSdkTokenRequest) -> Result<ResponseContent<BotPartnerSdkTokenSuccess>, Error<BotPartnerSdkTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_bot_partner_sdk_token_request = bot_partner_sdk_token_request;

    let uri_str = format!("{}/partner-sdk/token/bot", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bot_partner_sdk_token_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BotPartnerSdkTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BotPartnerSdkTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bot_partner_sdk_unmerge_provisional_account(configuration: &configuration::Configuration, bot_partner_sdk_unmerge_provisional_account_request: models::BotPartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<BotPartnerSdkUnmergeProvisionalAccountSuccess>, Error<BotPartnerSdkUnmergeProvisionalAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_bot_partner_sdk_unmerge_provisional_account_request = bot_partner_sdk_unmerge_provisional_account_request;

    let uri_str = format!("{}/partner-sdk/provisional-accounts/unmerge/bot", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bot_partner_sdk_unmerge_provisional_account_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BotPartnerSdkUnmergeProvisionalAccountSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BotPartnerSdkUnmergeProvisionalAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_ban_users_from_guild(configuration: &configuration::Configuration, guild_id: &str, bulk_ban_users_request: models::BulkBanUsersRequest) -> Result<ResponseContent<BulkBanUsersFromGuildSuccess>, Error<BulkBanUsersFromGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_bulk_ban_users_request = bulk_ban_users_request;

    let uri_str = format!("{}/guilds/{guild_id}/bulk-ban", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bulk_ban_users_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkBanUsersFromGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkBanUsersFromGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_delete_messages(configuration: &configuration::Configuration, channel_id: &str, bulk_delete_messages_request: models::BulkDeleteMessagesRequest) -> Result<ResponseContent<BulkDeleteMessagesSuccess>, Error<BulkDeleteMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_bulk_delete_messages_request = bulk_delete_messages_request;

    let uri_str = format!("{}/channels/{channel_id}/messages/bulk-delete", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bulk_delete_messages_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkDeleteMessagesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkDeleteMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_set_application_commands(configuration: &configuration::Configuration, application_id: &str, application_command_update_request: Option<Vec<models::ApplicationCommandUpdateRequest>>) -> Result<ResponseContent<BulkSetApplicationCommandsSuccess>, Error<BulkSetApplicationCommandsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_application_command_update_request = application_command_update_request;

    let uri_str = format!("{}/applications/{application_id}/commands", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_command_update_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkSetApplicationCommandsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkSetApplicationCommandsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_set_guild_application_commands(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, application_command_update_request: Option<Vec<models::ApplicationCommandUpdateRequest>>) -> Result<ResponseContent<BulkSetGuildApplicationCommandsSuccess>, Error<BulkSetGuildApplicationCommandsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_body_application_command_update_request = application_command_update_request;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_command_update_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkSetGuildApplicationCommandsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkSetGuildApplicationCommandsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_update_guild_channels(configuration: &configuration::Configuration, guild_id: &str, bulk_update_guild_channels_request_inner: Vec<models::BulkUpdateGuildChannelsRequestInner>) -> Result<ResponseContent<BulkUpdateGuildChannelsSuccess>, Error<BulkUpdateGuildChannelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_bulk_update_guild_channels_request_inner = bulk_update_guild_channels_request_inner;

    let uri_str = format!("{}/guilds/{guild_id}/channels", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bulk_update_guild_channels_request_inner);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkUpdateGuildChannelsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkUpdateGuildChannelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_update_guild_roles(configuration: &configuration::Configuration, guild_id: &str, update_role_positions_request: Vec<models::UpdateRolePositionsRequest>) -> Result<ResponseContent<BulkUpdateGuildRolesSuccess>, Error<BulkUpdateGuildRolesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_update_role_positions_request = update_role_positions_request;

    let uri_str = format!("{}/guilds/{guild_id}/roles", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_role_positions_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkUpdateGuildRolesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkUpdateGuildRolesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bulk_update_lobby_members(configuration: &configuration::Configuration, lobby_id: &str, bulk_lobby_member_request: Option<Vec<models::BulkLobbyMemberRequest>>) -> Result<ResponseContent<BulkUpdateLobbyMembersSuccess>, Error<BulkUpdateLobbyMembersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_body_bulk_lobby_member_request = bulk_lobby_member_request;

    let uri_str = format!("{}/lobbies/{lobby_id}/members/bulk", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bulk_lobby_member_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<BulkUpdateLobbyMembersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkUpdateLobbyMembersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn consume_entitlement(configuration: &configuration::Configuration, application_id: &str, entitlement_id: &str) -> Result<ResponseContent<ConsumeEntitlementSuccess>, Error<ConsumeEntitlementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_entitlement_id = entitlement_id;

    let uri_str = format!("{}/applications/{application_id}/entitlements/{entitlement_id}/consume", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), entitlement_id=crate::apis::urlencode(p_path_entitlement_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ConsumeEntitlementSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ConsumeEntitlementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_application_command(configuration: &configuration::Configuration, application_id: &str, application_command_create_request: models::ApplicationCommandCreateRequest) -> Result<ResponseContent<CreateApplicationCommandSuccess>, Error<CreateApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_application_command_create_request = application_command_create_request;

    let uri_str = format!("{}/applications/{application_id}/commands", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_command_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_application_emoji(configuration: &configuration::Configuration, application_id: &str, create_application_emoji_request: models::CreateApplicationEmojiRequest) -> Result<ResponseContent<CreateApplicationEmojiSuccess>, Error<CreateApplicationEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_create_application_emoji_request = create_application_emoji_request;

    let uri_str = format!("{}/applications/{application_id}/emojis", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_application_emoji_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateApplicationEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateApplicationEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_auto_moderation_rule(configuration: &configuration::Configuration, guild_id: &str, create_auto_moderation_rule_request: models::CreateAutoModerationRuleRequest) -> Result<ResponseContent<CreateAutoModerationRuleSuccess>, Error<CreateAutoModerationRuleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_create_auto_moderation_rule_request = create_auto_moderation_rule_request;

    let uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_auto_moderation_rule_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateAutoModerationRuleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateAutoModerationRuleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_channel_invite(configuration: &configuration::Configuration, channel_id: &str, create_channel_invite_request: models::CreateChannelInviteRequest) -> Result<ResponseContent<CreateChannelInviteSuccess>, Error<CreateChannelInviteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_create_channel_invite_request = create_channel_invite_request;

    let uri_str = format!("{}/channels/{channel_id}/invites", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_channel_invite_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateChannelInviteSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateChannelInviteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_dm(configuration: &configuration::Configuration, create_private_channel_request: models::CreatePrivateChannelRequest) -> Result<ResponseContent<CreateDmSuccess>, Error<CreateDmError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_private_channel_request = create_private_channel_request;

    let uri_str = format!("{}/users/@me/channels", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_private_channel_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateDmSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDmError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_entitlement(configuration: &configuration::Configuration, application_id: &str, create_entitlement_request_data: models::CreateEntitlementRequestData) -> Result<ResponseContent<CreateEntitlementSuccess>, Error<CreateEntitlementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_create_entitlement_request_data = create_entitlement_request_data;

    let uri_str = format!("{}/applications/{application_id}/entitlements", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_entitlement_request_data);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateEntitlementSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateEntitlementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_application_command(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, application_command_create_request: models::ApplicationCommandCreateRequest) -> Result<ResponseContent<CreateGuildApplicationCommandSuccess>, Error<CreateGuildApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_body_application_command_create_request = application_command_create_request;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_command_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_channel(configuration: &configuration::Configuration, guild_id: &str, create_guild_channel_request: models::CreateGuildChannelRequest) -> Result<ResponseContent<CreateGuildChannelSuccess>, Error<CreateGuildChannelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_create_guild_channel_request = create_guild_channel_request;

    let uri_str = format!("{}/guilds/{guild_id}/channels", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_guild_channel_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildChannelSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildChannelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_emoji(configuration: &configuration::Configuration, guild_id: &str, create_guild_emoji_request: models::CreateGuildEmojiRequest) -> Result<ResponseContent<CreateGuildEmojiSuccess>, Error<CreateGuildEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_create_guild_emoji_request = create_guild_emoji_request;

    let uri_str = format!("{}/guilds/{guild_id}/emojis", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_guild_emoji_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_role(configuration: &configuration::Configuration, guild_id: &str, create_role_request: models::CreateRoleRequest) -> Result<ResponseContent<CreateGuildRoleSuccess>, Error<CreateGuildRoleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_create_role_request = create_role_request;

    let uri_str = format!("{}/guilds/{guild_id}/roles", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_role_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildRoleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_scheduled_event(configuration: &configuration::Configuration, guild_id: &str, create_guild_scheduled_event_request: models::CreateGuildScheduledEventRequest) -> Result<ResponseContent<CreateGuildScheduledEventSuccess>, Error<CreateGuildScheduledEventError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_create_guild_scheduled_event_request = create_guild_scheduled_event_request;

    let uri_str = format!("{}/guilds/{guild_id}/scheduled-events", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_guild_scheduled_event_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildScheduledEventSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildScheduledEventError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_soundboard_sound(configuration: &configuration::Configuration, guild_id: &str, soundboard_create_request: models::SoundboardCreateRequest) -> Result<ResponseContent<CreateGuildSoundboardSoundSuccess>, Error<CreateGuildSoundboardSoundError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_soundboard_create_request = soundboard_create_request;

    let uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_soundboard_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildSoundboardSoundSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildSoundboardSoundError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_sticker(configuration: &configuration::Configuration, guild_id: &str, name: &str, tags: &str, file: &str, description: Option<&str>) -> Result<ResponseContent<CreateGuildStickerSuccess>, Error<CreateGuildStickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_form_name = name;
    let p_form_tags = tags;
    let p_form_file = file;
    let p_form_description = description;

    let uri_str = format!("{}/guilds/{guild_id}/stickers", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    let mut multipart_form = reqwest::multipart::Form::new();
    multipart_form = multipart_form.text("name", p_form_name.to_string());
    multipart_form = multipart_form.text("tags", p_form_tags.to_string());
    if let Some(param_value) = p_form_description {
        multipart_form = multipart_form.text("description", param_value.to_string());
    }
    multipart_form = multipart_form.text("file", p_form_file.to_string());
    req_builder = req_builder.multipart(multipart_form);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildStickerSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildStickerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_guild_template(configuration: &configuration::Configuration, guild_id: &str, create_guild_template_request: models::CreateGuildTemplateRequest) -> Result<ResponseContent<CreateGuildTemplateSuccess>, Error<CreateGuildTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_create_guild_template_request = create_guild_template_request;

    let uri_str = format!("{}/guilds/{guild_id}/templates", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_guild_template_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateGuildTemplateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateGuildTemplateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_interaction_response(configuration: &configuration::Configuration, interaction_id: &str, interaction_token: &str, create_interaction_response_request: models::CreateInteractionResponseRequest, with_response: Option<bool>) -> Result<ResponseContent<CreateInteractionResponseSuccess>, Error<CreateInteractionResponseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_interaction_id = interaction_id;
    let p_path_interaction_token = interaction_token;
    let p_body_create_interaction_response_request = create_interaction_response_request;
    let p_query_with_response = with_response;

    let uri_str = format!("{}/interactions/{interaction_id}/{interaction_token}/callback", configuration.base_path, interaction_id=crate::apis::urlencode(p_path_interaction_id), interaction_token=crate::apis::urlencode(p_path_interaction_token));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_query_with_response {
        req_builder = req_builder.query(&[("with_response", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_interaction_response_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateInteractionResponseSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateInteractionResponseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_linked_lobby_guild_invite_for_self(configuration: &configuration::Configuration, lobby_id: &str) -> Result<ResponseContent<CreateLinkedLobbyGuildInviteForSelfSuccess>, Error<CreateLinkedLobbyGuildInviteForSelfError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;

    let uri_str = format!("{}/lobbies/{lobby_id}/members/@me/invites", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateLinkedLobbyGuildInviteForSelfSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateLinkedLobbyGuildInviteForSelfError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_linked_lobby_guild_invite_for_user(configuration: &configuration::Configuration, lobby_id: &str, user_id: &str) -> Result<ResponseContent<CreateLinkedLobbyGuildInviteForUserSuccess>, Error<CreateLinkedLobbyGuildInviteForUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/lobbies/{lobby_id}/members/{user_id}/invites", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateLinkedLobbyGuildInviteForUserSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateLinkedLobbyGuildInviteForUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_lobby(configuration: &configuration::Configuration, create_lobby_request: models::CreateLobbyRequest) -> Result<ResponseContent<CreateLobbySuccess>, Error<CreateLobbyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_lobby_request = create_lobby_request;

    let uri_str = format!("{}/lobbies", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_lobby_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateLobbySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateLobbyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_lobby_message(configuration: &configuration::Configuration, lobby_id: &str, sdk_message_request: models::SdkMessageRequest) -> Result<ResponseContent<CreateLobbyMessageSuccess>, Error<CreateLobbyMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_body_sdk_message_request = sdk_message_request;

    let uri_str = format!("{}/lobbies/{lobby_id}/messages", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_sdk_message_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateLobbyMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateLobbyMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_message(configuration: &configuration::Configuration, channel_id: &str, message_create_request: models::MessageCreateRequest) -> Result<ResponseContent<CreateMessageSuccess>, Error<CreateMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_message_create_request = message_create_request;

    let uri_str = format!("{}/channels/{channel_id}/messages", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_message_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_or_join_lobby(configuration: &configuration::Configuration, create_or_join_lobby_request: models::CreateOrJoinLobbyRequest) -> Result<ResponseContent<CreateOrJoinLobbySuccess>, Error<CreateOrJoinLobbyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_or_join_lobby_request = create_or_join_lobby_request;

    let uri_str = format!("{}/lobbies", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_or_join_lobby_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateOrJoinLobbySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateOrJoinLobbyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_pin(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<CreatePinSuccess>, Error<CreatePinError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/pins/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreatePinSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreatePinError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_stage_instance(configuration: &configuration::Configuration, create_stage_instance_request: models::CreateStageInstanceRequest) -> Result<ResponseContent<CreateStageInstanceSuccess>, Error<CreateStageInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_stage_instance_request = create_stage_instance_request;

    let uri_str = format!("{}/stage-instances", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_stage_instance_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateStageInstanceSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateStageInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_thread(configuration: &configuration::Configuration, channel_id: &str, create_thread_request: models::CreateThreadRequest) -> Result<ResponseContent<CreateThreadSuccess>, Error<CreateThreadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_create_thread_request = create_thread_request;

    let uri_str = format!("{}/channels/{channel_id}/threads", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_thread_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateThreadSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateThreadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_thread_from_message(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, create_text_thread_with_message_request: models::CreateTextThreadWithMessageRequest) -> Result<ResponseContent<CreateThreadFromMessageSuccess>, Error<CreateThreadFromMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_body_create_text_thread_with_message_request = create_text_thread_with_message_request;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/threads", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_text_thread_with_message_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateThreadFromMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateThreadFromMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_webhook(configuration: &configuration::Configuration, channel_id: &str, create_webhook_request: models::CreateWebhookRequest) -> Result<ResponseContent<CreateWebhookSuccess>, Error<CreateWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_create_webhook_request = create_webhook_request;

    let uri_str = format!("{}/channels/{channel_id}/webhooks", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_webhook_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CreateWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn crosspost_message(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<CrosspostMessageSuccess>, Error<CrosspostMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/crosspost", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<CrosspostMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<CrosspostMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_all_message_reactions(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<DeleteAllMessageReactionsSuccess>, Error<DeleteAllMessageReactionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteAllMessageReactionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteAllMessageReactionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_all_message_reactions_by_emoji(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, emoji_name: &str) -> Result<ResponseContent<DeleteAllMessageReactionsByEmojiSuccess>, Error<DeleteAllMessageReactionsByEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_path_emoji_name = emoji_name;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id), emoji_name=crate::apis::urlencode(p_path_emoji_name));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteAllMessageReactionsByEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteAllMessageReactionsByEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_application_command(configuration: &configuration::Configuration, application_id: &str, command_id: &str) -> Result<ResponseContent<DeleteApplicationCommandSuccess>, Error<DeleteApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_command_id = command_id;

    let uri_str = format!("{}/applications/{application_id}/commands/{command_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_application_emoji(configuration: &configuration::Configuration, application_id: &str, emoji_id: &str) -> Result<ResponseContent<DeleteApplicationEmojiSuccess>, Error<DeleteApplicationEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_emoji_id = emoji_id;

    let uri_str = format!("{}/applications/{application_id}/emojis/{emoji_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), emoji_id=crate::apis::urlencode(p_path_emoji_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_application_user_role_connection(configuration: &configuration::Configuration, application_id: &str) -> Result<ResponseContent<DeleteApplicationUserRoleConnectionSuccess>, Error<DeleteApplicationUserRoleConnectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;

    let uri_str = format!("{}/users/@me/applications/{application_id}/role-connection", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationUserRoleConnectionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteApplicationUserRoleConnectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_auto_moderation_rule(configuration: &configuration::Configuration, guild_id: &str, rule_id: &str) -> Result<ResponseContent<DeleteAutoModerationRuleSuccess>, Error<DeleteAutoModerationRuleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_rule_id = rule_id;

    let uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules/{rule_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), rule_id=crate::apis::urlencode(p_path_rule_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteAutoModerationRuleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteAutoModerationRuleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_channel(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<DeleteChannelSuccess>, Error<DeleteChannelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteChannelSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteChannelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_channel_permission_overwrite(configuration: &configuration::Configuration, channel_id: &str, overwrite_id: &str) -> Result<ResponseContent<DeleteChannelPermissionOverwriteSuccess>, Error<DeleteChannelPermissionOverwriteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_overwrite_id = overwrite_id;

    let uri_str = format!("{}/channels/{channel_id}/permissions/{overwrite_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), overwrite_id=crate::apis::urlencode(p_path_overwrite_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteChannelPermissionOverwriteSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteChannelPermissionOverwriteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_entitlement(configuration: &configuration::Configuration, application_id: &str, entitlement_id: &str) -> Result<ResponseContent<DeleteEntitlementSuccess>, Error<DeleteEntitlementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_entitlement_id = entitlement_id;

    let uri_str = format!("{}/applications/{application_id}/entitlements/{entitlement_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), entitlement_id=crate::apis::urlencode(p_path_entitlement_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteEntitlementSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteEntitlementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_group_dm_user(configuration: &configuration::Configuration, channel_id: &str, user_id: &str) -> Result<ResponseContent<DeleteGroupDmUserSuccess>, Error<DeleteGroupDmUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/channels/{channel_id}/recipients/{user_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGroupDmUserSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGroupDmUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_application_command(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, command_id: &str) -> Result<ResponseContent<DeleteGuildApplicationCommandSuccess>, Error<DeleteGuildApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_path_command_id = command_id;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_emoji(configuration: &configuration::Configuration, guild_id: &str, emoji_id: &str) -> Result<ResponseContent<DeleteGuildEmojiSuccess>, Error<DeleteGuildEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_emoji_id = emoji_id;

    let uri_str = format!("{}/guilds/{guild_id}/emojis/{emoji_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), emoji_id=crate::apis::urlencode(p_path_emoji_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_integration(configuration: &configuration::Configuration, guild_id: &str, integration_id: &str) -> Result<ResponseContent<DeleteGuildIntegrationSuccess>, Error<DeleteGuildIntegrationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_integration_id = integration_id;

    let uri_str = format!("{}/guilds/{guild_id}/integrations/{integration_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), integration_id=crate::apis::urlencode(p_path_integration_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildIntegrationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildIntegrationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_member(configuration: &configuration::Configuration, guild_id: &str, user_id: &str) -> Result<ResponseContent<DeleteGuildMemberSuccess>, Error<DeleteGuildMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_member_role(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, role_id: &str) -> Result<ResponseContent<DeleteGuildMemberRoleSuccess>, Error<DeleteGuildMemberRoleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_path_role_id = role_id;

    let uri_str = format!("{}/guilds/{guild_id}/members/{user_id}/roles/{role_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id), role_id=crate::apis::urlencode(p_path_role_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildMemberRoleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildMemberRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_role(configuration: &configuration::Configuration, guild_id: &str, role_id: &str) -> Result<ResponseContent<DeleteGuildRoleSuccess>, Error<DeleteGuildRoleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_role_id = role_id;

    let uri_str = format!("{}/guilds/{guild_id}/roles/{role_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), role_id=crate::apis::urlencode(p_path_role_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildRoleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_scheduled_event(configuration: &configuration::Configuration, guild_id: &str, guild_scheduled_event_id: &str) -> Result<ResponseContent<DeleteGuildScheduledEventSuccess>, Error<DeleteGuildScheduledEventError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_guild_scheduled_event_id = guild_scheduled_event_id;

    let uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), guild_scheduled_event_id=crate::apis::urlencode(p_path_guild_scheduled_event_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildScheduledEventSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildScheduledEventError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_soundboard_sound(configuration: &configuration::Configuration, guild_id: &str, sound_id: &str) -> Result<ResponseContent<DeleteGuildSoundboardSoundSuccess>, Error<DeleteGuildSoundboardSoundError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_sound_id = sound_id;

    let uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds/{sound_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), sound_id=crate::apis::urlencode(p_path_sound_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildSoundboardSoundSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildSoundboardSoundError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_sticker(configuration: &configuration::Configuration, guild_id: &str, sticker_id: &str) -> Result<ResponseContent<DeleteGuildStickerSuccess>, Error<DeleteGuildStickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_sticker_id = sticker_id;

    let uri_str = format!("{}/guilds/{guild_id}/stickers/{sticker_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), sticker_id=crate::apis::urlencode(p_path_sticker_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildStickerSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildStickerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_guild_template(configuration: &configuration::Configuration, guild_id: &str, code: &str) -> Result<ResponseContent<DeleteGuildTemplateSuccess>, Error<DeleteGuildTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_code = code;

    let uri_str = format!("{}/guilds/{guild_id}/templates/{code}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), code=crate::apis::urlencode(p_path_code));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildTemplateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGuildTemplateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_lobby_member(configuration: &configuration::Configuration, lobby_id: &str, user_id: &str) -> Result<ResponseContent<DeleteLobbyMemberSuccess>, Error<DeleteLobbyMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/lobbies/{lobby_id}/members/{user_id}", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteLobbyMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteLobbyMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_message(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<DeleteMessageSuccess>, Error<DeleteMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_my_message_reaction(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, emoji_name: &str) -> Result<ResponseContent<DeleteMyMessageReactionSuccess>, Error<DeleteMyMessageReactionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_path_emoji_name = emoji_name;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id), emoji_name=crate::apis::urlencode(p_path_emoji_name));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteMyMessageReactionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteMyMessageReactionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_original_webhook_message(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, thread_id: Option<&str>) -> Result<ResponseContent<DeleteOriginalWebhookMessageSuccess>, Error<DeleteOriginalWebhookMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_query_thread_id = thread_id;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/@original", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteOriginalWebhookMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteOriginalWebhookMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_pin(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<DeletePinSuccess>, Error<DeletePinError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/pins/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeletePinSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeletePinError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_stage_instance(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<DeleteStageInstanceSuccess>, Error<DeleteStageInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/stage-instances/{channel_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteStageInstanceSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteStageInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_thread_member(configuration: &configuration::Configuration, channel_id: &str, user_id: &str) -> Result<ResponseContent<DeleteThreadMemberSuccess>, Error<DeleteThreadMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/channels/{channel_id}/thread-members/{user_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteThreadMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteThreadMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_user_message_reaction(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, emoji_name: &str, user_id: &str) -> Result<ResponseContent<DeleteUserMessageReactionSuccess>, Error<DeleteUserMessageReactionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_path_emoji_name = emoji_name;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/{user_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id), emoji_name=crate::apis::urlencode(p_path_emoji_name), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteUserMessageReactionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteUserMessageReactionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_webhook(configuration: &configuration::Configuration, webhook_id: &str) -> Result<ResponseContent<DeleteWebhookSuccess>, Error<DeleteWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;

    let uri_str = format!("{}/webhooks/{webhook_id}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_webhook_by_token(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str) -> Result<ResponseContent<DeleteWebhookByTokenSuccess>, Error<DeleteWebhookByTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteWebhookByTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteWebhookByTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_webhook_message(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, message_id: &str, thread_id: Option<&str>) -> Result<ResponseContent<DeleteWebhookMessageSuccess>, Error<DeleteWebhookMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_path_message_id = message_id;
    let p_query_thread_id = thread_id;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeleteWebhookMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteWebhookMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn deprecated_create_pin(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<DeprecatedCreatePinSuccess>, Error<DeprecatedCreatePinError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/pins/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeprecatedCreatePinSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeprecatedCreatePinError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn deprecated_delete_pin(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<DeprecatedDeletePinSuccess>, Error<DeprecatedDeletePinError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/pins/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeprecatedDeletePinSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeprecatedDeletePinError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn deprecated_list_pins(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<DeprecatedListPinsSuccess>, Error<DeprecatedListPinsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}/pins", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<DeprecatedListPinsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<DeprecatedListPinsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edit_lobby(configuration: &configuration::Configuration, lobby_id: &str, create_lobby_request: models::CreateLobbyRequest) -> Result<ResponseContent<EditLobbySuccess>, Error<EditLobbyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_body_create_lobby_request = create_lobby_request;

    let uri_str = format!("{}/lobbies/{lobby_id}", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_create_lobby_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<EditLobbySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<EditLobbyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edit_lobby_channel_link(configuration: &configuration::Configuration, lobby_id: &str, edit_lobby_channel_link_request: models::EditLobbyChannelLinkRequest) -> Result<ResponseContent<EditLobbyChannelLinkSuccess>, Error<EditLobbyChannelLinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_body_edit_lobby_channel_link_request = edit_lobby_channel_link_request;

    let uri_str = format!("{}/lobbies/{lobby_id}/channel-linking", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_edit_lobby_channel_link_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<EditLobbyChannelLinkSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<EditLobbyChannelLinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn execute_github_compatible_webhook(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, github_webhook: models::GithubWebhook, wait: Option<bool>, thread_id: Option<&str>) -> Result<ResponseContent<ExecuteGithubCompatibleWebhookSuccess>, Error<ExecuteGithubCompatibleWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_body_github_webhook = github_webhook;
    let p_query_wait = wait;
    let p_query_thread_id = thread_id;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/github", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_query_wait {
        req_builder = req_builder.query(&[("wait", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_github_webhook);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ExecuteGithubCompatibleWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ExecuteGithubCompatibleWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn execute_slack_compatible_webhook(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, slack_webhook: models::SlackWebhook, wait: Option<bool>, thread_id: Option<&str>) -> Result<ResponseContent<ExecuteSlackCompatibleWebhookSuccess>, Error<ExecuteSlackCompatibleWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_body_slack_webhook = slack_webhook;
    let p_query_wait = wait;
    let p_query_thread_id = thread_id;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/slack", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_query_wait {
        req_builder = req_builder.query(&[("wait", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_slack_webhook);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ExecuteSlackCompatibleWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ExecuteSlackCompatibleWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn execute_webhook(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, execute_webhook_request: models::ExecuteWebhookRequest, wait: Option<bool>, thread_id: Option<&str>, with_components: Option<bool>) -> Result<ResponseContent<ExecuteWebhookSuccess>, Error<ExecuteWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_body_execute_webhook_request = execute_webhook_request;
    let p_query_wait = wait;
    let p_query_thread_id = thread_id;
    let p_query_with_components = with_components;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_query_wait {
        req_builder = req_builder.query(&[("wait", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_with_components {
        req_builder = req_builder.query(&[("with_components", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_execute_webhook_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ExecuteWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ExecuteWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn follow_channel(configuration: &configuration::Configuration, channel_id: &str, follow_channel_request: models::FollowChannelRequest) -> Result<ResponseContent<FollowChannelSuccess>, Error<FollowChannelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_follow_channel_request = follow_channel_request;

    let uri_str = format!("{}/channels/{channel_id}/followers", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_follow_channel_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<FollowChannelSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<FollowChannelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_active_guild_threads(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetActiveGuildThreadsSuccess>, Error<GetActiveGuildThreadsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/threads/active", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetActiveGuildThreadsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetActiveGuildThreadsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_answer_voters(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, answer_id: i32, after: Option<&str>, limit: Option<i32>) -> Result<ResponseContent<GetAnswerVotersSuccess>, Error<GetAnswerVotersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_path_answer_id = answer_id;
    let p_query_after = after;
    let p_query_limit = limit;

    let uri_str = format!("{}/channels/{channel_id}/polls/{message_id}/answers/{answer_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id), answer_id=p_path_answer_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetAnswerVotersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAnswerVotersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_application(configuration: &configuration::Configuration, application_id: &str) -> Result<ResponseContent<GetApplicationSuccess>, Error<GetApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;

    let uri_str = format!("{}/applications/{application_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetApplicationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_application_command(configuration: &configuration::Configuration, application_id: &str, command_id: &str) -> Result<ResponseContent<GetApplicationCommandSuccess>, Error<GetApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_command_id = command_id;

    let uri_str = format!("{}/applications/{application_id}/commands/{command_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_application_emoji(configuration: &configuration::Configuration, application_id: &str, emoji_id: &str) -> Result<ResponseContent<GetApplicationEmojiSuccess>, Error<GetApplicationEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_emoji_id = emoji_id;

    let uri_str = format!("{}/applications/{application_id}/emojis/{emoji_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), emoji_id=crate::apis::urlencode(p_path_emoji_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetApplicationEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_application_role_connections_metadata(configuration: &configuration::Configuration, application_id: &str) -> Result<ResponseContent<GetApplicationRoleConnectionsMetadataSuccess>, Error<GetApplicationRoleConnectionsMetadataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;

    let uri_str = format!("{}/applications/{application_id}/role-connections/metadata", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetApplicationRoleConnectionsMetadataSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationRoleConnectionsMetadataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_application_user_role_connection(configuration: &configuration::Configuration, application_id: &str) -> Result<ResponseContent<GetApplicationUserRoleConnectionSuccess>, Error<GetApplicationUserRoleConnectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;

    let uri_str = format!("{}/users/@me/applications/{application_id}/role-connection", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetApplicationUserRoleConnectionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationUserRoleConnectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_auto_moderation_rule(configuration: &configuration::Configuration, guild_id: &str, rule_id: &str) -> Result<ResponseContent<GetAutoModerationRuleSuccess>, Error<GetAutoModerationRuleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_rule_id = rule_id;

    let uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules/{rule_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), rule_id=crate::apis::urlencode(p_path_rule_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetAutoModerationRuleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAutoModerationRuleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_bot_gateway(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetBotGatewaySuccess>, Error<GetBotGatewayError>> {

    let uri_str = format!("{}/gateway/bot", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetBotGatewaySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBotGatewayError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_channel(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<GetChannelSuccess>, Error<GetChannelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetChannelSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetChannelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_entitlement(configuration: &configuration::Configuration, application_id: &str, entitlement_id: &str) -> Result<ResponseContent<GetEntitlementSuccess>, Error<GetEntitlementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_entitlement_id = entitlement_id;

    let uri_str = format!("{}/applications/{application_id}/entitlements/{entitlement_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), entitlement_id=crate::apis::urlencode(p_path_entitlement_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetEntitlementSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEntitlementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_entitlements(configuration: &configuration::Configuration, application_id: &str, user_id: Option<&str>, sku_ids: Option<&str>, guild_id: Option<&str>, before: Option<&str>, after: Option<&str>, limit: Option<i32>, exclude_ended: Option<bool>, exclude_deleted: Option<bool>, only_active: Option<bool>) -> Result<ResponseContent<GetEntitlementsSuccess>, Error<GetEntitlementsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_query_user_id = user_id;
    let p_query_sku_ids = sku_ids;
    let p_query_guild_id = guild_id;
    let p_query_before = before;
    let p_query_after = after;
    let p_query_limit = limit;
    let p_query_exclude_ended = exclude_ended;
    let p_query_exclude_deleted = exclude_deleted;
    let p_query_only_active = only_active;

    let uri_str = format!("{}/applications/{application_id}/entitlements", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_user_id {
        req_builder = req_builder.query(&[("user_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sku_ids {
        req_builder = req_builder.query(&[("sku_ids", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_guild_id {
        req_builder = req_builder.query(&[("guild_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_exclude_ended {
        req_builder = req_builder.query(&[("exclude_ended", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_exclude_deleted {
        req_builder = req_builder.query(&[("exclude_deleted", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_only_active {
        req_builder = req_builder.query(&[("only_active", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetEntitlementsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEntitlementsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_gateway(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetGatewaySuccess>, Error<GetGatewayError>> {

    let uri_str = format!("{}/gateway", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGatewaySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGatewayError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild(configuration: &configuration::Configuration, guild_id: &str, with_counts: Option<bool>) -> Result<ResponseContent<GetGuildSuccess>, Error<GetGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_with_counts = with_counts;

    let uri_str = format!("{}/guilds/{guild_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_counts {
        req_builder = req_builder.query(&[("with_counts", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_application_command(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, command_id: &str) -> Result<ResponseContent<GetGuildApplicationCommandSuccess>, Error<GetGuildApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_path_command_id = command_id;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_application_command_permissions(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, command_id: &str) -> Result<ResponseContent<GetGuildApplicationCommandPermissionsSuccess>, Error<GetGuildApplicationCommandPermissionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_path_command_id = command_id;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildApplicationCommandPermissionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildApplicationCommandPermissionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_ban(configuration: &configuration::Configuration, guild_id: &str, user_id: &str) -> Result<ResponseContent<GetGuildBanSuccess>, Error<GetGuildBanError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/guilds/{guild_id}/bans/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildBanSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildBanError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_emoji(configuration: &configuration::Configuration, guild_id: &str, emoji_id: &str) -> Result<ResponseContent<GetGuildEmojiSuccess>, Error<GetGuildEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_emoji_id = emoji_id;

    let uri_str = format!("{}/guilds/{guild_id}/emojis/{emoji_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), emoji_id=crate::apis::urlencode(p_path_emoji_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_member(configuration: &configuration::Configuration, guild_id: &str, user_id: &str) -> Result<ResponseContent<GetGuildMemberSuccess>, Error<GetGuildMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_new_member_welcome(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildNewMemberWelcomeSuccess>, Error<GetGuildNewMemberWelcomeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/new-member-welcome", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildNewMemberWelcomeSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildNewMemberWelcomeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_preview(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildPreviewSuccess>, Error<GetGuildPreviewError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/preview", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildPreviewSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildPreviewError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_role(configuration: &configuration::Configuration, guild_id: &str, role_id: &str) -> Result<ResponseContent<GetGuildRoleSuccess>, Error<GetGuildRoleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_role_id = role_id;

    let uri_str = format!("{}/guilds/{guild_id}/roles/{role_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), role_id=crate::apis::urlencode(p_path_role_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildRoleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_scheduled_event(configuration: &configuration::Configuration, guild_id: &str, guild_scheduled_event_id: &str, with_user_count: Option<bool>) -> Result<ResponseContent<GetGuildScheduledEventSuccess>, Error<GetGuildScheduledEventError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_guild_scheduled_event_id = guild_scheduled_event_id;
    let p_query_with_user_count = with_user_count;

    let uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), guild_scheduled_event_id=crate::apis::urlencode(p_path_guild_scheduled_event_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_user_count {
        req_builder = req_builder.query(&[("with_user_count", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildScheduledEventSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildScheduledEventError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_soundboard_sound(configuration: &configuration::Configuration, guild_id: &str, sound_id: &str) -> Result<ResponseContent<GetGuildSoundboardSoundSuccess>, Error<GetGuildSoundboardSoundError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_sound_id = sound_id;

    let uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds/{sound_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), sound_id=crate::apis::urlencode(p_path_sound_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildSoundboardSoundSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildSoundboardSoundError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_sticker(configuration: &configuration::Configuration, guild_id: &str, sticker_id: &str) -> Result<ResponseContent<GetGuildStickerSuccess>, Error<GetGuildStickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_sticker_id = sticker_id;

    let uri_str = format!("{}/guilds/{guild_id}/stickers/{sticker_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), sticker_id=crate::apis::urlencode(p_path_sticker_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildStickerSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildStickerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_template(configuration: &configuration::Configuration, code: &str) -> Result<ResponseContent<GetGuildTemplateSuccess>, Error<GetGuildTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_code = code;

    let uri_str = format!("{}/guilds/templates/{code}", configuration.base_path, code=crate::apis::urlencode(p_path_code));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildTemplateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildTemplateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_vanity_url(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildVanityUrlSuccess>, Error<GetGuildVanityUrlError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/vanity-url", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildVanityUrlSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildVanityUrlError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_webhooks(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildWebhooksSuccess>, Error<GetGuildWebhooksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/webhooks", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildWebhooksSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildWebhooksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_welcome_screen(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildWelcomeScreenSuccess>, Error<GetGuildWelcomeScreenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/welcome-screen", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildWelcomeScreenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildWelcomeScreenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_widget(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildWidgetSuccess>, Error<GetGuildWidgetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/widget.json", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildWidgetSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildWidgetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_widget_png(configuration: &configuration::Configuration, guild_id: &str, style: Option<&str>) -> Result<ResponseContent<GetGuildWidgetPngSuccess>, Error<GetGuildWidgetPngError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_style = style;

    let uri_str = format!("{}/guilds/{guild_id}/widget.png", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_style {
        req_builder = req_builder.query(&[("style", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildWidgetPngSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildWidgetPngError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guild_widget_settings(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildWidgetSettingsSuccess>, Error<GetGuildWidgetSettingsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/widget", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildWidgetSettingsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildWidgetSettingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_guilds_onboarding(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetGuildsOnboardingSuccess>, Error<GetGuildsOnboardingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/onboarding", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetGuildsOnboardingSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGuildsOnboardingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_lobby(configuration: &configuration::Configuration, lobby_id: &str) -> Result<ResponseContent<GetLobbySuccess>, Error<GetLobbyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;

    let uri_str = format!("{}/lobbies/{lobby_id}", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetLobbySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetLobbyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_lobby_messages(configuration: &configuration::Configuration, lobby_id: &str, limit: Option<i32>) -> Result<ResponseContent<GetLobbyMessagesSuccess>, Error<GetLobbyMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;
    let p_query_limit = limit;

    let uri_str = format!("{}/lobbies/{lobby_id}/messages", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetLobbyMessagesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetLobbyMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_message(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<GetMessageSuccess>, Error<GetMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_my_application(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetMyApplicationSuccess>, Error<GetMyApplicationError>> {

    let uri_str = format!("{}/applications/@me", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetMyApplicationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMyApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_my_guild_member(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetMyGuildMemberSuccess>, Error<GetMyGuildMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/users/@me/guilds/{guild_id}/member", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetMyGuildMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMyGuildMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_my_oauth2_application(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetMyOauth2ApplicationSuccess>, Error<GetMyOauth2ApplicationError>> {

    let uri_str = format!("{}/oauth2/applications/@me", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetMyOauth2ApplicationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMyOauth2ApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_my_oauth2_authorization(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetMyOauth2AuthorizationSuccess>, Error<GetMyOauth2AuthorizationError>> {

    let uri_str = format!("{}/oauth2/@me", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetMyOauth2AuthorizationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMyOauth2AuthorizationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_my_user(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetMyUserSuccess>, Error<GetMyUserError>> {

    let uri_str = format!("{}/users/@me", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetMyUserSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMyUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_openid_connect_userinfo(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetOpenidConnectUserinfoSuccess>, Error<GetOpenidConnectUserinfoError>> {

    let uri_str = format!("{}/oauth2/userinfo", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetOpenidConnectUserinfoSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetOpenidConnectUserinfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_original_webhook_message(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, thread_id: Option<&str>) -> Result<ResponseContent<GetOriginalWebhookMessageSuccess>, Error<GetOriginalWebhookMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_query_thread_id = thread_id;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/@original", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetOriginalWebhookMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetOriginalWebhookMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_public_keys(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetPublicKeysSuccess>, Error<GetPublicKeysError>> {

    let uri_str = format!("{}/oauth2/keys", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetPublicKeysSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPublicKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_self_voice_state(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<GetSelfVoiceStateSuccess>, Error<GetSelfVoiceStateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/voice-states/@me", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetSelfVoiceStateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSelfVoiceStateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_soundboard_default_sounds(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetSoundboardDefaultSoundsSuccess>, Error<GetSoundboardDefaultSoundsError>> {

    let uri_str = format!("{}/soundboard-default-sounds", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetSoundboardDefaultSoundsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSoundboardDefaultSoundsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_stage_instance(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<GetStageInstanceSuccess>, Error<GetStageInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/stage-instances/{channel_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetStageInstanceSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStageInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_sticker(configuration: &configuration::Configuration, sticker_id: &str) -> Result<ResponseContent<GetStickerSuccess>, Error<GetStickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;

    let uri_str = format!("{}/stickers/{sticker_id}", configuration.base_path, sticker_id=crate::apis::urlencode(p_path_sticker_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetStickerSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStickerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_sticker_pack(configuration: &configuration::Configuration, pack_id: &str) -> Result<ResponseContent<GetStickerPackSuccess>, Error<GetStickerPackError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_pack_id = pack_id;

    let uri_str = format!("{}/sticker-packs/{pack_id}", configuration.base_path, pack_id=crate::apis::urlencode(p_path_pack_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetStickerPackSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStickerPackError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_thread_member(configuration: &configuration::Configuration, channel_id: &str, user_id: &str, with_member: Option<bool>) -> Result<ResponseContent<GetThreadMemberSuccess>, Error<GetThreadMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_user_id = user_id;
    let p_query_with_member = with_member;

    let uri_str = format!("{}/channels/{channel_id}/thread-members/{user_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_member {
        req_builder = req_builder.query(&[("with_member", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetThreadMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetThreadMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_user(configuration: &configuration::Configuration, user_id: &str) -> Result<ResponseContent<GetUserSuccess>, Error<GetUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_user_id = user_id;

    let uri_str = format!("{}/users/{user_id}", configuration.base_path, user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetUserSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_voice_state(configuration: &configuration::Configuration, guild_id: &str, user_id: &str) -> Result<ResponseContent<GetVoiceStateSuccess>, Error<GetVoiceStateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;

    let uri_str = format!("{}/guilds/{guild_id}/voice-states/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetVoiceStateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetVoiceStateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_webhook(configuration: &configuration::Configuration, webhook_id: &str) -> Result<ResponseContent<GetWebhookSuccess>, Error<GetWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;

    let uri_str = format!("{}/webhooks/{webhook_id}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_webhook_by_token(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str) -> Result<ResponseContent<GetWebhookByTokenSuccess>, Error<GetWebhookByTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetWebhookByTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWebhookByTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_webhook_message(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, message_id: &str, thread_id: Option<&str>) -> Result<ResponseContent<GetWebhookMessageSuccess>, Error<GetWebhookMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_path_message_id = message_id;
    let p_query_thread_id = thread_id;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<GetWebhookMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWebhookMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn invite_resolve(configuration: &configuration::Configuration, code: &str, with_counts: Option<bool>, guild_scheduled_event_id: Option<&str>) -> Result<ResponseContent<InviteResolveSuccess>, Error<InviteResolveError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_code = code;
    let p_query_with_counts = with_counts;
    let p_query_guild_scheduled_event_id = guild_scheduled_event_id;

    let uri_str = format!("{}/invites/{code}", configuration.base_path, code=crate::apis::urlencode(p_path_code));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_counts {
        req_builder = req_builder.query(&[("with_counts", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_guild_scheduled_event_id {
        req_builder = req_builder.query(&[("guild_scheduled_event_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<InviteResolveSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<InviteResolveError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn invite_revoke(configuration: &configuration::Configuration, code: &str) -> Result<ResponseContent<InviteRevokeSuccess>, Error<InviteRevokeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_code = code;

    let uri_str = format!("{}/invites/{code}", configuration.base_path, code=crate::apis::urlencode(p_path_code));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<InviteRevokeSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<InviteRevokeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn join_thread(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<JoinThreadSuccess>, Error<JoinThreadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}/thread-members/@me", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<JoinThreadSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<JoinThreadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn leave_guild(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<LeaveGuildSuccess>, Error<LeaveGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/users/@me/guilds/{guild_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<LeaveGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<LeaveGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn leave_lobby(configuration: &configuration::Configuration, lobby_id: &str) -> Result<ResponseContent<LeaveLobbySuccess>, Error<LeaveLobbyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_lobby_id = lobby_id;

    let uri_str = format!("{}/lobbies/{lobby_id}/members/@me", configuration.base_path, lobby_id=crate::apis::urlencode(p_path_lobby_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<LeaveLobbySuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<LeaveLobbyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn leave_thread(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<LeaveThreadSuccess>, Error<LeaveThreadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}/thread-members/@me", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<LeaveThreadSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<LeaveThreadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_application_commands(configuration: &configuration::Configuration, application_id: &str, with_localizations: Option<bool>) -> Result<ResponseContent<ListApplicationCommandsSuccess>, Error<ListApplicationCommandsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_query_with_localizations = with_localizations;

    let uri_str = format!("{}/applications/{application_id}/commands", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_localizations {
        req_builder = req_builder.query(&[("with_localizations", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListApplicationCommandsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListApplicationCommandsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_application_emojis(configuration: &configuration::Configuration, application_id: &str) -> Result<ResponseContent<ListApplicationEmojisSuccess>, Error<ListApplicationEmojisError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;

    let uri_str = format!("{}/applications/{application_id}/emojis", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListApplicationEmojisSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListApplicationEmojisError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_auto_moderation_rules(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListAutoModerationRulesSuccess>, Error<ListAutoModerationRulesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListAutoModerationRulesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListAutoModerationRulesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_channel_invites(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<ListChannelInvitesSuccess>, Error<ListChannelInvitesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}/invites", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListChannelInvitesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListChannelInvitesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_channel_webhooks(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<ListChannelWebhooksSuccess>, Error<ListChannelWebhooksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}/webhooks", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListChannelWebhooksSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListChannelWebhooksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_application_command_permissions(configuration: &configuration::Configuration, application_id: &str, guild_id: &str) -> Result<ResponseContent<ListGuildApplicationCommandPermissionsSuccess>, Error<ListGuildApplicationCommandPermissionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/permissions", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildApplicationCommandPermissionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildApplicationCommandPermissionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_application_commands(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, with_localizations: Option<bool>) -> Result<ResponseContent<ListGuildApplicationCommandsSuccess>, Error<ListGuildApplicationCommandsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_query_with_localizations = with_localizations;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_localizations {
        req_builder = req_builder.query(&[("with_localizations", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildApplicationCommandsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildApplicationCommandsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_audit_log_entries(configuration: &configuration::Configuration, guild_id: &str, user_id: Option<&str>, target_id: Option<&str>, action_type: Option<i32>, before: Option<&str>, after: Option<&str>, limit: Option<i32>) -> Result<ResponseContent<ListGuildAuditLogEntriesSuccess>, Error<ListGuildAuditLogEntriesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_user_id = user_id;
    let p_query_target_id = target_id;
    let p_query_action_type = action_type;
    let p_query_before = before;
    let p_query_after = after;
    let p_query_limit = limit;

    let uri_str = format!("{}/guilds/{guild_id}/audit-logs", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_user_id {
        req_builder = req_builder.query(&[("user_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_target_id {
        req_builder = req_builder.query(&[("target_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_action_type {
        req_builder = req_builder.query(&[("action_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildAuditLogEntriesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildAuditLogEntriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_bans(configuration: &configuration::Configuration, guild_id: &str, limit: Option<i32>, before: Option<&str>, after: Option<&str>) -> Result<ResponseContent<ListGuildBansSuccess>, Error<ListGuildBansError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_limit = limit;
    let p_query_before = before;
    let p_query_after = after;

    let uri_str = format!("{}/guilds/{guild_id}/bans", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildBansSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildBansError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_channels(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildChannelsSuccess>, Error<ListGuildChannelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/channels", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildChannelsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildChannelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_emojis(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildEmojisSuccess>, Error<ListGuildEmojisError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/emojis", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildEmojisSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildEmojisError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_integrations(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildIntegrationsSuccess>, Error<ListGuildIntegrationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/integrations", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildIntegrationsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildIntegrationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_invites(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildInvitesSuccess>, Error<ListGuildInvitesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/invites", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildInvitesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildInvitesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_members(configuration: &configuration::Configuration, guild_id: &str, limit: Option<i32>, after: Option<i32>) -> Result<ResponseContent<ListGuildMembersSuccess>, Error<ListGuildMembersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_limit = limit;
    let p_query_after = after;

    let uri_str = format!("{}/guilds/{guild_id}/members", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildMembersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildMembersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_roles(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildRolesSuccess>, Error<ListGuildRolesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/roles", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildRolesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildRolesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_scheduled_event_users(configuration: &configuration::Configuration, guild_id: &str, guild_scheduled_event_id: &str, with_member: Option<bool>, limit: Option<i32>, before: Option<&str>, after: Option<&str>) -> Result<ResponseContent<ListGuildScheduledEventUsersSuccess>, Error<ListGuildScheduledEventUsersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_guild_scheduled_event_id = guild_scheduled_event_id;
    let p_query_with_member = with_member;
    let p_query_limit = limit;
    let p_query_before = before;
    let p_query_after = after;

    let uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), guild_scheduled_event_id=crate::apis::urlencode(p_path_guild_scheduled_event_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_member {
        req_builder = req_builder.query(&[("with_member", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildScheduledEventUsersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildScheduledEventUsersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_scheduled_events(configuration: &configuration::Configuration, guild_id: &str, with_user_count: Option<bool>) -> Result<ResponseContent<ListGuildScheduledEventsSuccess>, Error<ListGuildScheduledEventsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_with_user_count = with_user_count;

    let uri_str = format!("{}/guilds/{guild_id}/scheduled-events", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_user_count {
        req_builder = req_builder.query(&[("with_user_count", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildScheduledEventsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildScheduledEventsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_soundboard_sounds(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildSoundboardSoundsSuccess>, Error<ListGuildSoundboardSoundsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildSoundboardSoundsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildSoundboardSoundsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_stickers(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildStickersSuccess>, Error<ListGuildStickersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/stickers", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildStickersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildStickersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_templates(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildTemplatesSuccess>, Error<ListGuildTemplatesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/templates", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildTemplatesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildTemplatesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_guild_voice_regions(configuration: &configuration::Configuration, guild_id: &str) -> Result<ResponseContent<ListGuildVoiceRegionsSuccess>, Error<ListGuildVoiceRegionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;

    let uri_str = format!("{}/guilds/{guild_id}/regions", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListGuildVoiceRegionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListGuildVoiceRegionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_message_reactions_by_emoji(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, emoji_name: &str, after: Option<&str>, limit: Option<i32>, r#type: Option<i32>) -> Result<ResponseContent<ListMessageReactionsByEmojiSuccess>, Error<ListMessageReactionsByEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_path_emoji_name = emoji_name;
    let p_query_after = after;
    let p_query_limit = limit;
    let p_query_type = r#type;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id), emoji_name=crate::apis::urlencode(p_path_emoji_name));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListMessageReactionsByEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListMessageReactionsByEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_messages(configuration: &configuration::Configuration, channel_id: &str, around: Option<&str>, before: Option<&str>, after: Option<&str>, limit: Option<i32>) -> Result<ResponseContent<ListMessagesSuccess>, Error<ListMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_around = around;
    let p_query_before = before;
    let p_query_after = after;
    let p_query_limit = limit;

    let uri_str = format!("{}/channels/{channel_id}/messages", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_around {
        req_builder = req_builder.query(&[("around", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListMessagesSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_my_connections(configuration: &configuration::Configuration, ) -> Result<ResponseContent<ListMyConnectionsSuccess>, Error<ListMyConnectionsError>> {

    let uri_str = format!("{}/users/@me/connections", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListMyConnectionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListMyConnectionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_my_guilds(configuration: &configuration::Configuration, before: Option<&str>, after: Option<&str>, limit: Option<i32>, with_counts: Option<bool>) -> Result<ResponseContent<ListMyGuildsSuccess>, Error<ListMyGuildsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_before = before;
    let p_query_after = after;
    let p_query_limit = limit;
    let p_query_with_counts = with_counts;

    let uri_str = format!("{}/users/@me/guilds", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_with_counts {
        req_builder = req_builder.query(&[("with_counts", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListMyGuildsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListMyGuildsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_my_private_archived_threads(configuration: &configuration::Configuration, channel_id: &str, before: Option<&str>, limit: Option<i32>) -> Result<ResponseContent<ListMyPrivateArchivedThreadsSuccess>, Error<ListMyPrivateArchivedThreadsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_before = before;
    let p_query_limit = limit;

    let uri_str = format!("{}/channels/{channel_id}/users/@me/threads/archived/private", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListMyPrivateArchivedThreadsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListMyPrivateArchivedThreadsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_pins(configuration: &configuration::Configuration, channel_id: &str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPinsSuccess>, Error<ListPinsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_before = before;
    let p_query_limit = limit;

    let uri_str = format!("{}/channels/{channel_id}/messages/pins", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListPinsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListPinsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_private_archived_threads(configuration: &configuration::Configuration, channel_id: &str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPrivateArchivedThreadsSuccess>, Error<ListPrivateArchivedThreadsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_before = before;
    let p_query_limit = limit;

    let uri_str = format!("{}/channels/{channel_id}/threads/archived/private", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListPrivateArchivedThreadsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListPrivateArchivedThreadsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_public_archived_threads(configuration: &configuration::Configuration, channel_id: &str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPublicArchivedThreadsSuccess>, Error<ListPublicArchivedThreadsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_before = before;
    let p_query_limit = limit;

    let uri_str = format!("{}/channels/{channel_id}/threads/archived/public", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListPublicArchivedThreadsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListPublicArchivedThreadsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_sticker_packs(configuration: &configuration::Configuration, ) -> Result<ResponseContent<ListStickerPacksSuccess>, Error<ListStickerPacksError>> {

    let uri_str = format!("{}/sticker-packs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListStickerPacksSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListStickerPacksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_thread_members(configuration: &configuration::Configuration, channel_id: &str, with_member: Option<bool>, limit: Option<i32>, after: Option<&str>) -> Result<ResponseContent<ListThreadMembersSuccess>, Error<ListThreadMembersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_with_member = with_member;
    let p_query_limit = limit;
    let p_query_after = after;

    let uri_str = format!("{}/channels/{channel_id}/thread-members", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_with_member {
        req_builder = req_builder.query(&[("with_member", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListThreadMembersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListThreadMembersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_voice_regions(configuration: &configuration::Configuration, ) -> Result<ResponseContent<ListVoiceRegionsSuccess>, Error<ListVoiceRegionsError>> {

    let uri_str = format!("{}/voice/regions", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ListVoiceRegionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ListVoiceRegionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn partner_sdk_token(configuration: &configuration::Configuration, partner_sdk_unmerge_provisional_account_request: models::PartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<PartnerSdkTokenSuccess>, Error<PartnerSdkTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_partner_sdk_unmerge_provisional_account_request = partner_sdk_unmerge_provisional_account_request;

    let uri_str = format!("{}/partner-sdk/token", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_partner_sdk_unmerge_provisional_account_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<PartnerSdkTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<PartnerSdkTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn partner_sdk_unmerge_provisional_account(configuration: &configuration::Configuration, partner_sdk_unmerge_provisional_account_request: models::PartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<PartnerSdkUnmergeProvisionalAccountSuccess>, Error<PartnerSdkUnmergeProvisionalAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_partner_sdk_unmerge_provisional_account_request = partner_sdk_unmerge_provisional_account_request;

    let uri_str = format!("{}/partner-sdk/provisional-accounts/unmerge", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_partner_sdk_unmerge_provisional_account_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<PartnerSdkUnmergeProvisionalAccountSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<PartnerSdkUnmergeProvisionalAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn poll_expire(configuration: &configuration::Configuration, channel_id: &str, message_id: &str) -> Result<ResponseContent<PollExpireSuccess>, Error<PollExpireError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;

    let uri_str = format!("{}/channels/{channel_id}/polls/{message_id}/expire", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<PollExpireSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<PollExpireError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn preview_prune_guild(configuration: &configuration::Configuration, guild_id: &str, days: Option<i32>, include_roles: Option<&str>) -> Result<ResponseContent<PreviewPruneGuildSuccess>, Error<PreviewPruneGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_query_days = days;
    let p_query_include_roles = include_roles;

    let uri_str = format!("{}/guilds/{guild_id}/prune", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_days {
        req_builder = req_builder.query(&[("days", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_include_roles {
        req_builder = req_builder.query(&[("include_roles", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<PreviewPruneGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<PreviewPruneGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn prune_guild(configuration: &configuration::Configuration, guild_id: &str, prune_guild_request: models::PruneGuildRequest) -> Result<ResponseContent<PruneGuildSuccess>, Error<PruneGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_prune_guild_request = prune_guild_request;

    let uri_str = format!("{}/guilds/{guild_id}/prune", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_prune_guild_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<PruneGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<PruneGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn put_guilds_onboarding(configuration: &configuration::Configuration, guild_id: &str, update_guild_onboarding_request: models::UpdateGuildOnboardingRequest) -> Result<ResponseContent<PutGuildsOnboardingSuccess>, Error<PutGuildsOnboardingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_update_guild_onboarding_request = update_guild_onboarding_request;

    let uri_str = format!("{}/guilds/{guild_id}/onboarding", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_onboarding_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<PutGuildsOnboardingSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<PutGuildsOnboardingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn search_guild_members(configuration: &configuration::Configuration, query: &str, guild_id: &str, limit: Option<i32>) -> Result<ResponseContent<SearchGuildMembersSuccess>, Error<SearchGuildMembersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_query = query;
    let p_path_guild_id = guild_id;
    let p_query_limit = limit;

    let uri_str = format!("{}/guilds/{guild_id}/members/search", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("query", &p_query_query.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<SearchGuildMembersSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchGuildMembersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn send_soundboard_sound(configuration: &configuration::Configuration, channel_id: &str, soundboard_sound_send_request: models::SoundboardSoundSendRequest) -> Result<ResponseContent<SendSoundboardSoundSuccess>, Error<SendSoundboardSoundError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_soundboard_sound_send_request = soundboard_sound_send_request;

    let uri_str = format!("{}/channels/{channel_id}/send-soundboard-sound", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_soundboard_sound_send_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<SendSoundboardSoundSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<SendSoundboardSoundError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn set_channel_permission_overwrite(configuration: &configuration::Configuration, channel_id: &str, overwrite_id: &str, set_channel_permission_overwrite_request: models::SetChannelPermissionOverwriteRequest) -> Result<ResponseContent<SetChannelPermissionOverwriteSuccess>, Error<SetChannelPermissionOverwriteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_overwrite_id = overwrite_id;
    let p_body_set_channel_permission_overwrite_request = set_channel_permission_overwrite_request;

    let uri_str = format!("{}/channels/{channel_id}/permissions/{overwrite_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), overwrite_id=crate::apis::urlencode(p_path_overwrite_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_set_channel_permission_overwrite_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<SetChannelPermissionOverwriteSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<SetChannelPermissionOverwriteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn set_guild_application_command_permissions(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, command_id: &str, set_guild_application_command_permissions_request: models::SetGuildApplicationCommandPermissionsRequest) -> Result<ResponseContent<SetGuildApplicationCommandPermissionsSuccess>, Error<SetGuildApplicationCommandPermissionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_path_command_id = command_id;
    let p_body_set_guild_application_command_permissions_request = set_guild_application_command_permissions_request;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_set_guild_application_command_permissions_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<SetGuildApplicationCommandPermissionsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<SetGuildApplicationCommandPermissionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn sync_guild_template(configuration: &configuration::Configuration, guild_id: &str, code: &str) -> Result<ResponseContent<SyncGuildTemplateSuccess>, Error<SyncGuildTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_code = code;

    let uri_str = format!("{}/guilds/{guild_id}/templates/{code}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), code=crate::apis::urlencode(p_path_code));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<SyncGuildTemplateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<SyncGuildTemplateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn thread_search(configuration: &configuration::Configuration, channel_id: &str, name: Option<&str>, slop: Option<i32>, min_id: Option<&str>, max_id: Option<&str>, tag: Option<&str>, tag_setting: Option<&str>, archived: Option<bool>, sort_by: Option<&str>, sort_order: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<ResponseContent<ThreadSearchSuccess>, Error<ThreadSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_query_name = name;
    let p_query_slop = slop;
    let p_query_min_id = min_id;
    let p_query_max_id = max_id;
    let p_query_tag = tag;
    let p_query_tag_setting = tag_setting;
    let p_query_archived = archived;
    let p_query_sort_by = sort_by;
    let p_query_sort_order = sort_order;
    let p_query_limit = limit;
    let p_query_offset = offset;

    let uri_str = format!("{}/channels/{channel_id}/threads/search", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_slop {
        req_builder = req_builder.query(&[("slop", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_min_id {
        req_builder = req_builder.query(&[("min_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_max_id {
        req_builder = req_builder.query(&[("max_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tag {
        req_builder = req_builder.query(&[("tag", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_tag_setting {
        req_builder = req_builder.query(&[("tag_setting", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_archived {
        req_builder = req_builder.query(&[("archived", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_by {
        req_builder = req_builder.query(&[("sort_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_order {
        req_builder = req_builder.query(&[("sort_order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<ThreadSearchSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<ThreadSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn trigger_typing_indicator(configuration: &configuration::Configuration, channel_id: &str) -> Result<ResponseContent<TriggerTypingIndicatorSuccess>, Error<TriggerTypingIndicatorError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;

    let uri_str = format!("{}/channels/{channel_id}/typing", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<TriggerTypingIndicatorSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<TriggerTypingIndicatorError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn unban_user_from_guild(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, body: serde_json::Value) -> Result<ResponseContent<UnbanUserFromGuildSuccess>, Error<UnbanUserFromGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_body_body = body;

    let uri_str = format!("{}/guilds/{guild_id}/bans/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UnbanUserFromGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UnbanUserFromGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_application(configuration: &configuration::Configuration, application_id: &str, application_form_partial: models::ApplicationFormPartial) -> Result<ResponseContent<UpdateApplicationSuccess>, Error<UpdateApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_application_form_partial = application_form_partial;

    let uri_str = format!("{}/applications/{application_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_form_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_application_command(configuration: &configuration::Configuration, application_id: &str, command_id: &str, application_command_patch_request_partial: models::ApplicationCommandPatchRequestPartial) -> Result<ResponseContent<UpdateApplicationCommandSuccess>, Error<UpdateApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_command_id = command_id;
    let p_body_application_command_patch_request_partial = application_command_patch_request_partial;

    let uri_str = format!("{}/applications/{application_id}/commands/{command_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_command_patch_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_application_emoji(configuration: &configuration::Configuration, application_id: &str, emoji_id: &str, update_application_emoji_request: models::UpdateApplicationEmojiRequest) -> Result<ResponseContent<UpdateApplicationEmojiSuccess>, Error<UpdateApplicationEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_emoji_id = emoji_id;
    let p_body_update_application_emoji_request = update_application_emoji_request;

    let uri_str = format!("{}/applications/{application_id}/emojis/{emoji_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), emoji_id=crate::apis::urlencode(p_path_emoji_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_application_emoji_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_application_role_connections_metadata(configuration: &configuration::Configuration, application_id: &str, application_role_connections_metadata_item_request: Option<Vec<models::ApplicationRoleConnectionsMetadataItemRequest>>) -> Result<ResponseContent<UpdateApplicationRoleConnectionsMetadataSuccess>, Error<UpdateApplicationRoleConnectionsMetadataError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_application_role_connections_metadata_item_request = application_role_connections_metadata_item_request;

    let uri_str = format!("{}/applications/{application_id}/role-connections/metadata", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_role_connections_metadata_item_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationRoleConnectionsMetadataSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationRoleConnectionsMetadataError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_application_user_role_connection(configuration: &configuration::Configuration, application_id: &str, update_application_user_role_connection_request: models::UpdateApplicationUserRoleConnectionRequest) -> Result<ResponseContent<UpdateApplicationUserRoleConnectionSuccess>, Error<UpdateApplicationUserRoleConnectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_body_update_application_user_role_connection_request = update_application_user_role_connection_request;

    let uri_str = format!("{}/users/@me/applications/{application_id}/role-connection", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_application_user_role_connection_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationUserRoleConnectionSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationUserRoleConnectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_auto_moderation_rule(configuration: &configuration::Configuration, guild_id: &str, rule_id: &str, update_auto_moderation_rule_request: models::UpdateAutoModerationRuleRequest) -> Result<ResponseContent<UpdateAutoModerationRuleSuccess>, Error<UpdateAutoModerationRuleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_rule_id = rule_id;
    let p_body_update_auto_moderation_rule_request = update_auto_moderation_rule_request;

    let uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules/{rule_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), rule_id=crate::apis::urlencode(p_path_rule_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_auto_moderation_rule_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateAutoModerationRuleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateAutoModerationRuleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_channel(configuration: &configuration::Configuration, channel_id: &str, update_channel_request: models::UpdateChannelRequest) -> Result<ResponseContent<UpdateChannelSuccess>, Error<UpdateChannelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_update_channel_request = update_channel_request;

    let uri_str = format!("{}/channels/{channel_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_channel_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateChannelSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateChannelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild(configuration: &configuration::Configuration, guild_id: &str, guild_patch_request_partial: models::GuildPatchRequestPartial) -> Result<ResponseContent<UpdateGuildSuccess>, Error<UpdateGuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_guild_patch_request_partial = guild_patch_request_partial;

    let uri_str = format!("{}/guilds/{guild_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_guild_patch_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_application_command(configuration: &configuration::Configuration, application_id: &str, guild_id: &str, command_id: &str, application_command_patch_request_partial: models::ApplicationCommandPatchRequestPartial) -> Result<ResponseContent<UpdateGuildApplicationCommandSuccess>, Error<UpdateGuildApplicationCommandError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_path_guild_id = guild_id;
    let p_path_command_id = command_id;
    let p_body_application_command_patch_request_partial = application_command_patch_request_partial;

    let uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id), guild_id=crate::apis::urlencode(p_path_guild_id), command_id=crate::apis::urlencode(p_path_command_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_command_patch_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildApplicationCommandSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildApplicationCommandError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_emoji(configuration: &configuration::Configuration, guild_id: &str, emoji_id: &str, update_guild_emoji_request: models::UpdateGuildEmojiRequest) -> Result<ResponseContent<UpdateGuildEmojiSuccess>, Error<UpdateGuildEmojiError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_emoji_id = emoji_id;
    let p_body_update_guild_emoji_request = update_guild_emoji_request;

    let uri_str = format!("{}/guilds/{guild_id}/emojis/{emoji_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), emoji_id=crate::apis::urlencode(p_path_emoji_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_emoji_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildEmojiSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildEmojiError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_member(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, update_guild_member_request: models::UpdateGuildMemberRequest) -> Result<ResponseContent<UpdateGuildMemberSuccess>, Error<UpdateGuildMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_body_update_guild_member_request = update_guild_member_request;

    let uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_member_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_role(configuration: &configuration::Configuration, guild_id: &str, role_id: &str, update_role_request_partial: models::UpdateRoleRequestPartial) -> Result<ResponseContent<UpdateGuildRoleSuccess>, Error<UpdateGuildRoleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_role_id = role_id;
    let p_body_update_role_request_partial = update_role_request_partial;

    let uri_str = format!("{}/guilds/{guild_id}/roles/{role_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), role_id=crate::apis::urlencode(p_path_role_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_role_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildRoleSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildRoleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_scheduled_event(configuration: &configuration::Configuration, guild_id: &str, guild_scheduled_event_id: &str, update_guild_scheduled_event_request: models::UpdateGuildScheduledEventRequest) -> Result<ResponseContent<UpdateGuildScheduledEventSuccess>, Error<UpdateGuildScheduledEventError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_guild_scheduled_event_id = guild_scheduled_event_id;
    let p_body_update_guild_scheduled_event_request = update_guild_scheduled_event_request;

    let uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), guild_scheduled_event_id=crate::apis::urlencode(p_path_guild_scheduled_event_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_scheduled_event_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildScheduledEventSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildScheduledEventError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_soundboard_sound(configuration: &configuration::Configuration, guild_id: &str, sound_id: &str, soundboard_patch_request_partial: models::SoundboardPatchRequestPartial) -> Result<ResponseContent<UpdateGuildSoundboardSoundSuccess>, Error<UpdateGuildSoundboardSoundError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_sound_id = sound_id;
    let p_body_soundboard_patch_request_partial = soundboard_patch_request_partial;

    let uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds/{sound_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), sound_id=crate::apis::urlencode(p_path_sound_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_soundboard_patch_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildSoundboardSoundSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildSoundboardSoundError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_sticker(configuration: &configuration::Configuration, guild_id: &str, sticker_id: &str, update_guild_sticker_request: models::UpdateGuildStickerRequest) -> Result<ResponseContent<UpdateGuildStickerSuccess>, Error<UpdateGuildStickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_sticker_id = sticker_id;
    let p_body_update_guild_sticker_request = update_guild_sticker_request;

    let uri_str = format!("{}/guilds/{guild_id}/stickers/{sticker_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), sticker_id=crate::apis::urlencode(p_path_sticker_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_sticker_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildStickerSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildStickerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_template(configuration: &configuration::Configuration, guild_id: &str, code: &str, update_guild_template_request: models::UpdateGuildTemplateRequest) -> Result<ResponseContent<UpdateGuildTemplateSuccess>, Error<UpdateGuildTemplateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_code = code;
    let p_body_update_guild_template_request = update_guild_template_request;

    let uri_str = format!("{}/guilds/{guild_id}/templates/{code}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), code=crate::apis::urlencode(p_path_code));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_template_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildTemplateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildTemplateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_welcome_screen(configuration: &configuration::Configuration, guild_id: &str, welcome_screen_patch_request_partial: models::WelcomeScreenPatchRequestPartial) -> Result<ResponseContent<UpdateGuildWelcomeScreenSuccess>, Error<UpdateGuildWelcomeScreenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_welcome_screen_patch_request_partial = welcome_screen_patch_request_partial;

    let uri_str = format!("{}/guilds/{guild_id}/welcome-screen", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_welcome_screen_patch_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildWelcomeScreenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildWelcomeScreenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_guild_widget_settings(configuration: &configuration::Configuration, guild_id: &str, update_guild_widget_settings_request: models::UpdateGuildWidgetSettingsRequest) -> Result<ResponseContent<UpdateGuildWidgetSettingsSuccess>, Error<UpdateGuildWidgetSettingsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_update_guild_widget_settings_request = update_guild_widget_settings_request;

    let uri_str = format!("{}/guilds/{guild_id}/widget", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_guild_widget_settings_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildWidgetSettingsSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGuildWidgetSettingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_message(configuration: &configuration::Configuration, channel_id: &str, message_id: &str, message_edit_request_partial: models::MessageEditRequestPartial) -> Result<ResponseContent<UpdateMessageSuccess>, Error<UpdateMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_path_message_id = message_id;
    let p_body_message_edit_request_partial = message_edit_request_partial;

    let uri_str = format!("{}/channels/{channel_id}/messages/{message_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_message_edit_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_my_application(configuration: &configuration::Configuration, application_form_partial: models::ApplicationFormPartial) -> Result<ResponseContent<UpdateMyApplicationSuccess>, Error<UpdateMyApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_application_form_partial = application_form_partial;

    let uri_str = format!("{}/applications/@me", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_application_form_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateMyApplicationSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateMyApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_my_guild_member(configuration: &configuration::Configuration, guild_id: &str, update_my_guild_member_request: models::UpdateMyGuildMemberRequest) -> Result<ResponseContent<UpdateMyGuildMemberSuccess>, Error<UpdateMyGuildMemberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_update_my_guild_member_request = update_my_guild_member_request;

    let uri_str = format!("{}/guilds/{guild_id}/members/@me", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_my_guild_member_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateMyGuildMemberSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateMyGuildMemberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_my_user(configuration: &configuration::Configuration, bot_account_patch_request: models::BotAccountPatchRequest) -> Result<ResponseContent<UpdateMyUserSuccess>, Error<UpdateMyUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_bot_account_patch_request = bot_account_patch_request;

    let uri_str = format!("{}/users/@me", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_bot_account_patch_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateMyUserSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateMyUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_original_webhook_message(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, incoming_webhook_update_request_partial: models::IncomingWebhookUpdateRequestPartial, thread_id: Option<&str>, with_components: Option<bool>) -> Result<ResponseContent<UpdateOriginalWebhookMessageSuccess>, Error<UpdateOriginalWebhookMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_body_incoming_webhook_update_request_partial = incoming_webhook_update_request_partial;
    let p_query_thread_id = thread_id;
    let p_query_with_components = with_components;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/@original", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_with_components {
        req_builder = req_builder.query(&[("with_components", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_incoming_webhook_update_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateOriginalWebhookMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateOriginalWebhookMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_self_voice_state(configuration: &configuration::Configuration, guild_id: &str, update_self_voice_state_request_partial: models::UpdateSelfVoiceStateRequestPartial) -> Result<ResponseContent<UpdateSelfVoiceStateSuccess>, Error<UpdateSelfVoiceStateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_body_update_self_voice_state_request_partial = update_self_voice_state_request_partial;

    let uri_str = format!("{}/guilds/{guild_id}/voice-states/@me", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_self_voice_state_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateSelfVoiceStateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateSelfVoiceStateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_stage_instance(configuration: &configuration::Configuration, channel_id: &str, update_stage_instance_request: models::UpdateStageInstanceRequest) -> Result<ResponseContent<UpdateStageInstanceSuccess>, Error<UpdateStageInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_channel_id = channel_id;
    let p_body_update_stage_instance_request = update_stage_instance_request;

    let uri_str = format!("{}/stage-instances/{channel_id}", configuration.base_path, channel_id=crate::apis::urlencode(p_path_channel_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_stage_instance_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateStageInstanceSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateStageInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_voice_state(configuration: &configuration::Configuration, guild_id: &str, user_id: &str, update_voice_state_request_partial: models::UpdateVoiceStateRequestPartial) -> Result<ResponseContent<UpdateVoiceStateSuccess>, Error<UpdateVoiceStateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_guild_id = guild_id;
    let p_path_user_id = user_id;
    let p_body_update_voice_state_request_partial = update_voice_state_request_partial;

    let uri_str = format!("{}/guilds/{guild_id}/voice-states/{user_id}", configuration.base_path, guild_id=crate::apis::urlencode(p_path_guild_id), user_id=crate::apis::urlencode(p_path_user_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_voice_state_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateVoiceStateSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateVoiceStateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_webhook(configuration: &configuration::Configuration, webhook_id: &str, update_webhook_request: models::UpdateWebhookRequest) -> Result<ResponseContent<UpdateWebhookSuccess>, Error<UpdateWebhookError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_body_update_webhook_request = update_webhook_request;

    let uri_str = format!("{}/webhooks/{webhook_id}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_webhook_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateWebhookSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateWebhookError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_webhook_by_token(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, update_webhook_by_token_request: models::UpdateWebhookByTokenRequest) -> Result<ResponseContent<UpdateWebhookByTokenSuccess>, Error<UpdateWebhookByTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_body_update_webhook_by_token_request = update_webhook_by_token_request;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_update_webhook_by_token_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateWebhookByTokenSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateWebhookByTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_webhook_message(configuration: &configuration::Configuration, webhook_id: &str, webhook_token: &str, message_id: &str, incoming_webhook_update_request_partial: models::IncomingWebhookUpdateRequestPartial, thread_id: Option<&str>, with_components: Option<bool>) -> Result<ResponseContent<UpdateWebhookMessageSuccess>, Error<UpdateWebhookMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_webhook_id = webhook_id;
    let p_path_webhook_token = webhook_token;
    let p_path_message_id = message_id;
    let p_body_incoming_webhook_update_request_partial = incoming_webhook_update_request_partial;
    let p_query_thread_id = thread_id;
    let p_query_with_components = with_components;

    let uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}", configuration.base_path, webhook_id=crate::apis::urlencode(p_path_webhook_id), webhook_token=crate::apis::urlencode(p_path_webhook_token), message_id=crate::apis::urlencode(p_path_message_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref param_value) = p_query_thread_id {
        req_builder = req_builder.query(&[("thread_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_with_components {
        req_builder = req_builder.query(&[("with_components", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_body_incoming_webhook_update_request_partial);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UpdateWebhookMessageSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateWebhookMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn upload_application_attachment(configuration: &configuration::Configuration, application_id: &str, file: &str) -> Result<ResponseContent<UploadApplicationAttachmentSuccess>, Error<UploadApplicationAttachmentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_application_id = application_id;
    let p_form_file = file;

    let uri_str = format!("{}/applications/{application_id}/attachment", configuration.base_path, application_id=crate::apis::urlencode(p_path_application_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    let mut multipart_form = reqwest::multipart::Form::new();
    multipart_form = multipart_form.text("file", p_form_file.to_string());
    req_builder = req_builder.multipart(multipart_form);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        let entity: Option<UploadApplicationAttachmentSuccess> = serde_json::from_str(&content).ok();
        Ok(ResponseContent { status, content, entity })
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadApplicationAttachmentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

