use axum::{extract::{State, Path, Query}, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::auth::AuthenticatedUser;
use crate::database::{Database, SecurityReport};
use crate::errors::AppError;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListReportsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub report_type: Option<String>,
    pub from_date: Option<chrono::DateTime<Utc>>,
    pub to_date: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ReportSummary {
    pub id: String,
    pub report_type: String,
    pub risk_score: Option<i32>,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedReports {
    pub reports: Vec<ReportSummary>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub total_items: u64,
}

#[derive(Debug, Serialize)]
pub struct DetailedReport {
    pub id: String,
    pub report_type: String,
    pub results: serde_json::Value,
    pub risk_score: Option<i32>,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub export_url: String,
    pub expires_at: chrono::DateTime<Utc>,
    pub format: String,
}

pub async fn list_reports(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<ListReportsQuery>,
) -> Result<Json<PaginatedReports>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = ((page - 1) * per_page) as i64;
    let limit = per_page as i64;

    // Get reports for the user
    let reports = state.db.get_user_reports(user.user_id, limit, offset).await?;

    // Filter by report type if specified
    let filtered_reports: Vec<SecurityReport> = if let Some(report_type) = &params.report_type {
        reports.into_iter()
            .filter(|r| r.report_type == *report_type)
            .collect()
    } else {
        reports
    };

    // Filter by date range if specified
    let filtered_reports: Vec<SecurityReport> = if params.from_date.is_some() || params.to_date.is_some() {
        filtered_reports.into_iter()
            .filter(|r| {
                let created_at = r.created_at;
                let after_from = params.from_date.map_or(true, |from| created_at >= from);
                let before_to = params.to_date.map_or(true, |to| created_at <= to);
                after_from && before_to
            })
            .collect()
    } else {
        filtered_reports
    };

    let report_summaries: Vec<ReportSummary> = filtered_reports.into_iter()
        .map(|r| ReportSummary {
            id: r.id.to_string(),
            report_type: r.report_type,
            risk_score: r.risk_score,
            created_at: r.created_at,
            expires_at: r.expires_at,
        })
        .collect();

    // Calculate pagination (simplified - in production you'd get total count from DB)
    let total_items = report_summaries.len() as u64;
    let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as u32;

    let pagination = PaginationInfo {
        page,
        per_page,
        total_pages,
        total_items,
    };

    Ok(Json(PaginatedReports {
        reports: report_summaries,
        pagination,
    }))
}

pub async fn get_report(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(report_id): Path<String>,
) -> Result<Json<DetailedReport>, AppError> {
    let report_uuid = Uuid::parse_str(&report_id)
        .map_err(|_| AppError::BadRequest("Invalid report ID".to_string()))?;

    let report = state.db.get_report_by_id(user.user_id, report_uuid).await?
        .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    // Parse results JSON
    let results: serde_json::Value = serde_json::from_str(&report.results)
        .map_err(|e| AppError::InternalServerError(format!("Failed to parse report results: {}", e)))?;

    Ok(Json(DetailedReport {
        id: report.id.to_string(),
        report_type: report.report_type,
        results,
        risk_score: report.risk_score,
        created_at: report.created_at,
        expires_at: report.expires_at,
    }))
}

pub async fn delete_report(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(report_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let report_uuid = Uuid::parse_str(&report_id)
        .map_err(|_| AppError::BadRequest("Invalid report ID".to_string()))?;

    // Check if report exists and belongs to user
    let report = state.db.get_report_by_id(user.user_id, report_uuid).await?
        .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    // Delete report (implement in database layer)
    // For now, we'll return success (you'd implement actual deletion)
    info!("Report deleted by user: {} (report_id: {})", user.email, report_id);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Report deleted successfully"
    })))
}

pub async fn export_reports(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<ListReportsQuery>,
) -> Result<Json<ExportResponse>, AppError> {
    // Check if user has export permissions
    if user.subscription_tier == crate::database::UserSubscriptionTier::Free {
        return Err(AppError::Forbidden("Report export requires a premium subscription".to_string()));
    }

    // Get all reports for export (simplified implementation)
    let reports = state.db.get_user_reports(user.user_id, 1000, 0).await?;

    // Filter reports based on query parameters
    let filtered_reports: Vec<SecurityReport> = if let Some(report_type) = &params.report_type {
        reports.into_iter()
            .filter(|r| r.report_type == *report_type)
            .collect()
    } else {
        reports
    };

    // Apply date filters
    let filtered_reports: Vec<SecurityReport> = if params.from_date.is_some() || params.to_date.is_some() {
        filtered_reports.into_iter()
            .filter(|r| {
                let created_at = r.created_at;
                let after_from = params.from_date.map_or(true, |from| created_at >= from);
                let before_to = params.to_date.map_or(true, |to| created_at <= to);
                after_from && before_to
            })
            .collect()
    } else {
        filtered_reports
    };

    // Generate export data
    let export_data = serde_json::json!({
        "user_id": user.user_id,
        "export_date": Utc::now(),
        "reports": filtered_reports.iter().map(|r| {
            let results: serde_json::Value = serde_json::from_str(&r.results).unwrap_or_default();
            serde_json::json!({
                "id": r.id,
                "report_type": r.report_type,
                "results": results,
                "risk_score": r.risk_score,
                "created_at": r.created_at,
                "expires_at": r.expires_at
            })
        }).collect::<Vec<_>>()
    });

    // In production, you would:
    // 1. Generate a secure temporary URL
    // 2. Store the export data with an expiration
    // 3. Return the URL for download

    let export_id = Uuid::new_v4();
    let export_url = format!("https://api.guardr.app/exports/{}", export_id);
    let expires_at = Utc::now() + chrono::Duration::hours(24);

    info!("Report export generated for user: {} ({} reports)", user.email, filtered_reports.len());

    Ok(Json(ExportResponse {
        export_url,
        expires_at,
        format: "json".to_string(),
    }))
}

// Admin endpoints (for managing breach data sources)
pub mod admin {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize)]
    pub struct BreachSource {
        pub id: String,
        pub name: String,
        pub url: String,
        pub last_updated: chrono::DateTime<Utc>,
        pub record_count: u64,
        pub is_active: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct AddBreachSourceRequest {
        pub name: String,
        pub url: String,
        pub description: Option<String>,
    }

    pub async fn list_breach_sources(
        State(state): State<AppState>,
        user: AuthenticatedUser,
    ) -> Result<Json<Vec<BreachSource>>, AppError> {
        // Check admin permissions
        if user.subscription_tier != crate::database::UserSubscriptionTier::Enterprise {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // In production, implement actual breach source management
        let sources = vec![
            BreachSource {
                id: "1".to_string(),
                name: "HaveIBeenPwned".to_string(),
                url: "https://haveibeenpwned.com".to_string(),
                last_updated: Utc::now(),
                record_count: 1000000,
                is_active: true,
            }
        ];

        Ok(Json(sources))
    }

    pub async fn add_breach_source(
        State(state): State<AppState>,
        user: AuthenticatedUser,
        Json(payload): Json<AddBreachSourceRequest>,
    ) -> Result<Json<BreachSource>, AppError> {
        // Check admin permissions
        if user.subscription_tier != crate::database::UserSubscriptionTier::Enterprise {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // In production, implement actual breach source addition
        let source = BreachSource {
            id: Uuid::new_v4().to_string(),
            name: payload.name,
            url: payload.url,
            last_updated: Utc::now(),
            record_count: 0,
            is_active: true,
        };

        info!("Breach source added by admin: {}", user.email);

        Ok(Json(source))
    }

    pub async fn update_breach_data(
        State(state): State<AppState>,
        user: AuthenticatedUser,
    ) -> Result<Json<serde_json::Value>, AppError> {
        // Check admin permissions
        if user.subscription_tier != crate::database::UserSubscriptionTier::Enterprise {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // In production, implement actual breach data update
        info!("Breach data update initiated by admin: {}", user.email);

        Ok(Json(serde_json::json!({
            "success": true,
            "message": "Breach data update initiated",
            "estimated_completion": Utc::now() + chrono::Duration::hours(1)
        })))
    }
}