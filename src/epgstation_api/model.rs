use core::fmt;

use serde::{Deserialize, Serialize};

pub type RecordId = u64;
pub type VideoFileId = u64;
pub type ChannelId = u64;
pub type RuleId = u64;
pub type ProgramId = u64;
pub type ThumbnailId = u64;
pub type EncodeId = u64;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecordedEndpointResponse {
    pub records: Vec<Record>,

    #[allow(dead_code)]
    pub total: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoFile {
    pub id: VideoFileId,
    pub name: String,
    pub filename: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub size: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: RecordId,
    pub channel_id: ChannelId,
    pub start_at: u64,
    pub end_at: u64,
    pub name: String,
    pub is_recording: bool,
    pub is_encoding: bool,
    pub is_protected: bool,
    pub rule_id: Option<RuleId>,
    pub program_id: ProgramId,
    pub description: Option<String>,
    pub extended: Option<String>,
    pub genre1: Option<u16>,
    pub sub_genre1: Option<u16>,
    pub video_type: String,
    pub video_resolution: String,
    pub video_stream_content: i32,
    pub video_component_type: i32,
    pub audio_sampling_rate: u32,
    pub audio_component_type: u8,
    pub thumbnails: Vec<ThumbnailId>,
    pub video_files: Vec<VideoFile>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestEncodeProperty {
    pub recorded_id: RecordId,
    pub source_video_file_id: VideoFileId,
    pub parent_dir: String,
    pub mode: String,
    pub remove_original: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodeItem {
    pub id: EncodeId,
    pub mode: String,
    pub recorded: Record,
    pub percent: Option<u64>,
    pub log: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetchEncodeResponse {
    pub running_items: Vec<EncodeItem>,
    pub wait_items: Vec<EncodeItem>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchRecordedProperty {
    pub is_half_width: bool,
    pub offset: i32,
    pub limit: i32,
    pub is_reverse: Option<bool>,
    pub rule_id: Option<RuleId>,
    pub channel_id: Option<ChannelId>,
    pub genre: Option<i32>,
    pub keyword: Option<String>,
    pub has_original_file: Option<bool>,
}

impl FetchRecordedProperty {
    pub fn to_parameter(&self) -> Vec<(String, String)> {
        let mut params = vec![
            ("isHalfWidth".to_string(), self.is_half_width.to_string()),
            ("offset".to_string(), self.offset.to_string()),
            ("limit".to_string(), self.limit.to_string()),
        ];

        if let Some(is_reverse) = self.is_reverse {
            params.push(("isReverse".to_string(), is_reverse.to_string()));
        }

        if let Some(rule_id) = self.rule_id {
            params.push(("ruleId".to_string(), rule_id.to_string()));
        }

        if let Some(channel_id) = self.channel_id {
            params.push(("channelId".to_string(), channel_id.to_string()));
        }

        if let Some(genre) = self.genre {
            params.push(("genre".to_string(), genre.to_string()));
        }

        if let Some(keyword) = &self.keyword {
            params.push(("keyword".to_string(), keyword.to_string()));
        }

        if let Some(has_original_file) = self.has_original_file {
            params.push(("hasOriginalFile".to_string(), has_original_file.to_string()));
        }

        params
    }
}

impl Default for FetchRecordedProperty {
    fn default() -> Self {
        FetchRecordedProperty {
            is_half_width: true,
            offset: 0,
            limit: 24,
            is_reverse: None,
            rule_id: None,
            channel_id: None,
            genre: None,
            keyword: None,
            has_original_file: None,
        }
    }
}

pub enum RuleFetchType {
    #[allow(dead_code)]
    All,
    #[allow(dead_code)]
    Normal,
    #[allow(dead_code)]
    Conflict,
    #[allow(dead_code)]
    Skip,
    #[allow(dead_code)]
    Overlap,
}

impl fmt::Display for RuleFetchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleFetchType::All => write!(f, "all"),
            RuleFetchType::Normal => write!(f, "normal"),
            RuleFetchType::Conflict => write!(f, "conflict"),
            RuleFetchType::Skip => write!(f, "skip"),
            RuleFetchType::Overlap => write!(f, "overlap"),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetchRulesResponse {
    pub rules: Vec<Rule>,
    pub total: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub is_time_specification: bool,
    pub search_option: RuleSearchOption,
    pub id: RuleId,
    // 本当はさらにパラメータがあるが省略
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuleSearchOption {
    pub keyword: String,
    pub ignore_keyword: Option<String>,
    // 本当はさらにパラメータがあるが省略
}
