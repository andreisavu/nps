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

extern crate nps;

use nps::{Rating, Summary};

pub fn main() {

    let ratings = vec![
        Rating::new(6),
        Rating::with_comment(8, "no feedback"),
        Rating::new(10)
    ];

    println!("Input: {:?}", ratings);

    let summary = Summary::new(ratings);

    println!("Output: {:?}", summary);
    println!("NPS: {:?}", summary.score());
}
