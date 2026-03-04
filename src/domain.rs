// Rust guideline compliant 2026-02-16

//! Pure BMI calculation and WHO classification with no I/O or framework dependencies.

use thiserror::Error;

/// WHO body mass index classification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BmiCategory {
    /// BMI < 18.5
    Underweight,
    /// 18.5 <= BMI <= 24.9
    Normal,
    /// 25.0 <= BMI <= 29.9
    Overweight,
    /// BMI >= 30.0
    Obese,
}

impl std::fmt::Display for BmiCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Underweight => f.write_str("Underweight"),
            Self::Normal => f.write_str("Normal"),
            Self::Overweight => f.write_str("Overweight"),
            Self::Obese => f.write_str("Obese"),
        }
    }
}

/// Result of a successful BMI calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct BmiResult {
    /// Calculated BMI rounded to 1 decimal place.
    pub bmi: f64,
    /// WHO classification for the calculated BMI.
    pub category: BmiCategory,
}

/// Domain-level error for invalid BMI inputs or non-finite results.
#[expect(
    clippy::module_name_repetitions,
    reason = "DomainError is the design-specified name; renaming would break contract"
)]
#[derive(Debug, Error)]
pub enum DomainError {
    /// Weight is zero or negative.
    #[error("{0}")]
    InvalidWeight(String),
    /// Height is zero or negative.
    #[error("{0}")]
    InvalidHeight(String),
    /// BMI result is Infinity or NaN.
    #[error("computed BMI is not finite")]
    NonFiniteResult,
}

/// Calculates BMI from weight and height, returns WHO category.
///
/// # Errors
///
/// Returns [`DomainError::InvalidWeight`] when `weight_kg` <= 0.
/// Returns [`DomainError::InvalidHeight`] when `height_m` <= 0.
/// Returns [`DomainError::NonFiniteResult`] when computed BMI is not finite.
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> Result<BmiResult, DomainError> {
    if weight_kg <= 0.0 {
        return Err(DomainError::InvalidWeight(
            "weight_kg must be positive".to_owned(),
        ));
    }
    if height_m <= 0.0 {
        return Err(DomainError::InvalidHeight(
            "height_m must be positive".to_owned(),
        ));
    }
    let bmi_raw = weight_kg / (height_m * height_m);
    if !bmi_raw.is_finite() {
        return Err(DomainError::NonFiniteResult);
    }
    // Round to 1 decimal: multiply by 10, round, divide back.
    // Example: 22.857 -> 228.57 -> 229.0 -> 22.9
    let bmi = (bmi_raw * 10.0).round() / 10.0;
    Ok(BmiResult {
        bmi,
        category: classify(bmi),
    })
}

/// Maps a finite BMI value to its WHO category.
///
/// WHO thresholds: <18.5 Underweight, 18.5-24.9 Normal,
/// 25.0-29.9 Overweight, >=30.0 Obese.
fn classify(bmi: f64) -> BmiCategory {
    if bmi < 18.5 {
        BmiCategory::Underweight
    } else if bmi <= 24.9 {
        BmiCategory::Normal
    } else if bmi <= 29.9 {
        BmiCategory::Overweight
    } else {
        BmiCategory::Obese
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- T004: WHO boundary values ---

    #[test]
    fn bmi_underweight_category() {
        // 50 kg / (1.80 m)^2 = 15.43 -> rounds to 15.4 -> Underweight
        let result = calculate_bmi(50.0, 1.80).unwrap();
        assert_eq!(result.bmi, 15.4);
        assert_eq!(result.category, BmiCategory::Underweight);
    }

    #[test]
    fn bmi_normal_category() {
        // 70 kg / (1.75 m)^2 = 22.86 -> rounds to 22.9 -> Normal
        let result = calculate_bmi(70.0, 1.75).unwrap();
        assert_eq!(result.bmi, 22.9);
        assert_eq!(result.category, BmiCategory::Normal);
    }

    #[test]
    fn bmi_overweight_boundary() {
        // weight=25.0, height=1.0 -> BMI=25.0 -> Overweight boundary
        let result = calculate_bmi(25.0, 1.0).unwrap();
        assert_eq!(result.bmi, 25.0);
        assert_eq!(result.category, BmiCategory::Overweight);
    }

    #[test]
    fn bmi_obese_boundary() {
        // weight=30.0, height=1.0 -> BMI=30.0 -> Obese boundary
        let result = calculate_bmi(30.0, 1.0).unwrap();
        assert_eq!(result.bmi, 30.0);
        assert_eq!(result.category, BmiCategory::Obese);
    }

    // --- T004: Error paths ---

    #[test]
    fn zero_weight_returns_invalid_weight() {
        let err = calculate_bmi(0.0, 1.75).unwrap_err();
        assert!(matches!(err, DomainError::InvalidWeight(_)));
        assert_eq!(err.to_string(), "weight_kg must be positive");
    }

    #[test]
    fn negative_height_returns_invalid_height() {
        let err = calculate_bmi(70.0, -1.0).unwrap_err();
        assert!(matches!(err, DomainError::InvalidHeight(_)));
        assert_eq!(err.to_string(), "height_m must be positive");
    }

    #[test]
    fn infinity_weight_returns_non_finite() {
        // f64::INFINITY passes >0 check; INFINITY/h^2 = INFINITY -> NonFiniteResult
        let err = calculate_bmi(f64::INFINITY, 1.75).unwrap_err();
        assert!(matches!(err, DomainError::NonFiniteResult));
        assert_eq!(err.to_string(), "computed BMI is not finite");
    }

    // --- Display impl ---

    #[test]
    fn bmi_category_display_strings() {
        assert_eq!(BmiCategory::Underweight.to_string(), "Underweight");
        assert_eq!(BmiCategory::Normal.to_string(), "Normal");
        assert_eq!(BmiCategory::Overweight.to_string(), "Overweight");
        assert_eq!(BmiCategory::Obese.to_string(), "Obese");
    }
}
