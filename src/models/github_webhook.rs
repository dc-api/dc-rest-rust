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
pub struct GithubWebhook {
    #[serde(rename = "action", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub action: Option<Option<String>>,
    #[serde(rename = "ref", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Option<String>>,
    #[serde(rename = "ref_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ref_type: Option<Option<String>>,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<Box<models::GithubComment>>>,
    #[serde(rename = "issue", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issue: Option<Option<Box<models::GithubIssue>>>,
    #[serde(rename = "pull_request", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Option<Box<models::GithubIssue>>>,
    #[serde(rename = "repository", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Option<Box<models::GithubRepository>>>,
    #[serde(rename = "forkee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forkee: Option<Option<Box<models::GithubRepository>>>,
    #[serde(rename = "sender")]
    pub sender: Box<models::GithubUser>,
    #[serde(rename = "member", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub member: Option<Option<Box<models::GithubUser>>>,
    #[serde(rename = "release", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release: Option<Option<Box<models::GithubRelease>>>,
    #[serde(rename = "head_commit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<Option<Box<models::GithubCommit>>>,
    #[serde(rename = "commits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commits: Option<Option<Vec<models::GithubCommit>>>,
    #[serde(rename = "forced", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forced: Option<Option<bool>>,
    #[serde(rename = "compare", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub compare: Option<Option<String>>,
    #[serde(rename = "review", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub review: Option<Option<Box<models::GithubReview>>>,
    #[serde(rename = "check_run", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub check_run: Option<Option<Box<models::GithubCheckRun>>>,
    #[serde(rename = "check_suite", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub check_suite: Option<Option<Box<models::GithubCheckSuite>>>,
    #[serde(rename = "discussion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discussion: Option<Option<Box<models::GithubDiscussion>>>,
    #[serde(rename = "answer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub answer: Option<Option<Box<models::GithubComment>>>,
}

impl GithubWebhook {
    pub fn new(sender: models::GithubUser) -> GithubWebhook {
        GithubWebhook {
            action: None,
            r#ref: None,
            ref_type: None,
            comment: None,
            issue: None,
            pull_request: None,
            repository: None,
            forkee: None,
            sender: Box::new(sender),
            member: None,
            release: None,
            head_commit: None,
            commits: None,
            forced: None,
            compare: None,
            review: None,
            check_run: None,
            check_suite: None,
            discussion: None,
            answer: None,
        }
    }
}

