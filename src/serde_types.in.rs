use serde::{Serialize, Serializer};
use chrono::NaiveDateTime;

/// Representation of any text sent through slack
/// the text must be processed to escape specific characters
#[derive(Serialize, Debug, Default, Clone)]
pub struct SlackText(String);

/// A `HexColor` `String` can be one of:
///
/// 1. `String`s: `good`, `warning`, `danger`
/// 2. Any valid hex color code: e.g. `#b13d41` or `#000`.
///
/// hex color codes will be checked to ensure a valid hex number is provided
#[derive(Serialize, Debug)]
pub struct HexColor(String);

/// Slack allows for attachments to be added to messages. See
/// https://api.slack.com/docs/attachments for more information.
#[derive(Serialize, Debug, Default)]
pub struct Attachment {
    /// Required text for attachment.
    /// Slack will use this text to display on devices that don't support markup.
    pub fallback: SlackText,
    /// Optional text for other devices, markup supported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackText>,
    /// Optional text that appears above attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<SlackText>,
    /// Optional color of attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HexColor>,
    /// Fields are defined as an array, and hashes contained within it will be
    /// displayed in a table inside the message attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    /// Optional small text used to display the author's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<SlackText>,
    /// Optional URL that will hyperlink the `author_name` text mentioned above. Will only
    /// work if `author_name` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_link: Option<Url>,
    /// Optional URL that displays a small 16x16px image to the left of
    /// the `author_name` text. Will only work if `author_name` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_icon: Option<Url>,
    /// Optional larger, bolder text above the main body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<SlackText>,
    /// Optional URL to link to from the title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<Url>,
    /// Optional URL to an image that will be displayed in the body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<Url>,
    /// Optional URL to an image that will be displayed as a thumbnail to the
    /// right of the body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<Url>,
    /// Optional text that will appear at the bottom of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<SlackText>,
    /// Optional URL to an image that will be displayed at the bottom of the
    /// attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_icon: Option<Url>,
    /// Optional timestamp to be displayed with the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<SlackTime>,
}

/// Slack timestamp
#[derive(Debug)]
pub struct SlackTime(NaiveDateTime);

impl SlackTime {
    /// Construct a new `SlackTime`
    pub fn new(time: &NaiveDateTime) -> SlackTime {
        SlackTime(time.clone())
    }
}

impl Serialize for SlackTime {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

/// Fields are defined as an array, and hashes contained within it will
/// be displayed in a table inside the message attachment.
#[derive(Serialize, Debug)]
pub struct Field {
    /// Shown as a bold heading above the value text.
    /// It cannot contain markup and will be escaped for you.
    pub title: String,
    /// The text value of the field. It may contain standard message markup
    /// and must be escaped as normal. May be multi-line.
    pub value: SlackText,
    /// An optional flag indicating whether the value is short enough to be
    /// displayed side-by-side with other values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<bool>,
}

/// Payload to send to slack
/// https://api.slack.com/incoming-webhooks
/// https://api.slack.com/methods/chat.postMessage
#[derive(Serialize, Debug, Default)]
pub struct Payload {
    /// text to send
    /// despite `text` stated as required, it does not seem to be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackText>,
    /// channel to send payload to
    /// note: if not provided, this will default to channel
    /// setup in slack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// username override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// specific url for icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Url>,
    /// emjoi for icon
    /// https://api.slack.com/methods/emoji.list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<String>,
    /// attachments to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// whether slack will try to fetch links and create an attachment
    /// https://api.slack.com/docs/unfurling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    /// find and link channel names and usernames
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<u8>,
    /// Change how messages are treated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Parse>,
}

/// Change how messages are treated.
#[derive(Debug)]
pub enum Parse {
    /// Full
    Full,
    /// None
    None,
}

impl Serialize for Parse {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: Serializer
    {
        let st = match *self {
            Parse::Full => "full",
            Parse::None => "none",
        };
        serializer.serialize_str(st)
    }
}
