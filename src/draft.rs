use anyhow::Result;
use serde::{Deserialize, Serialize};

use thiserror::Error;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
use rstest::*;

#[derive(Debug, Serialize)]
pub struct Draft(String);

#[derive(Error, Debug)]
pub enum DraftError {
    #[error("Too long. Please keep the number of characters under {0}: {1}")]
    TooLong(usize, String),
}

impl Draft {
    pub fn new(content: &str) -> Result<Self, DraftError> {
        let upper_limit_length: usize = 1000;
        if content.len() > upper_limit_length {
            return Err(DraftError::TooLong(upper_limit_length, content.to_string()));
        }

        Ok(Self(content.to_string()))
    }
}

#[cfg(test)]
speculate! {
    describe "500文字以内ならok" {
        #[rstest]
        fn is_ok_with_500() {
            let content: String = (0..500).map(|_| { "a" }).collect();
            let result = Draft::new(&content);

            assert_eq!(result.is_ok(), true);
        }
    }

    describe "1000文字以内ならok" {
        #[rstest]
        fn is_ok_with_1000() {
            let content: String = (0..1000).map(|_| { "a" }).collect();
            let result = Draft::new(&content);

            assert_eq!(result.is_ok(), true);
        }
    }

    describe "1001文字ならng" {
        #[rstest]
        fn is_ng_with_1001() {
            let content: String = (0..1001).map(|_| { "a" }).collect();
            let result = Draft::new(&content);

            assert_eq!(result.is_err(), true);
        }
    }

    describe "1500文字ならng" {
        #[rstest]
        fn is_ng_with_1500() {
            let content: String = (0..1500).map(|_| { "a" }).collect();
            let result = Draft::new(&content);

            assert_eq!(result.is_err(), true);
        }
    }
}
