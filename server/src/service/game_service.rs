use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*, update};
use protocol::{enum_iterator::all, protocol::Protocol};
use rand::seq::SliceRandom;

use crate::{
    model::{
        game::{Game, GameUpdate, NewGame},
        game_user_avatar_choices::NewGameUserAvatarChoice,
        game_users::{GameUser, NewGameUser},
        lobbies::Lobby,
        lobby_users::LobbyUser,
        polling::{ActivePolls, Channel},
    },
    schema::{game_user_avatar_choices, game_users, games, lobby_users},
    Database,
};

pub async fn start_game(db: &Database, lobby: &Lobby) {
    let lobby = lobby.clone();
    let game = db
        .run(move |con| {
            let new_game = insert_into(games::table)
                .values(NewGame::new())
                .returning(games::id)
                .get_result::<i32>(con)
                .unwrap();

            let mut heros = protocol::gods::GODS.clone();
            heros.shuffle(&mut rand::thread_rng());

            (
                new_game,
                LobbyUser::belonging_to(&lobby)
                    .select(lobby_users::user_id)
                    .load::<i32>(con)
                    .unwrap()
                    .iter()
                    .map(|user| {
                        let game_user = insert_into(game_users::table)
                            .values(NewGameUser::from_parents(new_game, *user))
                            .returning(game_users::id)
                            .get_result::<i32>(con)
                            .unwrap();

                        let hero_choices = Vec::drain(&mut heros, 0..4).collect::<Vec<_>>();

                        insert_into(game_user_avatar_choices::table)
                            .values(
                                hero_choices
                                    .iter()
                                    .map(|hero| {
                                        NewGameUserAvatarChoice::from_parents(
                                            new_game, game_user, hero.id,
                                        )
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .execute(con)
                            .unwrap();

                        (user.clone(), hero_choices)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .await;

    ActivePolls::join_users(
        Channel::Game(game.0),
        game.1
            .iter()
            .map(|(user, _)| user.clone())
            .collect::<Vec<_>>(),
    );
    for (user, hero_choices) in game.1 {
        ActivePolls::notify(&user, Protocol::GameStartResponse(hero_choices)).await;
    }
}

pub async fn next_turn(db: &Database, game: &Game) {
    debug!("Next turn for game {:?}", game);
    let game = game.clone();
    let next_turn = db
        .run(move |con| {
            let next_turn = game.current_round + 1;

            let active_users = GameUser::belonging_to(&game)
                .filter(game_users::health.gt(0))
                .load::<GameUser>(con)
                .unwrap_or(vec![]);

            if active_users.len() <= 1 {
                debug!("Game {} is over", game.id);
                close_game(con, game.id);
                return 0;
            }

            let game_update = GameUpdate {
                current_round: Some(next_turn),
                next_battle: Some(get_next_turn_time(next_turn)),
            };

            debug!("Updating game {:?} with {:?}", game, game_update);
            update(games::table)
                .filter(games::id.eq(game.id))
                .set(game_update)
                .execute(con)
                .unwrap();

            next_turn
        })
        .await;
}

fn get_next_turn_time(turn: i32) -> NaiveDateTime {
    let turn: i64 = turn.into();
    chrono::Utc::now().naive_utc() + chrono::Duration::seconds(30 + (turn - 1) * 5)
}

fn close_game(con: &mut PgConnection, game_id: i32) {
    debug!("Closing game {}", game_id);
    delete(games::table)
        .filter(games::id.eq(game_id))
        .execute(con)
        .unwrap();
}
