use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};

#[derive(Debug, Clone, Serialize)]
/// Use this method to edit live location messages. A location can be edited
/// until its live_period expires or editing is explicitly disabled by a
/// call to [`StopMessageLiveLocation`]. On success, if the edited message
/// was sent by the bot, the edited [`Message`] is returned, otherwise True
/// is returned.
///
/// [`StopMessageLiveLocation`]: crate::requests::StopMessageLiveLocation
/// [`Message`]: crate::types::Message
pub struct EditMessageLiveLocation<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat or username of the target channel (in the format
    /// @channelusername)
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if inline_message_id is not specified. Identifier of the
    /// message to edit
    message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Required if chat_id and message_id are not specified. Identifier of
    /// the inline message
    inline_message_id: Option<String>,
    /// Latitude of new location
    latitude: f64,
    /// Longitude of new location
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for EditMessageLiveLocation<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl EditMessageLiveLocation<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageLiveLocation",
            &self,
        )
        .await
    }
}

impl<'a> EditMessageLiveLocation<'a> {
    pub(crate) fn new<Lt, Lg>(bot: &'a Bot, latitude: Lt, longitude: Lg) -> Self
    where
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        Self {
            bot,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude: latitude.into(),
            longitude: longitude.into(),
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = Some(value.into());
        self
    }

    pub fn message_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.message_id = Some(value.into());
        self
    }

    pub fn inline_message_id<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.inline_message_id = Some(value.into());
        self
    }

    pub fn latitude<Lt>(mut self, value: Lt) -> Self
    where
        Lt: Into<f64>,
    {
        self.latitude = value.into();
        self
    }

    pub fn longitude<Lg>(mut self, value: Lg) -> Self
    where
        Lg: Into<f64>,
    {
        self.longitude = value.into();
        self
    }
}
