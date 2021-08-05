pub enum Method {
    ListMembersOfChannel,
    OpenDirectMessage,
    PostMessage,
}

impl Into<reqwest::Url> for Method {
    fn into(self) -> reqwest::Url {
        let url = match self {
            Method::ListMembersOfChannel => "https://slack.com/api/conversations.members",
            Method::OpenDirectMessage => "https://slack.com/api/conversations.open",
            Method::PostMessage => "https://slack.com/api/chat.postMessage",
        };

        reqwest::Url::parse(url).expect("failed to parse URL")
    }
}
