// Copyright 2021 The OpenXLA Authors.
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
// ==============================================================================

use std::fmt;

/// Represents operation metadata from HLO IR
#[derive(Debug, Clone, Default)]
pub struct OpMetadata {
    pub op_type: String,
    pub op_name: String,
    pub source_file: String,
    pub source_line: i32,
    pub source_end_line: i32,
    pub source_column: i32,
    pub source_end_column: i32,
    pub profile_type: Vec<String>,
    pub deduplicated_name: String,
    pub scheduling_name: String,
    pub stack_frame_id: i64,
}

impl OpMetadata {
    /// Converts the OpMetadata to a string representation.
    /// 
    /// # Arguments
    /// * `only_op_name` - If true, only returns the op_name field
    pub fn to_string(&self, only_op_name: bool) -> String {
        if only_op_name {
            if !self.op_name.is_empty() {
                return format!("op_name=\"{}\"", escape_string(&self.op_name));
            }
            return String::new();
        }

        let mut result = Vec::new();

        if !self.op_type.is_empty() {
            result.push(format!("op_type=\"{}\"", escape_string(&self.op_type)));
        }
        if !self.op_name.is_empty() {
            result.push(format!("op_name=\"{}\"", escape_string(&self.op_name)));
        }
        if !self.source_file.is_empty() {
            result.push(format!("source_file=\"{}\"", escape_string(&self.source_file)));
        }
        if self.source_line != 0 {
            result.push(format!("source_line={}", self.source_line));
        }
        if self.source_end_line != 0 {
            result.push(format!("source_end_line={}", self.source_end_line));
        }
        if self.source_column != 0 {
            result.push(format!("source_column={}", self.source_column));
        }
        if self.source_end_column != 0 {
            result.push(format!("source_end_column={}", self.source_end_column));
        }
        if !self.profile_type.is_empty() {
            result.push(format!("profile_type={{{}}}", self.profile_type.join(",")));
        }
        if !self.deduplicated_name.is_empty() {
            result.push(format!("deduplicated_name=\"{}\"", escape_string(&self.deduplicated_name)));
        }
        if !self.scheduling_name.is_empty() {
            result.push(format!("scheduling_name=\"{}\"", self.scheduling_name));
        }
        if self.stack_frame_id != 0 {
            result.push(format!("stack_frame_id={}", self.stack_frame_id));
        }

        result.join(" ")
    }
}

impl fmt::Display for OpMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(false))
    }
}

/// Escapes special characters in a string similar to absl::CEscape
fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '\"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ if c.is_ascii_control() => {
                result.push_str(&format!("\\{:03o}", c as u8));
            }
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello"), "hello");
        assert_eq!(escape_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_string("quote\"test"), "quote\\\"test");
        assert_eq!(escape_string("back\\slash"), "back\\\\slash");
    }

    #[test]
    fn test_to_string_only_op_name() {
        let metadata = OpMetadata {
            op_name: "test_op".to_string(),
            op_type: "test_type".to_string(),
            ..Default::default()
        };
        assert_eq!(metadata.to_string(true), "op_name=\"test_op\"");
    }

    #[test]
    fn test_to_string_only_op_name_empty() {
        let metadata = OpMetadata {
            op_name: String::new(),
            op_type: "test_type".to_string(),
            ..Default::default()
        };
        assert_eq!(metadata.to_string(true), "");
    }

    #[test]
    fn test_to_string_full() {
        let metadata = OpMetadata {
            op_type: "test_type".to_string(),
            op_name: "test_op".to_string(),
            source_file: "test_file.cc".to_string(),
            source_line: 42,
            source_end_line: 45,
            source_column: 10,
            source_end_column: 20,
            profile_type: vec!["profile1".to_string(), "profile2".to_string()],
            deduplicated_name: "dedup_op".to_string(),
            scheduling_name: "sched_op".to_string(),
            stack_frame_id: 123,
        };
        let result = metadata.to_string(false);
        assert!(result.contains("op_type=\"test_type\""));
        assert!(result.contains("op_name=\"test_op\""));
        assert!(result.contains("source_file=\"test_file.cc\""));
        assert!(result.contains("source_line=42"));
        assert!(result.contains("source_end_line=45"));
        assert!(result.contains("source_column=10"));
        assert!(result.contains("source_end_column=20"));
        assert!(result.contains("profile_type={profile1,profile2}"));
        assert!(result.contains("deduplicated_name=\"dedup_op\""));
        assert!(result.contains("scheduling_name=\"sched_op\""));
        assert!(result.contains("stack_frame_id=123"));
    }

    #[test]
    fn test_to_string_empty_metadata() {
        let metadata = OpMetadata::default();
        assert_eq!(metadata.to_string(false), "");
    }
}
