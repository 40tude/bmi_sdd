// Rust guideline compliant 2026-02-16

//! Pure BMI calculation and WHO classification with no I/O or framework dependencies.

use std::collections::VecDeque;

use thiserror::Error;

// --- T003: Generic bounded FIFO collection ---

/// Generic bounded FIFO collection; newest entries at front, oldest evicted at capacity.
///
/// `BoundedHistory<T>` encapsulates the max-size invariant so it can be unit-tested
/// independently of the HTTP layer. It is free of I/O and serialization dependencies.
#[derive(Debug)]
pub struct BoundedHistory<T> {
    deque: VecDeque<T>,
    /// Hard cap on the number of retained entries.
    ///
    /// When `push` would exceed this limit, the oldest entry (back of the deque)
    /// is evicted first. Set to `5` for BMI history at the call site.
    max_size: usize,
}

impl<T> BoundedHistory<T> {
    /// Creates an empty `BoundedHistory` preallocated for `max_size` entries.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// Inserts `item` at the front (newest position), evicting the oldest if at capacity.
    pub fn push(&mut self, item: T) {
        self.deque.push_front(item);
        if self.deque.len() > self.max_size {
            self.deque.pop_back();
        }
    }

    /// Returns an iterator over entries from newest to oldest.
    pub fn entries(&self) -> impl Iterator<Item = &T> + '_ {
        self.deque.iter()
    }

    /// Returns the current number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.deque.len()
    }

    /// Returns `true` when the collection contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}

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

// --- T002: Unit tests for BoundedHistory<T> ---

#[cfg(test)]
mod bounded_history_tests {
    use super::BoundedHistory;

    #[test]
    fn new_creates_empty_collection() {
        let h: BoundedHistory<i32> = BoundedHistory::new(3);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn push_adds_entry_at_front() {
        let mut h = BoundedHistory::new(3);
        h.push(1);
        h.push(2);
        let entries: Vec<_> = h.entries().copied().collect();
        assert_eq!(entries[0], 2, "newest entry must be at front");
        assert_eq!(entries[1], 1);
    }

    #[test]
    fn entries_returns_newest_first() {
        let mut h = BoundedHistory::new(5);
        h.push(10);
        h.push(20);
        h.push(30);
        let entries: Vec<i32> = h.entries().copied().collect();
        assert_eq!(entries, vec![30, 20, 10]);
    }

    #[test]
    fn len_tracks_count() {
        let mut h = BoundedHistory::new(5);
        assert_eq!(h.len(), 0);
        h.push(1);
        assert_eq!(h.len(), 1);
        h.push(2);
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn is_empty_reflects_state() {
        let mut h: BoundedHistory<u8> = BoundedHistory::new(2);
        assert!(h.is_empty());
        h.push(1);
        assert!(!h.is_empty());
    }

    #[test]
    fn fifo_eviction_at_capacity() {
        let mut h = BoundedHistory::new(3);
        h.push(1);
        h.push(2);
        h.push(3);
        // At capacity -- pushing a 4th must evict the oldest (1)
        h.push(4);
        assert_eq!(h.len(), 3, "len must not exceed max_size");
        let entries: Vec<i32> = h.entries().copied().collect();
        assert!(!entries.contains(&1), "oldest entry must have been evicted");
        assert_eq!(entries[0], 4, "newest must be at front after eviction");
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
