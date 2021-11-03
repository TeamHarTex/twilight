use crate::{
    datetime::Timestamp,
    gateway::presence::Presence,
    guild::Member,
    id::{ChannelId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMember {
    // Values currently unknown and undocumented.
    pub flags: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ChannelId>,
    pub join_timestamp: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ThreadMember, UserId};
    use crate::datetime::{Timestamp, TimestampParseError};
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_thread_member() -> Result<(), TimestampParseError> {
        const DATETIME: &str = "2021-09-19T14:17:32.000000+00:00";

        let join_timestamp = Timestamp::from_str(DATETIME)?;

        let value = ThreadMember {
            flags: 3,
            id: Some(ChannelId::new(1).expect("non zero")),
            member: None,
            presence: None,
            join_timestamp,
            user_id: Some(UserId::new(2).expect("non zero")),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("flags"),
                Token::U64(3),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("join_timestamp"),
                Token::Str(DATETIME),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}