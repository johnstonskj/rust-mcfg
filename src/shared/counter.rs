/*!
One-line description.

More detailed description, with

# Example

*/

use std::cell::RefCell;
use std::ops::RangeFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct StepCounter(RefCell<RangeFrom<u32>>);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for StepCounter {
    fn default() -> Self {
        Self::from_one()
    }
}

impl StepCounter {
    pub fn from_zero() -> Self {
        Self::from(0)
    }

    pub fn from_one() -> Self {
        Self::from(1)
    }

    pub fn from(start: u32) -> Self {
        Self(RefCell::from(start..))
    }

    pub fn step(&self) -> u32 {
        self.0.borrow_mut().next().unwrap()
    }

    pub fn steps(&self, skip: u32) -> Option<u32> {
        if skip == 0 {
            None
        } else {
            Some(self.0.borrow_mut().nth((skip - 1) as usize).unwrap())
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_at_zero() {
        let counter = StepCounter::from_zero();
        assert_eq!(counter.step(), 0);
    }

    #[test]
    fn test_start_at_one() {
        let counter = StepCounter::from_one();
        assert_eq!(counter.step(), 1);
    }

    #[test]
    fn test_start_at_ninety_nine() {
        let counter = StepCounter::from(99);
        assert_eq!(counter.step(), 99);
    }

    #[test]
    fn test_just_steps() {
        let counter = StepCounter::from_zero();
        assert_eq!(counter.step(), 0);
        assert_eq!(counter.step(), 1);
        assert_eq!(counter.step(), 2);
        assert_eq!(counter.step(), 3);
        assert_eq!(counter.step(), 4);
        assert_eq!(counter.step(), 5);
        assert_eq!(counter.step(), 6);
        assert_eq!(counter.step(), 7);
        assert_eq!(counter.step(), 8);
        assert_eq!(counter.step(), 9);
    }

    #[test]
    fn test_skip_steps() {
        let counter = StepCounter::from_zero();
        assert_eq!(counter.step(), 0);
        assert_eq!(counter.step(), 1);
        assert_eq!(counter.steps(5), Some(6));
        assert_eq!(counter.step(), 7);
        assert_eq!(counter.step(), 8);
        assert_eq!(counter.steps(10), Some(18));
        assert_eq!(counter.step(), 19);
        assert_eq!(counter.steps(0), None);
        assert_eq!(counter.step(), 20);
        assert_eq!(counter.step(), 21);
    }
}
