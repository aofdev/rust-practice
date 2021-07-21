use std::any::Any;

use chrono::NaiveDate;
use csv::StringRecord;
use serde::{Deserialize, Serialize};

use crate::config_reader::keyword_config::{KeywordConfig, KeywordMatch};
use crate::matcher;
pub const FIELD_NAMES: &[&str; 24] = &[
    "Account",
    "Message",
    "Direct URL",
    "Post URL",
    "Comment URL",
    "Reply comment URL",
    "Source",
    "Post time",
    "Engagement",
    "Main keyword",
    "Sub keyword",
    "Follower count",
    "Sentiment",
    "Category",
    "Track post",
    "Track account",
    "Note",
    "_id",
    "Image labels",
    "Image URL",
    "Account label audience size",
    "Account label categories",
    "Account label type",
    "Account label tza",
];
#[derive(Debug, Deserialize, Serialize)]
pub struct ZocialData<'row_life> {
    #[serde(alias = "Account")]
    #[serde(rename(serialize = "Account"))]
    account: Option<&'row_life str>,
    #[serde(alias = "Message")]
    #[serde(rename(serialize = "Message"))]
    message: Option<&'row_life str>,
    #[serde(alias = "\"Direct URL\"")]
    #[serde(rename(serialize = "Direct URL"))]
    direct_url: Option<&'row_life str>,
    #[serde(alias = "\"Post URL\"")]
    #[serde(rename(serialize = "Post URL"))]
    post_url: Option<&'row_life str>,
    #[serde(alias = "\"Comment URL\"")]
    #[serde(rename(serialize = "Comment URL"))]
    comment_url: Option<&'row_life str>,
    #[serde(alias = "\"Reply comment URL\"")]
    #[serde(rename(serialize = "Reply comment URL"))]
    reply_comment_url: Option<&'row_life str>,
    #[serde(alias = "Source")]
    #[serde(rename(serialize = "Source"))]
    source: Option<&'row_life str>,
    #[serde(alias = "\"Post Time\"")]
    #[serde(alias = "Post Time")]
    #[serde(rename(serialize = "Post Time"))]
    post_time: Option<&'row_life str>,
    #[serde(alias = "Engagement")]
    #[serde(rename(serialize = "Engagement"))]
    engagement: Option<i32>,
    #[serde(alias = "\"Main keyword\"")]
    #[serde(rename(serialize = "Main keyword"))]
    main_keyword: Option<&'row_life str>,
    #[serde(alias = "\"Sub keyword\"")]
    #[serde(rename(serialize = "Sub keyword"))]
    sub_keyword: Option<&'row_life str>,
    #[serde(alias = "\"Follower count\"")]
    #[serde(rename(serialize = "Follower count"))]
    follower_count: Option<u32>,
    #[serde(alias = "Sentiment")]
    #[serde(rename(serialize = "Sentiment"))]
    sentiment: Option<&'row_life str>,
    #[serde(alias = "Category")]
    #[serde(rename(serialize = "Category"))]
    category: Option<&'row_life str>,
    #[serde(alias = "\"Track post\"")]
    #[serde(rename(serialize = "Track post"))]
    track_post: Option<&'row_life str>,
    #[serde(alias = "\"Track account\"")]
    #[serde(rename(serialize = "Track account"))]
    track_account: Option<&'row_life str>,
    #[serde(alias = "Note")]
    #[serde(rename(serialize = "Note"))]
    note: Option<&'row_life str>,
    #[serde(alias = "_id")]
    #[serde(rename(serialize = "_id"))]
    _id: Option<&'row_life str>,
    #[serde(alias = "\"Image labels\"")]
    #[serde(rename(serialize = "Image labels"))]
    image_labels: Option<&'row_life str>,
    #[serde(alias = "\"Image URL\"")]
    #[serde(rename(serialize = "Image URL"))]
    image_url: Option<&'row_life str>,
    #[serde(alias = "\"Account label audience size\"")]
    #[serde(rename(serialize = "Account label audience size"))]
    account_audience_size: Option<&'row_life str>,
    #[serde(alias = "\"Account label categories\"")]
    #[serde(rename(serialize = "Account label categories"))]
    account_label_categories: Option<&'row_life str>,
    #[serde(alias = "\"Account label type\"")]
    #[serde(rename(serialize = "Account label type"))]
    account_label_type: Option<&'row_life str>,
    #[serde(alias = "\"Account label tza\"")]
    #[serde(rename(serialize = "Account label tza"))]
    account_label_tza: Option<&'row_life str>,
    #[serde(alias = "Tags")]
    #[serde(rename(serialize = "Tags"))]
    tags: Option<String>,
}
impl ZocialData<'_> {
    pub fn set_tag(&mut self, result_tag: &str) {
        self.tags = Some(result_tag.to_owned())
    }
    pub fn get_message(&self) -> &str {
        match self.message {
            Some(text) => text,
            None => &"",
        }
    }
    pub fn process_match_keyword(&mut self, keyword_config: &KeywordMatch) {
        if let Some(message) = &mut self.message {
            if keyword_config.is_match(message) {
                if let Some(tags) = &mut self.tags {
                    tags.push_str(",");
                    tags.push_str(&keyword_config.get_name());
                } else {
                    self.tags = Some(keyword_config.get_name());
                }
            }
        }
    }
}
