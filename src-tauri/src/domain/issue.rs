use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Provider-neutral issue representation used throughout the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    /// Internal UUID for local-first identity.
    pub id: String,
    /// Jira issue key (e.g. "TPT-7042").
    pub key: String,
    /// One-line summary / title.
    pub summary: String,
    /// Full description (markdown).
    pub description: Option<String>,
    /// Provider-agnostic status value.
    pub status: IssueStatus,
    /// Priority level.
    pub priority: IssuePriority,
    /// Project key (e.g. "TPT").
    pub project_key: String,
    /// Human-readable project name.
    pub project_name: String,
    /// Display name of the assignee.
    pub assignee: Option<String>,
    /// Display name of the reporter.
    pub reporter: Option<String>,
    /// Labels attached to the issue.
    pub labels: Vec<String>,
    /// Sprint name if the issue belongs to one.
    pub sprint: Option<String>,
    /// Due date if set.
    pub due_date: Option<NaiveDate>,
    /// When the issue was created in the provider.
    pub created_at: DateTime<Utc>,
    /// When the issue was last updated in the provider.
    pub updated_at: DateTime<Utc>,
    /// When this record was last synced from the provider.
    pub synced_at: DateTime<Utc>,
    /// Direct URL to open the issue in the provider's web UI.
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    Todo,
    InProgress,
    Review,
    Done,
    Unknown,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Todo => "To Do",
            Self::InProgress => "In Progress",
            Self::Review => "Review",
            Self::Done => "Done",
            Self::Unknown => "Unknown",
        }
    }

    pub fn from_jira(raw: &str) -> Self {
        match raw.to_lowercase().as_str() {
            "to do" | "todo" | "open" | "new" | "backlog" => Self::Todo,
            "in progress" | "in development" | "active" => Self::InProgress,
            "review" | "in review" | "code review" => Self::Review,
            "done" | "closed" | "resolved" | "complete" | "completed" => Self::Done,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuePriority {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
    Unknown,
}

impl IssuePriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Highest => "Highest",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
            Self::Lowest => "Lowest",
            Self::Unknown => "Unknown",
        }
    }

    pub fn from_jira(raw: &str) -> Self {
        match raw.to_lowercase().as_str() {
            "highest" | "blocker" | "critical" => Self::Highest,
            "high" | "major" => Self::High,
            "medium" | "normal" => Self::Medium,
            "low" | "minor" => Self::Low,
            "lowest" | "trivial" => Self::Lowest,
            _ => Self::Unknown,
        }
    }
}

/// Lightweight view model sent to the frontend over IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueView {
    pub id: String,
    pub key: String,
    pub summary: String,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub project_key: String,
    pub project_name: String,
    pub assignee: Option<String>,
    pub labels: Vec<String>,
    pub sprint: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub updated_at: DateTime<Utc>,
    pub web_url: Option<String>,
}

impl From<&Issue> for IssueView {
    fn from(issue: &Issue) -> Self {
        Self {
            id: issue.id.clone(),
            key: issue.key.clone(),
            summary: issue.summary.clone(),
            status: issue.status,
            priority: issue.priority,
            project_key: issue.project_key.clone(),
            project_name: issue.project_name.clone(),
            assignee: issue.assignee.clone(),
            labels: issue.labels.clone(),
            sprint: issue.sprint.clone(),
            due_date: issue.due_date,
            updated_at: issue.updated_at,
            web_url: issue.web_url.clone(),
        }
    }
}

/// Filters that the frontend can apply when querying issues.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFilter {
    pub status: Option<Vec<IssueStatus>>,
    pub priority: Option<Vec<IssuePriority>>,
    pub project_key: Option<String>,
    pub sprint: Option<String>,
    pub label: Option<String>,
    pub search: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_from_jira_maps_common_values() {
        assert_eq!(IssueStatus::from_jira("To Do"), IssueStatus::Todo);
        assert_eq!(
            IssueStatus::from_jira("IN PROGRESS"),
            IssueStatus::InProgress
        );
        assert_eq!(IssueStatus::from_jira("Done"), IssueStatus::Done);
        assert_eq!(IssueStatus::from_jira("weird"), IssueStatus::Unknown);
    }

    #[test]
    fn priority_from_jira_maps_common_values() {
        assert_eq!(IssuePriority::from_jira("Highest"), IssuePriority::Highest);
        assert_eq!(IssuePriority::from_jira("blocker"), IssuePriority::Highest);
        assert_eq!(IssuePriority::from_jira("Minor"), IssuePriority::Low);
        assert_eq!(IssuePriority::from_jira("nope"), IssuePriority::Unknown);
    }
}
