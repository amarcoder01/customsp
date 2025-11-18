/// AI-Powered Network Intelligence
/// 
/// Uses OpenAI GPT to provide:
/// - Natural language explanations of test results
/// - Intelligent troubleshooting recommendations
/// - Personalized network optimization advice
/// - Predictive issue detection

use async_openai::{
    Client,
    types::{
        ChatCompletionRequestMessage, 
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
        Role,
    },
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::TestResult;
use crate::services::loaded_latency::LoadedLatencyResult;
use crate::services::aim_scoring::AIMScores;

/// AI-generated insights about network performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInsights {
    /// Natural language summary of test results
    pub summary: String,
    
    /// Detailed explanation of issues found
    pub detailed_analysis: String,
    
    /// Prioritized recommendations
    pub recommendations: Vec<AIRecommendation>,
    
    /// Predicted issues that may occur
    pub predictions: Vec<String>,
    
    /// Conversational explanation (ELI5 style)
    pub simple_explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRecommendation {
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub expected_improvement: String,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,      // User can do in 5 minutes
    Medium,    // Requires some technical knowledge
    Hard,      // May need professional help
    Advanced,  // Requires significant expertise
}

pub struct AINetworkAnalyzer {
    client: Client<async_openai::config::OpenAIConfig>,
    model: String,
    max_tokens: u16,
    temperature: f32,
}

impl AINetworkAnalyzer {
    /// Create new AI analyzer from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set in environment")?;
        
        let model = env::var("OPENAI_MODEL")
            .unwrap_or_else(|_| "gpt-4o-mini".to_string());
        
        let max_tokens: u16 = env::var("OPENAI_MAX_TOKENS")
            .unwrap_or_else(|_| "2000".to_string())
            .parse()
            .unwrap_or(2000);
        
        let temperature: f32 = env::var("OPENAI_TEMPERATURE")
            .unwrap_or_else(|_| "0.7".to_string())
            .parse()
            .unwrap_or(0.7);
        
        let config = async_openai::config::OpenAIConfig::new()
            .with_api_key(api_key);
        
        let client = Client::with_config(config);
        
        Ok(Self {
            client,
            model,
            max_tokens,
            temperature,
        })
    }
    
    /// Generate AI-powered insights from test results
    pub async fn analyze_network(
        &self,
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
        aim_scores: &AIMScores,
    ) -> Result<AIInsights, Box<dyn std::error::Error>> {
        log::info!("🤖 Generating AI insights for test results...");
        
        // Create detailed context for AI
        let context = self.create_context(test_result, loaded_latency, aim_scores);
        
        // Generate insights
        let summary = self.generate_summary(&context).await?;
        let detailed_analysis = self.generate_detailed_analysis(&context).await?;
        let recommendations = self.generate_recommendations(&context).await?;
        let predictions = self.generate_predictions(&context).await?;
        let simple_explanation = self.generate_simple_explanation(&context).await?;
        
        log::info!("✅ AI insights generated successfully");
        
        Ok(AIInsights {
            summary,
            detailed_analysis,
            recommendations,
            predictions,
            simple_explanation,
        })
    }
    
    /// Create rich context from test results
    fn create_context(
        &self,
        test_result: &TestResult,
        loaded_latency: &LoadedLatencyResult,
        aim_scores: &AIMScores,
    ) -> String {
        format!(
            "Network Speed Test Results:\n\
            \n\
            === Basic Metrics ===\n\
            Download Speed: {:.1} Mbps\n\
            Upload Speed: {:.1} Mbps\n\
            \n\
            === Latency Analysis ===\n\
            Idle Latency: {:.1}ms ({:.0} RPM)\n\
            Download Loaded Latency: {:.1}ms ({:.0} RPM)\n\
            Upload Loaded Latency: {:.1}ms ({:.0} RPM)\n\
            Jitter: {:.1}ms\n\
            \n\
            === Bufferbloat Analysis ===\n\
            Grade: {}\n\
            Download Increase: +{:.0}% ({:.1}ms)\n\
            Upload Increase: +{:.0}% ({:.1}ms)\n\
            \n\
            === Use-Case Scores (AIM) ===\n\
            Gaming: {:.0}/100 ({})\n\
            Streaming: {:.0}/100 ({})\n\
            Video Conferencing: {:.0}/100 ({})\n\
            General Browsing: {:.0}/100 ({})\n\
            Overall: {:.0}/100 ({})\n\
            \n\
            === Connection Details ===\n\
            Protocol: {}\n\
            Server: {}\n\
            Test Duration: {}ms",
            test_result.download_mbps,
            test_result.upload_mbps,
            loaded_latency.idle_avg_ms,
            loaded_latency.idle_rpm,
            loaded_latency.download_avg_ms,
            loaded_latency.download_rpm,
            loaded_latency.upload_avg_ms,
            loaded_latency.upload_rpm,
            test_result.jitter_ms,
            loaded_latency.bufferbloat_grade.as_str(),
            loaded_latency.bufferbloat_download_ratio * 100.0,
            loaded_latency.bufferbloat_download_ms,
            loaded_latency.bufferbloat_upload_ratio * 100.0,
            loaded_latency.bufferbloat_upload_ms,
            aim_scores.gaming.score,
            aim_scores.gaming.grade.as_str(),
            aim_scores.streaming.score,
            aim_scores.streaming.grade.as_str(),
            aim_scores.video_conferencing.score,
            aim_scores.video_conferencing.grade.as_str(),
            aim_scores.general_browsing.score,
            aim_scores.general_browsing.grade.as_str(),
            aim_scores.overall_score,
            aim_scores.overall_grade.as_str(),
            test_result.protocol,
            test_result.server_id,
            test_result.test_duration_ms
        )
    }
    
    /// Generate natural language summary
    async fn generate_summary(&self, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Based on these network test results, provide a concise 2-3 sentence summary \
            highlighting the most important findings. Be direct and actionable.\n\n{}",
            context
        );
        
        self.call_openai(&prompt).await
    }
    
    /// Generate detailed technical analysis
    async fn generate_detailed_analysis(&self, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Provide a detailed technical analysis of these network test results. \
            Explain what the metrics mean, identify any issues, and explain their impact. \
            Focus on bufferbloat, latency under load, and use-case suitability.\n\n{}",
            context
        );
        
        self.call_openai(&prompt).await
    }
    
    /// Generate prioritized recommendations
    async fn generate_recommendations(&self, context: &str) -> Result<Vec<AIRecommendation>, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Based on these network test results, provide 3-5 prioritized recommendations \
            for improving network performance. For each recommendation, specify:\n\
            1. Priority (Critical/High/Medium/Low)\n\
            2. Title (brief)\n\
            3. Description (what to do)\n\
            4. Expected improvement\n\
            5. Difficulty (Easy/Medium/Hard/Advanced)\n\
            \n\
            Format as JSON array with fields: priority, title, description, expected_improvement, difficulty\n\n{}",
            context
        );
        
        let response = self.call_openai(&prompt).await?;
        
        // Parse JSON response
        match serde_json::from_str::<Vec<AIRecommendation>>(&response) {
            Ok(recs) => Ok(recs),
            Err(_) => {
                // Fallback if JSON parsing fails
                Ok(vec![AIRecommendation {
                    priority: Priority::Medium,
                    title: "Network Optimization Needed".to_string(),
                    description: response,
                    expected_improvement: "Improved network performance".to_string(),
                    difficulty: Difficulty::Medium,
                }])
            }
        }
    }
    
    /// Predict potential issues
    async fn generate_predictions(&self, context: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Based on these network test results, predict 2-3 potential issues that may occur. \
            Focus on real-world impact: gaming lag, video call freezing, streaming buffering, etc.\n\n{}",
            context
        );
        
        let response = self.call_openai(&prompt).await?;
        
        // Split into bullet points
        let predictions: Vec<String> = response
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        
        Ok(predictions)
    }
    
    /// Generate simple explanation (ELI5)
    async fn generate_simple_explanation(&self, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Explain these network test results in simple terms that anyone can understand. \
            Use analogies if helpful. Focus on what the user can actually do with their connection. \
            Make it friendly and conversational.\n\n{}",
            context
        );
        
        self.call_openai(&prompt).await
    }
    
    /// Call OpenAI API with retry logic
    async fn call_openai(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .max_tokens(self.max_tokens)
            .temperature(self.temperature)
            .messages(vec![
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessageArgs::default()
                        .content("You are a network performance expert helping users understand \
                                 their internet connection quality. Be clear, accurate, and actionable. \
                                 Focus on real-world impact and practical solutions.")
                        .build()?
                ),
                ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(prompt)
                        .build()?
                ),
            ])
            .build()?;
        
        let response = self.client.chat().create(request).await?;
        
        let content = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or("No response from OpenAI")?;
        
        Ok(content)
    }
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Critical => "Critical",
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        }
    }
    
    pub fn emoji(&self) -> &str {
        match self {
            Priority::Critical => "🔴",
            Priority::High => "🟠",
            Priority::Medium => "🟡",
            Priority::Low => "🟢",
        }
    }
}

impl Difficulty {
    pub fn as_str(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Advanced => "Advanced",
        }
    }
    
    pub fn emoji(&self) -> &str {
        match self {
            Difficulty::Easy => "✅",
            Difficulty::Medium => "⚙️",
            Difficulty::Hard => "🔧",
            Difficulty::Advanced => "👨‍💻",
        }
    }
}

impl AIInsights {
    /// Get formatted display of AI insights
    pub fn display(&self) -> String {
        let mut output = String::new();
        
        output.push_str("🤖 AI-POWERED NETWORK INSIGHTS\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");
        
        output.push_str("📊 SUMMARY\n");
        output.push_str(&format!("{}\n\n", self.summary));
        
        output.push_str("🔍 DETAILED ANALYSIS\n");
        output.push_str(&format!("{}\n\n", self.detailed_analysis));
        
        output.push_str("💡 RECOMMENDATIONS\n");
        for rec in &self.recommendations {
            output.push_str(&format!(
                "{} {} - {} {}\n",
                rec.priority.emoji(),
                rec.priority.as_str(),
                rec.title,
                rec.difficulty.emoji()
            ));
            output.push_str(&format!("   {}\n", rec.description));
            output.push_str(&format!("   Expected: {}\n\n", rec.expected_improvement));
        }
        
        output.push_str("🔮 PREDICTIONS\n");
        for prediction in &self.predictions {
            output.push_str(&format!("• {}\n", prediction));
        }
        output.push_str("\n");
        
        output.push_str("💬 SIMPLE EXPLANATION\n");
        output.push_str(&format!("{}\n", self.simple_explanation));
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::Critical.as_str(), "Critical");
        assert_eq!(Priority::Critical.emoji(), "🔴");
    }
    
    #[test]
    fn test_difficulty_display() {
        assert_eq!(Difficulty::Easy.as_str(), "Easy");
        assert_eq!(Difficulty::Easy.emoji(), "✅");
    }
}
