use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::auth::AuthenticatedUser;
use crate::errors::AppError;
use crate::state::AppState;

// Request/Response models for dating safety analysis

#[derive(Debug, Deserialize)]
pub struct ConversationAnalysisRequest {
    pub messages: Vec<Message>,
    pub participant_info: Option<ParticipantClaims>,
    pub analysis_depth: Option<String>, // "basic", "detailed", "comprehensive"
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub sender: String, // "user" or "match"
}

#[derive(Debug, Deserialize)]
pub struct ParticipantClaims {
    pub name: Option<String>,
    pub age: Option<u8>,
    pub location: Option<String>,
    pub occupation: Option<String>,
    pub education: Option<String>,
    pub social_media: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct SafetyAnalysisResponse {
    pub overall_risk_score: f32,
    pub risk_level: String, // "low", "medium", "high", "critical"
    pub risk_indicators: Vec<RiskIndicator>,
    pub identity_consistency: IdentityConsistency,
    pub conversation_patterns: ConversationPatterns,
    pub red_flags: Vec<RedFlag>,
    pub recommendations: Vec<String>,
    pub safety_tips: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RiskIndicator {
    pub indicator_type: String,
    pub confidence: f32,
    pub severity: String,
    pub description: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct IdentityConsistency {
    pub consistency_score: f32,
    pub inconsistencies: Vec<String>,
    pub verified_claims: u32,
    pub unverified_claims: u32,
    pub suspicious_patterns: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ConversationPatterns {
    pub love_bombing_score: f32,
    pub pressure_indicators: u32,
    pub manipulation_tactics: Vec<String>,
    pub response_time_analysis: ResponseTimeAnalysis,
    pub communication_style: CommunicationStyle,
}

#[derive(Debug, Serialize)]
pub struct ResponseTimeAnalysis {
    pub avg_response_time: f32, // in hours
    pub suspicious_patterns: Vec<String>,
    pub consistency_score: f32,
}

#[derive(Debug, Serialize)]
pub struct CommunicationStyle {
    pub formality_level: String,
    pub emotional_intensity: f32,
    pub topic_steering: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RedFlag {
    pub flag_type: String,
    pub severity: String,
    pub description: String,
    pub action_required: String,
}

#[derive(Debug, Deserialize)]
pub struct IdentityVerificationRequest {
    pub participant_claims: ParticipantClaims,
    pub verification_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct IdentityVerificationResponse {
    pub verification_score: f32,
    pub verified_claims: Vec<VerifiedClaim>,
    pub unverified_claims: Vec<String>,
    pub suspicious_indicators: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct VerifiedClaim {
    pub claim_type: String,
    pub claim_value: String,
    pub confidence: f32,
    pub verification_method: String,
}

#[derive(Debug, Deserialize)]
pub struct SafetyReportRequest {
    pub conversation_analysis: Option<ConversationAnalysisRequest>,
    pub identity_verification: Option<IdentityVerificationRequest>,
    pub additional_context: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ComprehensiveSafetyReport {
    pub report_id: String,
    pub overall_safety_score: f32,
    pub risk_level: String,
    pub conversation_analysis: Option<SafetyAnalysisResponse>,
    pub identity_verification: Option<IdentityVerificationResponse>,
    pub aggregated_recommendations: Vec<String>,
    pub safety_action_plan: Vec<SafetyAction>,
    pub emergency_contacts: Vec<EmergencyResource>,
    pub generated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SafetyAction {
    pub priority: String, // "immediate", "high", "medium", "low"
    pub action: String,
    pub description: String,
    pub timeframe: String,
}

#[derive(Debug, Serialize)]
pub struct EmergencyResource {
    pub resource_type: String,
    pub name: String,
    pub contact: String,
    pub availability: String,
}

// Helper functions for analysis
fn analyze_love_bombing_patterns(messages: &[Message]) -> f32 {
    let mut score = 0.0;
    let total_messages = messages.len() as f32;
    
    if total_messages == 0.0 {
        return 0.0;
    }

    // Check for excessive compliments
    let compliment_keywords = ["beautiful", "gorgeous", "perfect", "amazing", "incredible", "soulmate", "destiny"];
    let mut compliment_count = 0;

    // Check for intensity escalation
    let intense_keywords = ["love", "forever", "marry", "soul", "meant to be", "destiny"];
    let mut intensity_count = 0;

    for message in messages {
        let content_lower = message.content.to_lowercase();
        
        for keyword in &compliment_keywords {
            if content_lower.contains(keyword) {
                compliment_count += 1;
            }
        }
        
        for keyword in &intense_keywords {
            if content_lower.contains(keyword) {
                intensity_count += 1;
            }
        }
    }

    // Calculate love bombing score
    let compliment_ratio = (compliment_count as f32) / total_messages;
    let intensity_ratio = (intensity_count as f32) / total_messages;
    
    score = (compliment_ratio * 50.0) + (intensity_ratio * 40.0);
    score.min(100.0)
}

fn detect_pressure_indicators(messages: &[Message]) -> u32 {
    let pressure_patterns = [
        "meet tonight", "come over now", "don't tell anyone", "delete this",
        "send money", "urgent", "emergency", "secret", "private", "alone"
    ];

    let mut pressure_count = 0;

    for message in messages {
        let content_lower = message.content.to_lowercase();
        for pattern in &pressure_patterns {
            if content_lower.contains(pattern) {
                pressure_count += 1;
            }
        }
    }

    pressure_count
}

fn analyze_consistency(claims: &ParticipantClaims, messages: &[Message]) -> IdentityConsistency {
    let mut inconsistencies = Vec::new();
    let mut verified_claims = 0;
    let mut unverified_claims = 0;
    let mut suspicious_patterns = Vec::new();

    // Simple consistency checks (in production, this would be much more sophisticated)
    if let Some(name) = &claims.name {
        let name_lower = name.to_lowercase();
        let name_mentioned = messages.iter().any(|m| 
            m.content.to_lowercase().contains(&name_lower)
        );
        
        if name_mentioned {
            verified_claims += 1;
        } else {
            unverified_claims += 1;
            inconsistencies.push("Name not mentioned in conversation".to_string());
        }
    }

    if let Some(age) = claims.age {
        if !(18..=80).contains(&age) {
            suspicious_patterns.push("Unusual age range".to_string());
        }
    }

    let consistency_score = if verified_claims + unverified_claims > 0 {
        (verified_claims as f32) / ((verified_claims + unverified_claims) as f32) * 100.0
    } else {
        0.0
    };

    IdentityConsistency {
        consistency_score,
        inconsistencies,
        verified_claims,
        unverified_claims,
        suspicious_patterns,
    }
}

fn calculate_response_time_analysis(messages: &[Message]) -> ResponseTimeAnalysis {
    let mut response_times = Vec::new();
    let mut suspicious_patterns = Vec::new();

    // Calculate response times between messages
    for window in messages.windows(2) {
        if window[0].sender != window[1].sender {
            let time_diff = window[1].timestamp.signed_duration_since(window[0].timestamp);
            let hours = time_diff.num_milliseconds() as f32 / (1000.0 * 60.0 * 60.0);
            response_times.push(hours);
        }
    }

    let avg_response_time = if response_times.is_empty() {
        0.0
    } else {
        response_times.iter().sum::<f32>() / response_times.len() as f32
    };

    // Check for suspicious patterns
    if response_times.iter().all(|&t| t < 0.01) {
        suspicious_patterns.push("Unusually fast responses (possible bot)".to_string());
    }

    if response_times.iter().any(|&t| t > 72.0) {
        suspicious_patterns.push("Inconsistent response times".to_string());
    }

    let consistency_score = if response_times.len() > 1 {
        let variance = response_times.iter()
            .map(|t| (t - avg_response_time).powi(2))
            .sum::<f32>() / response_times.len() as f32;
        (100.0 - variance.sqrt().min(100.0)).max(0.0)
    } else {
        100.0
    };

    ResponseTimeAnalysis {
        avg_response_time,
        suspicious_patterns,
        consistency_score,
    }
}

// API Handlers
pub async fn analyze_conversation(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<ConversationAnalysisRequest>,
) -> Result<Json<SafetyAnalysisResponse>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "analyze_conversation").await?;

    // Check subscription limits
    if user.subscription_tier == crate::database::UserSubscriptionTier::Free && payload.messages.len() > 50 {
        return Err(AppError::BadRequest("Free tier limited to 50 messages per analysis".to_string()));
    }

    // Perform safety analysis
    let love_bombing_score = analyze_love_bombing_patterns(&payload.messages);
    let pressure_indicators = detect_pressure_indicators(&payload.messages);
    
    let identity_consistency = if let Some(claims) = &payload.participant_info {
        analyze_consistency(claims, &payload.messages)
    } else {
        IdentityConsistency {
            consistency_score: 0.0,
            inconsistencies: vec!["No participant information provided".to_string()],
            verified_claims: 0,
            unverified_claims: 0,
            suspicious_patterns: Vec::new(),
        }
    };

    let response_time_analysis = calculate_response_time_analysis(&payload.messages);

    // Calculate overall risk score
    let mut risk_score = 0.0;
    risk_score += love_bombing_score * 0.3;
    risk_score += (pressure_indicators as f32) * 10.0;
    risk_score += (100.0 - identity_consistency.consistency_score) * 0.2;
    risk_score += (100.0 - response_time_analysis.consistency_score) * 0.1;

    let risk_level = match risk_score {
        0.0..=25.0 => "low",
        25.1..=50.0 => "medium",
        50.1..=75.0 => "high",
        _ => "critical",
    }.to_string();

    // Generate risk indicators
    let mut risk_indicators = Vec::new();
    
    if love_bombing_score > 30.0 {
        risk_indicators.push(RiskIndicator {
            indicator_type: "love_bombing".to_string(),
            confidence: love_bombing_score / 100.0,
            severity: if love_bombing_score > 60.0 { "high" } else { "medium" }.to_string(),
            description: "Excessive romantic language detected".to_string(),
            evidence: vec!["Multiple intense compliments".to_string()],
        });
    }

    if pressure_indicators > 2 {
        risk_indicators.push(RiskIndicator {
            indicator_type: "pressure_tactics".to_string(),
            confidence: 0.8,
            severity: "high".to_string(),
            description: "Pressure tactics detected in conversation".to_string(),
            evidence: vec![format!("{} pressure indicators found", pressure_indicators)],
        });
    }

    // Generate red flags
    let mut red_flags = Vec::new();
    
    if pressure_indicators > 5 {
        red_flags.push(RedFlag {
            flag_type: "immediate_danger".to_string(),
            severity: "critical".to_string(),
            description: "Multiple high-pressure tactics detected".to_string(),
            action_required: "Consider ending communication immediately".to_string(),
        });
    }

    if identity_consistency.consistency_score < 30.0 {
        red_flags.push(RedFlag {
            flag_type: "identity_inconsistency".to_string(),
            severity: "medium".to_string(),
            description: "Significant inconsistencies in identity claims".to_string(),
            action_required: "Verify identity before meeting".to_string(),
        });
    }

    // Generate recommendations
    let mut recommendations = Vec::new();
    let mut safety_tips = Vec::new();

    match risk_level.as_str() {
        "critical" => {
            recommendations.push("Consider ending this communication immediately".to_string());
            recommendations.push("Block this person on all platforms".to_string());
            recommendations.push("Report this profile to the dating platform".to_string());
            safety_tips.push("Trust your instincts - if something feels wrong, it probably is".to_string());
        }
        "high" => {
            recommendations.push("Proceed with extreme caution".to_string());
            recommendations.push("Verify identity through video call before meeting".to_string());
            recommendations.push("Always meet in public places".to_string());
            safety_tips.push("Tell friends/family about your plans".to_string());
        }
        "medium" => {
            recommendations.push("Take additional safety precautions".to_string());
            recommendations.push("Verify identity before meeting".to_string());
            safety_tips.push("Meet in public places during daytime".to_string());
        }
        _ => {
            recommendations.push("Continue with normal safety precautions".to_string());
            safety_tips.push("Always inform someone of your whereabouts".to_string());
        }
    }

    let conversation_patterns = ConversationPatterns {
        love_bombing_score,
        pressure_indicators,
        manipulation_tactics: Vec::new(), // Would be implemented with more sophisticated analysis
        response_time_analysis,
        communication_style: CommunicationStyle {
            formality_level: "casual".to_string(), // Would be analyzed
            emotional_intensity: love_bombing_score / 100.0,
            topic_steering: Vec::new(), // Would be analyzed
        },
    };

    // Store analysis report
    let report_data = serde_json::json!({
        "conversation_analysis": {
            "overall_risk_score": risk_score,
            "risk_level": risk_level,
            "risk_indicators": risk_indicators,
            "identity_consistency": identity_consistency,
            "conversation_patterns": conversation_patterns,
            "red_flags": red_flags,
            "recommendations": recommendations
        }
    });

    let input_hash = format!("conversation_{}", uuid::Uuid::new_v4());
    state.db.create_security_report(
        user.user_id,
        "conversation_analysis",
        &input_hash,
        &report_data.to_string(),
        Some(risk_score as i32),
    ).await?;

    info!("Conversation analysis completed for user: {} (risk: {})", user.email, risk_level);

    Ok(Json(SafetyAnalysisResponse {
        overall_risk_score: risk_score,
        risk_level,
        risk_indicators,
        identity_consistency,
        conversation_patterns,
        red_flags,
        recommendations,
        safety_tips,
    }))
}

pub async fn verify_identity_claims(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<IdentityVerificationRequest>,
) -> Result<Json<IdentityVerificationResponse>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "verify_identity").await?;

    // Simple identity verification (in production, this would integrate with external services)
    let mut verified_claims = Vec::new();
    let mut unverified_claims = Vec::new();
    let mut suspicious_indicators = Vec::new();

    if let Some(name) = &payload.participant_claims.name {
        verified_claims.push(VerifiedClaim {
            claim_type: "name".to_string(),
            claim_value: name.clone(),
            confidence: 0.5, // Placeholder
            verification_method: "basic_validation".to_string(),
        });
    }

    if let Some(age) = payload.participant_claims.age {
        if !(18..=80).contains(&age) {
            suspicious_indicators.push("Age outside typical range".to_string());
        }
        verified_claims.push(VerifiedClaim {
            claim_type: "age".to_string(),
            claim_value: age.to_string(),
            confidence: 0.7,
            verification_method: "range_validation".to_string(),
        });
    }

    if payload.participant_claims.location.is_none() {
        unverified_claims.push("location".to_string());
    }

    if payload.participant_claims.occupation.is_none() {
        unverified_claims.push("occupation".to_string());
    }

    let verification_score = if verified_claims.len() + unverified_claims.len() > 0 {
        (verified_claims.len() as f32) / ((verified_claims.len() + unverified_claims.len()) as f32) * 100.0
    } else {
        0.0
    };

    let recommendations = vec![
        "Request additional verification through video call".to_string(),
        "Cross-reference social media profiles".to_string(),
        "Verify location claims during conversation".to_string(),
    ];

    info!("Identity verification completed for user: {} (score: {})", user.email, verification_score);

    Ok(Json(IdentityVerificationResponse {
        verification_score,
        verified_claims,
        unverified_claims,
        suspicious_indicators,
        recommendations,
    }))
}

pub async fn generate_safety_report(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<SafetyReportRequest>,
) -> Result<Json<ComprehensiveSafetyReport>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "safety_report").await?;

    let report_id = uuid::Uuid::new_v4().to_string();
    let mut overall_safety_score = 100.0;
    let mut aggregated_recommendations = Vec::new();

    // Analyze conversation if provided
    let conversation_analysis = if let Some(conv_req) = payload.conversation_analysis {
        let analysis = analyze_conversation(State(state.clone()), user.clone(), Json(conv_req)).await?;
        overall_safety_score -= analysis.0.overall_risk_score * 0.6;
        aggregated_recommendations.extend(analysis.0.recommendations.clone());
        Some(analysis.0)
    } else {
        None
    };

    // Verify identity if provided
    let identity_verification = if let Some(id_req) = payload.identity_verification {
        let verification = verify_identity_claims(State(state.clone()), user.clone(), Json(id_req)).await?;
        overall_safety_score -= (100.0 - verification.0.verification_score) * 0.4;
        aggregated_recommendations.extend(verification.0.recommendations.clone());
        Some(verification.0)
    } else {
        None
    };

    overall_safety_score = overall_safety_score.max(0.0);

    let risk_level = match overall_safety_score {
        80.0..=100.0 => "low",
        60.0..=79.9 => "medium",
        40.0..=59.9 => "high",
        _ => "critical",
    }.to_string();

    // Generate safety action plan
    let safety_action_plan = vec![
        SafetyAction {
            priority: "immediate".to_string(),
            action: "Inform trusted contact".to_string(),
            description: "Tell a friend or family member about your plans".to_string(),
            timeframe: "before meeting".to_string(),
        },
        SafetyAction {
            priority: "high".to_string(),
            action: "Meet in public".to_string(),
            description: "Always meet in a public, well-lit location".to_string(),
            timeframe: "for first meetings".to_string(),
        },
    ];

    // Emergency resources
    let emergency_contacts = vec![
        EmergencyResource {
            resource_type: "emergency".to_string(),
            name: "Emergency Services".to_string(),
            contact: "911".to_string(),
            availability: "24/7".to_string(),
        },
        EmergencyResource {
            resource_type: "support".to_string(),
            name: "National Dating Abuse Helpline".to_string(),
            contact: "1-866-331-9474".to_string(),
            availability: "24/7".to_string(),
        },
    ];

    // Store comprehensive report
    let report_data = serde_json::json!({
        "comprehensive_safety_report": {
            "report_id": report_id,
            "overall_safety_score": overall_safety_score,
            "risk_level": risk_level,
            "conversation_analysis": conversation_analysis,
            "identity_verification": identity_verification,
            "generated_at": Utc::now()
        }
    });

    let input_hash = format!("safety_report_{}", uuid::Uuid::new_v4());
    state.db.create_security_report(
        user.user_id,
        "comprehensive_safety_report",
        &input_hash,
        &report_data.to_string(),
        Some(overall_safety_score as i32),
    ).await?;

    info!("Comprehensive safety report generated for user: {} (score: {})", user.email, overall_safety_score);

    Ok(Json(ComprehensiveSafetyReport {
        report_id,
        overall_safety_score,
        risk_level,
        conversation_analysis,
        identity_verification,
        aggregated_recommendations,
        safety_action_plan,
        emergency_contacts,
        generated_at: Utc::now(),
    }))
}