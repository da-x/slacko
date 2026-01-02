# Integration Test Coverage Plan

## Current State
- **Methods Implemented**: 148
- **Methods Tested**: 71
- **Current Coverage**: 48%
- **Target Coverage**: 100%

---

## Phase 1: Quick Wins (Easy Tests)
**Effort**: Low | **Impact**: +15 methods | **New Coverage**: 58%

These methods can be tested with any valid Slack token.

### 1.1 api_test & bots (2 methods)
```
tests/integration_api.rs (NEW)
- api.test() - Simple API connectivity test
- bots.info() - Get bot info (may need bot ID from auth.test)
```

### 1.2 chat additions (4 methods)
```
tests/integration_chat.rs (ADD)
- me_message() - Send /me message to self-DM
- scheduled_messages_list() - List scheduled messages (may be empty)
- delete_scheduled_message() - Already called in schedule test, make explicit
- unfurl_with_options() - Variant of existing unfurl test
```

### 1.3 conversations core (9 methods)
```
tests/integration_conversations.rs (ADD)
- create() + archive() - Create temp channel, archive it
- unarchive() - Unarchive the temp channel, then delete
- rename() - Rename temp channel
- set_purpose() - Set purpose on temp channel
- set_topic() - Set topic on temp channel
- mark() - Mark channel as read
- close() - Close self-DM
- replies() - Get thread replies (use existing thread test)
```

---

## Phase 2: Files & Users (Medium Effort)
**Effort**: Medium | **Impact**: +12 methods | **New Coverage**: 66%

### 2.1 files additions (8 methods)
```
tests/integration_files.rs (ADD)
- upload_to_thread() - Upload with thread_ts option
- upload_with_options() - Upload with FileUploadOptions
- remote_add() - Register external URL as file
- remote_info() - Get remote file info
- remote_list() - List remote files
- remote_remove() - Remove remote file
- remote_share() - Share remote file
- get_upload_url_external() + complete_upload_external() - v2 upload flow
- comments_delete() - May fail (deprecated API) but should test
- revoke_public_url() - Revoke after sharedPublicURL
```

### 2.2 users additions (4 methods)
```
tests/integration_users.rs (ADD)
- identity() - Get OAuth identity (may fail without identity.basic scope)
- set_profile() - Set profile field (may require user token)
- list_with_options() - Paginated user list
- conversations_with_options() - Already have basic, add options test
- discoverable_contacts_lookup() - May fail (enterprise feature)
```

---

## Phase 3: Remaining Core APIs
**Effort**: Medium | **Impact**: +10 methods | **New Coverage**: 73%

### 3.1 reactions (1 method)
```
tests/integration_reactions.rs (ADD)
- list_with_options() - Paginated reactions list
```

### 3.2 reminders (3 methods)
```
tests/integration_reminders.rs (ADD)
- list_saved() - May return empty or error (internal API)
- list_saved_with_filter() - Same
- list_saved_page() - Same
```

### 3.3 search (2 methods)
```
tests/integration_search.rs (ADD)
- all_with_options() - Full options variant
- modules() - If exists, test it
```

### 3.4 emoji (2 methods)
```
tests/integration_emoji.rs (ADD)
- add_alias() - Create alias for existing emoji
- rename() - Rename custom emoji
```

### 3.5 rtm (1 method)
```
tests/integration_rtm.rs (ADD)
- start() - Deprecated but should test (returns URL)
```

### 3.6 usergroups (3 methods)
```
tests/integration_usergroups.rs (ADD)
- users_update() - Update usergroup members
```

---

## Phase 4: Apps & Workflows
**Effort**: Medium-High | **Impact**: +8 methods | **New Coverage**: 78%

### 4.1 apps (6 methods)
```
tests/integration_apps.rs (ADD)
- uninstall() - Will fail (can't uninstall self) but test error
- permissions_request() - Will fail (needs user interaction)
- manifest_create() - May need app config token
- manifest_delete() - May need app config token
- manifest_export() - May need app config token
- manifest_update() - May need app config token
```

### 4.2 workflows (2 methods)
```
tests/integration_workflows.rs (ADD)
- step_failed() - Test error case (no workflow context)
- update_step() - Test error case (no workflow context)
```

---

## Phase 5: Views & Admin (Hard)
**Effort**: High | **Impact**: +6 methods | **New Coverage**: 82%

### 5.1 views (3 methods)
```
tests/integration_views.rs (ADD)
- open() - Requires trigger_id (test error case)
- push() - Requires trigger_id (test error case)
- update() - Requires view_id (test error case)
```

### 5.2 admin (11 methods) - Enterprise Only
```
tests/integration_admin.rs (ADD)
Most will fail without admin token, but test error handling:
- apps.approve/restrict
- users.invite/remove/set_admin
- teams.create
- conversations.archive/delete
```

---

## Phase 6: Slack Connect & Special Features
**Effort**: Very High | **Impact**: +13 methods | **New Coverage**: 91%

### 6.1 conversations Slack Connect (8 methods)
```
tests/integration_conversations.rs (ADD)
All require Slack Connect (may not be available):
- accept_shared_invite() - Test error case
- approve_shared_invite() - Test error case
- decline_shared_invite() - Test error case
- invite_shared() - Test error case
- list_connect_invites() - May return empty
- request_shared_invite_approve() - Test error case
- request_shared_invite_deny() - Test error case
- request_shared_invite_list() - May return empty
```

### 6.2 conversations Canvas & Permissions (2 methods)
```
- canvases_create() - May require canvas feature
- external_invite_permissions_set() - May require Slack Connect
```

### 6.3 chat streaming (3 methods)
```
tests/integration_chat.rs (ADD)
Requires AI agent app capabilities:
- start_stream() - Test error case or success if available
- append_stream() - Test error case
- stop_stream() - Test error case
```

---

## Phase 7: OAuth/OIDC (Error Cases Only)
**Effort**: Low | **Impact**: +3 methods | **New Coverage**: 93%

These require browser OAuth flow - just test error handling.

```
tests/integration_oauth.rs (ADD)
- token_revoke() - Test error without valid refresh token

tests/integration_openid.rs (ADD)
- token_refresh() - Test error without valid refresh token
```

---

## Phase 8: Users Photo APIs
**Effort**: Medium | **Impact**: +3 methods | **New Coverage**: 95%

```
tests/integration_users.rs (ADD)
- delete_photo() - Delete profile photo
- set_photo() - Upload new photo (need image bytes)
- set_photo_with_crop() - Upload with crop params
```

---

## Implementation Order (Recommended)

| Phase | Tests to Add | Cumulative Coverage |
|-------|-------------|---------------------|
| 1 | 15 | 58% |
| 2 | 12 | 66% |
| 3 | 10 | 73% |
| 4 | 8 | 78% |
| 5 | 6 | 82% |
| 6 | 13 | 91% |
| 7 | 3 | 93% |
| 8 | 3 | 95% |

**Remaining 5%**: Methods that truly cannot be tested without:
- Active Slack Connect partnership
- Enterprise Grid admin access
- Real-time webhook triggers
- Browser-based OAuth flows

---

## Test File Summary

| File | Current Tests | Tests to Add |
|------|---------------|--------------|
| integration_api.rs | NEW | 2 |
| integration_chat.rs | 11 | 7 |
| integration_conversations.rs | 8 | 19 |
| integration_files.rs | 5 | 10 |
| integration_users.rs | 8 | 7 |
| integration_reactions.rs | 4 | 1 |
| integration_reminders.rs | 6 | 3 |
| integration_emoji.rs | 4 | 2 |
| integration_rtm.rs | 1 | 1 |
| integration_usergroups.rs | 4 | 1 |
| integration_apps.rs | 4 | 6 |
| integration_workflows.rs | 2 | 2 |
| integration_views.rs | 2 | 3 |
| integration_admin.rs | 1 | 8 |
| integration_oauth.rs | 2 | 1 |
| integration_openid.rs | 2 | 1 |

**Total New Tests**: ~77 test functions
**Estimated LOC**: ~2000 lines

---

## Notes

1. **Error case testing is valid** - Testing that methods fail gracefully with appropriate errors counts as coverage
2. **Skip macros** - Use `skip_if_no_client!` pattern for optional tests
3. **Cleanup** - Always clean up created resources (channels, files, etc.)
4. **Self-DM pattern** - Use self-DM for message tests to avoid spam
5. **Enterprise features** - Document when features require enterprise/admin tokens
