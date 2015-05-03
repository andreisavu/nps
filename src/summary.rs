// Copyright 2015 Andrei Savu <asavu@apache.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rating::Rating;

/// A NPS summary is computed starting from a vector of ratings.
///
/// It records a couple of counters that can be used to compute the
/// and it can also show a histogram of all the ratings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {

    /// Rated 9 or 10. Promoters are loyal enthusiasts.
    pub promoters: u32,

    /// Rated 0 to 6. Detractors are unhappy customers.
    pub detractors: u32,

    /// Rated 7 or 8. Satisfied but unenthusiastic customers.
    pub passive: u32,

    /// The distribution of ratings (expecting 0 to 10 values).
    pub histogram: [u32; 11]
}

impl Summary {

    pub fn new(ratings: Vec<Rating>) -> Summary {

        let mut promoters = 0;
        let mut detractors = 0;
        let mut passive = 0;

        let mut histogram = [0; 11];

        for rating in ratings {
            histogram[rating.value as usize] += 1;

            match rating.value {
                9 | 10 => promoters += 1,
                0 ... 6 => detractors += 1,
                _ => passive += 1
            }
        }

        Summary {
            promoters: promoters,
            detractors: detractors,
            passive: passive,
            histogram: histogram
        }
    }

    /// Compute the NPS score for this summary of ratings
    pub fn score(&self) -> f64 {
        let number_of_ratings = self.promoters + self.detractors + self.passive;
        let delta = self.promoters as i8 - self.detractors as i8;

        if number_of_ratings != 0 {
            delta as f64 / number_of_ratings as f64
        } else {
            0.0 // defaulting to 0 when no ratings provided
        }
    }
}

// Tests

#[test]
fn test_construct() {
    let s = Summary::new(vec![Rating::new(6)]);

    assert!(s.detractors == 1);
    assert!(s.promoters + s.passive == 0);
}

#[test]
fn test_empty() {
    let s = Summary::new(vec![]);

    assert!(s.score() == 0.0);
}

#[test]
fn test_all_positive() {
    let s = Summary::new(vec![Rating::new(10)]);

    assert!(s.score() == 1.0);
    assert!(s.promoters == 1);
    assert!(s.histogram[10] == 1);
}

#[test]
fn test_all_negative() {
    let s = Summary::new(vec![Rating::new(6)]);

    assert!(s.score() == -1.0);
    assert!(s.detractors == 1);
    assert!(s.histogram[6] == 1);
}

#[test]
fn test_equality() {
    let a = Summary::new(vec![Rating::new(5)]);
    let b = a.clone();

    assert!(a == b);
}
