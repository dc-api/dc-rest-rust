# Rust API client for dc_rest

Preview of the Discord v10 HTTP API specification. See https://discord.com/developers/docs for more details.


## Overview

- API version: 10
- Package version: 10
- Build date: 2025-07-05T02:42:20.508163788Z[Etc/UTC]

## Installation

Put the package under your project folder in a directory named `dc_rest` and add the following to `Cargo.toml` under `[dependencies]`:

```
dc_rest = { path = "./dc_rest" }
```

## Documentation for API Endpoints

All URIs are relative to *https://discord.com/api/v10*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**add_group_dm_user**](docs/DefaultApi.md#add_group_dm_user) | **PUT** /channels/{channel_id}/recipients/{user_id} | 
*DefaultApi* | [**add_guild_member**](docs/DefaultApi.md#add_guild_member) | **PUT** /guilds/{guild_id}/members/{user_id} | 
*DefaultApi* | [**add_guild_member_role**](docs/DefaultApi.md#add_guild_member_role) | **PUT** /guilds/{guild_id}/members/{user_id}/roles/{role_id} | 
*DefaultApi* | [**add_lobby_member**](docs/DefaultApi.md#add_lobby_member) | **PUT** /lobbies/{lobby_id}/members/{user_id} | 
*DefaultApi* | [**add_my_message_reaction**](docs/DefaultApi.md#add_my_message_reaction) | **PUT** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me | 
*DefaultApi* | [**add_thread_member**](docs/DefaultApi.md#add_thread_member) | **PUT** /channels/{channel_id}/thread-members/{user_id} | 
*DefaultApi* | [**applications_get_activity_instance**](docs/DefaultApi.md#applications_get_activity_instance) | **GET** /applications/{application_id}/activity-instances/{instance_id} | 
*DefaultApi* | [**ban_user_from_guild**](docs/DefaultApi.md#ban_user_from_guild) | **PUT** /guilds/{guild_id}/bans/{user_id} | 
*DefaultApi* | [**bulk_ban_users_from_guild**](docs/DefaultApi.md#bulk_ban_users_from_guild) | **POST** /guilds/{guild_id}/bulk-ban | 
*DefaultApi* | [**bulk_delete_messages**](docs/DefaultApi.md#bulk_delete_messages) | **POST** /channels/{channel_id}/messages/bulk-delete | 
*DefaultApi* | [**bulk_set_application_commands**](docs/DefaultApi.md#bulk_set_application_commands) | **PUT** /applications/{application_id}/commands | 
*DefaultApi* | [**bulk_set_guild_application_commands**](docs/DefaultApi.md#bulk_set_guild_application_commands) | **PUT** /applications/{application_id}/guilds/{guild_id}/commands | 
*DefaultApi* | [**bulk_update_guild_channels**](docs/DefaultApi.md#bulk_update_guild_channels) | **PATCH** /guilds/{guild_id}/channels | 
*DefaultApi* | [**bulk_update_guild_roles**](docs/DefaultApi.md#bulk_update_guild_roles) | **PATCH** /guilds/{guild_id}/roles | 
*DefaultApi* | [**bulk_update_lobby_members**](docs/DefaultApi.md#bulk_update_lobby_members) | **POST** /lobbies/{lobby_id}/members/bulk | 
*DefaultApi* | [**consume_entitlement**](docs/DefaultApi.md#consume_entitlement) | **POST** /applications/{application_id}/entitlements/{entitlement_id}/consume | 
*DefaultApi* | [**create_application_command**](docs/DefaultApi.md#create_application_command) | **POST** /applications/{application_id}/commands | 
*DefaultApi* | [**create_application_emoji**](docs/DefaultApi.md#create_application_emoji) | **POST** /applications/{application_id}/emojis | 
*DefaultApi* | [**create_auto_moderation_rule**](docs/DefaultApi.md#create_auto_moderation_rule) | **POST** /guilds/{guild_id}/auto-moderation/rules | 
*DefaultApi* | [**create_channel_invite**](docs/DefaultApi.md#create_channel_invite) | **POST** /channels/{channel_id}/invites | 
*DefaultApi* | [**create_dm**](docs/DefaultApi.md#create_dm) | **POST** /users/@me/channels | 
*DefaultApi* | [**create_entitlement**](docs/DefaultApi.md#create_entitlement) | **POST** /applications/{application_id}/entitlements | 
*DefaultApi* | [**create_guild**](docs/DefaultApi.md#create_guild) | **POST** /guilds | 
*DefaultApi* | [**create_guild_application_command**](docs/DefaultApi.md#create_guild_application_command) | **POST** /applications/{application_id}/guilds/{guild_id}/commands | 
*DefaultApi* | [**create_guild_channel**](docs/DefaultApi.md#create_guild_channel) | **POST** /guilds/{guild_id}/channels | 
*DefaultApi* | [**create_guild_emoji**](docs/DefaultApi.md#create_guild_emoji) | **POST** /guilds/{guild_id}/emojis | 
*DefaultApi* | [**create_guild_from_template**](docs/DefaultApi.md#create_guild_from_template) | **POST** /guilds/templates/{code} | 
*DefaultApi* | [**create_guild_role**](docs/DefaultApi.md#create_guild_role) | **POST** /guilds/{guild_id}/roles | 
*DefaultApi* | [**create_guild_scheduled_event**](docs/DefaultApi.md#create_guild_scheduled_event) | **POST** /guilds/{guild_id}/scheduled-events | 
*DefaultApi* | [**create_guild_soundboard_sound**](docs/DefaultApi.md#create_guild_soundboard_sound) | **POST** /guilds/{guild_id}/soundboard-sounds | 
*DefaultApi* | [**create_guild_sticker**](docs/DefaultApi.md#create_guild_sticker) | **POST** /guilds/{guild_id}/stickers | 
*DefaultApi* | [**create_guild_template**](docs/DefaultApi.md#create_guild_template) | **POST** /guilds/{guild_id}/templates | 
*DefaultApi* | [**create_interaction_response**](docs/DefaultApi.md#create_interaction_response) | **POST** /interactions/{interaction_id}/{interaction_token}/callback | 
*DefaultApi* | [**create_lobby**](docs/DefaultApi.md#create_lobby) | **POST** /lobbies | 
*DefaultApi* | [**create_lobby_message**](docs/DefaultApi.md#create_lobby_message) | **POST** /lobbies/{lobby_id}/messages | 
*DefaultApi* | [**create_message**](docs/DefaultApi.md#create_message) | **POST** /channels/{channel_id}/messages | 
*DefaultApi* | [**create_or_join_lobby**](docs/DefaultApi.md#create_or_join_lobby) | **PUT** /lobbies | 
*DefaultApi* | [**create_pin**](docs/DefaultApi.md#create_pin) | **PUT** /channels/{channel_id}/messages/pins/{message_id} | 
*DefaultApi* | [**create_stage_instance**](docs/DefaultApi.md#create_stage_instance) | **POST** /stage-instances | 
*DefaultApi* | [**create_thread**](docs/DefaultApi.md#create_thread) | **POST** /channels/{channel_id}/threads | 
*DefaultApi* | [**create_thread_from_message**](docs/DefaultApi.md#create_thread_from_message) | **POST** /channels/{channel_id}/messages/{message_id}/threads | 
*DefaultApi* | [**create_webhook**](docs/DefaultApi.md#create_webhook) | **POST** /channels/{channel_id}/webhooks | 
*DefaultApi* | [**crosspost_message**](docs/DefaultApi.md#crosspost_message) | **POST** /channels/{channel_id}/messages/{message_id}/crosspost | 
*DefaultApi* | [**delete_all_message_reactions**](docs/DefaultApi.md#delete_all_message_reactions) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions | 
*DefaultApi* | [**delete_all_message_reactions_by_emoji**](docs/DefaultApi.md#delete_all_message_reactions_by_emoji) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name} | 
*DefaultApi* | [**delete_application_command**](docs/DefaultApi.md#delete_application_command) | **DELETE** /applications/{application_id}/commands/{command_id} | 
*DefaultApi* | [**delete_application_emoji**](docs/DefaultApi.md#delete_application_emoji) | **DELETE** /applications/{application_id}/emojis/{emoji_id} | 
*DefaultApi* | [**delete_application_user_role_connection**](docs/DefaultApi.md#delete_application_user_role_connection) | **DELETE** /users/@me/applications/{application_id}/role-connection | 
*DefaultApi* | [**delete_auto_moderation_rule**](docs/DefaultApi.md#delete_auto_moderation_rule) | **DELETE** /guilds/{guild_id}/auto-moderation/rules/{rule_id} | 
*DefaultApi* | [**delete_channel**](docs/DefaultApi.md#delete_channel) | **DELETE** /channels/{channel_id} | 
*DefaultApi* | [**delete_channel_permission_overwrite**](docs/DefaultApi.md#delete_channel_permission_overwrite) | **DELETE** /channels/{channel_id}/permissions/{overwrite_id} | 
*DefaultApi* | [**delete_entitlement**](docs/DefaultApi.md#delete_entitlement) | **DELETE** /applications/{application_id}/entitlements/{entitlement_id} | 
*DefaultApi* | [**delete_group_dm_user**](docs/DefaultApi.md#delete_group_dm_user) | **DELETE** /channels/{channel_id}/recipients/{user_id} | 
*DefaultApi* | [**delete_guild**](docs/DefaultApi.md#delete_guild) | **DELETE** /guilds/{guild_id} | 
*DefaultApi* | [**delete_guild_application_command**](docs/DefaultApi.md#delete_guild_application_command) | **DELETE** /applications/{application_id}/guilds/{guild_id}/commands/{command_id} | 
*DefaultApi* | [**delete_guild_emoji**](docs/DefaultApi.md#delete_guild_emoji) | **DELETE** /guilds/{guild_id}/emojis/{emoji_id} | 
*DefaultApi* | [**delete_guild_integration**](docs/DefaultApi.md#delete_guild_integration) | **DELETE** /guilds/{guild_id}/integrations/{integration_id} | 
*DefaultApi* | [**delete_guild_member**](docs/DefaultApi.md#delete_guild_member) | **DELETE** /guilds/{guild_id}/members/{user_id} | 
*DefaultApi* | [**delete_guild_member_role**](docs/DefaultApi.md#delete_guild_member_role) | **DELETE** /guilds/{guild_id}/members/{user_id}/roles/{role_id} | 
*DefaultApi* | [**delete_guild_role**](docs/DefaultApi.md#delete_guild_role) | **DELETE** /guilds/{guild_id}/roles/{role_id} | 
*DefaultApi* | [**delete_guild_scheduled_event**](docs/DefaultApi.md#delete_guild_scheduled_event) | **DELETE** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id} | 
*DefaultApi* | [**delete_guild_soundboard_sound**](docs/DefaultApi.md#delete_guild_soundboard_sound) | **DELETE** /guilds/{guild_id}/soundboard-sounds/{sound_id} | 
*DefaultApi* | [**delete_guild_sticker**](docs/DefaultApi.md#delete_guild_sticker) | **DELETE** /guilds/{guild_id}/stickers/{sticker_id} | 
*DefaultApi* | [**delete_guild_template**](docs/DefaultApi.md#delete_guild_template) | **DELETE** /guilds/{guild_id}/templates/{code} | 
*DefaultApi* | [**delete_lobby_member**](docs/DefaultApi.md#delete_lobby_member) | **DELETE** /lobbies/{lobby_id}/members/{user_id} | 
*DefaultApi* | [**delete_message**](docs/DefaultApi.md#delete_message) | **DELETE** /channels/{channel_id}/messages/{message_id} | 
*DefaultApi* | [**delete_my_message_reaction**](docs/DefaultApi.md#delete_my_message_reaction) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me | 
*DefaultApi* | [**delete_original_webhook_message**](docs/DefaultApi.md#delete_original_webhook_message) | **DELETE** /webhooks/{webhook_id}/{webhook_token}/messages/@original | 
*DefaultApi* | [**delete_pin**](docs/DefaultApi.md#delete_pin) | **DELETE** /channels/{channel_id}/messages/pins/{message_id} | 
*DefaultApi* | [**delete_stage_instance**](docs/DefaultApi.md#delete_stage_instance) | **DELETE** /stage-instances/{channel_id} | 
*DefaultApi* | [**delete_thread_member**](docs/DefaultApi.md#delete_thread_member) | **DELETE** /channels/{channel_id}/thread-members/{user_id} | 
*DefaultApi* | [**delete_user_message_reaction**](docs/DefaultApi.md#delete_user_message_reaction) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/{user_id} | 
*DefaultApi* | [**delete_webhook**](docs/DefaultApi.md#delete_webhook) | **DELETE** /webhooks/{webhook_id} | 
*DefaultApi* | [**delete_webhook_by_token**](docs/DefaultApi.md#delete_webhook_by_token) | **DELETE** /webhooks/{webhook_id}/{webhook_token} | 
*DefaultApi* | [**delete_webhook_message**](docs/DefaultApi.md#delete_webhook_message) | **DELETE** /webhooks/{webhook_id}/{webhook_token}/messages/{message_id} | 
*DefaultApi* | [**deprecated_create_pin**](docs/DefaultApi.md#deprecated_create_pin) | **PUT** /channels/{channel_id}/pins/{message_id} | 
*DefaultApi* | [**deprecated_delete_pin**](docs/DefaultApi.md#deprecated_delete_pin) | **DELETE** /channels/{channel_id}/pins/{message_id} | 
*DefaultApi* | [**deprecated_list_pins**](docs/DefaultApi.md#deprecated_list_pins) | **GET** /channels/{channel_id}/pins | 
*DefaultApi* | [**edit_lobby**](docs/DefaultApi.md#edit_lobby) | **PATCH** /lobbies/{lobby_id} | 
*DefaultApi* | [**edit_lobby_channel_link**](docs/DefaultApi.md#edit_lobby_channel_link) | **PATCH** /lobbies/{lobby_id}/channel-linking | 
*DefaultApi* | [**execute_github_compatible_webhook**](docs/DefaultApi.md#execute_github_compatible_webhook) | **POST** /webhooks/{webhook_id}/{webhook_token}/github | 
*DefaultApi* | [**execute_slack_compatible_webhook**](docs/DefaultApi.md#execute_slack_compatible_webhook) | **POST** /webhooks/{webhook_id}/{webhook_token}/slack | 
*DefaultApi* | [**execute_webhook**](docs/DefaultApi.md#execute_webhook) | **POST** /webhooks/{webhook_id}/{webhook_token} | 
*DefaultApi* | [**follow_channel**](docs/DefaultApi.md#follow_channel) | **POST** /channels/{channel_id}/followers | 
*DefaultApi* | [**get_active_guild_threads**](docs/DefaultApi.md#get_active_guild_threads) | **GET** /guilds/{guild_id}/threads/active | 
*DefaultApi* | [**get_answer_voters**](docs/DefaultApi.md#get_answer_voters) | **GET** /channels/{channel_id}/polls/{message_id}/answers/{answer_id} | 
*DefaultApi* | [**get_application**](docs/DefaultApi.md#get_application) | **GET** /applications/{application_id} | 
*DefaultApi* | [**get_application_command**](docs/DefaultApi.md#get_application_command) | **GET** /applications/{application_id}/commands/{command_id} | 
*DefaultApi* | [**get_application_emoji**](docs/DefaultApi.md#get_application_emoji) | **GET** /applications/{application_id}/emojis/{emoji_id} | 
*DefaultApi* | [**get_application_role_connections_metadata**](docs/DefaultApi.md#get_application_role_connections_metadata) | **GET** /applications/{application_id}/role-connections/metadata | 
*DefaultApi* | [**get_application_user_role_connection**](docs/DefaultApi.md#get_application_user_role_connection) | **GET** /users/@me/applications/{application_id}/role-connection | 
*DefaultApi* | [**get_auto_moderation_rule**](docs/DefaultApi.md#get_auto_moderation_rule) | **GET** /guilds/{guild_id}/auto-moderation/rules/{rule_id} | 
*DefaultApi* | [**get_bot_gateway**](docs/DefaultApi.md#get_bot_gateway) | **GET** /gateway/bot | 
*DefaultApi* | [**get_channel**](docs/DefaultApi.md#get_channel) | **GET** /channels/{channel_id} | 
*DefaultApi* | [**get_entitlement**](docs/DefaultApi.md#get_entitlement) | **GET** /applications/{application_id}/entitlements/{entitlement_id} | 
*DefaultApi* | [**get_entitlements**](docs/DefaultApi.md#get_entitlements) | **GET** /applications/{application_id}/entitlements | 
*DefaultApi* | [**get_gateway**](docs/DefaultApi.md#get_gateway) | **GET** /gateway | 
*DefaultApi* | [**get_guild**](docs/DefaultApi.md#get_guild) | **GET** /guilds/{guild_id} | 
*DefaultApi* | [**get_guild_application_command**](docs/DefaultApi.md#get_guild_application_command) | **GET** /applications/{application_id}/guilds/{guild_id}/commands/{command_id} | 
*DefaultApi* | [**get_guild_application_command_permissions**](docs/DefaultApi.md#get_guild_application_command_permissions) | **GET** /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions | 
*DefaultApi* | [**get_guild_ban**](docs/DefaultApi.md#get_guild_ban) | **GET** /guilds/{guild_id}/bans/{user_id} | 
*DefaultApi* | [**get_guild_emoji**](docs/DefaultApi.md#get_guild_emoji) | **GET** /guilds/{guild_id}/emojis/{emoji_id} | 
*DefaultApi* | [**get_guild_member**](docs/DefaultApi.md#get_guild_member) | **GET** /guilds/{guild_id}/members/{user_id} | 
*DefaultApi* | [**get_guild_new_member_welcome**](docs/DefaultApi.md#get_guild_new_member_welcome) | **GET** /guilds/{guild_id}/new-member-welcome | 
*DefaultApi* | [**get_guild_preview**](docs/DefaultApi.md#get_guild_preview) | **GET** /guilds/{guild_id}/preview | 
*DefaultApi* | [**get_guild_role**](docs/DefaultApi.md#get_guild_role) | **GET** /guilds/{guild_id}/roles/{role_id} | 
*DefaultApi* | [**get_guild_scheduled_event**](docs/DefaultApi.md#get_guild_scheduled_event) | **GET** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id} | 
*DefaultApi* | [**get_guild_soundboard_sound**](docs/DefaultApi.md#get_guild_soundboard_sound) | **GET** /guilds/{guild_id}/soundboard-sounds/{sound_id} | 
*DefaultApi* | [**get_guild_sticker**](docs/DefaultApi.md#get_guild_sticker) | **GET** /guilds/{guild_id}/stickers/{sticker_id} | 
*DefaultApi* | [**get_guild_template**](docs/DefaultApi.md#get_guild_template) | **GET** /guilds/templates/{code} | 
*DefaultApi* | [**get_guild_vanity_url**](docs/DefaultApi.md#get_guild_vanity_url) | **GET** /guilds/{guild_id}/vanity-url | 
*DefaultApi* | [**get_guild_webhooks**](docs/DefaultApi.md#get_guild_webhooks) | **GET** /guilds/{guild_id}/webhooks | 
*DefaultApi* | [**get_guild_welcome_screen**](docs/DefaultApi.md#get_guild_welcome_screen) | **GET** /guilds/{guild_id}/welcome-screen | 
*DefaultApi* | [**get_guild_widget**](docs/DefaultApi.md#get_guild_widget) | **GET** /guilds/{guild_id}/widget.json | 
*DefaultApi* | [**get_guild_widget_png**](docs/DefaultApi.md#get_guild_widget_png) | **GET** /guilds/{guild_id}/widget.png | 
*DefaultApi* | [**get_guild_widget_settings**](docs/DefaultApi.md#get_guild_widget_settings) | **GET** /guilds/{guild_id}/widget | 
*DefaultApi* | [**get_guilds_onboarding**](docs/DefaultApi.md#get_guilds_onboarding) | **GET** /guilds/{guild_id}/onboarding | 
*DefaultApi* | [**get_lobby**](docs/DefaultApi.md#get_lobby) | **GET** /lobbies/{lobby_id} | 
*DefaultApi* | [**get_lobby_messages**](docs/DefaultApi.md#get_lobby_messages) | **GET** /lobbies/{lobby_id}/messages | 
*DefaultApi* | [**get_message**](docs/DefaultApi.md#get_message) | **GET** /channels/{channel_id}/messages/{message_id} | 
*DefaultApi* | [**get_my_application**](docs/DefaultApi.md#get_my_application) | **GET** /applications/@me | 
*DefaultApi* | [**get_my_guild_member**](docs/DefaultApi.md#get_my_guild_member) | **GET** /users/@me/guilds/{guild_id}/member | 
*DefaultApi* | [**get_my_oauth2_application**](docs/DefaultApi.md#get_my_oauth2_application) | **GET** /oauth2/applications/@me | 
*DefaultApi* | [**get_my_oauth2_authorization**](docs/DefaultApi.md#get_my_oauth2_authorization) | **GET** /oauth2/@me | 
*DefaultApi* | [**get_my_user**](docs/DefaultApi.md#get_my_user) | **GET** /users/@me | 
*DefaultApi* | [**get_openid_connect_userinfo**](docs/DefaultApi.md#get_openid_connect_userinfo) | **GET** /oauth2/userinfo | 
*DefaultApi* | [**get_original_webhook_message**](docs/DefaultApi.md#get_original_webhook_message) | **GET** /webhooks/{webhook_id}/{webhook_token}/messages/@original | 
*DefaultApi* | [**get_public_keys**](docs/DefaultApi.md#get_public_keys) | **GET** /oauth2/keys | 
*DefaultApi* | [**get_self_voice_state**](docs/DefaultApi.md#get_self_voice_state) | **GET** /guilds/{guild_id}/voice-states/@me | 
*DefaultApi* | [**get_soundboard_default_sounds**](docs/DefaultApi.md#get_soundboard_default_sounds) | **GET** /soundboard-default-sounds | 
*DefaultApi* | [**get_stage_instance**](docs/DefaultApi.md#get_stage_instance) | **GET** /stage-instances/{channel_id} | 
*DefaultApi* | [**get_sticker**](docs/DefaultApi.md#get_sticker) | **GET** /stickers/{sticker_id} | 
*DefaultApi* | [**get_sticker_pack**](docs/DefaultApi.md#get_sticker_pack) | **GET** /sticker-packs/{pack_id} | 
*DefaultApi* | [**get_thread_member**](docs/DefaultApi.md#get_thread_member) | **GET** /channels/{channel_id}/thread-members/{user_id} | 
*DefaultApi* | [**get_user**](docs/DefaultApi.md#get_user) | **GET** /users/{user_id} | 
*DefaultApi* | [**get_voice_state**](docs/DefaultApi.md#get_voice_state) | **GET** /guilds/{guild_id}/voice-states/{user_id} | 
*DefaultApi* | [**get_webhook**](docs/DefaultApi.md#get_webhook) | **GET** /webhooks/{webhook_id} | 
*DefaultApi* | [**get_webhook_by_token**](docs/DefaultApi.md#get_webhook_by_token) | **GET** /webhooks/{webhook_id}/{webhook_token} | 
*DefaultApi* | [**get_webhook_message**](docs/DefaultApi.md#get_webhook_message) | **GET** /webhooks/{webhook_id}/{webhook_token}/messages/{message_id} | 
*DefaultApi* | [**invite_resolve**](docs/DefaultApi.md#invite_resolve) | **GET** /invites/{code} | 
*DefaultApi* | [**invite_revoke**](docs/DefaultApi.md#invite_revoke) | **DELETE** /invites/{code} | 
*DefaultApi* | [**join_thread**](docs/DefaultApi.md#join_thread) | **PUT** /channels/{channel_id}/thread-members/@me | 
*DefaultApi* | [**leave_guild**](docs/DefaultApi.md#leave_guild) | **DELETE** /users/@me/guilds/{guild_id} | 
*DefaultApi* | [**leave_lobby**](docs/DefaultApi.md#leave_lobby) | **DELETE** /lobbies/{lobby_id}/members/@me | 
*DefaultApi* | [**leave_thread**](docs/DefaultApi.md#leave_thread) | **DELETE** /channels/{channel_id}/thread-members/@me | 
*DefaultApi* | [**list_application_commands**](docs/DefaultApi.md#list_application_commands) | **GET** /applications/{application_id}/commands | 
*DefaultApi* | [**list_application_emojis**](docs/DefaultApi.md#list_application_emojis) | **GET** /applications/{application_id}/emojis | 
*DefaultApi* | [**list_auto_moderation_rules**](docs/DefaultApi.md#list_auto_moderation_rules) | **GET** /guilds/{guild_id}/auto-moderation/rules | 
*DefaultApi* | [**list_channel_invites**](docs/DefaultApi.md#list_channel_invites) | **GET** /channels/{channel_id}/invites | 
*DefaultApi* | [**list_channel_webhooks**](docs/DefaultApi.md#list_channel_webhooks) | **GET** /channels/{channel_id}/webhooks | 
*DefaultApi* | [**list_guild_application_command_permissions**](docs/DefaultApi.md#list_guild_application_command_permissions) | **GET** /applications/{application_id}/guilds/{guild_id}/commands/permissions | 
*DefaultApi* | [**list_guild_application_commands**](docs/DefaultApi.md#list_guild_application_commands) | **GET** /applications/{application_id}/guilds/{guild_id}/commands | 
*DefaultApi* | [**list_guild_audit_log_entries**](docs/DefaultApi.md#list_guild_audit_log_entries) | **GET** /guilds/{guild_id}/audit-logs | 
*DefaultApi* | [**list_guild_bans**](docs/DefaultApi.md#list_guild_bans) | **GET** /guilds/{guild_id}/bans | 
*DefaultApi* | [**list_guild_channels**](docs/DefaultApi.md#list_guild_channels) | **GET** /guilds/{guild_id}/channels | 
*DefaultApi* | [**list_guild_emojis**](docs/DefaultApi.md#list_guild_emojis) | **GET** /guilds/{guild_id}/emojis | 
*DefaultApi* | [**list_guild_integrations**](docs/DefaultApi.md#list_guild_integrations) | **GET** /guilds/{guild_id}/integrations | 
*DefaultApi* | [**list_guild_invites**](docs/DefaultApi.md#list_guild_invites) | **GET** /guilds/{guild_id}/invites | 
*DefaultApi* | [**list_guild_members**](docs/DefaultApi.md#list_guild_members) | **GET** /guilds/{guild_id}/members | 
*DefaultApi* | [**list_guild_roles**](docs/DefaultApi.md#list_guild_roles) | **GET** /guilds/{guild_id}/roles | 
*DefaultApi* | [**list_guild_scheduled_event_users**](docs/DefaultApi.md#list_guild_scheduled_event_users) | **GET** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users | 
*DefaultApi* | [**list_guild_scheduled_events**](docs/DefaultApi.md#list_guild_scheduled_events) | **GET** /guilds/{guild_id}/scheduled-events | 
*DefaultApi* | [**list_guild_soundboard_sounds**](docs/DefaultApi.md#list_guild_soundboard_sounds) | **GET** /guilds/{guild_id}/soundboard-sounds | 
*DefaultApi* | [**list_guild_stickers**](docs/DefaultApi.md#list_guild_stickers) | **GET** /guilds/{guild_id}/stickers | 
*DefaultApi* | [**list_guild_templates**](docs/DefaultApi.md#list_guild_templates) | **GET** /guilds/{guild_id}/templates | 
*DefaultApi* | [**list_guild_voice_regions**](docs/DefaultApi.md#list_guild_voice_regions) | **GET** /guilds/{guild_id}/regions | 
*DefaultApi* | [**list_message_reactions_by_emoji**](docs/DefaultApi.md#list_message_reactions_by_emoji) | **GET** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name} | 
*DefaultApi* | [**list_messages**](docs/DefaultApi.md#list_messages) | **GET** /channels/{channel_id}/messages | 
*DefaultApi* | [**list_my_connections**](docs/DefaultApi.md#list_my_connections) | **GET** /users/@me/connections | 
*DefaultApi* | [**list_my_guilds**](docs/DefaultApi.md#list_my_guilds) | **GET** /users/@me/guilds | 
*DefaultApi* | [**list_my_private_archived_threads**](docs/DefaultApi.md#list_my_private_archived_threads) | **GET** /channels/{channel_id}/users/@me/threads/archived/private | 
*DefaultApi* | [**list_pins**](docs/DefaultApi.md#list_pins) | **GET** /channels/{channel_id}/messages/pins | 
*DefaultApi* | [**list_private_archived_threads**](docs/DefaultApi.md#list_private_archived_threads) | **GET** /channels/{channel_id}/threads/archived/private | 
*DefaultApi* | [**list_public_archived_threads**](docs/DefaultApi.md#list_public_archived_threads) | **GET** /channels/{channel_id}/threads/archived/public | 
*DefaultApi* | [**list_sticker_packs**](docs/DefaultApi.md#list_sticker_packs) | **GET** /sticker-packs | 
*DefaultApi* | [**list_thread_members**](docs/DefaultApi.md#list_thread_members) | **GET** /channels/{channel_id}/thread-members | 
*DefaultApi* | [**list_voice_regions**](docs/DefaultApi.md#list_voice_regions) | **GET** /voice/regions | 
*DefaultApi* | [**partner_sdk_token**](docs/DefaultApi.md#partner_sdk_token) | **POST** /partner-sdk/token | 
*DefaultApi* | [**partner_sdk_unmerge_provisional_account**](docs/DefaultApi.md#partner_sdk_unmerge_provisional_account) | **POST** /partner-sdk/provisional-accounts/unmerge | 
*DefaultApi* | [**poll_expire**](docs/DefaultApi.md#poll_expire) | **POST** /channels/{channel_id}/polls/{message_id}/expire | 
*DefaultApi* | [**preview_prune_guild**](docs/DefaultApi.md#preview_prune_guild) | **GET** /guilds/{guild_id}/prune | 
*DefaultApi* | [**prune_guild**](docs/DefaultApi.md#prune_guild) | **POST** /guilds/{guild_id}/prune | 
*DefaultApi* | [**put_guilds_onboarding**](docs/DefaultApi.md#put_guilds_onboarding) | **PUT** /guilds/{guild_id}/onboarding | 
*DefaultApi* | [**search_guild_members**](docs/DefaultApi.md#search_guild_members) | **GET** /guilds/{guild_id}/members/search | 
*DefaultApi* | [**send_soundboard_sound**](docs/DefaultApi.md#send_soundboard_sound) | **POST** /channels/{channel_id}/send-soundboard-sound | 
*DefaultApi* | [**set_channel_permission_overwrite**](docs/DefaultApi.md#set_channel_permission_overwrite) | **PUT** /channels/{channel_id}/permissions/{overwrite_id} | 
*DefaultApi* | [**set_guild_application_command_permissions**](docs/DefaultApi.md#set_guild_application_command_permissions) | **PUT** /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions | 
*DefaultApi* | [**set_guild_mfa_level**](docs/DefaultApi.md#set_guild_mfa_level) | **POST** /guilds/{guild_id}/mfa | 
*DefaultApi* | [**sync_guild_template**](docs/DefaultApi.md#sync_guild_template) | **PUT** /guilds/{guild_id}/templates/{code} | 
*DefaultApi* | [**thread_search**](docs/DefaultApi.md#thread_search) | **GET** /channels/{channel_id}/threads/search | 
*DefaultApi* | [**trigger_typing_indicator**](docs/DefaultApi.md#trigger_typing_indicator) | **POST** /channels/{channel_id}/typing | 
*DefaultApi* | [**unban_user_from_guild**](docs/DefaultApi.md#unban_user_from_guild) | **DELETE** /guilds/{guild_id}/bans/{user_id} | 
*DefaultApi* | [**update_application**](docs/DefaultApi.md#update_application) | **PATCH** /applications/{application_id} | 
*DefaultApi* | [**update_application_command**](docs/DefaultApi.md#update_application_command) | **PATCH** /applications/{application_id}/commands/{command_id} | 
*DefaultApi* | [**update_application_emoji**](docs/DefaultApi.md#update_application_emoji) | **PATCH** /applications/{application_id}/emojis/{emoji_id} | 
*DefaultApi* | [**update_application_role_connections_metadata**](docs/DefaultApi.md#update_application_role_connections_metadata) | **PUT** /applications/{application_id}/role-connections/metadata | 
*DefaultApi* | [**update_application_user_role_connection**](docs/DefaultApi.md#update_application_user_role_connection) | **PUT** /users/@me/applications/{application_id}/role-connection | 
*DefaultApi* | [**update_auto_moderation_rule**](docs/DefaultApi.md#update_auto_moderation_rule) | **PATCH** /guilds/{guild_id}/auto-moderation/rules/{rule_id} | 
*DefaultApi* | [**update_channel**](docs/DefaultApi.md#update_channel) | **PATCH** /channels/{channel_id} | 
*DefaultApi* | [**update_guild**](docs/DefaultApi.md#update_guild) | **PATCH** /guilds/{guild_id} | 
*DefaultApi* | [**update_guild_application_command**](docs/DefaultApi.md#update_guild_application_command) | **PATCH** /applications/{application_id}/guilds/{guild_id}/commands/{command_id} | 
*DefaultApi* | [**update_guild_emoji**](docs/DefaultApi.md#update_guild_emoji) | **PATCH** /guilds/{guild_id}/emojis/{emoji_id} | 
*DefaultApi* | [**update_guild_member**](docs/DefaultApi.md#update_guild_member) | **PATCH** /guilds/{guild_id}/members/{user_id} | 
*DefaultApi* | [**update_guild_role**](docs/DefaultApi.md#update_guild_role) | **PATCH** /guilds/{guild_id}/roles/{role_id} | 
*DefaultApi* | [**update_guild_scheduled_event**](docs/DefaultApi.md#update_guild_scheduled_event) | **PATCH** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id} | 
*DefaultApi* | [**update_guild_soundboard_sound**](docs/DefaultApi.md#update_guild_soundboard_sound) | **PATCH** /guilds/{guild_id}/soundboard-sounds/{sound_id} | 
*DefaultApi* | [**update_guild_sticker**](docs/DefaultApi.md#update_guild_sticker) | **PATCH** /guilds/{guild_id}/stickers/{sticker_id} | 
*DefaultApi* | [**update_guild_template**](docs/DefaultApi.md#update_guild_template) | **PATCH** /guilds/{guild_id}/templates/{code} | 
*DefaultApi* | [**update_guild_welcome_screen**](docs/DefaultApi.md#update_guild_welcome_screen) | **PATCH** /guilds/{guild_id}/welcome-screen | 
*DefaultApi* | [**update_guild_widget_settings**](docs/DefaultApi.md#update_guild_widget_settings) | **PATCH** /guilds/{guild_id}/widget | 
*DefaultApi* | [**update_message**](docs/DefaultApi.md#update_message) | **PATCH** /channels/{channel_id}/messages/{message_id} | 
*DefaultApi* | [**update_my_application**](docs/DefaultApi.md#update_my_application) | **PATCH** /applications/@me | 
*DefaultApi* | [**update_my_guild_member**](docs/DefaultApi.md#update_my_guild_member) | **PATCH** /guilds/{guild_id}/members/@me | 
*DefaultApi* | [**update_my_user**](docs/DefaultApi.md#update_my_user) | **PATCH** /users/@me | 
*DefaultApi* | [**update_original_webhook_message**](docs/DefaultApi.md#update_original_webhook_message) | **PATCH** /webhooks/{webhook_id}/{webhook_token}/messages/@original | 
*DefaultApi* | [**update_self_voice_state**](docs/DefaultApi.md#update_self_voice_state) | **PATCH** /guilds/{guild_id}/voice-states/@me | 
*DefaultApi* | [**update_stage_instance**](docs/DefaultApi.md#update_stage_instance) | **PATCH** /stage-instances/{channel_id} | 
*DefaultApi* | [**update_voice_state**](docs/DefaultApi.md#update_voice_state) | **PATCH** /guilds/{guild_id}/voice-states/{user_id} | 
*DefaultApi* | [**update_webhook**](docs/DefaultApi.md#update_webhook) | **PATCH** /webhooks/{webhook_id} | 
*DefaultApi* | [**update_webhook_by_token**](docs/DefaultApi.md#update_webhook_by_token) | **PATCH** /webhooks/{webhook_id}/{webhook_token} | 
*DefaultApi* | [**update_webhook_message**](docs/DefaultApi.md#update_webhook_message) | **PATCH** /webhooks/{webhook_id}/{webhook_token}/messages/{message_id} | 
*DefaultApi* | [**upload_application_attachment**](docs/DefaultApi.md#upload_application_attachment) | **POST** /applications/{application_id}/attachment | 


## Documentation For Models

 - [AccountResponse](docs/AccountResponse.md)
 - [ActionRowComponentForMessageRequest](docs/ActionRowComponentForMessageRequest.md)
 - [ActionRowComponentForMessageRequestComponentsInner](docs/ActionRowComponentForMessageRequestComponentsInner.md)
 - [ActionRowComponentForModalRequest](docs/ActionRowComponentForModalRequest.md)
 - [ActionRowComponentResponse](docs/ActionRowComponentResponse.md)
 - [ActionRowComponentResponseComponentsInner](docs/ActionRowComponentResponseComponentsInner.md)
 - [ActivitiesAttachmentResponse](docs/ActivitiesAttachmentResponse.md)
 - [AddGroupDmUser201Response](docs/AddGroupDmUser201Response.md)
 - [AddGroupDmUserRequest](docs/AddGroupDmUserRequest.md)
 - [AddGuildMemberRequest](docs/AddGuildMemberRequest.md)
 - [AddLobbyMemberRequest](docs/AddLobbyMemberRequest.md)
 - [ApplicationCommandAttachmentOption](docs/ApplicationCommandAttachmentOption.md)
 - [ApplicationCommandAttachmentOptionResponse](docs/ApplicationCommandAttachmentOptionResponse.md)
 - [ApplicationCommandAutocompleteCallbackRequest](docs/ApplicationCommandAutocompleteCallbackRequest.md)
 - [ApplicationCommandAutocompleteCallbackRequestData](docs/ApplicationCommandAutocompleteCallbackRequestData.md)
 - [ApplicationCommandBooleanOption](docs/ApplicationCommandBooleanOption.md)
 - [ApplicationCommandBooleanOptionResponse](docs/ApplicationCommandBooleanOptionResponse.md)
 - [ApplicationCommandChannelOption](docs/ApplicationCommandChannelOption.md)
 - [ApplicationCommandChannelOptionResponse](docs/ApplicationCommandChannelOptionResponse.md)
 - [ApplicationCommandCreateRequest](docs/ApplicationCommandCreateRequest.md)
 - [ApplicationCommandCreateRequestOptionsInner](docs/ApplicationCommandCreateRequestOptionsInner.md)
 - [ApplicationCommandIntegerOption](docs/ApplicationCommandIntegerOption.md)
 - [ApplicationCommandIntegerOptionResponse](docs/ApplicationCommandIntegerOptionResponse.md)
 - [ApplicationCommandInteractionMetadataResponse](docs/ApplicationCommandInteractionMetadataResponse.md)
 - [ApplicationCommandMentionableOption](docs/ApplicationCommandMentionableOption.md)
 - [ApplicationCommandMentionableOptionResponse](docs/ApplicationCommandMentionableOptionResponse.md)
 - [ApplicationCommandNumberOption](docs/ApplicationCommandNumberOption.md)
 - [ApplicationCommandNumberOptionResponse](docs/ApplicationCommandNumberOptionResponse.md)
 - [ApplicationCommandOptionIntegerChoice](docs/ApplicationCommandOptionIntegerChoice.md)
 - [ApplicationCommandOptionIntegerChoiceResponse](docs/ApplicationCommandOptionIntegerChoiceResponse.md)
 - [ApplicationCommandOptionNumberChoice](docs/ApplicationCommandOptionNumberChoice.md)
 - [ApplicationCommandOptionNumberChoiceResponse](docs/ApplicationCommandOptionNumberChoiceResponse.md)
 - [ApplicationCommandOptionStringChoice](docs/ApplicationCommandOptionStringChoice.md)
 - [ApplicationCommandOptionStringChoiceResponse](docs/ApplicationCommandOptionStringChoiceResponse.md)
 - [ApplicationCommandPatchRequestPartial](docs/ApplicationCommandPatchRequestPartial.md)
 - [ApplicationCommandPermission](docs/ApplicationCommandPermission.md)
 - [ApplicationCommandResponse](docs/ApplicationCommandResponse.md)
 - [ApplicationCommandResponseOptionsInner](docs/ApplicationCommandResponseOptionsInner.md)
 - [ApplicationCommandRoleOption](docs/ApplicationCommandRoleOption.md)
 - [ApplicationCommandRoleOptionResponse](docs/ApplicationCommandRoleOptionResponse.md)
 - [ApplicationCommandStringOption](docs/ApplicationCommandStringOption.md)
 - [ApplicationCommandStringOptionResponse](docs/ApplicationCommandStringOptionResponse.md)
 - [ApplicationCommandSubcommandGroupOption](docs/ApplicationCommandSubcommandGroupOption.md)
 - [ApplicationCommandSubcommandGroupOptionResponse](docs/ApplicationCommandSubcommandGroupOptionResponse.md)
 - [ApplicationCommandSubcommandOption](docs/ApplicationCommandSubcommandOption.md)
 - [ApplicationCommandSubcommandOptionOptionsInner](docs/ApplicationCommandSubcommandOptionOptionsInner.md)
 - [ApplicationCommandSubcommandOptionResponse](docs/ApplicationCommandSubcommandOptionResponse.md)
 - [ApplicationCommandSubcommandOptionResponseOptionsInner](docs/ApplicationCommandSubcommandOptionResponseOptionsInner.md)
 - [ApplicationCommandUpdateRequest](docs/ApplicationCommandUpdateRequest.md)
 - [ApplicationCommandUserOption](docs/ApplicationCommandUserOption.md)
 - [ApplicationCommandUserOptionResponse](docs/ApplicationCommandUserOptionResponse.md)
 - [ApplicationFormPartial](docs/ApplicationFormPartial.md)
 - [ApplicationFormPartialDescription](docs/ApplicationFormPartialDescription.md)
 - [ApplicationFormPartialIntegrationTypesConfigValue](docs/ApplicationFormPartialIntegrationTypesConfigValue.md)
 - [ApplicationIncomingWebhookResponse](docs/ApplicationIncomingWebhookResponse.md)
 - [ApplicationIntegrationTypeConfiguration](docs/ApplicationIntegrationTypeConfiguration.md)
 - [ApplicationIntegrationTypeConfigurationResponse](docs/ApplicationIntegrationTypeConfigurationResponse.md)
 - [ApplicationOAuth2InstallParams](docs/ApplicationOAuth2InstallParams.md)
 - [ApplicationOAuth2InstallParamsResponse](docs/ApplicationOAuth2InstallParamsResponse.md)
 - [ApplicationResponse](docs/ApplicationResponse.md)
 - [ApplicationRoleConnectionsMetadataItemRequest](docs/ApplicationRoleConnectionsMetadataItemRequest.md)
 - [ApplicationRoleConnectionsMetadataItemResponse](docs/ApplicationRoleConnectionsMetadataItemResponse.md)
 - [ApplicationUserRoleConnectionResponse](docs/ApplicationUserRoleConnectionResponse.md)
 - [AttachmentResponse](docs/AttachmentResponse.md)
 - [AuditLogEntryResponse](docs/AuditLogEntryResponse.md)
 - [AuditLogObjectChangeResponse](docs/AuditLogObjectChangeResponse.md)
 - [BanUserFromGuildRequest](docs/BanUserFromGuildRequest.md)
 - [BaseCreateMessageCreateRequest](docs/BaseCreateMessageCreateRequest.md)
 - [BaseCreateMessageCreateRequestComponentsInner](docs/BaseCreateMessageCreateRequestComponentsInner.md)
 - [BasicApplicationResponse](docs/BasicApplicationResponse.md)
 - [BasicMessageResponse](docs/BasicMessageResponse.md)
 - [BasicMessageResponseComponentsInner](docs/BasicMessageResponseComponentsInner.md)
 - [BasicMessageResponseInteractionMetadata](docs/BasicMessageResponseInteractionMetadata.md)
 - [BasicMessageResponseNonce](docs/BasicMessageResponseNonce.md)
 - [BlockMessageAction](docs/BlockMessageAction.md)
 - [BlockMessageActionMetadata](docs/BlockMessageActionMetadata.md)
 - [BlockMessageActionMetadataResponse](docs/BlockMessageActionMetadataResponse.md)
 - [BlockMessageActionResponse](docs/BlockMessageActionResponse.md)
 - [BotAccountPatchRequest](docs/BotAccountPatchRequest.md)
 - [BulkBanUsersFromGuildRequest](docs/BulkBanUsersFromGuildRequest.md)
 - [BulkBanUsersResponse](docs/BulkBanUsersResponse.md)
 - [BulkDeleteMessagesRequest](docs/BulkDeleteMessagesRequest.md)
 - [BulkLobbyMemberRequest](docs/BulkLobbyMemberRequest.md)
 - [BulkUpdateGuildChannelsRequestInner](docs/BulkUpdateGuildChannelsRequestInner.md)
 - [BulkUpdateGuildRolesRequestInner](docs/BulkUpdateGuildRolesRequestInner.md)
 - [ButtonComponentForMessageRequest](docs/ButtonComponentForMessageRequest.md)
 - [ButtonComponentResponse](docs/ButtonComponentResponse.md)
 - [ChannelFollowerResponse](docs/ChannelFollowerResponse.md)
 - [ChannelFollowerWebhookResponse](docs/ChannelFollowerWebhookResponse.md)
 - [ChannelPermissionOverwriteRequest](docs/ChannelPermissionOverwriteRequest.md)
 - [ChannelPermissionOverwriteResponse](docs/ChannelPermissionOverwriteResponse.md)
 - [ChannelSelectComponentForMessageRequest](docs/ChannelSelectComponentForMessageRequest.md)
 - [ChannelSelectComponentResponse](docs/ChannelSelectComponentResponse.md)
 - [ChannelSelectDefaultValue](docs/ChannelSelectDefaultValue.md)
 - [ChannelSelectDefaultValueResponse](docs/ChannelSelectDefaultValueResponse.md)
 - [CommandPermissionResponse](docs/CommandPermissionResponse.md)
 - [CommandPermissionsResponse](docs/CommandPermissionsResponse.md)
 - [ComponentEmojiForMessageRequest](docs/ComponentEmojiForMessageRequest.md)
 - [ComponentEmojiResponse](docs/ComponentEmojiResponse.md)
 - [ConnectedAccountGuildResponse](docs/ConnectedAccountGuildResponse.md)
 - [ConnectedAccountIntegrationResponse](docs/ConnectedAccountIntegrationResponse.md)
 - [ConnectedAccountResponse](docs/ConnectedAccountResponse.md)
 - [ContainerComponentForMessageRequest](docs/ContainerComponentForMessageRequest.md)
 - [ContainerComponentForMessageRequestComponentsInner](docs/ContainerComponentForMessageRequestComponentsInner.md)
 - [ContainerComponentResponse](docs/ContainerComponentResponse.md)
 - [ContainerComponentResponseComponentsInner](docs/ContainerComponentResponseComponentsInner.md)
 - [CreateApplicationEmojiRequest](docs/CreateApplicationEmojiRequest.md)
 - [CreateAutoModerationRule200Response](docs/CreateAutoModerationRule200Response.md)
 - [CreateAutoModerationRuleRequest](docs/CreateAutoModerationRuleRequest.md)
 - [CreateChannelInviteRequest](docs/CreateChannelInviteRequest.md)
 - [CreateEntitlementRequestData](docs/CreateEntitlementRequestData.md)
 - [CreateForumThreadRequest](docs/CreateForumThreadRequest.md)
 - [CreateGroupDmInviteRequest](docs/CreateGroupDmInviteRequest.md)
 - [CreateGuildChannelRequest](docs/CreateGuildChannelRequest.md)
 - [CreateGuildEmojiRequest](docs/CreateGuildEmojiRequest.md)
 - [CreateGuildFromTemplateRequest](docs/CreateGuildFromTemplateRequest.md)
 - [CreateGuildInviteRequest](docs/CreateGuildInviteRequest.md)
 - [CreateGuildRequestChannelItem](docs/CreateGuildRequestChannelItem.md)
 - [CreateGuildRequestRoleItem](docs/CreateGuildRequestRoleItem.md)
 - [CreateGuildRoleRequest](docs/CreateGuildRoleRequest.md)
 - [CreateGuildScheduledEventRequest](docs/CreateGuildScheduledEventRequest.md)
 - [CreateGuildTemplateRequest](docs/CreateGuildTemplateRequest.md)
 - [CreateInteractionResponseRequest](docs/CreateInteractionResponseRequest.md)
 - [CreateLobbyRequest](docs/CreateLobbyRequest.md)
 - [CreateMessageInteractionCallbackRequest](docs/CreateMessageInteractionCallbackRequest.md)
 - [CreateMessageInteractionCallbackResponse](docs/CreateMessageInteractionCallbackResponse.md)
 - [CreateOrJoinLobbyRequest](docs/CreateOrJoinLobbyRequest.md)
 - [CreateOrUpdateThreadTagRequest](docs/CreateOrUpdateThreadTagRequest.md)
 - [CreatePrivateChannelRequest](docs/CreatePrivateChannelRequest.md)
 - [CreateStageInstanceRequest](docs/CreateStageInstanceRequest.md)
 - [CreateTextThreadWithMessageRequest](docs/CreateTextThreadWithMessageRequest.md)
 - [CreateTextThreadWithoutMessageRequest](docs/CreateTextThreadWithoutMessageRequest.md)
 - [CreateThreadRequest](docs/CreateThreadRequest.md)
 - [CreateWebhookRequest](docs/CreateWebhookRequest.md)
 - [CreatedThreadResponse](docs/CreatedThreadResponse.md)
 - [DefaultKeywordListTriggerMetadata](docs/DefaultKeywordListTriggerMetadata.md)
 - [DefaultKeywordListTriggerMetadataResponse](docs/DefaultKeywordListTriggerMetadataResponse.md)
 - [DefaultKeywordListUpsertRequest](docs/DefaultKeywordListUpsertRequest.md)
 - [DefaultKeywordListUpsertRequestActionsInner](docs/DefaultKeywordListUpsertRequestActionsInner.md)
 - [DefaultKeywordListUpsertRequestPartial](docs/DefaultKeywordListUpsertRequestPartial.md)
 - [DefaultKeywordRuleResponse](docs/DefaultKeywordRuleResponse.md)
 - [DefaultKeywordRuleResponseActionsInner](docs/DefaultKeywordRuleResponseActionsInner.md)
 - [DefaultReactionEmojiResponse](docs/DefaultReactionEmojiResponse.md)
 - [DiscordIntegrationResponse](docs/DiscordIntegrationResponse.md)
 - [EditLobbyChannelLinkRequest](docs/EditLobbyChannelLinkRequest.md)
 - [EmbeddedActivityInstance](docs/EmbeddedActivityInstance.md)
 - [EmbeddedActivityInstanceLocation](docs/EmbeddedActivityInstanceLocation.md)
 - [EmojiResponse](docs/EmojiResponse.md)
 - [EntitlementResponse](docs/EntitlementResponse.md)
 - [EntityMetadataExternal](docs/EntityMetadataExternal.md)
 - [EntityMetadataExternalResponse](docs/EntityMetadataExternalResponse.md)
 - [Error](docs/Error.md)
 - [ErrorDetails](docs/ErrorDetails.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ExecuteWebhookRequest](docs/ExecuteWebhookRequest.md)
 - [ExternalConnectionIntegrationResponse](docs/ExternalConnectionIntegrationResponse.md)
 - [ExternalScheduledEventCreateRequest](docs/ExternalScheduledEventCreateRequest.md)
 - [ExternalScheduledEventPatchRequestPartial](docs/ExternalScheduledEventPatchRequestPartial.md)
 - [ExternalScheduledEventResponse](docs/ExternalScheduledEventResponse.md)
 - [FileComponentForMessageRequest](docs/FileComponentForMessageRequest.md)
 - [FileComponentResponse](docs/FileComponentResponse.md)
 - [FlagToChannelAction](docs/FlagToChannelAction.md)
 - [FlagToChannelActionMetadata](docs/FlagToChannelActionMetadata.md)
 - [FlagToChannelActionMetadataResponse](docs/FlagToChannelActionMetadataResponse.md)
 - [FlagToChannelActionResponse](docs/FlagToChannelActionResponse.md)
 - [FollowChannelRequest](docs/FollowChannelRequest.md)
 - [ForumTagResponse](docs/ForumTagResponse.md)
 - [FriendInviteResponse](docs/FriendInviteResponse.md)
 - [GatewayBotResponse](docs/GatewayBotResponse.md)
 - [GatewayBotSessionStartLimitResponse](docs/GatewayBotSessionStartLimitResponse.md)
 - [GatewayResponse](docs/GatewayResponse.md)
 - [GetChannel200Response](docs/GetChannel200Response.md)
 - [GetEntitlementsSkuIdsParameter](docs/GetEntitlementsSkuIdsParameter.md)
 - [GetSticker200Response](docs/GetSticker200Response.md)
 - [GithubAuthor](docs/GithubAuthor.md)
 - [GithubCheckApp](docs/GithubCheckApp.md)
 - [GithubCheckPullRequest](docs/GithubCheckPullRequest.md)
 - [GithubCheckRun](docs/GithubCheckRun.md)
 - [GithubCheckRunOutput](docs/GithubCheckRunOutput.md)
 - [GithubCheckSuite](docs/GithubCheckSuite.md)
 - [GithubComment](docs/GithubComment.md)
 - [GithubCommit](docs/GithubCommit.md)
 - [GithubDiscussion](docs/GithubDiscussion.md)
 - [GithubIssue](docs/GithubIssue.md)
 - [GithubRelease](docs/GithubRelease.md)
 - [GithubRepository](docs/GithubRepository.md)
 - [GithubReview](docs/GithubReview.md)
 - [GithubUser](docs/GithubUser.md)
 - [GithubWebhook](docs/GithubWebhook.md)
 - [GroupDmInviteResponse](docs/GroupDmInviteResponse.md)
 - [GuildAuditLogResponse](docs/GuildAuditLogResponse.md)
 - [GuildAuditLogResponseIntegrationsInner](docs/GuildAuditLogResponseIntegrationsInner.md)
 - [GuildBanResponse](docs/GuildBanResponse.md)
 - [GuildChannelLocation](docs/GuildChannelLocation.md)
 - [GuildChannelResponse](docs/GuildChannelResponse.md)
 - [GuildCreateRequest](docs/GuildCreateRequest.md)
 - [GuildHomeSettingsResponse](docs/GuildHomeSettingsResponse.md)
 - [GuildIncomingWebhookResponse](docs/GuildIncomingWebhookResponse.md)
 - [GuildInviteResponse](docs/GuildInviteResponse.md)
 - [GuildMemberResponse](docs/GuildMemberResponse.md)
 - [GuildMfaLevelResponse](docs/GuildMfaLevelResponse.md)
 - [GuildOnboardingResponse](docs/GuildOnboardingResponse.md)
 - [GuildPatchRequestPartial](docs/GuildPatchRequestPartial.md)
 - [GuildPreviewResponse](docs/GuildPreviewResponse.md)
 - [GuildProductPurchaseResponse](docs/GuildProductPurchaseResponse.md)
 - [GuildPruneResponse](docs/GuildPruneResponse.md)
 - [GuildResponse](docs/GuildResponse.md)
 - [GuildRoleColorsResponse](docs/GuildRoleColorsResponse.md)
 - [GuildRoleResponse](docs/GuildRoleResponse.md)
 - [GuildRoleTagsResponse](docs/GuildRoleTagsResponse.md)
 - [GuildStickerResponse](docs/GuildStickerResponse.md)
 - [GuildSubscriptionIntegrationResponse](docs/GuildSubscriptionIntegrationResponse.md)
 - [GuildTemplateChannelResponse](docs/GuildTemplateChannelResponse.md)
 - [GuildTemplateChannelTags](docs/GuildTemplateChannelTags.md)
 - [GuildTemplateResponse](docs/GuildTemplateResponse.md)
 - [GuildTemplateRoleResponse](docs/GuildTemplateRoleResponse.md)
 - [GuildTemplateSnapshotResponse](docs/GuildTemplateSnapshotResponse.md)
 - [GuildWelcomeChannel](docs/GuildWelcomeChannel.md)
 - [GuildWelcomeScreenChannelResponse](docs/GuildWelcomeScreenChannelResponse.md)
 - [GuildWelcomeScreenResponse](docs/GuildWelcomeScreenResponse.md)
 - [GuildWithCountsResponse](docs/GuildWithCountsResponse.md)
 - [IncomingWebhookInteractionRequest](docs/IncomingWebhookInteractionRequest.md)
 - [IncomingWebhookRequestPartial](docs/IncomingWebhookRequestPartial.md)
 - [IncomingWebhookUpdateForInteractionCallbackRequestPartial](docs/IncomingWebhookUpdateForInteractionCallbackRequestPartial.md)
 - [IncomingWebhookUpdateRequestPartial](docs/IncomingWebhookUpdateRequestPartial.md)
 - [InnerErrors](docs/InnerErrors.md)
 - [IntegrationApplicationResponse](docs/IntegrationApplicationResponse.md)
 - [InteractionApplicationCommandAutocompleteCallbackIntegerData](docs/InteractionApplicationCommandAutocompleteCallbackIntegerData.md)
 - [InteractionApplicationCommandAutocompleteCallbackNumberData](docs/InteractionApplicationCommandAutocompleteCallbackNumberData.md)
 - [InteractionApplicationCommandAutocompleteCallbackStringData](docs/InteractionApplicationCommandAutocompleteCallbackStringData.md)
 - [InteractionCallbackResponse](docs/InteractionCallbackResponse.md)
 - [InteractionCallbackResponseResource](docs/InteractionCallbackResponseResource.md)
 - [InteractionResponse](docs/InteractionResponse.md)
 - [InviteApplicationResponse](docs/InviteApplicationResponse.md)
 - [InviteChannelRecipientResponse](docs/InviteChannelRecipientResponse.md)
 - [InviteChannelResponse](docs/InviteChannelResponse.md)
 - [InviteGuildResponse](docs/InviteGuildResponse.md)
 - [KeywordRuleResponse](docs/KeywordRuleResponse.md)
 - [KeywordTriggerMetadata](docs/KeywordTriggerMetadata.md)
 - [KeywordTriggerMetadataResponse](docs/KeywordTriggerMetadataResponse.md)
 - [KeywordUpsertRequest](docs/KeywordUpsertRequest.md)
 - [KeywordUpsertRequestPartial](docs/KeywordUpsertRequestPartial.md)
 - [LaunchActivityInteractionCallbackRequest](docs/LaunchActivityInteractionCallbackRequest.md)
 - [LaunchActivityInteractionCallbackResponse](docs/LaunchActivityInteractionCallbackResponse.md)
 - [ListApplicationEmojisResponse](docs/ListApplicationEmojisResponse.md)
 - [ListAutoModerationRules200ResponseInner](docs/ListAutoModerationRules200ResponseInner.md)
 - [ListChannelInvites200ResponseInner](docs/ListChannelInvites200ResponseInner.md)
 - [ListChannelWebhooks200ResponseInner](docs/ListChannelWebhooks200ResponseInner.md)
 - [ListGuildIntegrations200ResponseInner](docs/ListGuildIntegrations200ResponseInner.md)
 - [ListGuildScheduledEvents200ResponseInner](docs/ListGuildScheduledEvents200ResponseInner.md)
 - [ListGuildSoundboardSoundsResponse](docs/ListGuildSoundboardSoundsResponse.md)
 - [LobbyMemberRequest](docs/LobbyMemberRequest.md)
 - [LobbyMemberResponse](docs/LobbyMemberResponse.md)
 - [LobbyMessageResponse](docs/LobbyMessageResponse.md)
 - [LobbyResponse](docs/LobbyResponse.md)
 - [MediaGalleryComponentForMessageRequest](docs/MediaGalleryComponentForMessageRequest.md)
 - [MediaGalleryComponentResponse](docs/MediaGalleryComponentResponse.md)
 - [MediaGalleryItemRequest](docs/MediaGalleryItemRequest.md)
 - [MediaGalleryItemResponse](docs/MediaGalleryItemResponse.md)
 - [MentionSpamRuleResponse](docs/MentionSpamRuleResponse.md)
 - [MentionSpamTriggerMetadata](docs/MentionSpamTriggerMetadata.md)
 - [MentionSpamTriggerMetadataResponse](docs/MentionSpamTriggerMetadataResponse.md)
 - [MentionSpamUpsertRequest](docs/MentionSpamUpsertRequest.md)
 - [MentionSpamUpsertRequestPartial](docs/MentionSpamUpsertRequestPartial.md)
 - [MentionableSelectComponentForMessageRequest](docs/MentionableSelectComponentForMessageRequest.md)
 - [MentionableSelectComponentForMessageRequestDefaultValuesInner](docs/MentionableSelectComponentForMessageRequestDefaultValuesInner.md)
 - [MentionableSelectComponentResponse](docs/MentionableSelectComponentResponse.md)
 - [MentionableSelectComponentResponseDefaultValuesInner](docs/MentionableSelectComponentResponseDefaultValuesInner.md)
 - [MessageAllowedMentionsRequest](docs/MessageAllowedMentionsRequest.md)
 - [MessageAttachmentRequest](docs/MessageAttachmentRequest.md)
 - [MessageAttachmentResponse](docs/MessageAttachmentResponse.md)
 - [MessageCallResponse](docs/MessageCallResponse.md)
 - [MessageComponentInteractionMetadataResponse](docs/MessageComponentInteractionMetadataResponse.md)
 - [MessageCreateRequest](docs/MessageCreateRequest.md)
 - [MessageEditRequestPartial](docs/MessageEditRequestPartial.md)
 - [MessageEmbedAuthorResponse](docs/MessageEmbedAuthorResponse.md)
 - [MessageEmbedFieldResponse](docs/MessageEmbedFieldResponse.md)
 - [MessageEmbedFooterResponse](docs/MessageEmbedFooterResponse.md)
 - [MessageEmbedImageResponse](docs/MessageEmbedImageResponse.md)
 - [MessageEmbedProviderResponse](docs/MessageEmbedProviderResponse.md)
 - [MessageEmbedResponse](docs/MessageEmbedResponse.md)
 - [MessageEmbedVideoResponse](docs/MessageEmbedVideoResponse.md)
 - [MessageInteractionResponse](docs/MessageInteractionResponse.md)
 - [MessageMentionChannelResponse](docs/MessageMentionChannelResponse.md)
 - [MessageReactionCountDetailsResponse](docs/MessageReactionCountDetailsResponse.md)
 - [MessageReactionEmojiResponse](docs/MessageReactionEmojiResponse.md)
 - [MessageReactionResponse](docs/MessageReactionResponse.md)
 - [MessageReferenceRequest](docs/MessageReferenceRequest.md)
 - [MessageReferenceResponse](docs/MessageReferenceResponse.md)
 - [MessageResponse](docs/MessageResponse.md)
 - [MessageRoleSubscriptionDataResponse](docs/MessageRoleSubscriptionDataResponse.md)
 - [MessageSnapshotResponse](docs/MessageSnapshotResponse.md)
 - [MessageStickerItemResponse](docs/MessageStickerItemResponse.md)
 - [MinimalContentMessageResponse](docs/MinimalContentMessageResponse.md)
 - [MlSpamRuleResponse](docs/MlSpamRuleResponse.md)
 - [MlSpamUpsertRequest](docs/MlSpamUpsertRequest.md)
 - [MlSpamUpsertRequestPartial](docs/MlSpamUpsertRequestPartial.md)
 - [ModalInteractionCallbackRequest](docs/ModalInteractionCallbackRequest.md)
 - [ModalInteractionCallbackRequestData](docs/ModalInteractionCallbackRequestData.md)
 - [ModalSubmitInteractionMetadataResponse](docs/ModalSubmitInteractionMetadataResponse.md)
 - [ModalSubmitInteractionMetadataResponseTriggeringInteractionMetadata](docs/ModalSubmitInteractionMetadataResponseTriggeringInteractionMetadata.md)
 - [MyGuildResponse](docs/MyGuildResponse.md)
 - [NewMemberActionResponse](docs/NewMemberActionResponse.md)
 - [OAuth2GetAuthorizationResponse](docs/OAuth2GetAuthorizationResponse.md)
 - [OAuth2GetKeys](docs/OAuth2GetKeys.md)
 - [OAuth2GetOpenIdConnectUserInfoResponse](docs/OAuth2GetOpenIdConnectUserInfoResponse.md)
 - [OAuth2Key](docs/OAuth2Key.md)
 - [OnboardingPromptOptionRequest](docs/OnboardingPromptOptionRequest.md)
 - [OnboardingPromptOptionResponse](docs/OnboardingPromptOptionResponse.md)
 - [OnboardingPromptResponse](docs/OnboardingPromptResponse.md)
 - [PartialDiscordIntegrationResponse](docs/PartialDiscordIntegrationResponse.md)
 - [PartialExternalConnectionIntegrationResponse](docs/PartialExternalConnectionIntegrationResponse.md)
 - [PartialGuildSubscriptionIntegrationResponse](docs/PartialGuildSubscriptionIntegrationResponse.md)
 - [PartnerSdkUnmergeProvisionalAccountRequest](docs/PartnerSdkUnmergeProvisionalAccountRequest.md)
 - [PinnedMessageResponse](docs/PinnedMessageResponse.md)
 - [PinnedMessagesResponse](docs/PinnedMessagesResponse.md)
 - [PollAnswerCreateRequest](docs/PollAnswerCreateRequest.md)
 - [PollAnswerDetailsResponse](docs/PollAnswerDetailsResponse.md)
 - [PollAnswerResponse](docs/PollAnswerResponse.md)
 - [PollCreateRequest](docs/PollCreateRequest.md)
 - [PollEmoji](docs/PollEmoji.md)
 - [PollEmojiCreateRequest](docs/PollEmojiCreateRequest.md)
 - [PollMedia](docs/PollMedia.md)
 - [PollMediaCreateRequest](docs/PollMediaCreateRequest.md)
 - [PollMediaResponse](docs/PollMediaResponse.md)
 - [PollResponse](docs/PollResponse.md)
 - [PollResultsEntryResponse](docs/PollResultsEntryResponse.md)
 - [PollResultsResponse](docs/PollResultsResponse.md)
 - [PongInteractionCallbackRequest](docs/PongInteractionCallbackRequest.md)
 - [PrivateApplicationResponse](docs/PrivateApplicationResponse.md)
 - [PrivateChannelLocation](docs/PrivateChannelLocation.md)
 - [PrivateChannelResponse](docs/PrivateChannelResponse.md)
 - [PrivateGroupChannelResponse](docs/PrivateGroupChannelResponse.md)
 - [PrivateGuildMemberResponse](docs/PrivateGuildMemberResponse.md)
 - [ProvisionalTokenResponse](docs/ProvisionalTokenResponse.md)
 - [PruneGuildRequest](docs/PruneGuildRequest.md)
 - [PruneGuildRequestIncludeRoles](docs/PruneGuildRequestIncludeRoles.md)
 - [PurchaseNotificationResponse](docs/PurchaseNotificationResponse.md)
 - [QuarantineUserAction](docs/QuarantineUserAction.md)
 - [QuarantineUserActionResponse](docs/QuarantineUserActionResponse.md)
 - [ResolvedObjectsResponse](docs/ResolvedObjectsResponse.md)
 - [ResourceChannelResponse](docs/ResourceChannelResponse.md)
 - [RichEmbed](docs/RichEmbed.md)
 - [RichEmbedAuthor](docs/RichEmbedAuthor.md)
 - [RichEmbedField](docs/RichEmbedField.md)
 - [RichEmbedFooter](docs/RichEmbedFooter.md)
 - [RichEmbedImage](docs/RichEmbedImage.md)
 - [RichEmbedProvider](docs/RichEmbedProvider.md)
 - [RichEmbedThumbnail](docs/RichEmbedThumbnail.md)
 - [RichEmbedVideo](docs/RichEmbedVideo.md)
 - [RoleSelectComponentForMessageRequest](docs/RoleSelectComponentForMessageRequest.md)
 - [RoleSelectComponentResponse](docs/RoleSelectComponentResponse.md)
 - [RoleSelectDefaultValue](docs/RoleSelectDefaultValue.md)
 - [RoleSelectDefaultValueResponse](docs/RoleSelectDefaultValueResponse.md)
 - [ScheduledEventResponse](docs/ScheduledEventResponse.md)
 - [ScheduledEventUserResponse](docs/ScheduledEventUserResponse.md)
 - [SdkMessageRequest](docs/SdkMessageRequest.md)
 - [SectionComponentForMessageRequest](docs/SectionComponentForMessageRequest.md)
 - [SectionComponentForMessageRequestAccessory](docs/SectionComponentForMessageRequestAccessory.md)
 - [SectionComponentResponse](docs/SectionComponentResponse.md)
 - [SectionComponentResponseAccessory](docs/SectionComponentResponseAccessory.md)
 - [SeparatorComponentForMessageRequest](docs/SeparatorComponentForMessageRequest.md)
 - [SeparatorComponentResponse](docs/SeparatorComponentResponse.md)
 - [SetChannelPermissionOverwriteRequest](docs/SetChannelPermissionOverwriteRequest.md)
 - [SetGuildApplicationCommandPermissionsRequest](docs/SetGuildApplicationCommandPermissionsRequest.md)
 - [SetGuildMfaLevelRequest](docs/SetGuildMfaLevelRequest.md)
 - [SettingsEmojiResponse](docs/SettingsEmojiResponse.md)
 - [SlackWebhook](docs/SlackWebhook.md)
 - [SoundboardCreateRequest](docs/SoundboardCreateRequest.md)
 - [SoundboardPatchRequestPartial](docs/SoundboardPatchRequestPartial.md)
 - [SoundboardSoundResponse](docs/SoundboardSoundResponse.md)
 - [SoundboardSoundSendRequest](docs/SoundboardSoundSendRequest.md)
 - [SpamLinkRuleResponse](docs/SpamLinkRuleResponse.md)
 - [StageInstanceResponse](docs/StageInstanceResponse.md)
 - [StageScheduledEventCreateRequest](docs/StageScheduledEventCreateRequest.md)
 - [StageScheduledEventPatchRequestPartial](docs/StageScheduledEventPatchRequestPartial.md)
 - [StageScheduledEventResponse](docs/StageScheduledEventResponse.md)
 - [StandardStickerResponse](docs/StandardStickerResponse.md)
 - [StickerPackCollectionResponse](docs/StickerPackCollectionResponse.md)
 - [StickerPackResponse](docs/StickerPackResponse.md)
 - [StringSelectComponentForMessageRequest](docs/StringSelectComponentForMessageRequest.md)
 - [StringSelectComponentResponse](docs/StringSelectComponentResponse.md)
 - [StringSelectOptionForMessageRequest](docs/StringSelectOptionForMessageRequest.md)
 - [StringSelectOptionResponse](docs/StringSelectOptionResponse.md)
 - [TeamMemberResponse](docs/TeamMemberResponse.md)
 - [TeamResponse](docs/TeamResponse.md)
 - [TextDisplayComponentForMessageRequest](docs/TextDisplayComponentForMessageRequest.md)
 - [TextDisplayComponentResponse](docs/TextDisplayComponentResponse.md)
 - [TextInputComponentForModalRequest](docs/TextInputComponentForModalRequest.md)
 - [TextInputComponentResponse](docs/TextInputComponentResponse.md)
 - [ThreadMemberResponse](docs/ThreadMemberResponse.md)
 - [ThreadMetadataResponse](docs/ThreadMetadataResponse.md)
 - [ThreadResponse](docs/ThreadResponse.md)
 - [ThreadSearchResponse](docs/ThreadSearchResponse.md)
 - [ThreadSearchTagParameter](docs/ThreadSearchTagParameter.md)
 - [ThreadsResponse](docs/ThreadsResponse.md)
 - [ThumbnailComponentForMessageRequest](docs/ThumbnailComponentForMessageRequest.md)
 - [ThumbnailComponentResponse](docs/ThumbnailComponentResponse.md)
 - [UnfurledMediaRequest](docs/UnfurledMediaRequest.md)
 - [UnfurledMediaRequestWithAttachmentReferenceRequired](docs/UnfurledMediaRequestWithAttachmentReferenceRequired.md)
 - [UnfurledMediaResponse](docs/UnfurledMediaResponse.md)
 - [UpdateApplicationEmojiRequest](docs/UpdateApplicationEmojiRequest.md)
 - [UpdateApplicationUserRoleConnectionRequest](docs/UpdateApplicationUserRoleConnectionRequest.md)
 - [UpdateAutoModerationRuleRequest](docs/UpdateAutoModerationRuleRequest.md)
 - [UpdateChannelRequest](docs/UpdateChannelRequest.md)
 - [UpdateDefaultReactionEmojiRequest](docs/UpdateDefaultReactionEmojiRequest.md)
 - [UpdateDmRequestPartial](docs/UpdateDmRequestPartial.md)
 - [UpdateGroupDmRequestPartial](docs/UpdateGroupDmRequestPartial.md)
 - [UpdateGuildChannelRequestPartial](docs/UpdateGuildChannelRequestPartial.md)
 - [UpdateGuildEmojiRequest](docs/UpdateGuildEmojiRequest.md)
 - [UpdateGuildMemberRequest](docs/UpdateGuildMemberRequest.md)
 - [UpdateGuildOnboardingRequest](docs/UpdateGuildOnboardingRequest.md)
 - [UpdateGuildScheduledEventRequest](docs/UpdateGuildScheduledEventRequest.md)
 - [UpdateGuildStickerRequest](docs/UpdateGuildStickerRequest.md)
 - [UpdateGuildTemplateRequest](docs/UpdateGuildTemplateRequest.md)
 - [UpdateGuildWidgetSettingsRequest](docs/UpdateGuildWidgetSettingsRequest.md)
 - [UpdateMessageInteractionCallbackRequest](docs/UpdateMessageInteractionCallbackRequest.md)
 - [UpdateMessageInteractionCallbackResponse](docs/UpdateMessageInteractionCallbackResponse.md)
 - [UpdateMyGuildMemberRequest](docs/UpdateMyGuildMemberRequest.md)
 - [UpdateOnboardingPromptRequest](docs/UpdateOnboardingPromptRequest.md)
 - [UpdateSelfVoiceStateRequest](docs/UpdateSelfVoiceStateRequest.md)
 - [UpdateStageInstanceRequest](docs/UpdateStageInstanceRequest.md)
 - [UpdateThreadRequestPartial](docs/UpdateThreadRequestPartial.md)
 - [UpdateThreadTagRequest](docs/UpdateThreadTagRequest.md)
 - [UpdateVoiceStateRequest](docs/UpdateVoiceStateRequest.md)
 - [UpdateWebhookByTokenRequest](docs/UpdateWebhookByTokenRequest.md)
 - [UpdateWebhookRequest](docs/UpdateWebhookRequest.md)
 - [UserAvatarDecorationResponse](docs/UserAvatarDecorationResponse.md)
 - [UserCollectiblesResponse](docs/UserCollectiblesResponse.md)
 - [UserCommunicationDisabledAction](docs/UserCommunicationDisabledAction.md)
 - [UserCommunicationDisabledActionMetadata](docs/UserCommunicationDisabledActionMetadata.md)
 - [UserCommunicationDisabledActionMetadataResponse](docs/UserCommunicationDisabledActionMetadataResponse.md)
 - [UserCommunicationDisabledActionResponse](docs/UserCommunicationDisabledActionResponse.md)
 - [UserGuildOnboardingResponse](docs/UserGuildOnboardingResponse.md)
 - [UserNameplateResponse](docs/UserNameplateResponse.md)
 - [UserPiiResponse](docs/UserPiiResponse.md)
 - [UserPrimaryGuildResponse](docs/UserPrimaryGuildResponse.md)
 - [UserResponse](docs/UserResponse.md)
 - [UserSelectComponentForMessageRequest](docs/UserSelectComponentForMessageRequest.md)
 - [UserSelectComponentResponse](docs/UserSelectComponentResponse.md)
 - [UserSelectDefaultValue](docs/UserSelectDefaultValue.md)
 - [UserSelectDefaultValueResponse](docs/UserSelectDefaultValueResponse.md)
 - [VanityUrlErrorResponse](docs/VanityUrlErrorResponse.md)
 - [VanityUrlResponse](docs/VanityUrlResponse.md)
 - [VoiceRegionResponse](docs/VoiceRegionResponse.md)
 - [VoiceScheduledEventCreateRequest](docs/VoiceScheduledEventCreateRequest.md)
 - [VoiceScheduledEventPatchRequestPartial](docs/VoiceScheduledEventPatchRequestPartial.md)
 - [VoiceScheduledEventResponse](docs/VoiceScheduledEventResponse.md)
 - [VoiceStateResponse](docs/VoiceStateResponse.md)
 - [WebhookSlackEmbed](docs/WebhookSlackEmbed.md)
 - [WebhookSlackEmbedField](docs/WebhookSlackEmbedField.md)
 - [WebhookSourceChannelResponse](docs/WebhookSourceChannelResponse.md)
 - [WebhookSourceGuildResponse](docs/WebhookSourceGuildResponse.md)
 - [WelcomeMessageResponse](docs/WelcomeMessageResponse.md)
 - [WelcomeScreenPatchRequestPartial](docs/WelcomeScreenPatchRequestPartial.md)
 - [WidgetActivity](docs/WidgetActivity.md)
 - [WidgetChannel](docs/WidgetChannel.md)
 - [WidgetMember](docs/WidgetMember.md)
 - [WidgetResponse](docs/WidgetResponse.md)
 - [WidgetSettingsResponse](docs/WidgetSettingsResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



