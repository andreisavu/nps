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

use std::cmp::Ordering;

/// A Net Promoter Score rating is a simple value between 0 and 10
/// with an optional comment attached that provides additional details
#[derive(Debug, Clone, Copy)]
pub struct Rating {
    pub value: u8,
    pub comment: Option<&'static str>,
}

impl Rating {
    /// Construct a new rating without a comment
    pub fn new(value: u8) -> Rating {
        assert!(value <= 10, "a rating should be a value between 0 and 10");
        Rating {value: value, comment: None}
    }

    /// Construct a new rating that includes a comment
    pub fn with_comment(value: u8, c: &'static str) -> Rating {
        assert!(value <= 10, "a rating should be a value between 0 and 10");
        Rating {value: value, comment: Some(c)}
    }
}

impl PartialEq<Rating> for Rating {
    /// Equality is computed only based on the value of the rating
    fn eq(&self, other: &Rating) -> bool {
        self.value == other.value
    }
}

impl Eq for Rating {}

impl PartialOrd<Rating> for Rating {
    /// Ratings are compared only based on the value (the comment is ignored)
    fn partial_cmp(&self, other: &Rating) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Rating {
    fn cmp(&self, other: &Rating) -> Ordering {
        self.value.cmp(&other.value)
    }
}

// Unit tests

#[test]
fn test_construction() {
    let a = Rating::new(4);

    assert!(a.value == 4);
    assert!(a.comment == None);

    let b = Rating::with_comment(5, "abc");

    assert!(b.value == 5);
    assert!(b.comment == Some("abc"));
}

#[test]
fn test_equality() {
    let a = Rating::new(5);

    assert!(a == Rating::new(5));
    assert!(a == Rating::with_comment(5, "Hello World!"));

    assert!(a != Rating::new(6));
}

#[test]
fn test_ratings_are_comparable() {
    let a = Rating::new(5);
    let b = Rating::new(6);

    assert!(a < b);
    assert!(b > a);
    assert!(a != b);
}

#[test]
fn test_clone() {
    let a = Rating::new(5);
    let b = a.clone();

    assert!(a == b);
    assert!(b.value == 5);
}

#[test]
#[should_panic]
fn test_rating_out_range() {
    Rating::new(11);
}
