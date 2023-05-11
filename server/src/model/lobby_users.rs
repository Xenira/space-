use crate::schema::lobby_users;
use chrono::NaiveDateTime;
use protocol::protocol;

use super::{lobbies::Lobby, users::User};

#[derive(Identifiable, Queryable, Associations, Clone)]
#[belongs_to(Lobby)]
#[belongs_to(User)]
#[table_name = "lobby_users"]
pub struct LobbyUser {
    pub id: i32,
    pub lobby_id: i32,
    pub user_id: i32,
    username: String,
    pub ready: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Into<protocol::LobbyUser> for LobbyUser {
    fn into(self) -> protocol::LobbyUser {
        protocol::LobbyUser {
            id: self.id,
            name: self.username,
            ready: self.ready,
        }
    }
}

#[derive(Insertable)]
#[table_name = "lobby_users"]
pub struct NewLobbyUser {
    lobby_id: i32,
    user_id: i32,
    username: String,
}

impl NewLobbyUser {
    pub fn from_parents(lobby: &Lobby, user: &User) -> Self {
        Self {
            lobby_id: lobby.id,
            user_id: user.id,
            username: user.username.to_string(),
        }
    }
}
