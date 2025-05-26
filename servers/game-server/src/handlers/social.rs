use vivian_codegen::handlers;
use super::NetContext;
use tracing::error;
use vivian_proto::{EmojiInfo, GetChatEmojiListCsReq, GetChatEmojiListScRsp, GetFriendListCsReq, GetFriendListScRsp, GetOnlineFriendsListCsReq, GetOnlineFriendsListScRsp};

pub struct SocialHandler;

#[handlers]
impl SocialHandler {
    pub fn on_get_friend_list_cs_req(
        context: &mut NetContext<'_>,
        _request: GetFriendListCsReq,
    ) -> GetFriendListScRsp {
        if context.player.uid == 0 {
            error!("No valid player UID in NetContext for GetFriendListCsReq");
            return GetFriendListScRsp { retcode: -1 }; // Unauthorized
        }

        // Get friends from buddy_model
        

        GetFriendListScRsp {
            retcode: 0,
        }
    }

    pub fn on_get_online_friends_list_cs_req(
        context: &mut NetContext<'_>,
        _request: GetOnlineFriendsListCsReq,
    ) -> GetOnlineFriendsListScRsp {
        if context.player.uid == 0 {
            error!("No valid player UID in NetContext for GetOnlineFriendsListCsReq");
            return GetOnlineFriendsListScRsp { retcode: -1, online_friends: vec![] }; // Unauthorized
        }

        // Get online friends from buddy_model
  

        GetOnlineFriendsListScRsp {
            retcode: 0,
            online_friends: vec![],
        }
    }

    pub fn on_get_chat_emoji_list_cs_req(
        context: &mut NetContext<'_>,
        _request: GetChatEmojiListCsReq,
    ) -> GetChatEmojiListScRsp {
        // Mock emoji list (global)
        vec![
            EmojiInfo {
                emoji_id: 1,
                name: "smile".to_string(),
            },
            EmojiInfo {
                emoji_id: 2,
                name: "heart".to_string(),
            },
        ];

        GetChatEmojiListScRsp {
            retcode: 0,
            emojis: vec![],
        }
    }
}