/// Bandwidth Consistency Score Calculator
/// 
/// Measures how stable the connection speed is over time.
/// A consistent connection is better than an inconsistent fast connection.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyScore {
    pub coefficient_of_variation: f64,
    pub stability_grade: String,
    pub mean_speed: f64,
    pub std_deviation: f64,
    pub measurements_range: (f64, f64),
}

pub struct ConsistencyCalculator;

impl ConsistencyCalculator {
    /// Calculate consistency score from speed measurements
    pub fn calculate(measurements: &[f64]) -> ConsistencyScore {
        if measurements.is_empty() {
            return ConsistencyScore {
                coefficient_of_variation: 0.0,
                stability_grade: "Unknown".to_string(),
                mean_speed: 0.0,
                std_deviation: 0.0,
                measurements_range: (0.0, 0.0),
            };
        }

        let mean = Self::calculate_mean(measurements);
        let std_dev = Self::calculate_std_deviation(measurements, mean);
        let cv = if mean > 0.0 {
            (std_dev / mean) * 100.0
        } else {
            0.0
        };

        let grade = Self::grade_consistency(cv);
        
        let min = measurements.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(0.0);
        
        let max = measurements.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap_or(0.0);

        ConsistencyScore {
            coefficient_of_variation: cv,
            stability_grade: grade,
            mean_speed: mean,
            std_deviation: std_dev,
            measurements_range: (min, max),
        }
    }

    fn calculate_mean(values: &[f64]) -> f64 {
        values.iter().sum::<f64>() / values.len() as f64
    }

    fn calculate_std_deviation(values: &[f64], mean: f64) -> f64 {
        let variance = values.iter()
            .map(|value| {
                let diff = value - mean;
                diff * diff
            })
            .sum::<f64>() / values.len() as f64;
        
        variance.sqrt()
    }

    fn grade_consistency(cv: f64) -> String {
        match cv {
            x if x < 5.0 => "Excellent - Very stable".to_string(),
            x if x < 15.0 => "Good - Minor fluctuations".to_string(),
            x if x < 30.0 => "Fair - Noticeable variation".to_string(),
            _ => "Poor - Highly unstable".to_string(),
        }
    }
}

impl ConsistencyScore {
    /// Generate user-friendly summary
    pub fn summary(&self) -> String {
        let grade_emoji = if self.coefficient_of_variation < 5.0 {
            "⭐"
        } else if self.coefficient_of_variation < 15.0 {
            "✓"
        } else if self.coefficient_of_variation < 30.0 {
            "⚠️"
        } else {
            "🔴"
        };

        format!(
            "📊 Connection Consistency Analysis\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             Grade:              {} {}\n\
             Mean Speed:         {:.2} Mbps\n\
             Std Deviation:      {:.2} Mbps\n\
             Coefficient of Variation: {:.1}%\n\
             Speed Range:        {:.2} - {:.2} Mbps",
            self.stability_grade,
            grade_emoji,
            self.mean_speed,
            self.std_deviation,
            self.coefficient_of_variation,
            self.measurements_range.0,
            self.measurements_range.1
        )
    }

    /// Get recommendation based on consistency
    pub fn recommendation(&self) -> &str {
        if self.coefficient_of_variation < 5.0 {
            "Excellent consistency! Your connection is very stable, ideal for all online activities."
        } else if self.coefficient_of_variation < 15.0 {
            "Good consistency with minor fluctuations. Suitable for most activities."
        } else if self.coefficient_of_variation < 30.0 {
            "Fair consistency with noticeable variations. May experience occasional slowdowns. \
             Consider:\n\
             1. Checking for background downloads\n\
             2. Reducing number of connected devices\n\
             3. Moving closer to WiFi router"
        } else {
            "Poor consistency! Your connection speed varies significantly. This indicates:\n\
             1. Network congestion\n\
             2. Weak WiFi signal\n\
             3. ISP issues\n\
             4. Interference from other devices\n\n\
             Recommendations:\n\
             - Use wired Ethernet connection\n\
             - Test at different times\n\
             - Contact your ISP"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistency_calculation() {
        // Stable connection
        let stable = vec![100.0, 101.0, 99.0, 100.5, 99.5];
        let score = ConsistencyCalculator::calculate(&stable);
        assert!(score.coefficient_of_variation < 5.0);
        assert_eq!(score.stability_grade, "Excellent - Very stable");

        // Unstable connection
        let unstable = vec![100.0, 50.0, 150.0, 75.0, 125.0];
        let score = ConsistencyCalculator::calculate(&unstable);
        assert!(score.coefficient_of_variation > 30.0);
        assert_eq!(score.stability_grade, "Poor - Highly unstable");
    }

    #[test]
    fn test_mean_calculation() {
        let values = vec![10.0, 20.0, 30.0];
        assert_eq!(ConsistencyCalculator::calculate_mean(&values), 20.0);
    }
}
