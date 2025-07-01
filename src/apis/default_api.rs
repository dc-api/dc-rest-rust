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

use async_trait::async_trait;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
use crate::apis::ContentType;

#[async_trait]
pub trait DefaultApi: Send + Sync {

    /// PUT /channels/{channel_id}/recipients/{user_id}
    ///
    /// 
    async fn add_group_dm_user<'channel_id, 'user_id, 'add_group_dm_user_request>(&self, channel_id: &'channel_id str, user_id: &'user_id str, add_group_dm_user_request: models::AddGroupDmUserRequest) -> Result<ResponseContent<AddGroupDmUserSuccess>, Error<AddGroupDmUserError>>;

    /// PUT /guilds/{guild_id}/members/{user_id}
    ///
    /// 
    async fn add_guild_member<'guild_id, 'user_id, 'add_guild_member_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, add_guild_member_request: models::AddGuildMemberRequest) -> Result<ResponseContent<AddGuildMemberSuccess>, Error<AddGuildMemberError>>;

    /// PUT /guilds/{guild_id}/members/{user_id}/roles/{role_id}
    ///
    /// 
    async fn add_guild_member_role<'guild_id, 'user_id, 'role_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str, role_id: &'role_id str) -> Result<ResponseContent<AddGuildMemberRoleSuccess>, Error<AddGuildMemberRoleError>>;

    /// PUT /lobbies/{lobby_id}/members/{user_id}
    ///
    /// 
    async fn add_lobby_member<'lobby_id, 'user_id, 'add_lobby_member_request>(&self, lobby_id: &'lobby_id str, user_id: &'user_id str, add_lobby_member_request: models::AddLobbyMemberRequest) -> Result<ResponseContent<AddLobbyMemberSuccess>, Error<AddLobbyMemberError>>;

    /// PUT /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me
    ///
    /// 
    async fn add_my_message_reaction<'channel_id, 'message_id, 'emoji_name>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str) -> Result<ResponseContent<AddMyMessageReactionSuccess>, Error<AddMyMessageReactionError>>;

    /// PUT /channels/{channel_id}/thread-members/{user_id}
    ///
    /// 
    async fn add_thread_member<'channel_id, 'user_id>(&self, channel_id: &'channel_id str, user_id: &'user_id str) -> Result<ResponseContent<AddThreadMemberSuccess>, Error<AddThreadMemberError>>;

    /// GET /applications/{application_id}/activity-instances/{instance_id}
    ///
    /// 
    async fn applications_get_activity_instance<'application_id, 'instance_id>(&self, application_id: &'application_id str, instance_id: &'instance_id str) -> Result<ResponseContent<ApplicationsGetActivityInstanceSuccess>, Error<ApplicationsGetActivityInstanceError>>;

    /// PUT /guilds/{guild_id}/bans/{user_id}
    ///
    /// 
    async fn ban_user_from_guild<'guild_id, 'user_id, 'ban_user_from_guild_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, ban_user_from_guild_request: models::BanUserFromGuildRequest) -> Result<ResponseContent<BanUserFromGuildSuccess>, Error<BanUserFromGuildError>>;

    /// POST /guilds/{guild_id}/bulk-ban
    ///
    /// 
    async fn bulk_ban_users_from_guild<'guild_id, 'bulk_ban_users_from_guild_request>(&self, guild_id: &'guild_id str, bulk_ban_users_from_guild_request: models::BulkBanUsersFromGuildRequest) -> Result<ResponseContent<BulkBanUsersFromGuildSuccess>, Error<BulkBanUsersFromGuildError>>;

    /// POST /channels/{channel_id}/messages/bulk-delete
    ///
    /// 
    async fn bulk_delete_messages<'channel_id, 'bulk_delete_messages_request>(&self, channel_id: &'channel_id str, bulk_delete_messages_request: models::BulkDeleteMessagesRequest) -> Result<ResponseContent<BulkDeleteMessagesSuccess>, Error<BulkDeleteMessagesError>>;

    /// PUT /applications/{application_id}/commands
    ///
    /// 
    async fn bulk_set_application_commands<'application_id, 'application_command_update_request>(&self, application_id: &'application_id str, application_command_update_request: Option<Vec<models::ApplicationCommandUpdateRequest>>) -> Result<ResponseContent<BulkSetApplicationCommandsSuccess>, Error<BulkSetApplicationCommandsError>>;

    /// PUT /applications/{application_id}/guilds/{guild_id}/commands
    ///
    /// 
    async fn bulk_set_guild_application_commands<'application_id, 'guild_id, 'application_command_update_request>(&self, application_id: &'application_id str, guild_id: &'guild_id str, application_command_update_request: Option<Vec<models::ApplicationCommandUpdateRequest>>) -> Result<ResponseContent<BulkSetGuildApplicationCommandsSuccess>, Error<BulkSetGuildApplicationCommandsError>>;

    /// PATCH /guilds/{guild_id}/channels
    ///
    /// 
    async fn bulk_update_guild_channels<'guild_id, 'bulk_update_guild_channels_request_inner>(&self, guild_id: &'guild_id str, bulk_update_guild_channels_request_inner: Vec<models::BulkUpdateGuildChannelsRequestInner>) -> Result<ResponseContent<BulkUpdateGuildChannelsSuccess>, Error<BulkUpdateGuildChannelsError>>;

    /// PATCH /guilds/{guild_id}/roles
    ///
    /// 
    async fn bulk_update_guild_roles<'guild_id, 'bulk_update_guild_roles_request_inner>(&self, guild_id: &'guild_id str, bulk_update_guild_roles_request_inner: Vec<models::BulkUpdateGuildRolesRequestInner>) -> Result<ResponseContent<BulkUpdateGuildRolesSuccess>, Error<BulkUpdateGuildRolesError>>;

    /// POST /lobbies/{lobby_id}/members/bulk
    ///
    /// 
    async fn bulk_update_lobby_members<'lobby_id, 'bulk_lobby_member_request>(&self, lobby_id: &'lobby_id str, bulk_lobby_member_request: Option<Vec<models::BulkLobbyMemberRequest>>) -> Result<ResponseContent<BulkUpdateLobbyMembersSuccess>, Error<BulkUpdateLobbyMembersError>>;

    /// POST /applications/{application_id}/entitlements/{entitlement_id}/consume
    ///
    /// 
    async fn consume_entitlement<'application_id, 'entitlement_id>(&self, application_id: &'application_id str, entitlement_id: &'entitlement_id str) -> Result<ResponseContent<ConsumeEntitlementSuccess>, Error<ConsumeEntitlementError>>;

    /// POST /applications/{application_id}/commands
    ///
    /// 
    async fn create_application_command<'application_id, 'application_command_create_request>(&self, application_id: &'application_id str, application_command_create_request: models::ApplicationCommandCreateRequest) -> Result<ResponseContent<CreateApplicationCommandSuccess>, Error<CreateApplicationCommandError>>;

    /// POST /applications/{application_id}/emojis
    ///
    /// 
    async fn create_application_emoji<'application_id, 'create_application_emoji_request>(&self, application_id: &'application_id str, create_application_emoji_request: models::CreateApplicationEmojiRequest) -> Result<ResponseContent<CreateApplicationEmojiSuccess>, Error<CreateApplicationEmojiError>>;

    /// POST /guilds/{guild_id}/auto-moderation/rules
    ///
    /// 
    async fn create_auto_moderation_rule<'guild_id, 'create_auto_moderation_rule_request>(&self, guild_id: &'guild_id str, create_auto_moderation_rule_request: models::CreateAutoModerationRuleRequest) -> Result<ResponseContent<CreateAutoModerationRuleSuccess>, Error<CreateAutoModerationRuleError>>;

    /// POST /channels/{channel_id}/invites
    ///
    /// 
    async fn create_channel_invite<'channel_id, 'create_channel_invite_request>(&self, channel_id: &'channel_id str, create_channel_invite_request: models::CreateChannelInviteRequest) -> Result<ResponseContent<CreateChannelInviteSuccess>, Error<CreateChannelInviteError>>;

    /// POST /users/@me/channels
    ///
    /// 
    async fn create_dm<'create_private_channel_request>(&self, create_private_channel_request: models::CreatePrivateChannelRequest) -> Result<ResponseContent<CreateDmSuccess>, Error<CreateDmError>>;

    /// POST /applications/{application_id}/entitlements
    ///
    /// 
    async fn create_entitlement<'application_id, 'create_entitlement_request_data>(&self, application_id: &'application_id str, create_entitlement_request_data: models::CreateEntitlementRequestData) -> Result<ResponseContent<CreateEntitlementSuccess>, Error<CreateEntitlementError>>;

    /// POST /guilds
    ///
    /// 
    async fn create_guild<'guild_create_request>(&self, guild_create_request: models::GuildCreateRequest) -> Result<ResponseContent<CreateGuildSuccess>, Error<CreateGuildError>>;

    /// POST /applications/{application_id}/guilds/{guild_id}/commands
    ///
    /// 
    async fn create_guild_application_command<'application_id, 'guild_id, 'application_command_create_request>(&self, application_id: &'application_id str, guild_id: &'guild_id str, application_command_create_request: models::ApplicationCommandCreateRequest) -> Result<ResponseContent<CreateGuildApplicationCommandSuccess>, Error<CreateGuildApplicationCommandError>>;

    /// POST /guilds/{guild_id}/channels
    ///
    /// 
    async fn create_guild_channel<'guild_id, 'create_guild_channel_request>(&self, guild_id: &'guild_id str, create_guild_channel_request: models::CreateGuildChannelRequest) -> Result<ResponseContent<CreateGuildChannelSuccess>, Error<CreateGuildChannelError>>;

    /// POST /guilds/{guild_id}/emojis
    ///
    /// 
    async fn create_guild_emoji<'guild_id, 'create_guild_emoji_request>(&self, guild_id: &'guild_id str, create_guild_emoji_request: models::CreateGuildEmojiRequest) -> Result<ResponseContent<CreateGuildEmojiSuccess>, Error<CreateGuildEmojiError>>;

    /// POST /guilds/templates/{code}
    ///
    /// 
    async fn create_guild_from_template<'code, 'create_guild_from_template_request>(&self, code: &'code str, create_guild_from_template_request: models::CreateGuildFromTemplateRequest) -> Result<ResponseContent<CreateGuildFromTemplateSuccess>, Error<CreateGuildFromTemplateError>>;

    /// POST /guilds/{guild_id}/roles
    ///
    /// 
    async fn create_guild_role<'guild_id, 'create_guild_role_request>(&self, guild_id: &'guild_id str, create_guild_role_request: models::CreateGuildRoleRequest) -> Result<ResponseContent<CreateGuildRoleSuccess>, Error<CreateGuildRoleError>>;

    /// POST /guilds/{guild_id}/scheduled-events
    ///
    /// 
    async fn create_guild_scheduled_event<'guild_id, 'create_guild_scheduled_event_request>(&self, guild_id: &'guild_id str, create_guild_scheduled_event_request: models::CreateGuildScheduledEventRequest) -> Result<ResponseContent<CreateGuildScheduledEventSuccess>, Error<CreateGuildScheduledEventError>>;

    /// POST /guilds/{guild_id}/soundboard-sounds
    ///
    /// 
    async fn create_guild_soundboard_sound<'guild_id, 'soundboard_create_request>(&self, guild_id: &'guild_id str, soundboard_create_request: models::SoundboardCreateRequest) -> Result<ResponseContent<CreateGuildSoundboardSoundSuccess>, Error<CreateGuildSoundboardSoundError>>;

    /// POST /guilds/{guild_id}/stickers
    ///
    /// 
    async fn create_guild_sticker<'guild_id, 'name, 'tags, 'file, 'description>(&self, guild_id: &'guild_id str, name: &'name str, tags: &'tags str, file: &'file str, description: Option<&'description str>) -> Result<ResponseContent<CreateGuildStickerSuccess>, Error<CreateGuildStickerError>>;

    /// POST /guilds/{guild_id}/templates
    ///
    /// 
    async fn create_guild_template<'guild_id, 'create_guild_template_request>(&self, guild_id: &'guild_id str, create_guild_template_request: models::CreateGuildTemplateRequest) -> Result<ResponseContent<CreateGuildTemplateSuccess>, Error<CreateGuildTemplateError>>;

    /// POST /interactions/{interaction_id}/{interaction_token}/callback
    ///
    /// 
    async fn create_interaction_response<'interaction_id, 'interaction_token, 'create_interaction_response_request, 'with_response>(&self, interaction_id: &'interaction_id str, interaction_token: &'interaction_token str, create_interaction_response_request: models::CreateInteractionResponseRequest, with_response: Option<bool>) -> Result<ResponseContent<CreateInteractionResponseSuccess>, Error<CreateInteractionResponseError>>;

    /// POST /lobbies
    ///
    /// 
    async fn create_lobby<'create_lobby_request>(&self, create_lobby_request: models::CreateLobbyRequest) -> Result<ResponseContent<CreateLobbySuccess>, Error<CreateLobbyError>>;

    /// POST /lobbies/{lobby_id}/messages
    ///
    /// 
    async fn create_lobby_message<'lobby_id, 'sdk_message_request>(&self, lobby_id: &'lobby_id str, sdk_message_request: models::SdkMessageRequest) -> Result<ResponseContent<CreateLobbyMessageSuccess>, Error<CreateLobbyMessageError>>;

    /// POST /channels/{channel_id}/messages
    ///
    /// 
    async fn create_message<'channel_id, 'message_create_request>(&self, channel_id: &'channel_id str, message_create_request: models::MessageCreateRequest) -> Result<ResponseContent<CreateMessageSuccess>, Error<CreateMessageError>>;

    /// PUT /lobbies
    ///
    /// 
    async fn create_or_join_lobby<'create_or_join_lobby_request>(&self, create_or_join_lobby_request: models::CreateOrJoinLobbyRequest) -> Result<ResponseContent<CreateOrJoinLobbySuccess>, Error<CreateOrJoinLobbyError>>;

    /// PUT /channels/{channel_id}/messages/pins/{message_id}
    ///
    /// 
    async fn create_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<CreatePinSuccess>, Error<CreatePinError>>;

    /// POST /stage-instances
    ///
    /// 
    async fn create_stage_instance<'create_stage_instance_request>(&self, create_stage_instance_request: models::CreateStageInstanceRequest) -> Result<ResponseContent<CreateStageInstanceSuccess>, Error<CreateStageInstanceError>>;

    /// POST /channels/{channel_id}/threads
    ///
    /// 
    async fn create_thread<'channel_id, 'create_thread_request>(&self, channel_id: &'channel_id str, create_thread_request: models::CreateThreadRequest) -> Result<ResponseContent<CreateThreadSuccess>, Error<CreateThreadError>>;

    /// POST /channels/{channel_id}/messages/{message_id}/threads
    ///
    /// 
    async fn create_thread_from_message<'channel_id, 'message_id, 'create_text_thread_with_message_request>(&self, channel_id: &'channel_id str, message_id: &'message_id str, create_text_thread_with_message_request: models::CreateTextThreadWithMessageRequest) -> Result<ResponseContent<CreateThreadFromMessageSuccess>, Error<CreateThreadFromMessageError>>;

    /// POST /channels/{channel_id}/webhooks
    ///
    /// 
    async fn create_webhook<'channel_id, 'create_webhook_request>(&self, channel_id: &'channel_id str, create_webhook_request: models::CreateWebhookRequest) -> Result<ResponseContent<CreateWebhookSuccess>, Error<CreateWebhookError>>;

    /// POST /channels/{channel_id}/messages/{message_id}/crosspost
    ///
    /// 
    async fn crosspost_message<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<CrosspostMessageSuccess>, Error<CrosspostMessageError>>;

    /// DELETE /channels/{channel_id}/messages/{message_id}/reactions
    ///
    /// 
    async fn delete_all_message_reactions<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeleteAllMessageReactionsSuccess>, Error<DeleteAllMessageReactionsError>>;

    /// DELETE /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}
    ///
    /// 
    async fn delete_all_message_reactions_by_emoji<'channel_id, 'message_id, 'emoji_name>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str) -> Result<ResponseContent<DeleteAllMessageReactionsByEmojiSuccess>, Error<DeleteAllMessageReactionsByEmojiError>>;

    /// DELETE /applications/{application_id}/commands/{command_id}
    ///
    /// 
    async fn delete_application_command<'application_id, 'command_id>(&self, application_id: &'application_id str, command_id: &'command_id str) -> Result<ResponseContent<DeleteApplicationCommandSuccess>, Error<DeleteApplicationCommandError>>;

    /// DELETE /applications/{application_id}/emojis/{emoji_id}
    ///
    /// 
    async fn delete_application_emoji<'application_id, 'emoji_id>(&self, application_id: &'application_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<DeleteApplicationEmojiSuccess>, Error<DeleteApplicationEmojiError>>;

    /// DELETE /users/@me/applications/{application_id}/role-connection
    ///
    /// 
    async fn delete_application_user_role_connection<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<DeleteApplicationUserRoleConnectionSuccess>, Error<DeleteApplicationUserRoleConnectionError>>;

    /// DELETE /guilds/{guild_id}/auto-moderation/rules/{rule_id}
    ///
    /// 
    async fn delete_auto_moderation_rule<'guild_id, 'rule_id>(&self, guild_id: &'guild_id str, rule_id: &'rule_id str) -> Result<ResponseContent<DeleteAutoModerationRuleSuccess>, Error<DeleteAutoModerationRuleError>>;

    /// DELETE /channels/{channel_id}
    ///
    /// 
    async fn delete_channel<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<DeleteChannelSuccess>, Error<DeleteChannelError>>;

    /// DELETE /channels/{channel_id}/permissions/{overwrite_id}
    ///
    /// 
    async fn delete_channel_permission_overwrite<'channel_id, 'overwrite_id>(&self, channel_id: &'channel_id str, overwrite_id: &'overwrite_id str) -> Result<ResponseContent<DeleteChannelPermissionOverwriteSuccess>, Error<DeleteChannelPermissionOverwriteError>>;

    /// DELETE /applications/{application_id}/entitlements/{entitlement_id}
    ///
    /// 
    async fn delete_entitlement<'application_id, 'entitlement_id>(&self, application_id: &'application_id str, entitlement_id: &'entitlement_id str) -> Result<ResponseContent<DeleteEntitlementSuccess>, Error<DeleteEntitlementError>>;

    /// DELETE /channels/{channel_id}/recipients/{user_id}
    ///
    /// 
    async fn delete_group_dm_user<'channel_id, 'user_id>(&self, channel_id: &'channel_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteGroupDmUserSuccess>, Error<DeleteGroupDmUserError>>;

    /// DELETE /guilds/{guild_id}
    ///
    /// 
    async fn delete_guild<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<DeleteGuildSuccess>, Error<DeleteGuildError>>;

    /// DELETE /applications/{application_id}/guilds/{guild_id}/commands/{command_id}
    ///
    /// 
    async fn delete_guild_application_command<'application_id, 'guild_id, 'command_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str) -> Result<ResponseContent<DeleteGuildApplicationCommandSuccess>, Error<DeleteGuildApplicationCommandError>>;

    /// DELETE /guilds/{guild_id}/emojis/{emoji_id}
    ///
    /// 
    async fn delete_guild_emoji<'guild_id, 'emoji_id>(&self, guild_id: &'guild_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<DeleteGuildEmojiSuccess>, Error<DeleteGuildEmojiError>>;

    /// DELETE /guilds/{guild_id}/integrations/{integration_id}
    ///
    /// 
    async fn delete_guild_integration<'guild_id, 'integration_id>(&self, guild_id: &'guild_id str, integration_id: &'integration_id str) -> Result<ResponseContent<DeleteGuildIntegrationSuccess>, Error<DeleteGuildIntegrationError>>;

    /// DELETE /guilds/{guild_id}/members/{user_id}
    ///
    /// 
    async fn delete_guild_member<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteGuildMemberSuccess>, Error<DeleteGuildMemberError>>;

    /// DELETE /guilds/{guild_id}/members/{user_id}/roles/{role_id}
    ///
    /// 
    async fn delete_guild_member_role<'guild_id, 'user_id, 'role_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str, role_id: &'role_id str) -> Result<ResponseContent<DeleteGuildMemberRoleSuccess>, Error<DeleteGuildMemberRoleError>>;

    /// DELETE /guilds/{guild_id}/roles/{role_id}
    ///
    /// 
    async fn delete_guild_role<'guild_id, 'role_id>(&self, guild_id: &'guild_id str, role_id: &'role_id str) -> Result<ResponseContent<DeleteGuildRoleSuccess>, Error<DeleteGuildRoleError>>;

    /// DELETE /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}
    ///
    /// 
    async fn delete_guild_scheduled_event<'guild_id, 'guild_scheduled_event_id>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str) -> Result<ResponseContent<DeleteGuildScheduledEventSuccess>, Error<DeleteGuildScheduledEventError>>;

    /// DELETE /guilds/{guild_id}/soundboard-sounds/{sound_id}
    ///
    /// 
    async fn delete_guild_soundboard_sound<'guild_id, 'sound_id>(&self, guild_id: &'guild_id str, sound_id: &'sound_id str) -> Result<ResponseContent<DeleteGuildSoundboardSoundSuccess>, Error<DeleteGuildSoundboardSoundError>>;

    /// DELETE /guilds/{guild_id}/stickers/{sticker_id}
    ///
    /// 
    async fn delete_guild_sticker<'guild_id, 'sticker_id>(&self, guild_id: &'guild_id str, sticker_id: &'sticker_id str) -> Result<ResponseContent<DeleteGuildStickerSuccess>, Error<DeleteGuildStickerError>>;

    /// DELETE /guilds/{guild_id}/templates/{code}
    ///
    /// 
    async fn delete_guild_template<'guild_id, 'code>(&self, guild_id: &'guild_id str, code: &'code str) -> Result<ResponseContent<DeleteGuildTemplateSuccess>, Error<DeleteGuildTemplateError>>;

    /// DELETE /lobbies/{lobby_id}/members/{user_id}
    ///
    /// 
    async fn delete_lobby_member<'lobby_id, 'user_id>(&self, lobby_id: &'lobby_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteLobbyMemberSuccess>, Error<DeleteLobbyMemberError>>;

    /// DELETE /channels/{channel_id}/messages/{message_id}
    ///
    /// 
    async fn delete_message<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeleteMessageSuccess>, Error<DeleteMessageError>>;

    /// DELETE /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me
    ///
    /// 
    async fn delete_my_message_reaction<'channel_id, 'message_id, 'emoji_name>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str) -> Result<ResponseContent<DeleteMyMessageReactionSuccess>, Error<DeleteMyMessageReactionError>>;

    /// DELETE /webhooks/{webhook_id}/{webhook_token}/messages/@original
    ///
    /// 
    async fn delete_original_webhook_message<'webhook_id, 'webhook_token, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<DeleteOriginalWebhookMessageSuccess>, Error<DeleteOriginalWebhookMessageError>>;

    /// DELETE /channels/{channel_id}/messages/pins/{message_id}
    ///
    /// 
    async fn delete_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeletePinSuccess>, Error<DeletePinError>>;

    /// DELETE /stage-instances/{channel_id}
    ///
    /// 
    async fn delete_stage_instance<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<DeleteStageInstanceSuccess>, Error<DeleteStageInstanceError>>;

    /// DELETE /channels/{channel_id}/thread-members/{user_id}
    ///
    /// 
    async fn delete_thread_member<'channel_id, 'user_id>(&self, channel_id: &'channel_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteThreadMemberSuccess>, Error<DeleteThreadMemberError>>;

    /// DELETE /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/{user_id}
    ///
    /// 
    async fn delete_user_message_reaction<'channel_id, 'message_id, 'emoji_name, 'user_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str, user_id: &'user_id str) -> Result<ResponseContent<DeleteUserMessageReactionSuccess>, Error<DeleteUserMessageReactionError>>;

    /// DELETE /webhooks/{webhook_id}
    ///
    /// 
    async fn delete_webhook<'webhook_id>(&self, webhook_id: &'webhook_id str) -> Result<ResponseContent<DeleteWebhookSuccess>, Error<DeleteWebhookError>>;

    /// DELETE /webhooks/{webhook_id}/{webhook_token}
    ///
    /// 
    async fn delete_webhook_by_token<'webhook_id, 'webhook_token>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str) -> Result<ResponseContent<DeleteWebhookByTokenSuccess>, Error<DeleteWebhookByTokenError>>;

    /// DELETE /webhooks/{webhook_id}/{webhook_token}/messages/{message_id}
    ///
    /// 
    async fn delete_webhook_message<'webhook_id, 'webhook_token, 'message_id, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, message_id: &'message_id str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<DeleteWebhookMessageSuccess>, Error<DeleteWebhookMessageError>>;

    /// PUT /channels/{channel_id}/pins/{message_id}
    ///
    /// 
    async fn deprecated_create_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeprecatedCreatePinSuccess>, Error<DeprecatedCreatePinError>>;

    /// DELETE /channels/{channel_id}/pins/{message_id}
    ///
    /// 
    async fn deprecated_delete_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeprecatedDeletePinSuccess>, Error<DeprecatedDeletePinError>>;

    /// GET /channels/{channel_id}/pins
    ///
    /// 
    async fn deprecated_list_pins<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<DeprecatedListPinsSuccess>, Error<DeprecatedListPinsError>>;

    /// PATCH /lobbies/{lobby_id}
    ///
    /// 
    async fn edit_lobby<'lobby_id, 'create_lobby_request>(&self, lobby_id: &'lobby_id str, create_lobby_request: models::CreateLobbyRequest) -> Result<ResponseContent<EditLobbySuccess>, Error<EditLobbyError>>;

    /// PATCH /lobbies/{lobby_id}/channel-linking
    ///
    /// 
    async fn edit_lobby_channel_link<'lobby_id, 'edit_lobby_channel_link_request>(&self, lobby_id: &'lobby_id str, edit_lobby_channel_link_request: models::EditLobbyChannelLinkRequest) -> Result<ResponseContent<EditLobbyChannelLinkSuccess>, Error<EditLobbyChannelLinkError>>;

    /// POST /webhooks/{webhook_id}/{webhook_token}/github
    ///
    /// 
    async fn execute_github_compatible_webhook<'webhook_id, 'webhook_token, 'github_webhook, 'wait, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, github_webhook: models::GithubWebhook, wait: Option<bool>, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<ExecuteGithubCompatibleWebhookSuccess>, Error<ExecuteGithubCompatibleWebhookError>>;

    /// POST /webhooks/{webhook_id}/{webhook_token}/slack
    ///
    /// 
    async fn execute_slack_compatible_webhook<'webhook_id, 'webhook_token, 'slack_webhook, 'wait, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, slack_webhook: models::SlackWebhook, wait: Option<bool>, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<ExecuteSlackCompatibleWebhookSuccess>, Error<ExecuteSlackCompatibleWebhookError>>;

    /// POST /webhooks/{webhook_id}/{webhook_token}
    ///
    /// 
    async fn execute_webhook<'webhook_id, 'webhook_token, 'execute_webhook_request, 'wait, 'thread_id, 'with_components>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, execute_webhook_request: models::ExecuteWebhookRequest, wait: Option<bool>, thread_id: Option<&'thread_id str>, with_components: Option<bool>) -> Result<ResponseContent<ExecuteWebhookSuccess>, Error<ExecuteWebhookError>>;

    /// POST /channels/{channel_id}/followers
    ///
    /// 
    async fn follow_channel<'channel_id, 'follow_channel_request>(&self, channel_id: &'channel_id str, follow_channel_request: models::FollowChannelRequest) -> Result<ResponseContent<FollowChannelSuccess>, Error<FollowChannelError>>;

    /// GET /guilds/{guild_id}/threads/active
    ///
    /// 
    async fn get_active_guild_threads<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetActiveGuildThreadsSuccess>, Error<GetActiveGuildThreadsError>>;

    /// GET /channels/{channel_id}/polls/{message_id}/answers/{answer_id}
    ///
    /// 
    async fn get_answer_voters<'channel_id, 'message_id, 'answer_id, 'after, 'limit>(&self, channel_id: &'channel_id str, message_id: &'message_id str, answer_id: i32, after: Option<&'after str>, limit: Option<i32>) -> Result<ResponseContent<GetAnswerVotersSuccess>, Error<GetAnswerVotersError>>;

    /// GET /applications/{application_id}
    ///
    /// 
    async fn get_application<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<GetApplicationSuccess>, Error<GetApplicationError>>;

    /// GET /applications/{application_id}/commands/{command_id}
    ///
    /// 
    async fn get_application_command<'application_id, 'command_id>(&self, application_id: &'application_id str, command_id: &'command_id str) -> Result<ResponseContent<GetApplicationCommandSuccess>, Error<GetApplicationCommandError>>;

    /// GET /applications/{application_id}/emojis/{emoji_id}
    ///
    /// 
    async fn get_application_emoji<'application_id, 'emoji_id>(&self, application_id: &'application_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<GetApplicationEmojiSuccess>, Error<GetApplicationEmojiError>>;

    /// GET /applications/{application_id}/role-connections/metadata
    ///
    /// 
    async fn get_application_role_connections_metadata<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<GetApplicationRoleConnectionsMetadataSuccess>, Error<GetApplicationRoleConnectionsMetadataError>>;

    /// GET /users/@me/applications/{application_id}/role-connection
    ///
    /// 
    async fn get_application_user_role_connection<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<GetApplicationUserRoleConnectionSuccess>, Error<GetApplicationUserRoleConnectionError>>;

    /// GET /guilds/{guild_id}/auto-moderation/rules/{rule_id}
    ///
    /// 
    async fn get_auto_moderation_rule<'guild_id, 'rule_id>(&self, guild_id: &'guild_id str, rule_id: &'rule_id str) -> Result<ResponseContent<GetAutoModerationRuleSuccess>, Error<GetAutoModerationRuleError>>;

    /// GET /gateway/bot
    ///
    /// 
    async fn get_bot_gateway<>(&self, ) -> Result<ResponseContent<GetBotGatewaySuccess>, Error<GetBotGatewayError>>;

    /// GET /channels/{channel_id}
    ///
    /// 
    async fn get_channel<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<GetChannelSuccess>, Error<GetChannelError>>;

    /// GET /applications/{application_id}/entitlements/{entitlement_id}
    ///
    /// 
    async fn get_entitlement<'application_id, 'entitlement_id>(&self, application_id: &'application_id str, entitlement_id: &'entitlement_id str) -> Result<ResponseContent<GetEntitlementSuccess>, Error<GetEntitlementError>>;

    /// GET /applications/{application_id}/entitlements
    ///
    /// 
    async fn get_entitlements<'sku_ids, 'application_id, 'user_id, 'guild_id, 'before, 'after, 'limit, 'exclude_ended, 'exclude_deleted, 'only_active>(&self, sku_ids: &'sku_ids str, application_id: &'application_id str, user_id: Option<&'user_id str>, guild_id: Option<&'guild_id str>, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>, exclude_ended: Option<bool>, exclude_deleted: Option<bool>, only_active: Option<bool>) -> Result<ResponseContent<GetEntitlementsSuccess>, Error<GetEntitlementsError>>;

    /// GET /gateway
    ///
    /// 
    async fn get_gateway<>(&self, ) -> Result<ResponseContent<GetGatewaySuccess>, Error<GetGatewayError>>;

    /// GET /guilds/{guild_id}
    ///
    /// 
    async fn get_guild<'guild_id, 'with_counts>(&self, guild_id: &'guild_id str, with_counts: Option<bool>) -> Result<ResponseContent<GetGuildSuccess>, Error<GetGuildError>>;

    /// GET /applications/{application_id}/guilds/{guild_id}/commands/{command_id}
    ///
    /// 
    async fn get_guild_application_command<'application_id, 'guild_id, 'command_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str) -> Result<ResponseContent<GetGuildApplicationCommandSuccess>, Error<GetGuildApplicationCommandError>>;

    /// GET /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions
    ///
    /// 
    async fn get_guild_application_command_permissions<'application_id, 'guild_id, 'command_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str) -> Result<ResponseContent<GetGuildApplicationCommandPermissionsSuccess>, Error<GetGuildApplicationCommandPermissionsError>>;

    /// GET /guilds/{guild_id}/bans/{user_id}
    ///
    /// 
    async fn get_guild_ban<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<GetGuildBanSuccess>, Error<GetGuildBanError>>;

    /// GET /guilds/{guild_id}/emojis/{emoji_id}
    ///
    /// 
    async fn get_guild_emoji<'guild_id, 'emoji_id>(&self, guild_id: &'guild_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<GetGuildEmojiSuccess>, Error<GetGuildEmojiError>>;

    /// GET /guilds/{guild_id}/members/{user_id}
    ///
    /// 
    async fn get_guild_member<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<GetGuildMemberSuccess>, Error<GetGuildMemberError>>;

    /// GET /guilds/{guild_id}/new-member-welcome
    ///
    /// 
    async fn get_guild_new_member_welcome<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildNewMemberWelcomeSuccess>, Error<GetGuildNewMemberWelcomeError>>;

    /// GET /guilds/{guild_id}/preview
    ///
    /// 
    async fn get_guild_preview<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildPreviewSuccess>, Error<GetGuildPreviewError>>;

    /// GET /guilds/{guild_id}/roles/{role_id}
    ///
    /// 
    async fn get_guild_role<'guild_id, 'role_id>(&self, guild_id: &'guild_id str, role_id: &'role_id str) -> Result<ResponseContent<GetGuildRoleSuccess>, Error<GetGuildRoleError>>;

    /// GET /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}
    ///
    /// 
    async fn get_guild_scheduled_event<'guild_id, 'guild_scheduled_event_id, 'with_user_count>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str, with_user_count: Option<bool>) -> Result<ResponseContent<GetGuildScheduledEventSuccess>, Error<GetGuildScheduledEventError>>;

    /// GET /guilds/{guild_id}/soundboard-sounds/{sound_id}
    ///
    /// 
    async fn get_guild_soundboard_sound<'guild_id, 'sound_id>(&self, guild_id: &'guild_id str, sound_id: &'sound_id str) -> Result<ResponseContent<GetGuildSoundboardSoundSuccess>, Error<GetGuildSoundboardSoundError>>;

    /// GET /guilds/{guild_id}/stickers/{sticker_id}
    ///
    /// 
    async fn get_guild_sticker<'guild_id, 'sticker_id>(&self, guild_id: &'guild_id str, sticker_id: &'sticker_id str) -> Result<ResponseContent<GetGuildStickerSuccess>, Error<GetGuildStickerError>>;

    /// GET /guilds/templates/{code}
    ///
    /// 
    async fn get_guild_template<'code>(&self, code: &'code str) -> Result<ResponseContent<GetGuildTemplateSuccess>, Error<GetGuildTemplateError>>;

    /// GET /guilds/{guild_id}/vanity-url
    ///
    /// 
    async fn get_guild_vanity_url<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildVanityUrlSuccess>, Error<GetGuildVanityUrlError>>;

    /// GET /guilds/{guild_id}/webhooks
    ///
    /// 
    async fn get_guild_webhooks<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWebhooksSuccess>, Error<GetGuildWebhooksError>>;

    /// GET /guilds/{guild_id}/welcome-screen
    ///
    /// 
    async fn get_guild_welcome_screen<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWelcomeScreenSuccess>, Error<GetGuildWelcomeScreenError>>;

    /// GET /guilds/{guild_id}/widget.json
    ///
    /// 
    async fn get_guild_widget<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWidgetSuccess>, Error<GetGuildWidgetError>>;

    /// GET /guilds/{guild_id}/widget.png
    ///
    /// 
    async fn get_guild_widget_png<'guild_id, 'style>(&self, guild_id: &'guild_id str, style: Option<&'style str>) -> Result<ResponseContent<GetGuildWidgetPngSuccess>, Error<GetGuildWidgetPngError>>;

    /// GET /guilds/{guild_id}/widget
    ///
    /// 
    async fn get_guild_widget_settings<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWidgetSettingsSuccess>, Error<GetGuildWidgetSettingsError>>;

    /// GET /guilds/{guild_id}/onboarding
    ///
    /// 
    async fn get_guilds_onboarding<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildsOnboardingSuccess>, Error<GetGuildsOnboardingError>>;

    /// GET /lobbies/{lobby_id}
    ///
    /// 
    async fn get_lobby<'lobby_id>(&self, lobby_id: &'lobby_id str) -> Result<ResponseContent<GetLobbySuccess>, Error<GetLobbyError>>;

    /// GET /lobbies/{lobby_id}/messages
    ///
    /// 
    async fn get_lobby_messages<'lobby_id, 'limit>(&self, lobby_id: &'lobby_id str, limit: Option<i32>) -> Result<ResponseContent<GetLobbyMessagesSuccess>, Error<GetLobbyMessagesError>>;

    /// GET /channels/{channel_id}/messages/{message_id}
    ///
    /// 
    async fn get_message<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<GetMessageSuccess>, Error<GetMessageError>>;

    /// GET /applications/@me
    ///
    /// 
    async fn get_my_application<>(&self, ) -> Result<ResponseContent<GetMyApplicationSuccess>, Error<GetMyApplicationError>>;

    /// GET /users/@me/guilds/{guild_id}/member
    ///
    /// 
    async fn get_my_guild_member<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetMyGuildMemberSuccess>, Error<GetMyGuildMemberError>>;

    /// GET /oauth2/applications/@me
    ///
    /// 
    async fn get_my_oauth2_application<>(&self, ) -> Result<ResponseContent<GetMyOauth2ApplicationSuccess>, Error<GetMyOauth2ApplicationError>>;

    /// GET /oauth2/@me
    ///
    /// 
    async fn get_my_oauth2_authorization<>(&self, ) -> Result<ResponseContent<GetMyOauth2AuthorizationSuccess>, Error<GetMyOauth2AuthorizationError>>;

    /// GET /users/@me
    ///
    /// 
    async fn get_my_user<>(&self, ) -> Result<ResponseContent<GetMyUserSuccess>, Error<GetMyUserError>>;

    /// GET /oauth2/userinfo
    ///
    /// 
    async fn get_openid_connect_userinfo<>(&self, ) -> Result<ResponseContent<GetOpenidConnectUserinfoSuccess>, Error<GetOpenidConnectUserinfoError>>;

    /// GET /webhooks/{webhook_id}/{webhook_token}/messages/@original
    ///
    /// 
    async fn get_original_webhook_message<'webhook_id, 'webhook_token, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<GetOriginalWebhookMessageSuccess>, Error<GetOriginalWebhookMessageError>>;

    /// GET /oauth2/keys
    ///
    /// 
    async fn get_public_keys<>(&self, ) -> Result<ResponseContent<GetPublicKeysSuccess>, Error<GetPublicKeysError>>;

    /// GET /guilds/{guild_id}/voice-states/@me
    ///
    /// 
    async fn get_self_voice_state<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetSelfVoiceStateSuccess>, Error<GetSelfVoiceStateError>>;

    /// GET /soundboard-default-sounds
    ///
    /// 
    async fn get_soundboard_default_sounds<>(&self, ) -> Result<ResponseContent<GetSoundboardDefaultSoundsSuccess>, Error<GetSoundboardDefaultSoundsError>>;

    /// GET /stage-instances/{channel_id}
    ///
    /// 
    async fn get_stage_instance<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<GetStageInstanceSuccess>, Error<GetStageInstanceError>>;

    /// GET /stickers/{sticker_id}
    ///
    /// 
    async fn get_sticker<'sticker_id>(&self, sticker_id: &'sticker_id str) -> Result<ResponseContent<GetStickerSuccess>, Error<GetStickerError>>;

    /// GET /sticker-packs/{pack_id}
    ///
    /// 
    async fn get_sticker_pack<'pack_id>(&self, pack_id: &'pack_id str) -> Result<ResponseContent<GetStickerPackSuccess>, Error<GetStickerPackError>>;

    /// GET /channels/{channel_id}/thread-members/{user_id}
    ///
    /// 
    async fn get_thread_member<'channel_id, 'user_id, 'with_member>(&self, channel_id: &'channel_id str, user_id: &'user_id str, with_member: Option<bool>) -> Result<ResponseContent<GetThreadMemberSuccess>, Error<GetThreadMemberError>>;

    /// GET /users/{user_id}
    ///
    /// 
    async fn get_user<'user_id>(&self, user_id: &'user_id str) -> Result<ResponseContent<GetUserSuccess>, Error<GetUserError>>;

    /// GET /guilds/{guild_id}/voice-states/{user_id}
    ///
    /// 
    async fn get_voice_state<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<GetVoiceStateSuccess>, Error<GetVoiceStateError>>;

    /// GET /webhooks/{webhook_id}
    ///
    /// 
    async fn get_webhook<'webhook_id>(&self, webhook_id: &'webhook_id str) -> Result<ResponseContent<GetWebhookSuccess>, Error<GetWebhookError>>;

    /// GET /webhooks/{webhook_id}/{webhook_token}
    ///
    /// 
    async fn get_webhook_by_token<'webhook_id, 'webhook_token>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str) -> Result<ResponseContent<GetWebhookByTokenSuccess>, Error<GetWebhookByTokenError>>;

    /// GET /webhooks/{webhook_id}/{webhook_token}/messages/{message_id}
    ///
    /// 
    async fn get_webhook_message<'webhook_id, 'webhook_token, 'message_id, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, message_id: &'message_id str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<GetWebhookMessageSuccess>, Error<GetWebhookMessageError>>;

    /// GET /invites/{code}
    ///
    /// 
    async fn invite_resolve<'code, 'with_counts, 'guild_scheduled_event_id>(&self, code: &'code str, with_counts: Option<bool>, guild_scheduled_event_id: Option<&'guild_scheduled_event_id str>) -> Result<ResponseContent<InviteResolveSuccess>, Error<InviteResolveError>>;

    /// DELETE /invites/{code}
    ///
    /// 
    async fn invite_revoke<'code>(&self, code: &'code str) -> Result<ResponseContent<InviteRevokeSuccess>, Error<InviteRevokeError>>;

    /// PUT /channels/{channel_id}/thread-members/@me
    ///
    /// 
    async fn join_thread<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<JoinThreadSuccess>, Error<JoinThreadError>>;

    /// DELETE /users/@me/guilds/{guild_id}
    ///
    /// 
    async fn leave_guild<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<LeaveGuildSuccess>, Error<LeaveGuildError>>;

    /// DELETE /lobbies/{lobby_id}/members/@me
    ///
    /// 
    async fn leave_lobby<'lobby_id>(&self, lobby_id: &'lobby_id str) -> Result<ResponseContent<LeaveLobbySuccess>, Error<LeaveLobbyError>>;

    /// DELETE /channels/{channel_id}/thread-members/@me
    ///
    /// 
    async fn leave_thread<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<LeaveThreadSuccess>, Error<LeaveThreadError>>;

    /// GET /applications/{application_id}/commands
    ///
    /// 
    async fn list_application_commands<'application_id, 'with_localizations>(&self, application_id: &'application_id str, with_localizations: Option<bool>) -> Result<ResponseContent<ListApplicationCommandsSuccess>, Error<ListApplicationCommandsError>>;

    /// GET /applications/{application_id}/emojis
    ///
    /// 
    async fn list_application_emojis<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<ListApplicationEmojisSuccess>, Error<ListApplicationEmojisError>>;

    /// GET /guilds/{guild_id}/auto-moderation/rules
    ///
    /// 
    async fn list_auto_moderation_rules<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListAutoModerationRulesSuccess>, Error<ListAutoModerationRulesError>>;

    /// GET /channels/{channel_id}/invites
    ///
    /// 
    async fn list_channel_invites<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<ListChannelInvitesSuccess>, Error<ListChannelInvitesError>>;

    /// GET /channels/{channel_id}/webhooks
    ///
    /// 
    async fn list_channel_webhooks<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<ListChannelWebhooksSuccess>, Error<ListChannelWebhooksError>>;

    /// GET /applications/{application_id}/guilds/{guild_id}/commands/permissions
    ///
    /// 
    async fn list_guild_application_command_permissions<'application_id, 'guild_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildApplicationCommandPermissionsSuccess>, Error<ListGuildApplicationCommandPermissionsError>>;

    /// GET /applications/{application_id}/guilds/{guild_id}/commands
    ///
    /// 
    async fn list_guild_application_commands<'application_id, 'guild_id, 'with_localizations>(&self, application_id: &'application_id str, guild_id: &'guild_id str, with_localizations: Option<bool>) -> Result<ResponseContent<ListGuildApplicationCommandsSuccess>, Error<ListGuildApplicationCommandsError>>;

    /// GET /guilds/{guild_id}/audit-logs
    ///
    /// 
    async fn list_guild_audit_log_entries<'guild_id, 'user_id, 'target_id, 'action_type, 'before, 'after, 'limit>(&self, guild_id: &'guild_id str, user_id: Option<&'user_id str>, target_id: Option<&'target_id str>, action_type: Option<i32>, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>) -> Result<ResponseContent<ListGuildAuditLogEntriesSuccess>, Error<ListGuildAuditLogEntriesError>>;

    /// GET /guilds/{guild_id}/bans
    ///
    /// 
    async fn list_guild_bans<'guild_id, 'limit, 'before, 'after>(&self, guild_id: &'guild_id str, limit: Option<i32>, before: Option<&'before str>, after: Option<&'after str>) -> Result<ResponseContent<ListGuildBansSuccess>, Error<ListGuildBansError>>;

    /// GET /guilds/{guild_id}/channels
    ///
    /// 
    async fn list_guild_channels<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildChannelsSuccess>, Error<ListGuildChannelsError>>;

    /// GET /guilds/{guild_id}/emojis
    ///
    /// 
    async fn list_guild_emojis<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildEmojisSuccess>, Error<ListGuildEmojisError>>;

    /// GET /guilds/{guild_id}/integrations
    ///
    /// 
    async fn list_guild_integrations<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildIntegrationsSuccess>, Error<ListGuildIntegrationsError>>;

    /// GET /guilds/{guild_id}/invites
    ///
    /// 
    async fn list_guild_invites<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildInvitesSuccess>, Error<ListGuildInvitesError>>;

    /// GET /guilds/{guild_id}/members
    ///
    /// 
    async fn list_guild_members<'guild_id, 'limit, 'after>(&self, guild_id: &'guild_id str, limit: Option<i32>, after: Option<i32>) -> Result<ResponseContent<ListGuildMembersSuccess>, Error<ListGuildMembersError>>;

    /// GET /guilds/{guild_id}/roles
    ///
    /// 
    async fn list_guild_roles<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildRolesSuccess>, Error<ListGuildRolesError>>;

    /// GET /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users
    ///
    /// 
    async fn list_guild_scheduled_event_users<'guild_id, 'guild_scheduled_event_id, 'with_member, 'limit, 'before, 'after>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str, with_member: Option<bool>, limit: Option<i32>, before: Option<&'before str>, after: Option<&'after str>) -> Result<ResponseContent<ListGuildScheduledEventUsersSuccess>, Error<ListGuildScheduledEventUsersError>>;

    /// GET /guilds/{guild_id}/scheduled-events
    ///
    /// 
    async fn list_guild_scheduled_events<'guild_id, 'with_user_count>(&self, guild_id: &'guild_id str, with_user_count: Option<bool>) -> Result<ResponseContent<ListGuildScheduledEventsSuccess>, Error<ListGuildScheduledEventsError>>;

    /// GET /guilds/{guild_id}/soundboard-sounds
    ///
    /// 
    async fn list_guild_soundboard_sounds<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildSoundboardSoundsSuccess>, Error<ListGuildSoundboardSoundsError>>;

    /// GET /guilds/{guild_id}/stickers
    ///
    /// 
    async fn list_guild_stickers<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildStickersSuccess>, Error<ListGuildStickersError>>;

    /// GET /guilds/{guild_id}/templates
    ///
    /// 
    async fn list_guild_templates<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildTemplatesSuccess>, Error<ListGuildTemplatesError>>;

    /// GET /guilds/{guild_id}/regions
    ///
    /// 
    async fn list_guild_voice_regions<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildVoiceRegionsSuccess>, Error<ListGuildVoiceRegionsError>>;

    /// GET /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}
    ///
    /// 
    async fn list_message_reactions_by_emoji<'channel_id, 'message_id, 'emoji_name, 'after, 'limit, 'r_type>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str, after: Option<&'after str>, limit: Option<i32>, r#type: Option<i32>) -> Result<ResponseContent<ListMessageReactionsByEmojiSuccess>, Error<ListMessageReactionsByEmojiError>>;

    /// GET /channels/{channel_id}/messages
    ///
    /// 
    async fn list_messages<'channel_id, 'around, 'before, 'after, 'limit>(&self, channel_id: &'channel_id str, around: Option<&'around str>, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>) -> Result<ResponseContent<ListMessagesSuccess>, Error<ListMessagesError>>;

    /// GET /users/@me/connections
    ///
    /// 
    async fn list_my_connections<>(&self, ) -> Result<ResponseContent<ListMyConnectionsSuccess>, Error<ListMyConnectionsError>>;

    /// GET /users/@me/guilds
    ///
    /// 
    async fn list_my_guilds<'before, 'after, 'limit, 'with_counts>(&self, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>, with_counts: Option<bool>) -> Result<ResponseContent<ListMyGuildsSuccess>, Error<ListMyGuildsError>>;

    /// GET /channels/{channel_id}/users/@me/threads/archived/private
    ///
    /// 
    async fn list_my_private_archived_threads<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<&'before str>, limit: Option<i32>) -> Result<ResponseContent<ListMyPrivateArchivedThreadsSuccess>, Error<ListMyPrivateArchivedThreadsError>>;

    /// GET /channels/{channel_id}/messages/pins
    ///
    /// 
    async fn list_pins<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPinsSuccess>, Error<ListPinsError>>;

    /// GET /channels/{channel_id}/threads/archived/private
    ///
    /// 
    async fn list_private_archived_threads<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPrivateArchivedThreadsSuccess>, Error<ListPrivateArchivedThreadsError>>;

    /// GET /channels/{channel_id}/threads/archived/public
    ///
    /// 
    async fn list_public_archived_threads<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPublicArchivedThreadsSuccess>, Error<ListPublicArchivedThreadsError>>;

    /// GET /sticker-packs
    ///
    /// 
    async fn list_sticker_packs<>(&self, ) -> Result<ResponseContent<ListStickerPacksSuccess>, Error<ListStickerPacksError>>;

    /// GET /channels/{channel_id}/thread-members
    ///
    /// 
    async fn list_thread_members<'channel_id, 'with_member, 'limit, 'after>(&self, channel_id: &'channel_id str, with_member: Option<bool>, limit: Option<i32>, after: Option<&'after str>) -> Result<ResponseContent<ListThreadMembersSuccess>, Error<ListThreadMembersError>>;

    /// GET /voice/regions
    ///
    /// 
    async fn list_voice_regions<>(&self, ) -> Result<ResponseContent<ListVoiceRegionsSuccess>, Error<ListVoiceRegionsError>>;

    /// POST /partner-sdk/token
    ///
    /// 
    async fn partner_sdk_token<'partner_sdk_unmerge_provisional_account_request>(&self, partner_sdk_unmerge_provisional_account_request: models::PartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<PartnerSdkTokenSuccess>, Error<PartnerSdkTokenError>>;

    /// POST /partner-sdk/provisional-accounts/unmerge
    ///
    /// 
    async fn partner_sdk_unmerge_provisional_account<'partner_sdk_unmerge_provisional_account_request>(&self, partner_sdk_unmerge_provisional_account_request: models::PartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<PartnerSdkUnmergeProvisionalAccountSuccess>, Error<PartnerSdkUnmergeProvisionalAccountError>>;

    /// POST /channels/{channel_id}/polls/{message_id}/expire
    ///
    /// 
    async fn poll_expire<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<PollExpireSuccess>, Error<PollExpireError>>;

    /// GET /guilds/{guild_id}/prune
    ///
    /// 
    async fn preview_prune_guild<'guild_id, 'days, 'include_roles>(&self, guild_id: &'guild_id str, days: Option<i32>, include_roles: Option<&'include_roles str>) -> Result<ResponseContent<PreviewPruneGuildSuccess>, Error<PreviewPruneGuildError>>;

    /// POST /guilds/{guild_id}/prune
    ///
    /// 
    async fn prune_guild<'guild_id, 'prune_guild_request>(&self, guild_id: &'guild_id str, prune_guild_request: models::PruneGuildRequest) -> Result<ResponseContent<PruneGuildSuccess>, Error<PruneGuildError>>;

    /// PUT /guilds/{guild_id}/onboarding
    ///
    /// 
    async fn put_guilds_onboarding<'guild_id, 'update_guild_onboarding_request>(&self, guild_id: &'guild_id str, update_guild_onboarding_request: models::UpdateGuildOnboardingRequest) -> Result<ResponseContent<PutGuildsOnboardingSuccess>, Error<PutGuildsOnboardingError>>;

    /// GET /guilds/{guild_id}/members/search
    ///
    /// 
    async fn search_guild_members<'limit, 'query, 'guild_id>(&self, limit: i32, query: &'query str, guild_id: &'guild_id str) -> Result<ResponseContent<SearchGuildMembersSuccess>, Error<SearchGuildMembersError>>;

    /// POST /channels/{channel_id}/send-soundboard-sound
    ///
    /// 
    async fn send_soundboard_sound<'channel_id, 'soundboard_sound_send_request>(&self, channel_id: &'channel_id str, soundboard_sound_send_request: models::SoundboardSoundSendRequest) -> Result<ResponseContent<SendSoundboardSoundSuccess>, Error<SendSoundboardSoundError>>;

    /// PUT /channels/{channel_id}/permissions/{overwrite_id}
    ///
    /// 
    async fn set_channel_permission_overwrite<'channel_id, 'overwrite_id, 'set_channel_permission_overwrite_request>(&self, channel_id: &'channel_id str, overwrite_id: &'overwrite_id str, set_channel_permission_overwrite_request: models::SetChannelPermissionOverwriteRequest) -> Result<ResponseContent<SetChannelPermissionOverwriteSuccess>, Error<SetChannelPermissionOverwriteError>>;

    /// PUT /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions
    ///
    /// 
    async fn set_guild_application_command_permissions<'application_id, 'guild_id, 'command_id, 'set_guild_application_command_permissions_request>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str, set_guild_application_command_permissions_request: models::SetGuildApplicationCommandPermissionsRequest) -> Result<ResponseContent<SetGuildApplicationCommandPermissionsSuccess>, Error<SetGuildApplicationCommandPermissionsError>>;

    /// POST /guilds/{guild_id}/mfa
    ///
    /// 
    async fn set_guild_mfa_level<'guild_id, 'set_guild_mfa_level_request>(&self, guild_id: &'guild_id str, set_guild_mfa_level_request: models::SetGuildMfaLevelRequest) -> Result<ResponseContent<SetGuildMfaLevelSuccess>, Error<SetGuildMfaLevelError>>;

    /// PUT /guilds/{guild_id}/templates/{code}
    ///
    /// 
    async fn sync_guild_template<'guild_id, 'code>(&self, guild_id: &'guild_id str, code: &'code str) -> Result<ResponseContent<SyncGuildTemplateSuccess>, Error<SyncGuildTemplateError>>;

    /// GET /channels/{channel_id}/threads/search
    ///
    /// 
    async fn thread_search<'channel_id, 'name, 'slop, 'min_id, 'max_id, 'tag, 'tag_setting, 'archived, 'sort_by, 'sort_order, 'limit, 'offset>(&self, channel_id: &'channel_id str, name: Option<&'name str>, slop: Option<i32>, min_id: Option<&'min_id str>, max_id: Option<&'max_id str>, tag: Option<&'tag str>, tag_setting: Option<&'tag_setting str>, archived: Option<bool>, sort_by: Option<&'sort_by str>, sort_order: Option<&'sort_order str>, limit: Option<i32>, offset: Option<i32>) -> Result<ResponseContent<ThreadSearchSuccess>, Error<ThreadSearchError>>;

    /// POST /channels/{channel_id}/typing
    ///
    /// 
    async fn trigger_typing_indicator<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<TriggerTypingIndicatorSuccess>, Error<TriggerTypingIndicatorError>>;

    /// DELETE /guilds/{guild_id}/bans/{user_id}
    ///
    /// 
    async fn unban_user_from_guild<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<UnbanUserFromGuildSuccess>, Error<UnbanUserFromGuildError>>;

    /// PATCH /applications/{application_id}
    ///
    /// 
    async fn update_application<'application_id, 'application_form_partial>(&self, application_id: &'application_id str, application_form_partial: models::ApplicationFormPartial) -> Result<ResponseContent<UpdateApplicationSuccess>, Error<UpdateApplicationError>>;

    /// PATCH /applications/{application_id}/commands/{command_id}
    ///
    /// 
    async fn update_application_command<'application_id, 'command_id, 'application_command_patch_request_partial>(&self, application_id: &'application_id str, command_id: &'command_id str, application_command_patch_request_partial: models::ApplicationCommandPatchRequestPartial) -> Result<ResponseContent<UpdateApplicationCommandSuccess>, Error<UpdateApplicationCommandError>>;

    /// PATCH /applications/{application_id}/emojis/{emoji_id}
    ///
    /// 
    async fn update_application_emoji<'application_id, 'emoji_id, 'update_application_emoji_request>(&self, application_id: &'application_id str, emoji_id: &'emoji_id str, update_application_emoji_request: models::UpdateApplicationEmojiRequest) -> Result<ResponseContent<UpdateApplicationEmojiSuccess>, Error<UpdateApplicationEmojiError>>;

    /// PUT /applications/{application_id}/role-connections/metadata
    ///
    /// 
    async fn update_application_role_connections_metadata<'application_id, 'application_role_connections_metadata_item_request>(&self, application_id: &'application_id str, application_role_connections_metadata_item_request: Option<Vec<models::ApplicationRoleConnectionsMetadataItemRequest>>) -> Result<ResponseContent<UpdateApplicationRoleConnectionsMetadataSuccess>, Error<UpdateApplicationRoleConnectionsMetadataError>>;

    /// PUT /users/@me/applications/{application_id}/role-connection
    ///
    /// 
    async fn update_application_user_role_connection<'application_id, 'update_application_user_role_connection_request>(&self, application_id: &'application_id str, update_application_user_role_connection_request: models::UpdateApplicationUserRoleConnectionRequest) -> Result<ResponseContent<UpdateApplicationUserRoleConnectionSuccess>, Error<UpdateApplicationUserRoleConnectionError>>;

    /// PATCH /guilds/{guild_id}/auto-moderation/rules/{rule_id}
    ///
    /// 
    async fn update_auto_moderation_rule<'guild_id, 'rule_id, 'update_auto_moderation_rule_request>(&self, guild_id: &'guild_id str, rule_id: &'rule_id str, update_auto_moderation_rule_request: models::UpdateAutoModerationRuleRequest) -> Result<ResponseContent<UpdateAutoModerationRuleSuccess>, Error<UpdateAutoModerationRuleError>>;

    /// PATCH /channels/{channel_id}
    ///
    /// 
    async fn update_channel<'channel_id, 'update_channel_request>(&self, channel_id: &'channel_id str, update_channel_request: models::UpdateChannelRequest) -> Result<ResponseContent<UpdateChannelSuccess>, Error<UpdateChannelError>>;

    /// PATCH /guilds/{guild_id}
    ///
    /// 
    async fn update_guild<'guild_id, 'guild_patch_request_partial>(&self, guild_id: &'guild_id str, guild_patch_request_partial: models::GuildPatchRequestPartial) -> Result<ResponseContent<UpdateGuildSuccess>, Error<UpdateGuildError>>;

    /// PATCH /applications/{application_id}/guilds/{guild_id}/commands/{command_id}
    ///
    /// 
    async fn update_guild_application_command<'application_id, 'guild_id, 'command_id, 'application_command_patch_request_partial>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str, application_command_patch_request_partial: models::ApplicationCommandPatchRequestPartial) -> Result<ResponseContent<UpdateGuildApplicationCommandSuccess>, Error<UpdateGuildApplicationCommandError>>;

    /// PATCH /guilds/{guild_id}/emojis/{emoji_id}
    ///
    /// 
    async fn update_guild_emoji<'guild_id, 'emoji_id, 'update_guild_emoji_request>(&self, guild_id: &'guild_id str, emoji_id: &'emoji_id str, update_guild_emoji_request: models::UpdateGuildEmojiRequest) -> Result<ResponseContent<UpdateGuildEmojiSuccess>, Error<UpdateGuildEmojiError>>;

    /// PATCH /guilds/{guild_id}/members/{user_id}
    ///
    /// 
    async fn update_guild_member<'guild_id, 'user_id, 'update_guild_member_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, update_guild_member_request: models::UpdateGuildMemberRequest) -> Result<ResponseContent<UpdateGuildMemberSuccess>, Error<UpdateGuildMemberError>>;

    /// PATCH /guilds/{guild_id}/roles/{role_id}
    ///
    /// 
    async fn update_guild_role<'guild_id, 'role_id, 'create_guild_role_request>(&self, guild_id: &'guild_id str, role_id: &'role_id str, create_guild_role_request: models::CreateGuildRoleRequest) -> Result<ResponseContent<UpdateGuildRoleSuccess>, Error<UpdateGuildRoleError>>;

    /// PATCH /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}
    ///
    /// 
    async fn update_guild_scheduled_event<'guild_id, 'guild_scheduled_event_id, 'update_guild_scheduled_event_request>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str, update_guild_scheduled_event_request: models::UpdateGuildScheduledEventRequest) -> Result<ResponseContent<UpdateGuildScheduledEventSuccess>, Error<UpdateGuildScheduledEventError>>;

    /// PATCH /guilds/{guild_id}/soundboard-sounds/{sound_id}
    ///
    /// 
    async fn update_guild_soundboard_sound<'guild_id, 'sound_id, 'soundboard_patch_request_partial>(&self, guild_id: &'guild_id str, sound_id: &'sound_id str, soundboard_patch_request_partial: models::SoundboardPatchRequestPartial) -> Result<ResponseContent<UpdateGuildSoundboardSoundSuccess>, Error<UpdateGuildSoundboardSoundError>>;

    /// PATCH /guilds/{guild_id}/stickers/{sticker_id}
    ///
    /// 
    async fn update_guild_sticker<'guild_id, 'sticker_id, 'update_guild_sticker_request>(&self, guild_id: &'guild_id str, sticker_id: &'sticker_id str, update_guild_sticker_request: models::UpdateGuildStickerRequest) -> Result<ResponseContent<UpdateGuildStickerSuccess>, Error<UpdateGuildStickerError>>;

    /// PATCH /guilds/{guild_id}/templates/{code}
    ///
    /// 
    async fn update_guild_template<'guild_id, 'code, 'update_guild_template_request>(&self, guild_id: &'guild_id str, code: &'code str, update_guild_template_request: models::UpdateGuildTemplateRequest) -> Result<ResponseContent<UpdateGuildTemplateSuccess>, Error<UpdateGuildTemplateError>>;

    /// PATCH /guilds/{guild_id}/welcome-screen
    ///
    /// 
    async fn update_guild_welcome_screen<'guild_id, 'welcome_screen_patch_request_partial>(&self, guild_id: &'guild_id str, welcome_screen_patch_request_partial: models::WelcomeScreenPatchRequestPartial) -> Result<ResponseContent<UpdateGuildWelcomeScreenSuccess>, Error<UpdateGuildWelcomeScreenError>>;

    /// PATCH /guilds/{guild_id}/widget
    ///
    /// 
    async fn update_guild_widget_settings<'guild_id, 'update_guild_widget_settings_request>(&self, guild_id: &'guild_id str, update_guild_widget_settings_request: models::UpdateGuildWidgetSettingsRequest) -> Result<ResponseContent<UpdateGuildWidgetSettingsSuccess>, Error<UpdateGuildWidgetSettingsError>>;

    /// PATCH /channels/{channel_id}/messages/{message_id}
    ///
    /// 
    async fn update_message<'channel_id, 'message_id, 'message_edit_request_partial>(&self, channel_id: &'channel_id str, message_id: &'message_id str, message_edit_request_partial: models::MessageEditRequestPartial) -> Result<ResponseContent<UpdateMessageSuccess>, Error<UpdateMessageError>>;

    /// PATCH /applications/@me
    ///
    /// 
    async fn update_my_application<'application_form_partial>(&self, application_form_partial: models::ApplicationFormPartial) -> Result<ResponseContent<UpdateMyApplicationSuccess>, Error<UpdateMyApplicationError>>;

    /// PATCH /guilds/{guild_id}/members/@me
    ///
    /// 
    async fn update_my_guild_member<'guild_id, 'update_my_guild_member_request>(&self, guild_id: &'guild_id str, update_my_guild_member_request: models::UpdateMyGuildMemberRequest) -> Result<ResponseContent<UpdateMyGuildMemberSuccess>, Error<UpdateMyGuildMemberError>>;

    /// PATCH /users/@me
    ///
    /// 
    async fn update_my_user<'bot_account_patch_request>(&self, bot_account_patch_request: models::BotAccountPatchRequest) -> Result<ResponseContent<UpdateMyUserSuccess>, Error<UpdateMyUserError>>;

    /// PATCH /webhooks/{webhook_id}/{webhook_token}/messages/@original
    ///
    /// 
    async fn update_original_webhook_message<'webhook_id, 'webhook_token, 'incoming_webhook_update_request_partial, 'thread_id, 'with_components>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, incoming_webhook_update_request_partial: models::IncomingWebhookUpdateRequestPartial, thread_id: Option<&'thread_id str>, with_components: Option<bool>) -> Result<ResponseContent<UpdateOriginalWebhookMessageSuccess>, Error<UpdateOriginalWebhookMessageError>>;

    /// PATCH /guilds/{guild_id}/voice-states/@me
    ///
    /// 
    async fn update_self_voice_state<'guild_id, 'update_self_voice_state_request>(&self, guild_id: &'guild_id str, update_self_voice_state_request: models::UpdateSelfVoiceStateRequest) -> Result<ResponseContent<UpdateSelfVoiceStateSuccess>, Error<UpdateSelfVoiceStateError>>;

    /// PATCH /stage-instances/{channel_id}
    ///
    /// 
    async fn update_stage_instance<'channel_id, 'update_stage_instance_request>(&self, channel_id: &'channel_id str, update_stage_instance_request: models::UpdateStageInstanceRequest) -> Result<ResponseContent<UpdateStageInstanceSuccess>, Error<UpdateStageInstanceError>>;

    /// PATCH /guilds/{guild_id}/voice-states/{user_id}
    ///
    /// 
    async fn update_voice_state<'guild_id, 'user_id, 'update_voice_state_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, update_voice_state_request: models::UpdateVoiceStateRequest) -> Result<ResponseContent<UpdateVoiceStateSuccess>, Error<UpdateVoiceStateError>>;

    /// PATCH /webhooks/{webhook_id}
    ///
    /// 
    async fn update_webhook<'webhook_id, 'update_webhook_request>(&self, webhook_id: &'webhook_id str, update_webhook_request: models::UpdateWebhookRequest) -> Result<ResponseContent<UpdateWebhookSuccess>, Error<UpdateWebhookError>>;

    /// PATCH /webhooks/{webhook_id}/{webhook_token}
    ///
    /// 
    async fn update_webhook_by_token<'webhook_id, 'webhook_token, 'update_webhook_by_token_request>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, update_webhook_by_token_request: models::UpdateWebhookByTokenRequest) -> Result<ResponseContent<UpdateWebhookByTokenSuccess>, Error<UpdateWebhookByTokenError>>;

    /// PATCH /webhooks/{webhook_id}/{webhook_token}/messages/{message_id}
    ///
    /// 
    async fn update_webhook_message<'webhook_id, 'webhook_token, 'message_id, 'incoming_webhook_update_request_partial, 'thread_id, 'with_components>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, message_id: &'message_id str, incoming_webhook_update_request_partial: models::IncomingWebhookUpdateRequestPartial, thread_id: Option<&'thread_id str>, with_components: Option<bool>) -> Result<ResponseContent<UpdateWebhookMessageSuccess>, Error<UpdateWebhookMessageError>>;

    /// POST /applications/{application_id}/attachment
    ///
    /// 
    async fn upload_application_attachment<'application_id, 'file>(&self, application_id: &'application_id str, file: &'file str) -> Result<ResponseContent<UploadApplicationAttachmentSuccess>, Error<UploadApplicationAttachmentError>>;
}

pub struct DefaultApiClient {
    configuration: Arc<configuration::Configuration>
}

impl DefaultApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl DefaultApi for DefaultApiClient {
    async fn add_group_dm_user<'channel_id, 'user_id, 'add_group_dm_user_request>(&self, channel_id: &'channel_id str, user_id: &'user_id str, add_group_dm_user_request: models::AddGroupDmUserRequest) -> Result<ResponseContent<AddGroupDmUserSuccess>, Error<AddGroupDmUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/recipients/{user_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&add_group_dm_user_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<AddGroupDmUserSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddGroupDmUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn add_guild_member<'guild_id, 'user_id, 'add_guild_member_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, add_guild_member_request: models::AddGuildMemberRequest) -> Result<ResponseContent<AddGuildMemberSuccess>, Error<AddGuildMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&add_guild_member_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<AddGuildMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddGuildMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn add_guild_member_role<'guild_id, 'user_id, 'role_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str, role_id: &'role_id str) -> Result<ResponseContent<AddGuildMemberRoleSuccess>, Error<AddGuildMemberRoleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/{user_id}/roles/{role_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id), role_id=crate::apis::urlencode(role_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<AddGuildMemberRoleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddGuildMemberRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn add_lobby_member<'lobby_id, 'user_id, 'add_lobby_member_request>(&self, lobby_id: &'lobby_id str, user_id: &'user_id str, add_lobby_member_request: models::AddLobbyMemberRequest) -> Result<ResponseContent<AddLobbyMemberSuccess>, Error<AddLobbyMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/members/{user_id}", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&add_lobby_member_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<AddLobbyMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddLobbyMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn add_my_message_reaction<'channel_id, 'message_id, 'emoji_name>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str) -> Result<ResponseContent<AddMyMessageReactionSuccess>, Error<AddMyMessageReactionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id), emoji_name=crate::apis::urlencode(emoji_name));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<AddMyMessageReactionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddMyMessageReactionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn add_thread_member<'channel_id, 'user_id>(&self, channel_id: &'channel_id str, user_id: &'user_id str) -> Result<ResponseContent<AddThreadMemberSuccess>, Error<AddThreadMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/thread-members/{user_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<AddThreadMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<AddThreadMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn applications_get_activity_instance<'application_id, 'instance_id>(&self, application_id: &'application_id str, instance_id: &'instance_id str) -> Result<ResponseContent<ApplicationsGetActivityInstanceSuccess>, Error<ApplicationsGetActivityInstanceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/activity-instances/{instance_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), instance_id=crate::apis::urlencode(instance_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ApplicationsGetActivityInstanceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ApplicationsGetActivityInstanceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn ban_user_from_guild<'guild_id, 'user_id, 'ban_user_from_guild_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, ban_user_from_guild_request: models::BanUserFromGuildRequest) -> Result<ResponseContent<BanUserFromGuildSuccess>, Error<BanUserFromGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/bans/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&ban_user_from_guild_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BanUserFromGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BanUserFromGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_ban_users_from_guild<'guild_id, 'bulk_ban_users_from_guild_request>(&self, guild_id: &'guild_id str, bulk_ban_users_from_guild_request: models::BulkBanUsersFromGuildRequest) -> Result<ResponseContent<BulkBanUsersFromGuildSuccess>, Error<BulkBanUsersFromGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/bulk-ban", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&bulk_ban_users_from_guild_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkBanUsersFromGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkBanUsersFromGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_delete_messages<'channel_id, 'bulk_delete_messages_request>(&self, channel_id: &'channel_id str, bulk_delete_messages_request: models::BulkDeleteMessagesRequest) -> Result<ResponseContent<BulkDeleteMessagesSuccess>, Error<BulkDeleteMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/bulk-delete", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&bulk_delete_messages_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkDeleteMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkDeleteMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_set_application_commands<'application_id, 'application_command_update_request>(&self, application_id: &'application_id str, application_command_update_request: Option<Vec<models::ApplicationCommandUpdateRequest>>) -> Result<ResponseContent<BulkSetApplicationCommandsSuccess>, Error<BulkSetApplicationCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/commands", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_command_update_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkSetApplicationCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkSetApplicationCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_set_guild_application_commands<'application_id, 'guild_id, 'application_command_update_request>(&self, application_id: &'application_id str, guild_id: &'guild_id str, application_command_update_request: Option<Vec<models::ApplicationCommandUpdateRequest>>) -> Result<ResponseContent<BulkSetGuildApplicationCommandsSuccess>, Error<BulkSetGuildApplicationCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_command_update_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkSetGuildApplicationCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkSetGuildApplicationCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_update_guild_channels<'guild_id, 'bulk_update_guild_channels_request_inner>(&self, guild_id: &'guild_id str, bulk_update_guild_channels_request_inner: Vec<models::BulkUpdateGuildChannelsRequestInner>) -> Result<ResponseContent<BulkUpdateGuildChannelsSuccess>, Error<BulkUpdateGuildChannelsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/channels", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&bulk_update_guild_channels_request_inner);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkUpdateGuildChannelsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkUpdateGuildChannelsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_update_guild_roles<'guild_id, 'bulk_update_guild_roles_request_inner>(&self, guild_id: &'guild_id str, bulk_update_guild_roles_request_inner: Vec<models::BulkUpdateGuildRolesRequestInner>) -> Result<ResponseContent<BulkUpdateGuildRolesSuccess>, Error<BulkUpdateGuildRolesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/roles", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&bulk_update_guild_roles_request_inner);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkUpdateGuildRolesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkUpdateGuildRolesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn bulk_update_lobby_members<'lobby_id, 'bulk_lobby_member_request>(&self, lobby_id: &'lobby_id str, bulk_lobby_member_request: Option<Vec<models::BulkLobbyMemberRequest>>) -> Result<ResponseContent<BulkUpdateLobbyMembersSuccess>, Error<BulkUpdateLobbyMembersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/members/bulk", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&bulk_lobby_member_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<BulkUpdateLobbyMembersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<BulkUpdateLobbyMembersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn consume_entitlement<'application_id, 'entitlement_id>(&self, application_id: &'application_id str, entitlement_id: &'entitlement_id str) -> Result<ResponseContent<ConsumeEntitlementSuccess>, Error<ConsumeEntitlementError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/entitlements/{entitlement_id}/consume", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), entitlement_id=crate::apis::urlencode(entitlement_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ConsumeEntitlementSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ConsumeEntitlementError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_application_command<'application_id, 'application_command_create_request>(&self, application_id: &'application_id str, application_command_create_request: models::ApplicationCommandCreateRequest) -> Result<ResponseContent<CreateApplicationCommandSuccess>, Error<CreateApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/commands", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_command_create_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_application_emoji<'application_id, 'create_application_emoji_request>(&self, application_id: &'application_id str, create_application_emoji_request: models::CreateApplicationEmojiRequest) -> Result<ResponseContent<CreateApplicationEmojiSuccess>, Error<CreateApplicationEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/emojis", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_application_emoji_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateApplicationEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateApplicationEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_auto_moderation_rule<'guild_id, 'create_auto_moderation_rule_request>(&self, guild_id: &'guild_id str, create_auto_moderation_rule_request: models::CreateAutoModerationRuleRequest) -> Result<ResponseContent<CreateAutoModerationRuleSuccess>, Error<CreateAutoModerationRuleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_auto_moderation_rule_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateAutoModerationRuleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateAutoModerationRuleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_channel_invite<'channel_id, 'create_channel_invite_request>(&self, channel_id: &'channel_id str, create_channel_invite_request: models::CreateChannelInviteRequest) -> Result<ResponseContent<CreateChannelInviteSuccess>, Error<CreateChannelInviteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/invites", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_channel_invite_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateChannelInviteSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateChannelInviteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_dm<'create_private_channel_request>(&self, create_private_channel_request: models::CreatePrivateChannelRequest) -> Result<ResponseContent<CreateDmSuccess>, Error<CreateDmError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/channels", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_private_channel_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateDmSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateDmError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_entitlement<'application_id, 'create_entitlement_request_data>(&self, application_id: &'application_id str, create_entitlement_request_data: models::CreateEntitlementRequestData) -> Result<ResponseContent<CreateEntitlementSuccess>, Error<CreateEntitlementError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/entitlements", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_entitlement_request_data);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateEntitlementSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateEntitlementError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild<'guild_create_request>(&self, guild_create_request: models::GuildCreateRequest) -> Result<ResponseContent<CreateGuildSuccess>, Error<CreateGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&guild_create_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_application_command<'application_id, 'guild_id, 'application_command_create_request>(&self, application_id: &'application_id str, guild_id: &'guild_id str, application_command_create_request: models::ApplicationCommandCreateRequest) -> Result<ResponseContent<CreateGuildApplicationCommandSuccess>, Error<CreateGuildApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_command_create_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_channel<'guild_id, 'create_guild_channel_request>(&self, guild_id: &'guild_id str, create_guild_channel_request: models::CreateGuildChannelRequest) -> Result<ResponseContent<CreateGuildChannelSuccess>, Error<CreateGuildChannelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/channels", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_channel_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildChannelSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildChannelError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_emoji<'guild_id, 'create_guild_emoji_request>(&self, guild_id: &'guild_id str, create_guild_emoji_request: models::CreateGuildEmojiRequest) -> Result<ResponseContent<CreateGuildEmojiSuccess>, Error<CreateGuildEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/emojis", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_emoji_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_from_template<'code, 'create_guild_from_template_request>(&self, code: &'code str, create_guild_from_template_request: models::CreateGuildFromTemplateRequest) -> Result<ResponseContent<CreateGuildFromTemplateSuccess>, Error<CreateGuildFromTemplateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/templates/{code}", local_var_configuration.base_path, code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_from_template_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildFromTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildFromTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_role<'guild_id, 'create_guild_role_request>(&self, guild_id: &'guild_id str, create_guild_role_request: models::CreateGuildRoleRequest) -> Result<ResponseContent<CreateGuildRoleSuccess>, Error<CreateGuildRoleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/roles", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_role_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildRoleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_scheduled_event<'guild_id, 'create_guild_scheduled_event_request>(&self, guild_id: &'guild_id str, create_guild_scheduled_event_request: models::CreateGuildScheduledEventRequest) -> Result<ResponseContent<CreateGuildScheduledEventSuccess>, Error<CreateGuildScheduledEventError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/scheduled-events", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_scheduled_event_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildScheduledEventSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildScheduledEventError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_soundboard_sound<'guild_id, 'soundboard_create_request>(&self, guild_id: &'guild_id str, soundboard_create_request: models::SoundboardCreateRequest) -> Result<ResponseContent<CreateGuildSoundboardSoundSuccess>, Error<CreateGuildSoundboardSoundError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&soundboard_create_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildSoundboardSoundSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildSoundboardSoundError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_sticker<'guild_id, 'name, 'tags, 'file, 'description>(&self, guild_id: &'guild_id str, name: &'name str, tags: &'tags str, file: &'file str, description: Option<&'description str>) -> Result<ResponseContent<CreateGuildStickerSuccess>, Error<CreateGuildStickerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/stickers", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("name", name.to_string());
        local_var_form = local_var_form.text("tags", tags.to_string());
        if let Some(local_var_param_value) = description {
            local_var_form = local_var_form.text("description", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("file", file.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildStickerSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildStickerError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_guild_template<'guild_id, 'create_guild_template_request>(&self, guild_id: &'guild_id str, create_guild_template_request: models::CreateGuildTemplateRequest) -> Result<ResponseContent<CreateGuildTemplateSuccess>, Error<CreateGuildTemplateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/templates", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_template_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateGuildTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateGuildTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_interaction_response<'interaction_id, 'interaction_token, 'create_interaction_response_request, 'with_response>(&self, interaction_id: &'interaction_id str, interaction_token: &'interaction_token str, create_interaction_response_request: models::CreateInteractionResponseRequest, with_response: Option<bool>) -> Result<ResponseContent<CreateInteractionResponseSuccess>, Error<CreateInteractionResponseError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/interactions/{interaction_id}/{interaction_token}/callback", local_var_configuration.base_path, interaction_id=crate::apis::urlencode(interaction_id), interaction_token=crate::apis::urlencode(interaction_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_response {
            local_var_req_builder = local_var_req_builder.query(&[("with_response", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_interaction_response_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateInteractionResponseSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateInteractionResponseError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_lobby<'create_lobby_request>(&self, create_lobby_request: models::CreateLobbyRequest) -> Result<ResponseContent<CreateLobbySuccess>, Error<CreateLobbyError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_lobby_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateLobbySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateLobbyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_lobby_message<'lobby_id, 'sdk_message_request>(&self, lobby_id: &'lobby_id str, sdk_message_request: models::SdkMessageRequest) -> Result<ResponseContent<CreateLobbyMessageSuccess>, Error<CreateLobbyMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/messages", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&sdk_message_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateLobbyMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateLobbyMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_message<'channel_id, 'message_create_request>(&self, channel_id: &'channel_id str, message_create_request: models::MessageCreateRequest) -> Result<ResponseContent<CreateMessageSuccess>, Error<CreateMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&message_create_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_or_join_lobby<'create_or_join_lobby_request>(&self, create_or_join_lobby_request: models::CreateOrJoinLobbyRequest) -> Result<ResponseContent<CreateOrJoinLobbySuccess>, Error<CreateOrJoinLobbyError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_or_join_lobby_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateOrJoinLobbySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateOrJoinLobbyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<CreatePinSuccess>, Error<CreatePinError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/pins/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreatePinSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreatePinError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_stage_instance<'create_stage_instance_request>(&self, create_stage_instance_request: models::CreateStageInstanceRequest) -> Result<ResponseContent<CreateStageInstanceSuccess>, Error<CreateStageInstanceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stage-instances", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_stage_instance_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateStageInstanceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateStageInstanceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_thread<'channel_id, 'create_thread_request>(&self, channel_id: &'channel_id str, create_thread_request: models::CreateThreadRequest) -> Result<ResponseContent<CreateThreadSuccess>, Error<CreateThreadError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/threads", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_thread_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateThreadSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateThreadError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_thread_from_message<'channel_id, 'message_id, 'create_text_thread_with_message_request>(&self, channel_id: &'channel_id str, message_id: &'message_id str, create_text_thread_with_message_request: models::CreateTextThreadWithMessageRequest) -> Result<ResponseContent<CreateThreadFromMessageSuccess>, Error<CreateThreadFromMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/threads", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_text_thread_with_message_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateThreadFromMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateThreadFromMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn create_webhook<'channel_id, 'create_webhook_request>(&self, channel_id: &'channel_id str, create_webhook_request: models::CreateWebhookRequest) -> Result<ResponseContent<CreateWebhookSuccess>, Error<CreateWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/webhooks", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_webhook_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CreateWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CreateWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn crosspost_message<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<CrosspostMessageSuccess>, Error<CrosspostMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/crosspost", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<CrosspostMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<CrosspostMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_all_message_reactions<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeleteAllMessageReactionsSuccess>, Error<DeleteAllMessageReactionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteAllMessageReactionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteAllMessageReactionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_all_message_reactions_by_emoji<'channel_id, 'message_id, 'emoji_name>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str) -> Result<ResponseContent<DeleteAllMessageReactionsByEmojiSuccess>, Error<DeleteAllMessageReactionsByEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id), emoji_name=crate::apis::urlencode(emoji_name));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteAllMessageReactionsByEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteAllMessageReactionsByEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_application_command<'application_id, 'command_id>(&self, application_id: &'application_id str, command_id: &'command_id str) -> Result<ResponseContent<DeleteApplicationCommandSuccess>, Error<DeleteApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/commands/{command_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_application_emoji<'application_id, 'emoji_id>(&self, application_id: &'application_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<DeleteApplicationEmojiSuccess>, Error<DeleteApplicationEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/emojis/{emoji_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), emoji_id=crate::apis::urlencode(emoji_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteApplicationEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteApplicationEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_application_user_role_connection<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<DeleteApplicationUserRoleConnectionSuccess>, Error<DeleteApplicationUserRoleConnectionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/applications/{application_id}/role-connection", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteApplicationUserRoleConnectionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteApplicationUserRoleConnectionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_auto_moderation_rule<'guild_id, 'rule_id>(&self, guild_id: &'guild_id str, rule_id: &'rule_id str) -> Result<ResponseContent<DeleteAutoModerationRuleSuccess>, Error<DeleteAutoModerationRuleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules/{rule_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), rule_id=crate::apis::urlencode(rule_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteAutoModerationRuleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteAutoModerationRuleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_channel<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<DeleteChannelSuccess>, Error<DeleteChannelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteChannelSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteChannelError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_channel_permission_overwrite<'channel_id, 'overwrite_id>(&self, channel_id: &'channel_id str, overwrite_id: &'overwrite_id str) -> Result<ResponseContent<DeleteChannelPermissionOverwriteSuccess>, Error<DeleteChannelPermissionOverwriteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/permissions/{overwrite_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), overwrite_id=crate::apis::urlencode(overwrite_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteChannelPermissionOverwriteSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteChannelPermissionOverwriteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_entitlement<'application_id, 'entitlement_id>(&self, application_id: &'application_id str, entitlement_id: &'entitlement_id str) -> Result<ResponseContent<DeleteEntitlementSuccess>, Error<DeleteEntitlementError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/entitlements/{entitlement_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), entitlement_id=crate::apis::urlencode(entitlement_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteEntitlementSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteEntitlementError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_group_dm_user<'channel_id, 'user_id>(&self, channel_id: &'channel_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteGroupDmUserSuccess>, Error<DeleteGroupDmUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/recipients/{user_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGroupDmUserSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGroupDmUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<DeleteGuildSuccess>, Error<DeleteGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_application_command<'application_id, 'guild_id, 'command_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str) -> Result<ResponseContent<DeleteGuildApplicationCommandSuccess>, Error<DeleteGuildApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_emoji<'guild_id, 'emoji_id>(&self, guild_id: &'guild_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<DeleteGuildEmojiSuccess>, Error<DeleteGuildEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/emojis/{emoji_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), emoji_id=crate::apis::urlencode(emoji_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_integration<'guild_id, 'integration_id>(&self, guild_id: &'guild_id str, integration_id: &'integration_id str) -> Result<ResponseContent<DeleteGuildIntegrationSuccess>, Error<DeleteGuildIntegrationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/integrations/{integration_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), integration_id=crate::apis::urlencode(integration_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildIntegrationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildIntegrationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_member<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteGuildMemberSuccess>, Error<DeleteGuildMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_member_role<'guild_id, 'user_id, 'role_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str, role_id: &'role_id str) -> Result<ResponseContent<DeleteGuildMemberRoleSuccess>, Error<DeleteGuildMemberRoleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/{user_id}/roles/{role_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id), role_id=crate::apis::urlencode(role_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildMemberRoleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildMemberRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_role<'guild_id, 'role_id>(&self, guild_id: &'guild_id str, role_id: &'role_id str) -> Result<ResponseContent<DeleteGuildRoleSuccess>, Error<DeleteGuildRoleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/roles/{role_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), role_id=crate::apis::urlencode(role_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildRoleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_scheduled_event<'guild_id, 'guild_scheduled_event_id>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str) -> Result<ResponseContent<DeleteGuildScheduledEventSuccess>, Error<DeleteGuildScheduledEventError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), guild_scheduled_event_id=crate::apis::urlencode(guild_scheduled_event_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildScheduledEventSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildScheduledEventError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_soundboard_sound<'guild_id, 'sound_id>(&self, guild_id: &'guild_id str, sound_id: &'sound_id str) -> Result<ResponseContent<DeleteGuildSoundboardSoundSuccess>, Error<DeleteGuildSoundboardSoundError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds/{sound_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), sound_id=crate::apis::urlencode(sound_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildSoundboardSoundSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildSoundboardSoundError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_sticker<'guild_id, 'sticker_id>(&self, guild_id: &'guild_id str, sticker_id: &'sticker_id str) -> Result<ResponseContent<DeleteGuildStickerSuccess>, Error<DeleteGuildStickerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/stickers/{sticker_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), sticker_id=crate::apis::urlencode(sticker_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildStickerSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildStickerError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_guild_template<'guild_id, 'code>(&self, guild_id: &'guild_id str, code: &'code str) -> Result<ResponseContent<DeleteGuildTemplateSuccess>, Error<DeleteGuildTemplateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/templates/{code}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteGuildTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteGuildTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_lobby_member<'lobby_id, 'user_id>(&self, lobby_id: &'lobby_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteLobbyMemberSuccess>, Error<DeleteLobbyMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/members/{user_id}", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteLobbyMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteLobbyMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_message<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeleteMessageSuccess>, Error<DeleteMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_my_message_reaction<'channel_id, 'message_id, 'emoji_name>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str) -> Result<ResponseContent<DeleteMyMessageReactionSuccess>, Error<DeleteMyMessageReactionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id), emoji_name=crate::apis::urlencode(emoji_name));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteMyMessageReactionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteMyMessageReactionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_original_webhook_message<'webhook_id, 'webhook_token, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<DeleteOriginalWebhookMessageSuccess>, Error<DeleteOriginalWebhookMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/@original", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteOriginalWebhookMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteOriginalWebhookMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeletePinSuccess>, Error<DeletePinError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/pins/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeletePinSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeletePinError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_stage_instance<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<DeleteStageInstanceSuccess>, Error<DeleteStageInstanceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stage-instances/{channel_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteStageInstanceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteStageInstanceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_thread_member<'channel_id, 'user_id>(&self, channel_id: &'channel_id str, user_id: &'user_id str) -> Result<ResponseContent<DeleteThreadMemberSuccess>, Error<DeleteThreadMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/thread-members/{user_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteThreadMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteThreadMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_user_message_reaction<'channel_id, 'message_id, 'emoji_name, 'user_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str, user_id: &'user_id str) -> Result<ResponseContent<DeleteUserMessageReactionSuccess>, Error<DeleteUserMessageReactionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/{user_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id), emoji_name=crate::apis::urlencode(emoji_name), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteUserMessageReactionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteUserMessageReactionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_webhook<'webhook_id>(&self, webhook_id: &'webhook_id str) -> Result<ResponseContent<DeleteWebhookSuccess>, Error<DeleteWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_webhook_by_token<'webhook_id, 'webhook_token>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str) -> Result<ResponseContent<DeleteWebhookByTokenSuccess>, Error<DeleteWebhookByTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteWebhookByTokenSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteWebhookByTokenError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_webhook_message<'webhook_id, 'webhook_token, 'message_id, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, message_id: &'message_id str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<DeleteWebhookMessageSuccess>, Error<DeleteWebhookMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeleteWebhookMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeleteWebhookMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn deprecated_create_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeprecatedCreatePinSuccess>, Error<DeprecatedCreatePinError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/pins/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeprecatedCreatePinSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeprecatedCreatePinError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn deprecated_delete_pin<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<DeprecatedDeletePinSuccess>, Error<DeprecatedDeletePinError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/pins/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeprecatedDeletePinSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeprecatedDeletePinError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn deprecated_list_pins<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<DeprecatedListPinsSuccess>, Error<DeprecatedListPinsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/pins", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<DeprecatedListPinsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<DeprecatedListPinsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn edit_lobby<'lobby_id, 'create_lobby_request>(&self, lobby_id: &'lobby_id str, create_lobby_request: models::CreateLobbyRequest) -> Result<ResponseContent<EditLobbySuccess>, Error<EditLobbyError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_lobby_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<EditLobbySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditLobbyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn edit_lobby_channel_link<'lobby_id, 'edit_lobby_channel_link_request>(&self, lobby_id: &'lobby_id str, edit_lobby_channel_link_request: models::EditLobbyChannelLinkRequest) -> Result<ResponseContent<EditLobbyChannelLinkSuccess>, Error<EditLobbyChannelLinkError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/channel-linking", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&edit_lobby_channel_link_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<EditLobbyChannelLinkSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<EditLobbyChannelLinkError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn execute_github_compatible_webhook<'webhook_id, 'webhook_token, 'github_webhook, 'wait, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, github_webhook: models::GithubWebhook, wait: Option<bool>, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<ExecuteGithubCompatibleWebhookSuccess>, Error<ExecuteGithubCompatibleWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/github", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref param_value) = wait {
            local_var_req_builder = local_var_req_builder.query(&[("wait", &param_value.to_string())]);
        }
        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&github_webhook);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ExecuteGithubCompatibleWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ExecuteGithubCompatibleWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn execute_slack_compatible_webhook<'webhook_id, 'webhook_token, 'slack_webhook, 'wait, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, slack_webhook: models::SlackWebhook, wait: Option<bool>, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<ExecuteSlackCompatibleWebhookSuccess>, Error<ExecuteSlackCompatibleWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/slack", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref param_value) = wait {
            local_var_req_builder = local_var_req_builder.query(&[("wait", &param_value.to_string())]);
        }
        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&slack_webhook);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ExecuteSlackCompatibleWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ExecuteSlackCompatibleWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn execute_webhook<'webhook_id, 'webhook_token, 'execute_webhook_request, 'wait, 'thread_id, 'with_components>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, execute_webhook_request: models::ExecuteWebhookRequest, wait: Option<bool>, thread_id: Option<&'thread_id str>, with_components: Option<bool>) -> Result<ResponseContent<ExecuteWebhookSuccess>, Error<ExecuteWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref param_value) = wait {
            local_var_req_builder = local_var_req_builder.query(&[("wait", &param_value.to_string())]);
        }
        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = with_components {
            local_var_req_builder = local_var_req_builder.query(&[("with_components", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&execute_webhook_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ExecuteWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ExecuteWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn follow_channel<'channel_id, 'follow_channel_request>(&self, channel_id: &'channel_id str, follow_channel_request: models::FollowChannelRequest) -> Result<ResponseContent<FollowChannelSuccess>, Error<FollowChannelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/followers", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&follow_channel_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<FollowChannelSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<FollowChannelError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_active_guild_threads<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetActiveGuildThreadsSuccess>, Error<GetActiveGuildThreadsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/threads/active", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetActiveGuildThreadsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetActiveGuildThreadsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_answer_voters<'channel_id, 'message_id, 'answer_id, 'after, 'limit>(&self, channel_id: &'channel_id str, message_id: &'message_id str, answer_id: i32, after: Option<&'after str>, limit: Option<i32>) -> Result<ResponseContent<GetAnswerVotersSuccess>, Error<GetAnswerVotersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/polls/{message_id}/answers/{answer_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id), answer_id=answer_id);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetAnswerVotersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetAnswerVotersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_application<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<GetApplicationSuccess>, Error<GetApplicationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetApplicationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetApplicationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_application_command<'application_id, 'command_id>(&self, application_id: &'application_id str, command_id: &'command_id str) -> Result<ResponseContent<GetApplicationCommandSuccess>, Error<GetApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/commands/{command_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_application_emoji<'application_id, 'emoji_id>(&self, application_id: &'application_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<GetApplicationEmojiSuccess>, Error<GetApplicationEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/emojis/{emoji_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), emoji_id=crate::apis::urlencode(emoji_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetApplicationEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetApplicationEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_application_role_connections_metadata<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<GetApplicationRoleConnectionsMetadataSuccess>, Error<GetApplicationRoleConnectionsMetadataError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/role-connections/metadata", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetApplicationRoleConnectionsMetadataSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetApplicationRoleConnectionsMetadataError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_application_user_role_connection<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<GetApplicationUserRoleConnectionSuccess>, Error<GetApplicationUserRoleConnectionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/applications/{application_id}/role-connection", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetApplicationUserRoleConnectionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetApplicationUserRoleConnectionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_auto_moderation_rule<'guild_id, 'rule_id>(&self, guild_id: &'guild_id str, rule_id: &'rule_id str) -> Result<ResponseContent<GetAutoModerationRuleSuccess>, Error<GetAutoModerationRuleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules/{rule_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), rule_id=crate::apis::urlencode(rule_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetAutoModerationRuleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetAutoModerationRuleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_bot_gateway<>(&self, ) -> Result<ResponseContent<GetBotGatewaySuccess>, Error<GetBotGatewayError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gateway/bot", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetBotGatewaySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetBotGatewayError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_channel<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<GetChannelSuccess>, Error<GetChannelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetChannelSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetChannelError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_entitlement<'application_id, 'entitlement_id>(&self, application_id: &'application_id str, entitlement_id: &'entitlement_id str) -> Result<ResponseContent<GetEntitlementSuccess>, Error<GetEntitlementError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/entitlements/{entitlement_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), entitlement_id=crate::apis::urlencode(entitlement_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetEntitlementSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetEntitlementError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_entitlements<'sku_ids, 'application_id, 'user_id, 'guild_id, 'before, 'after, 'limit, 'exclude_ended, 'exclude_deleted, 'only_active>(&self, sku_ids: &'sku_ids str, application_id: &'application_id str, user_id: Option<&'user_id str>, guild_id: Option<&'guild_id str>, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>, exclude_ended: Option<bool>, exclude_deleted: Option<bool>, only_active: Option<bool>) -> Result<ResponseContent<GetEntitlementsSuccess>, Error<GetEntitlementsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/entitlements", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = user_id {
            local_var_req_builder = local_var_req_builder.query(&[("user_id", &param_value.to_string())]);
        }
        local_var_req_builder = local_var_req_builder.query(&[("sku_ids", &sku_ids.to_string())]);
        if let Some(ref param_value) = guild_id {
            local_var_req_builder = local_var_req_builder.query(&[("guild_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = exclude_ended {
            local_var_req_builder = local_var_req_builder.query(&[("exclude_ended", &param_value.to_string())]);
        }
        if let Some(ref param_value) = exclude_deleted {
            local_var_req_builder = local_var_req_builder.query(&[("exclude_deleted", &param_value.to_string())]);
        }
        if let Some(ref param_value) = only_active {
            local_var_req_builder = local_var_req_builder.query(&[("only_active", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetEntitlementsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetEntitlementsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_gateway<>(&self, ) -> Result<ResponseContent<GetGatewaySuccess>, Error<GetGatewayError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/gateway", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGatewaySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGatewayError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild<'guild_id, 'with_counts>(&self, guild_id: &'guild_id str, with_counts: Option<bool>) -> Result<ResponseContent<GetGuildSuccess>, Error<GetGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_counts {
            local_var_req_builder = local_var_req_builder.query(&[("with_counts", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_application_command<'application_id, 'guild_id, 'command_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str) -> Result<ResponseContent<GetGuildApplicationCommandSuccess>, Error<GetGuildApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_application_command_permissions<'application_id, 'guild_id, 'command_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str) -> Result<ResponseContent<GetGuildApplicationCommandPermissionsSuccess>, Error<GetGuildApplicationCommandPermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildApplicationCommandPermissionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildApplicationCommandPermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_ban<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<GetGuildBanSuccess>, Error<GetGuildBanError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/bans/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildBanSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildBanError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_emoji<'guild_id, 'emoji_id>(&self, guild_id: &'guild_id str, emoji_id: &'emoji_id str) -> Result<ResponseContent<GetGuildEmojiSuccess>, Error<GetGuildEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/emojis/{emoji_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), emoji_id=crate::apis::urlencode(emoji_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_member<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<GetGuildMemberSuccess>, Error<GetGuildMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_new_member_welcome<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildNewMemberWelcomeSuccess>, Error<GetGuildNewMemberWelcomeError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/new-member-welcome", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildNewMemberWelcomeSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildNewMemberWelcomeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_preview<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildPreviewSuccess>, Error<GetGuildPreviewError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/preview", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildPreviewSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildPreviewError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_role<'guild_id, 'role_id>(&self, guild_id: &'guild_id str, role_id: &'role_id str) -> Result<ResponseContent<GetGuildRoleSuccess>, Error<GetGuildRoleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/roles/{role_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), role_id=crate::apis::urlencode(role_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildRoleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_scheduled_event<'guild_id, 'guild_scheduled_event_id, 'with_user_count>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str, with_user_count: Option<bool>) -> Result<ResponseContent<GetGuildScheduledEventSuccess>, Error<GetGuildScheduledEventError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), guild_scheduled_event_id=crate::apis::urlencode(guild_scheduled_event_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_user_count {
            local_var_req_builder = local_var_req_builder.query(&[("with_user_count", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildScheduledEventSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildScheduledEventError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_soundboard_sound<'guild_id, 'sound_id>(&self, guild_id: &'guild_id str, sound_id: &'sound_id str) -> Result<ResponseContent<GetGuildSoundboardSoundSuccess>, Error<GetGuildSoundboardSoundError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds/{sound_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), sound_id=crate::apis::urlencode(sound_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildSoundboardSoundSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildSoundboardSoundError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_sticker<'guild_id, 'sticker_id>(&self, guild_id: &'guild_id str, sticker_id: &'sticker_id str) -> Result<ResponseContent<GetGuildStickerSuccess>, Error<GetGuildStickerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/stickers/{sticker_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), sticker_id=crate::apis::urlencode(sticker_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildStickerSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildStickerError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_template<'code>(&self, code: &'code str) -> Result<ResponseContent<GetGuildTemplateSuccess>, Error<GetGuildTemplateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/templates/{code}", local_var_configuration.base_path, code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_vanity_url<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildVanityUrlSuccess>, Error<GetGuildVanityUrlError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/vanity-url", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildVanityUrlSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildVanityUrlError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_webhooks<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWebhooksSuccess>, Error<GetGuildWebhooksError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/webhooks", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildWebhooksSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildWebhooksError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_welcome_screen<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWelcomeScreenSuccess>, Error<GetGuildWelcomeScreenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/welcome-screen", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildWelcomeScreenSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildWelcomeScreenError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_widget<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWidgetSuccess>, Error<GetGuildWidgetError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/widget.json", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildWidgetSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildWidgetError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_widget_png<'guild_id, 'style>(&self, guild_id: &'guild_id str, style: Option<&'style str>) -> Result<ResponseContent<GetGuildWidgetPngSuccess>, Error<GetGuildWidgetPngError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/widget.png", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = style {
            local_var_req_builder = local_var_req_builder.query(&[("style", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildWidgetPngSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildWidgetPngError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guild_widget_settings<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildWidgetSettingsSuccess>, Error<GetGuildWidgetSettingsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/widget", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildWidgetSettingsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildWidgetSettingsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_guilds_onboarding<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetGuildsOnboardingSuccess>, Error<GetGuildsOnboardingError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/onboarding", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetGuildsOnboardingSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetGuildsOnboardingError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_lobby<'lobby_id>(&self, lobby_id: &'lobby_id str) -> Result<ResponseContent<GetLobbySuccess>, Error<GetLobbyError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetLobbySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetLobbyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_lobby_messages<'lobby_id, 'limit>(&self, lobby_id: &'lobby_id str, limit: Option<i32>) -> Result<ResponseContent<GetLobbyMessagesSuccess>, Error<GetLobbyMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/messages", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetLobbyMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetLobbyMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_message<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<GetMessageSuccess>, Error<GetMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_my_application<>(&self, ) -> Result<ResponseContent<GetMyApplicationSuccess>, Error<GetMyApplicationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/@me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetMyApplicationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyApplicationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_my_guild_member<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetMyGuildMemberSuccess>, Error<GetMyGuildMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/guilds/{guild_id}/member", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetMyGuildMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyGuildMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_my_oauth2_application<>(&self, ) -> Result<ResponseContent<GetMyOauth2ApplicationSuccess>, Error<GetMyOauth2ApplicationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/oauth2/applications/@me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetMyOauth2ApplicationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyOauth2ApplicationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_my_oauth2_authorization<>(&self, ) -> Result<ResponseContent<GetMyOauth2AuthorizationSuccess>, Error<GetMyOauth2AuthorizationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/oauth2/@me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetMyOauth2AuthorizationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyOauth2AuthorizationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_my_user<>(&self, ) -> Result<ResponseContent<GetMyUserSuccess>, Error<GetMyUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetMyUserSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetMyUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_openid_connect_userinfo<>(&self, ) -> Result<ResponseContent<GetOpenidConnectUserinfoSuccess>, Error<GetOpenidConnectUserinfoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/oauth2/userinfo", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetOpenidConnectUserinfoSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetOpenidConnectUserinfoError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_original_webhook_message<'webhook_id, 'webhook_token, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<GetOriginalWebhookMessageSuccess>, Error<GetOriginalWebhookMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/@original", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetOriginalWebhookMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetOriginalWebhookMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_public_keys<>(&self, ) -> Result<ResponseContent<GetPublicKeysSuccess>, Error<GetPublicKeysError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/oauth2/keys", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetPublicKeysSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetPublicKeysError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_self_voice_state<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<GetSelfVoiceStateSuccess>, Error<GetSelfVoiceStateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/voice-states/@me", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetSelfVoiceStateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetSelfVoiceStateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_soundboard_default_sounds<>(&self, ) -> Result<ResponseContent<GetSoundboardDefaultSoundsSuccess>, Error<GetSoundboardDefaultSoundsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/soundboard-default-sounds", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetSoundboardDefaultSoundsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetSoundboardDefaultSoundsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_stage_instance<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<GetStageInstanceSuccess>, Error<GetStageInstanceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stage-instances/{channel_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetStageInstanceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetStageInstanceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sticker<'sticker_id>(&self, sticker_id: &'sticker_id str) -> Result<ResponseContent<GetStickerSuccess>, Error<GetStickerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stickers/{sticker_id}", local_var_configuration.base_path, sticker_id=crate::apis::urlencode(sticker_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetStickerSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetStickerError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sticker_pack<'pack_id>(&self, pack_id: &'pack_id str) -> Result<ResponseContent<GetStickerPackSuccess>, Error<GetStickerPackError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sticker-packs/{pack_id}", local_var_configuration.base_path, pack_id=crate::apis::urlencode(pack_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetStickerPackSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetStickerPackError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_thread_member<'channel_id, 'user_id, 'with_member>(&self, channel_id: &'channel_id str, user_id: &'user_id str, with_member: Option<bool>) -> Result<ResponseContent<GetThreadMemberSuccess>, Error<GetThreadMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/thread-members/{user_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_member {
            local_var_req_builder = local_var_req_builder.query(&[("with_member", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetThreadMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetThreadMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_user<'user_id>(&self, user_id: &'user_id str) -> Result<ResponseContent<GetUserSuccess>, Error<GetUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/{user_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetUserSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_voice_state<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<GetVoiceStateSuccess>, Error<GetVoiceStateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/voice-states/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetVoiceStateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetVoiceStateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_webhook<'webhook_id>(&self, webhook_id: &'webhook_id str) -> Result<ResponseContent<GetWebhookSuccess>, Error<GetWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_webhook_by_token<'webhook_id, 'webhook_token>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str) -> Result<ResponseContent<GetWebhookByTokenSuccess>, Error<GetWebhookByTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetWebhookByTokenSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetWebhookByTokenError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_webhook_message<'webhook_id, 'webhook_token, 'message_id, 'thread_id>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, message_id: &'message_id str, thread_id: Option<&'thread_id str>) -> Result<ResponseContent<GetWebhookMessageSuccess>, Error<GetWebhookMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<GetWebhookMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetWebhookMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn invite_resolve<'code, 'with_counts, 'guild_scheduled_event_id>(&self, code: &'code str, with_counts: Option<bool>, guild_scheduled_event_id: Option<&'guild_scheduled_event_id str>) -> Result<ResponseContent<InviteResolveSuccess>, Error<InviteResolveError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/invites/{code}", local_var_configuration.base_path, code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_counts {
            local_var_req_builder = local_var_req_builder.query(&[("with_counts", &param_value.to_string())]);
        }
        if let Some(ref param_value) = guild_scheduled_event_id {
            local_var_req_builder = local_var_req_builder.query(&[("guild_scheduled_event_id", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<InviteResolveSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<InviteResolveError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn invite_revoke<'code>(&self, code: &'code str) -> Result<ResponseContent<InviteRevokeSuccess>, Error<InviteRevokeError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/invites/{code}", local_var_configuration.base_path, code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<InviteRevokeSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<InviteRevokeError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn join_thread<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<JoinThreadSuccess>, Error<JoinThreadError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/thread-members/@me", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<JoinThreadSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<JoinThreadError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn leave_guild<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<LeaveGuildSuccess>, Error<LeaveGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/guilds/{guild_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<LeaveGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<LeaveGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn leave_lobby<'lobby_id>(&self, lobby_id: &'lobby_id str) -> Result<ResponseContent<LeaveLobbySuccess>, Error<LeaveLobbyError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/lobbies/{lobby_id}/members/@me", local_var_configuration.base_path, lobby_id=crate::apis::urlencode(lobby_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<LeaveLobbySuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<LeaveLobbyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn leave_thread<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<LeaveThreadSuccess>, Error<LeaveThreadError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/thread-members/@me", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<LeaveThreadSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<LeaveThreadError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_application_commands<'application_id, 'with_localizations>(&self, application_id: &'application_id str, with_localizations: Option<bool>) -> Result<ResponseContent<ListApplicationCommandsSuccess>, Error<ListApplicationCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/commands", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_localizations {
            local_var_req_builder = local_var_req_builder.query(&[("with_localizations", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListApplicationCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListApplicationCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_application_emojis<'application_id>(&self, application_id: &'application_id str) -> Result<ResponseContent<ListApplicationEmojisSuccess>, Error<ListApplicationEmojisError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/emojis", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListApplicationEmojisSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListApplicationEmojisError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_auto_moderation_rules<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListAutoModerationRulesSuccess>, Error<ListAutoModerationRulesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListAutoModerationRulesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListAutoModerationRulesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_channel_invites<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<ListChannelInvitesSuccess>, Error<ListChannelInvitesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/invites", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListChannelInvitesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListChannelInvitesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_channel_webhooks<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<ListChannelWebhooksSuccess>, Error<ListChannelWebhooksError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/webhooks", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListChannelWebhooksSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListChannelWebhooksError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_application_command_permissions<'application_id, 'guild_id>(&self, application_id: &'application_id str, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildApplicationCommandPermissionsSuccess>, Error<ListGuildApplicationCommandPermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/permissions", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildApplicationCommandPermissionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildApplicationCommandPermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_application_commands<'application_id, 'guild_id, 'with_localizations>(&self, application_id: &'application_id str, guild_id: &'guild_id str, with_localizations: Option<bool>) -> Result<ResponseContent<ListGuildApplicationCommandsSuccess>, Error<ListGuildApplicationCommandsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_localizations {
            local_var_req_builder = local_var_req_builder.query(&[("with_localizations", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildApplicationCommandsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildApplicationCommandsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_audit_log_entries<'guild_id, 'user_id, 'target_id, 'action_type, 'before, 'after, 'limit>(&self, guild_id: &'guild_id str, user_id: Option<&'user_id str>, target_id: Option<&'target_id str>, action_type: Option<i32>, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>) -> Result<ResponseContent<ListGuildAuditLogEntriesSuccess>, Error<ListGuildAuditLogEntriesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/audit-logs", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = user_id {
            local_var_req_builder = local_var_req_builder.query(&[("user_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = target_id {
            local_var_req_builder = local_var_req_builder.query(&[("target_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = action_type {
            local_var_req_builder = local_var_req_builder.query(&[("action_type", &param_value.to_string())]);
        }
        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildAuditLogEntriesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildAuditLogEntriesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_bans<'guild_id, 'limit, 'before, 'after>(&self, guild_id: &'guild_id str, limit: Option<i32>, before: Option<&'before str>, after: Option<&'after str>) -> Result<ResponseContent<ListGuildBansSuccess>, Error<ListGuildBansError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/bans", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildBansSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildBansError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_channels<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildChannelsSuccess>, Error<ListGuildChannelsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/channels", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildChannelsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildChannelsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_emojis<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildEmojisSuccess>, Error<ListGuildEmojisError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/emojis", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildEmojisSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildEmojisError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_integrations<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildIntegrationsSuccess>, Error<ListGuildIntegrationsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/integrations", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildIntegrationsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildIntegrationsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_invites<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildInvitesSuccess>, Error<ListGuildInvitesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/invites", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildInvitesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildInvitesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_members<'guild_id, 'limit, 'after>(&self, guild_id: &'guild_id str, limit: Option<i32>, after: Option<i32>) -> Result<ResponseContent<ListGuildMembersSuccess>, Error<ListGuildMembersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildMembersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildMembersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_roles<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildRolesSuccess>, Error<ListGuildRolesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/roles", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildRolesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildRolesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_scheduled_event_users<'guild_id, 'guild_scheduled_event_id, 'with_member, 'limit, 'before, 'after>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str, with_member: Option<bool>, limit: Option<i32>, before: Option<&'before str>, after: Option<&'after str>) -> Result<ResponseContent<ListGuildScheduledEventUsersSuccess>, Error<ListGuildScheduledEventUsersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), guild_scheduled_event_id=crate::apis::urlencode(guild_scheduled_event_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_member {
            local_var_req_builder = local_var_req_builder.query(&[("with_member", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildScheduledEventUsersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildScheduledEventUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_scheduled_events<'guild_id, 'with_user_count>(&self, guild_id: &'guild_id str, with_user_count: Option<bool>) -> Result<ResponseContent<ListGuildScheduledEventsSuccess>, Error<ListGuildScheduledEventsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/scheduled-events", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_user_count {
            local_var_req_builder = local_var_req_builder.query(&[("with_user_count", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildScheduledEventsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildScheduledEventsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_soundboard_sounds<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildSoundboardSoundsSuccess>, Error<ListGuildSoundboardSoundsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildSoundboardSoundsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildSoundboardSoundsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_stickers<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildStickersSuccess>, Error<ListGuildStickersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/stickers", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildStickersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildStickersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_templates<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildTemplatesSuccess>, Error<ListGuildTemplatesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/templates", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildTemplatesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildTemplatesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_guild_voice_regions<'guild_id>(&self, guild_id: &'guild_id str) -> Result<ResponseContent<ListGuildVoiceRegionsSuccess>, Error<ListGuildVoiceRegionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/regions", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListGuildVoiceRegionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListGuildVoiceRegionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_message_reactions_by_emoji<'channel_id, 'message_id, 'emoji_name, 'after, 'limit, 'r_type>(&self, channel_id: &'channel_id str, message_id: &'message_id str, emoji_name: &'emoji_name str, after: Option<&'after str>, limit: Option<i32>, r#type: Option<i32>) -> Result<ResponseContent<ListMessageReactionsByEmojiSuccess>, Error<ListMessageReactionsByEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id), emoji_name=crate::apis::urlencode(emoji_name));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = r#type {
            local_var_req_builder = local_var_req_builder.query(&[("type", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListMessageReactionsByEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListMessageReactionsByEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_messages<'channel_id, 'around, 'before, 'after, 'limit>(&self, channel_id: &'channel_id str, around: Option<&'around str>, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>) -> Result<ResponseContent<ListMessagesSuccess>, Error<ListMessagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = around {
            local_var_req_builder = local_var_req_builder.query(&[("around", &param_value.to_string())]);
        }
        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListMessagesSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListMessagesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_my_connections<>(&self, ) -> Result<ResponseContent<ListMyConnectionsSuccess>, Error<ListMyConnectionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/connections", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListMyConnectionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListMyConnectionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_my_guilds<'before, 'after, 'limit, 'with_counts>(&self, before: Option<&'before str>, after: Option<&'after str>, limit: Option<i32>, with_counts: Option<bool>) -> Result<ResponseContent<ListMyGuildsSuccess>, Error<ListMyGuildsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/guilds", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = with_counts {
            local_var_req_builder = local_var_req_builder.query(&[("with_counts", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListMyGuildsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListMyGuildsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_my_private_archived_threads<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<&'before str>, limit: Option<i32>) -> Result<ResponseContent<ListMyPrivateArchivedThreadsSuccess>, Error<ListMyPrivateArchivedThreadsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/users/@me/threads/archived/private", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListMyPrivateArchivedThreadsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListMyPrivateArchivedThreadsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_pins<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPinsSuccess>, Error<ListPinsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/pins", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListPinsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListPinsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_private_archived_threads<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPrivateArchivedThreadsSuccess>, Error<ListPrivateArchivedThreadsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/threads/archived/private", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListPrivateArchivedThreadsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListPrivateArchivedThreadsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_public_archived_threads<'channel_id, 'before, 'limit>(&self, channel_id: &'channel_id str, before: Option<String>, limit: Option<i32>) -> Result<ResponseContent<ListPublicArchivedThreadsSuccess>, Error<ListPublicArchivedThreadsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/threads/archived/public", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = before {
            local_var_req_builder = local_var_req_builder.query(&[("before", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListPublicArchivedThreadsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListPublicArchivedThreadsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_sticker_packs<>(&self, ) -> Result<ResponseContent<ListStickerPacksSuccess>, Error<ListStickerPacksError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sticker-packs", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListStickerPacksSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListStickerPacksError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_thread_members<'channel_id, 'with_member, 'limit, 'after>(&self, channel_id: &'channel_id str, with_member: Option<bool>, limit: Option<i32>, after: Option<&'after str>) -> Result<ResponseContent<ListThreadMembersSuccess>, Error<ListThreadMembersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/thread-members", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = with_member {
            local_var_req_builder = local_var_req_builder.query(&[("with_member", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = after {
            local_var_req_builder = local_var_req_builder.query(&[("after", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListThreadMembersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListThreadMembersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn list_voice_regions<>(&self, ) -> Result<ResponseContent<ListVoiceRegionsSuccess>, Error<ListVoiceRegionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/voice/regions", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ListVoiceRegionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ListVoiceRegionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn partner_sdk_token<'partner_sdk_unmerge_provisional_account_request>(&self, partner_sdk_unmerge_provisional_account_request: models::PartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<PartnerSdkTokenSuccess>, Error<PartnerSdkTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/partner-sdk/token", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&partner_sdk_unmerge_provisional_account_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PartnerSdkTokenSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PartnerSdkTokenError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn partner_sdk_unmerge_provisional_account<'partner_sdk_unmerge_provisional_account_request>(&self, partner_sdk_unmerge_provisional_account_request: models::PartnerSdkUnmergeProvisionalAccountRequest) -> Result<ResponseContent<PartnerSdkUnmergeProvisionalAccountSuccess>, Error<PartnerSdkUnmergeProvisionalAccountError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/partner-sdk/provisional-accounts/unmerge", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&partner_sdk_unmerge_provisional_account_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PartnerSdkUnmergeProvisionalAccountSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PartnerSdkUnmergeProvisionalAccountError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn poll_expire<'channel_id, 'message_id>(&self, channel_id: &'channel_id str, message_id: &'message_id str) -> Result<ResponseContent<PollExpireSuccess>, Error<PollExpireError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/polls/{message_id}/expire", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PollExpireSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PollExpireError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn preview_prune_guild<'guild_id, 'days, 'include_roles>(&self, guild_id: &'guild_id str, days: Option<i32>, include_roles: Option<&'include_roles str>) -> Result<ResponseContent<PreviewPruneGuildSuccess>, Error<PreviewPruneGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/prune", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = days {
            local_var_req_builder = local_var_req_builder.query(&[("days", &param_value.to_string())]);
        }
        if let Some(ref param_value) = include_roles {
            local_var_req_builder = local_var_req_builder.query(&[("include_roles", &serde_json::to_value(param_value)?)]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PreviewPruneGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PreviewPruneGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn prune_guild<'guild_id, 'prune_guild_request>(&self, guild_id: &'guild_id str, prune_guild_request: models::PruneGuildRequest) -> Result<ResponseContent<PruneGuildSuccess>, Error<PruneGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/prune", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&prune_guild_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PruneGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PruneGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn put_guilds_onboarding<'guild_id, 'update_guild_onboarding_request>(&self, guild_id: &'guild_id str, update_guild_onboarding_request: models::UpdateGuildOnboardingRequest) -> Result<ResponseContent<PutGuildsOnboardingSuccess>, Error<PutGuildsOnboardingError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/onboarding", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_onboarding_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<PutGuildsOnboardingSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<PutGuildsOnboardingError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn search_guild_members<'limit, 'query, 'guild_id>(&self, limit: i32, query: &'query str, guild_id: &'guild_id str) -> Result<ResponseContent<SearchGuildMembersSuccess>, Error<SearchGuildMembersError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/search", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("limit", &limit.to_string())]);
        local_var_req_builder = local_var_req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SearchGuildMembersSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SearchGuildMembersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn send_soundboard_sound<'channel_id, 'soundboard_sound_send_request>(&self, channel_id: &'channel_id str, soundboard_sound_send_request: models::SoundboardSoundSendRequest) -> Result<ResponseContent<SendSoundboardSoundSuccess>, Error<SendSoundboardSoundError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/send-soundboard-sound", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&soundboard_sound_send_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SendSoundboardSoundSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SendSoundboardSoundError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn set_channel_permission_overwrite<'channel_id, 'overwrite_id, 'set_channel_permission_overwrite_request>(&self, channel_id: &'channel_id str, overwrite_id: &'overwrite_id str, set_channel_permission_overwrite_request: models::SetChannelPermissionOverwriteRequest) -> Result<ResponseContent<SetChannelPermissionOverwriteSuccess>, Error<SetChannelPermissionOverwriteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/permissions/{overwrite_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), overwrite_id=crate::apis::urlencode(overwrite_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&set_channel_permission_overwrite_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SetChannelPermissionOverwriteSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetChannelPermissionOverwriteError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn set_guild_application_command_permissions<'application_id, 'guild_id, 'command_id, 'set_guild_application_command_permissions_request>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str, set_guild_application_command_permissions_request: models::SetGuildApplicationCommandPermissionsRequest) -> Result<ResponseContent<SetGuildApplicationCommandPermissionsSuccess>, Error<SetGuildApplicationCommandPermissionsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&set_guild_application_command_permissions_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SetGuildApplicationCommandPermissionsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetGuildApplicationCommandPermissionsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn set_guild_mfa_level<'guild_id, 'set_guild_mfa_level_request>(&self, guild_id: &'guild_id str, set_guild_mfa_level_request: models::SetGuildMfaLevelRequest) -> Result<ResponseContent<SetGuildMfaLevelSuccess>, Error<SetGuildMfaLevelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/mfa", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&set_guild_mfa_level_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SetGuildMfaLevelSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SetGuildMfaLevelError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn sync_guild_template<'guild_id, 'code>(&self, guild_id: &'guild_id str, code: &'code str) -> Result<ResponseContent<SyncGuildTemplateSuccess>, Error<SyncGuildTemplateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/templates/{code}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<SyncGuildTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<SyncGuildTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn thread_search<'channel_id, 'name, 'slop, 'min_id, 'max_id, 'tag, 'tag_setting, 'archived, 'sort_by, 'sort_order, 'limit, 'offset>(&self, channel_id: &'channel_id str, name: Option<&'name str>, slop: Option<i32>, min_id: Option<&'min_id str>, max_id: Option<&'max_id str>, tag: Option<&'tag str>, tag_setting: Option<&'tag_setting str>, archived: Option<bool>, sort_by: Option<&'sort_by str>, sort_order: Option<&'sort_order str>, limit: Option<i32>, offset: Option<i32>) -> Result<ResponseContent<ThreadSearchSuccess>, Error<ThreadSearchError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/threads/search", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref param_value) = name {
            local_var_req_builder = local_var_req_builder.query(&[("name", &param_value.to_string())]);
        }
        if let Some(ref param_value) = slop {
            local_var_req_builder = local_var_req_builder.query(&[("slop", &param_value.to_string())]);
        }
        if let Some(ref param_value) = min_id {
            local_var_req_builder = local_var_req_builder.query(&[("min_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = max_id {
            local_var_req_builder = local_var_req_builder.query(&[("max_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = tag {
            local_var_req_builder = local_var_req_builder.query(&[("tag", &serde_json::to_value(param_value)?)]);
        }
        if let Some(ref param_value) = tag_setting {
            local_var_req_builder = local_var_req_builder.query(&[("tag_setting", &param_value.to_string())]);
        }
        if let Some(ref param_value) = archived {
            local_var_req_builder = local_var_req_builder.query(&[("archived", &param_value.to_string())]);
        }
        if let Some(ref param_value) = sort_by {
            local_var_req_builder = local_var_req_builder.query(&[("sort_by", &param_value.to_string())]);
        }
        if let Some(ref param_value) = sort_order {
            local_var_req_builder = local_var_req_builder.query(&[("sort_order", &param_value.to_string())]);
        }
        if let Some(ref param_value) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = offset {
            local_var_req_builder = local_var_req_builder.query(&[("offset", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<ThreadSearchSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<ThreadSearchError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn trigger_typing_indicator<'channel_id>(&self, channel_id: &'channel_id str) -> Result<ResponseContent<TriggerTypingIndicatorSuccess>, Error<TriggerTypingIndicatorError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/typing", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<TriggerTypingIndicatorSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<TriggerTypingIndicatorError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn unban_user_from_guild<'guild_id, 'user_id>(&self, guild_id: &'guild_id str, user_id: &'user_id str) -> Result<ResponseContent<UnbanUserFromGuildSuccess>, Error<UnbanUserFromGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/bans/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UnbanUserFromGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UnbanUserFromGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_application<'application_id, 'application_form_partial>(&self, application_id: &'application_id str, application_form_partial: models::ApplicationFormPartial) -> Result<ResponseContent<UpdateApplicationSuccess>, Error<UpdateApplicationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_form_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateApplicationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateApplicationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_application_command<'application_id, 'command_id, 'application_command_patch_request_partial>(&self, application_id: &'application_id str, command_id: &'command_id str, application_command_patch_request_partial: models::ApplicationCommandPatchRequestPartial) -> Result<ResponseContent<UpdateApplicationCommandSuccess>, Error<UpdateApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/commands/{command_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_command_patch_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_application_emoji<'application_id, 'emoji_id, 'update_application_emoji_request>(&self, application_id: &'application_id str, emoji_id: &'emoji_id str, update_application_emoji_request: models::UpdateApplicationEmojiRequest) -> Result<ResponseContent<UpdateApplicationEmojiSuccess>, Error<UpdateApplicationEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/emojis/{emoji_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), emoji_id=crate::apis::urlencode(emoji_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_application_emoji_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateApplicationEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateApplicationEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_application_role_connections_metadata<'application_id, 'application_role_connections_metadata_item_request>(&self, application_id: &'application_id str, application_role_connections_metadata_item_request: Option<Vec<models::ApplicationRoleConnectionsMetadataItemRequest>>) -> Result<ResponseContent<UpdateApplicationRoleConnectionsMetadataSuccess>, Error<UpdateApplicationRoleConnectionsMetadataError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/role-connections/metadata", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_role_connections_metadata_item_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateApplicationRoleConnectionsMetadataSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateApplicationRoleConnectionsMetadataError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_application_user_role_connection<'application_id, 'update_application_user_role_connection_request>(&self, application_id: &'application_id str, update_application_user_role_connection_request: models::UpdateApplicationUserRoleConnectionRequest) -> Result<ResponseContent<UpdateApplicationUserRoleConnectionSuccess>, Error<UpdateApplicationUserRoleConnectionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me/applications/{application_id}/role-connection", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&update_application_user_role_connection_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateApplicationUserRoleConnectionSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateApplicationUserRoleConnectionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_auto_moderation_rule<'guild_id, 'rule_id, 'update_auto_moderation_rule_request>(&self, guild_id: &'guild_id str, rule_id: &'rule_id str, update_auto_moderation_rule_request: models::UpdateAutoModerationRuleRequest) -> Result<ResponseContent<UpdateAutoModerationRuleSuccess>, Error<UpdateAutoModerationRuleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/auto-moderation/rules/{rule_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), rule_id=crate::apis::urlencode(rule_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_auto_moderation_rule_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateAutoModerationRuleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateAutoModerationRuleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_channel<'channel_id, 'update_channel_request>(&self, channel_id: &'channel_id str, update_channel_request: models::UpdateChannelRequest) -> Result<ResponseContent<UpdateChannelSuccess>, Error<UpdateChannelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_channel_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateChannelSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateChannelError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild<'guild_id, 'guild_patch_request_partial>(&self, guild_id: &'guild_id str, guild_patch_request_partial: models::GuildPatchRequestPartial) -> Result<ResponseContent<UpdateGuildSuccess>, Error<UpdateGuildError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&guild_patch_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_application_command<'application_id, 'guild_id, 'command_id, 'application_command_patch_request_partial>(&self, application_id: &'application_id str, guild_id: &'guild_id str, command_id: &'command_id str, application_command_patch_request_partial: models::ApplicationCommandPatchRequestPartial) -> Result<ResponseContent<UpdateGuildApplicationCommandSuccess>, Error<UpdateGuildApplicationCommandError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id), guild_id=crate::apis::urlencode(guild_id), command_id=crate::apis::urlencode(command_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_command_patch_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildApplicationCommandSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildApplicationCommandError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_emoji<'guild_id, 'emoji_id, 'update_guild_emoji_request>(&self, guild_id: &'guild_id str, emoji_id: &'emoji_id str, update_guild_emoji_request: models::UpdateGuildEmojiRequest) -> Result<ResponseContent<UpdateGuildEmojiSuccess>, Error<UpdateGuildEmojiError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/emojis/{emoji_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), emoji_id=crate::apis::urlencode(emoji_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_emoji_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildEmojiSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildEmojiError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_member<'guild_id, 'user_id, 'update_guild_member_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, update_guild_member_request: models::UpdateGuildMemberRequest) -> Result<ResponseContent<UpdateGuildMemberSuccess>, Error<UpdateGuildMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_member_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_role<'guild_id, 'role_id, 'create_guild_role_request>(&self, guild_id: &'guild_id str, role_id: &'role_id str, create_guild_role_request: models::CreateGuildRoleRequest) -> Result<ResponseContent<UpdateGuildRoleSuccess>, Error<UpdateGuildRoleError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/roles/{role_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), role_id=crate::apis::urlencode(role_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&create_guild_role_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildRoleSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_scheduled_event<'guild_id, 'guild_scheduled_event_id, 'update_guild_scheduled_event_request>(&self, guild_id: &'guild_id str, guild_scheduled_event_id: &'guild_scheduled_event_id str, update_guild_scheduled_event_request: models::UpdateGuildScheduledEventRequest) -> Result<ResponseContent<UpdateGuildScheduledEventSuccess>, Error<UpdateGuildScheduledEventError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), guild_scheduled_event_id=crate::apis::urlencode(guild_scheduled_event_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_scheduled_event_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildScheduledEventSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildScheduledEventError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_soundboard_sound<'guild_id, 'sound_id, 'soundboard_patch_request_partial>(&self, guild_id: &'guild_id str, sound_id: &'sound_id str, soundboard_patch_request_partial: models::SoundboardPatchRequestPartial) -> Result<ResponseContent<UpdateGuildSoundboardSoundSuccess>, Error<UpdateGuildSoundboardSoundError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/soundboard-sounds/{sound_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), sound_id=crate::apis::urlencode(sound_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&soundboard_patch_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildSoundboardSoundSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildSoundboardSoundError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_sticker<'guild_id, 'sticker_id, 'update_guild_sticker_request>(&self, guild_id: &'guild_id str, sticker_id: &'sticker_id str, update_guild_sticker_request: models::UpdateGuildStickerRequest) -> Result<ResponseContent<UpdateGuildStickerSuccess>, Error<UpdateGuildStickerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/stickers/{sticker_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), sticker_id=crate::apis::urlencode(sticker_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_sticker_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildStickerSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildStickerError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_template<'guild_id, 'code, 'update_guild_template_request>(&self, guild_id: &'guild_id str, code: &'code str, update_guild_template_request: models::UpdateGuildTemplateRequest) -> Result<ResponseContent<UpdateGuildTemplateSuccess>, Error<UpdateGuildTemplateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/templates/{code}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), code=crate::apis::urlencode(code));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_template_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildTemplateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildTemplateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_welcome_screen<'guild_id, 'welcome_screen_patch_request_partial>(&self, guild_id: &'guild_id str, welcome_screen_patch_request_partial: models::WelcomeScreenPatchRequestPartial) -> Result<ResponseContent<UpdateGuildWelcomeScreenSuccess>, Error<UpdateGuildWelcomeScreenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/welcome-screen", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&welcome_screen_patch_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildWelcomeScreenSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildWelcomeScreenError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_guild_widget_settings<'guild_id, 'update_guild_widget_settings_request>(&self, guild_id: &'guild_id str, update_guild_widget_settings_request: models::UpdateGuildWidgetSettingsRequest) -> Result<ResponseContent<UpdateGuildWidgetSettingsSuccess>, Error<UpdateGuildWidgetSettingsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/widget", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_guild_widget_settings_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateGuildWidgetSettingsSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateGuildWidgetSettingsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_message<'channel_id, 'message_id, 'message_edit_request_partial>(&self, channel_id: &'channel_id str, message_id: &'message_id str, message_edit_request_partial: models::MessageEditRequestPartial) -> Result<ResponseContent<UpdateMessageSuccess>, Error<UpdateMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channels/{channel_id}/messages/{message_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&message_edit_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_my_application<'application_form_partial>(&self, application_form_partial: models::ApplicationFormPartial) -> Result<ResponseContent<UpdateMyApplicationSuccess>, Error<UpdateMyApplicationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/@me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&application_form_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateMyApplicationSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateMyApplicationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_my_guild_member<'guild_id, 'update_my_guild_member_request>(&self, guild_id: &'guild_id str, update_my_guild_member_request: models::UpdateMyGuildMemberRequest) -> Result<ResponseContent<UpdateMyGuildMemberSuccess>, Error<UpdateMyGuildMemberError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/members/@me", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_my_guild_member_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateMyGuildMemberSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateMyGuildMemberError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_my_user<'bot_account_patch_request>(&self, bot_account_patch_request: models::BotAccountPatchRequest) -> Result<ResponseContent<UpdateMyUserSuccess>, Error<UpdateMyUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/users/@me", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&bot_account_patch_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateMyUserSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateMyUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_original_webhook_message<'webhook_id, 'webhook_token, 'incoming_webhook_update_request_partial, 'thread_id, 'with_components>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, incoming_webhook_update_request_partial: models::IncomingWebhookUpdateRequestPartial, thread_id: Option<&'thread_id str>, with_components: Option<bool>) -> Result<ResponseContent<UpdateOriginalWebhookMessageSuccess>, Error<UpdateOriginalWebhookMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/@original", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = with_components {
            local_var_req_builder = local_var_req_builder.query(&[("with_components", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&incoming_webhook_update_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateOriginalWebhookMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateOriginalWebhookMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_self_voice_state<'guild_id, 'update_self_voice_state_request>(&self, guild_id: &'guild_id str, update_self_voice_state_request: models::UpdateSelfVoiceStateRequest) -> Result<ResponseContent<UpdateSelfVoiceStateSuccess>, Error<UpdateSelfVoiceStateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/voice-states/@me", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_self_voice_state_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateSelfVoiceStateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateSelfVoiceStateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_stage_instance<'channel_id, 'update_stage_instance_request>(&self, channel_id: &'channel_id str, update_stage_instance_request: models::UpdateStageInstanceRequest) -> Result<ResponseContent<UpdateStageInstanceSuccess>, Error<UpdateStageInstanceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/stage-instances/{channel_id}", local_var_configuration.base_path, channel_id=crate::apis::urlencode(channel_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_stage_instance_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateStageInstanceSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateStageInstanceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_voice_state<'guild_id, 'user_id, 'update_voice_state_request>(&self, guild_id: &'guild_id str, user_id: &'user_id str, update_voice_state_request: models::UpdateVoiceStateRequest) -> Result<ResponseContent<UpdateVoiceStateSuccess>, Error<UpdateVoiceStateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/guilds/{guild_id}/voice-states/{user_id}", local_var_configuration.base_path, guild_id=crate::apis::urlencode(guild_id), user_id=crate::apis::urlencode(user_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_voice_state_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateVoiceStateSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateVoiceStateError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_webhook<'webhook_id, 'update_webhook_request>(&self, webhook_id: &'webhook_id str, update_webhook_request: models::UpdateWebhookRequest) -> Result<ResponseContent<UpdateWebhookSuccess>, Error<UpdateWebhookError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_webhook_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateWebhookSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateWebhookError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_webhook_by_token<'webhook_id, 'webhook_token, 'update_webhook_by_token_request>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, update_webhook_by_token_request: models::UpdateWebhookByTokenRequest) -> Result<ResponseContent<UpdateWebhookByTokenSuccess>, Error<UpdateWebhookByTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&update_webhook_by_token_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateWebhookByTokenSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateWebhookByTokenError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_webhook_message<'webhook_id, 'webhook_token, 'message_id, 'incoming_webhook_update_request_partial, 'thread_id, 'with_components>(&self, webhook_id: &'webhook_id str, webhook_token: &'webhook_token str, message_id: &'message_id str, incoming_webhook_update_request_partial: models::IncomingWebhookUpdateRequestPartial, thread_id: Option<&'thread_id str>, with_components: Option<bool>) -> Result<ResponseContent<UpdateWebhookMessageSuccess>, Error<UpdateWebhookMessageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}", local_var_configuration.base_path, webhook_id=crate::apis::urlencode(webhook_id), webhook_token=crate::apis::urlencode(webhook_token), message_id=crate::apis::urlencode(message_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref param_value) = thread_id {
            local_var_req_builder = local_var_req_builder.query(&[("thread_id", &param_value.to_string())]);
        }
        if let Some(ref param_value) = with_components {
            local_var_req_builder = local_var_req_builder.query(&[("with_components", &param_value.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&incoming_webhook_update_request_partial);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UpdateWebhookMessageSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UpdateWebhookMessageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn upload_application_attachment<'application_id, 'file>(&self, application_id: &'application_id str, file: &'file str) -> Result<ResponseContent<UploadApplicationAttachmentSuccess>, Error<UploadApplicationAttachmentError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/applications/{application_id}/attachment", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
        };
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("file", file.to_string());
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            let local_var_entity: Option<UploadApplicationAttachmentSuccess> = serde_json::from_str(&local_var_content).ok();
            let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<UploadApplicationAttachmentError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

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
    Status200(models::ListChannelInvites200ResponseInner),
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

/// struct for typed successes of method [`create_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildSuccess {
    Status201(models::GuildResponse),
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

/// struct for typed successes of method [`create_guild_from_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildFromTemplateSuccess {
    Status201(models::GuildResponse),
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

/// struct for typed successes of method [`delete_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildSuccess {
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
    Status200(models::ListChannelInvites200ResponseInner),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`invite_revoke`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRevokeSuccess {
    Status200(models::ListChannelInvites200ResponseInner),
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

/// struct for typed successes of method [`set_guild_mfa_level`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGuildMfaLevelSuccess {
    Status200(models::GuildMfaLevelResponse),
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
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGuildMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_guild_member_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGuildMemberRoleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_lobby_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddLobbyMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_my_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddMyMessageReactionError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddThreadMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`applications_get_activity_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationsGetActivityInstanceError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ban_user_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanUserFromGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_ban_users_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkBanUsersFromGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_delete_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkDeleteMessagesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_set_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkSetApplicationCommandsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_set_guild_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkSetGuildApplicationCommandsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_guild_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateGuildChannelsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_guild_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateGuildRolesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_lobby_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateLobbyMembersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`consume_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConsumeEntitlementError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAutoModerationRuleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_channel_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChannelInviteError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dm`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEntitlementError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildChannelError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_from_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildFromTemplateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildRoleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildScheduledEventError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildSoundboardSoundError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildStickerError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGuildTemplateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_interaction_response`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInteractionResponseError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLobbyError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_lobby_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLobbyMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_or_join_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrJoinLobbyError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePinError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateStageInstanceError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_thread_from_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadFromMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`crosspost_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrosspostMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_all_message_reactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAllMessageReactionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_all_message_reactions_by_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAllMessageReactionsByEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationUserRoleConnectionError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAutoModerationRuleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChannelError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_channel_permission_overwrite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteChannelPermissionOverwriteError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEntitlementError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_group_dm_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGroupDmUserError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildIntegrationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_member_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildMemberRoleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildRoleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildScheduledEventError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildSoundboardSoundError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildStickerError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGuildTemplateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_lobby_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLobbyMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_my_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMyMessageReactionError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOriginalWebhookMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePinError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStageInstanceError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteThreadMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user_message_reaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserMessageReactionError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookByTokenError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhookMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deprecated_create_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedCreatePinError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deprecated_delete_pin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedDeletePinError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deprecated_list_pins`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprecatedListPinsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditLobbyError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edit_lobby_channel_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditLobbyChannelLinkError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`execute_github_compatible_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteGithubCompatibleWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`execute_slack_compatible_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteSlackCompatibleWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`execute_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`follow_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FollowChannelError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_active_guild_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActiveGuildThreadsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_answer_voters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAnswerVotersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_role_connections_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationRoleConnectionsMetadataError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationUserRoleConnectionError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAutoModerationRuleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_bot_gateway`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBotGatewayError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChannelError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_entitlement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEntitlementError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_entitlements`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEntitlementsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_gateway`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGatewayError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildApplicationCommandPermissionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_ban`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildBanError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_new_member_welcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildNewMemberWelcomeError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_preview`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildPreviewError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildRoleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildScheduledEventError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildSoundboardSoundError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildStickerError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildTemplateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_vanity_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildVanityUrlError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWebhooksError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_welcome_screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWelcomeScreenError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_widget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_widget_png`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetPngError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guild_widget_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildWidgetSettingsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_guilds_onboarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGuildsOnboardingError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLobbyError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lobby_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLobbyMessagesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyApplicationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyGuildMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_oauth2_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyOauth2ApplicationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_oauth2_authorization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyOauth2AuthorizationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyUserError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_openid_connect_userinfo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOpenidConnectUserinfoError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOriginalWebhookMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_public_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicKeysError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_self_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSelfVoiceStateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_soundboard_default_sounds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSoundboardDefaultSoundsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStageInstanceError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sticker_pack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStickerPackError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_thread_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetThreadMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVoiceStateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookByTokenError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhookMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`invite_resolve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteResolveError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`invite_revoke`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InviteRevokeError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`join_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JoinThreadError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_lobby`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveLobbyError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leave_thread`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaveThreadError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationCommandsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_application_emojis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationEmojisError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_auto_moderation_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAutoModerationRulesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_channel_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListChannelInvitesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_channel_webhooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListChannelWebhooksError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildApplicationCommandPermissionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_application_commands`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildApplicationCommandsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_audit_log_entries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildAuditLogEntriesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_bans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildBansError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildChannelsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_emojis`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildEmojisError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_integrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildIntegrationsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_invites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildInvitesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildMembersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildRolesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_scheduled_event_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildScheduledEventUsersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_scheduled_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildScheduledEventsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_soundboard_sounds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildSoundboardSoundsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_stickers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildStickersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildTemplatesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_guild_voice_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGuildVoiceRegionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_message_reactions_by_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessageReactionsByEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessagesError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_my_connections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyConnectionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_my_guilds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyGuildsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_my_private_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMyPrivateArchivedThreadsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_pins`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPinsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_private_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrivateArchivedThreadsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_public_archived_threads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPublicArchivedThreadsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sticker_packs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStickerPacksError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_thread_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListThreadMembersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_voice_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceRegionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`partner_sdk_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartnerSdkTokenError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`partner_sdk_unmerge_provisional_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartnerSdkUnmergeProvisionalAccountError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`poll_expire`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollExpireError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`preview_prune_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreviewPruneGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`prune_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PruneGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_guilds_onboarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuildsOnboardingError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_guild_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchGuildMembersError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendSoundboardSoundError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_channel_permission_overwrite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChannelPermissionOverwriteError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_guild_application_command_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGuildApplicationCommandPermissionsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_guild_mfa_level`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetGuildMfaLevelError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sync_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncGuildTemplateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`thread_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThreadSearchError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`trigger_typing_indicator`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerTypingIndicatorError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unban_user_from_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanUserFromGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_role_connections_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationRoleConnectionsMetadataError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application_user_role_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationUserRoleConnectionError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_auto_moderation_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAutoModerationRuleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateChannelError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_application_command`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildApplicationCommandError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_emoji`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildEmojiError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildRoleError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_scheduled_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildScheduledEventError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_soundboard_sound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildSoundboardSoundError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_sticker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildStickerError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildTemplateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_welcome_screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildWelcomeScreenError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_guild_widget_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGuildWidgetSettingsError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyApplicationError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_guild_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyGuildMemberError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyUserError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_original_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOriginalWebhookMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_self_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSelfVoiceStateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_stage_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateStageInstanceError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_voice_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateVoiceStateError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook_by_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookByTokenError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_webhook_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhookMessageError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_application_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadApplicationAttachmentError {
    Status4XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

