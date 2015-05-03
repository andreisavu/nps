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

///
/// Net Promoter Score (NPS®) is based on a simple question:
/// "How likely is that you would recommend [something] to a
/// friend or colleague?" on scale from 0 to 10
///
/// See the following pages for more details:
/// http://www.netpromoter.com/why-net-promoter/know
/// https://hbr.org/2003/12/the-one-number-you-need-to-grow
///
/// You don't need to pay to use NPS:
/// http://www.genroe.com/blog/do-i-need-to-pay-to-use-net-promoter/7445
///
/// This library provides utilities for summarizing NPS® surveys.
///

pub mod rating;
pub mod summary;

pub use rating::Rating;
pub use summary::Summary;
