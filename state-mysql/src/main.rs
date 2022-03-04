//! state-mysql capability provider
//!
//!
//use log::debug;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use mysql_async::prelude::*;
use mysql_async::Params;
use pwatrip_twenty48_state::GetLeaderboardRequest;
use pwatrip_twenty48_state::GetLeaderboardResponse;
use pwatrip_twenty48_state::{
    CreateGameRequest, CreateGameResponse, CreatePlayerGameRequest, CreatePlayerGameResponse,
    CreatePlayerRequest, CreatePlayerResponse, GetGameRequest, GetGameResponse,
    GetPlayerGameRequest, GetPlayerGameResponse, GetPlayerGamesRequest, GetPlayerGamesResponse,
    GetPlayerRequest, GetPlayerResponse, LeaderboardEntry, State, StateReceiver,
    UpdatePlayerGameRequest, UpdatePlayerGameResponse, UpdatePlayerRequest, UpdatePlayerResponse,
};
use uuid::Uuid;
use wasmbus_rpc::Timestamp;
//use unwrap_or::unwrap_ok_or;
use wasmbus_rpc::provider::prelude::*;

const DATABASE_URL: &str = "mysql://twenty48:twenty48@localhost/twenty48";

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(StateMysqlProvider::new())?;

    eprintln!("state-mysql provider exiting");
    Ok(())
}

/// state-mysql capability provider implementation
#[derive(Clone, Provider)]
#[services(State)]
struct StateMysqlProvider {
    pool: mysql_async::Pool,
}

impl StateMysqlProvider {
    fn new() -> StateMysqlProvider {
        StateMysqlProvider {
            pool: mysql_async::Pool::new(DATABASE_URL),
        }
    }
}

/// use default implementations of provider message handlers
impl ProviderDispatch for StateMysqlProvider {}
impl ProviderHandler for StateMysqlProvider {}

impl StateMysqlProvider {
    async fn get_conn(&self) -> RpcResult<mysql_async::Conn> {
        match self.pool.get_conn().await {
            Ok(conn) => Ok(conn),
            Err(_) => Err(RpcError::Other("Database error".to_string())),
        }
    }
}

fn naivedatetime_to_timestamp(dt: &NaiveDateTime) -> Timestamp {
    Timestamp::from(Utc.from_utc_datetime(dt))
}

#[async_trait]
impl State for StateMysqlProvider {
    async fn get_player_game(
        &self,
        _ctx: &Context,
        arg: &GetPlayerGameRequest,
    ) -> RpcResult<GetPlayerGameResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Option<(u64, String, String)>, mysql_async::Error> = conn
            .exec_first(
                "
                select score, moves, bin_to_uuid(revision)
                from players_games
                where player_id = UUID_TO_BIN(:player_id)
                and game_id = UUID_TO_BIN(:game_id);
            ",
                params! {"player_id" => arg.player_id.clone(), "game_id" => arg.game_id.clone()},
            )
            .await;

        drop(conn);

        match result {
            Ok(option) => match option {
                Some((score, moves, revision)) => Ok(GetPlayerGameResponse {
                    message: None,
                    moves: Some(moves),
                    revision: Some(revision),
                    score: Some(score),
                    success: true,
                }),
                None => Ok(GetPlayerGameResponse {
                    message: Some("Not found".to_owned()),
                    moves: None,
                    revision: None,
                    score: None,
                    success: false,
                }),
            },
            Err(_) => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn get_player_games(
        &self,
        _ctx: &Context,
        arg: &GetPlayerGamesRequest,
    ) -> RpcResult<GetPlayerGamesResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Vec<String>, mysql_async::Error> = conn.exec_map("
            select bin_to_uuid(game_id) from players_games where player_id = uuid_to_bin(:player_id);
            "
            , params!{"player_id" => arg.player_id.clone()}
            , |(id,)| {id})
        .await;

        drop(conn);

        match result {
            Ok(ids) => Ok(GetPlayerGamesResponse {
                ids: Some(ids),
                message: None,
                success: true,
            }),
            Err(_) => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn create_player_game(
        &self,
        _ctx: &Context,
        arg: &CreatePlayerGameRequest,
    ) -> RpcResult<CreatePlayerGameResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
            insert ignore into players_games(player_id, game_id, revision, score, moves)
            values (uuid_to_bin(:player_id)
            , uuid_to_bin(:game_id)
            , uuid_to_bin(:revision)
            , :score
            , :moves);
            ",
                params! {
                    "player_id" => arg.player_id.clone()
                    , "game_id" => arg.game_id.clone()
                    , "revision" => Uuid::new_v4().to_hyphenated().to_string()
                    , "score" => arg.score
                    , "moves" => arg.moves.clone()
                },
            )
            .await;

        if result.is_err() {
            drop(conn);
            return Err(RpcError::Other("Database error".to_owned()));
        }

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
            select row_count();
        ",
                Params::Empty,
            )
            .await;

        drop(conn);

        match result {
            Ok(Some(value)) => match value {
                1 => Ok(CreatePlayerGameResponse {
                    message: None,
                    success: true,
                }),
                _ => Ok(CreatePlayerGameResponse {
                    message: Some("Duplicate key".to_owned()),
                    success: false,
                }),
            },
            _ => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn update_player_game(
        &self,
        _ctx: &Context,
        arg: &UpdatePlayerGameRequest,
    ) -> RpcResult<UpdatePlayerGameResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
                update players_games set 
                score = :score
                , moves = :moves
                , revision = uuid_to_bin(:next_revision)
                where
                player_id = uuid_to_bin(:player_id)
                and game_id = uuid_to_bin(:game_id)
                and revision = uuid_to_bin(:revision);
            ",
                params! {
                    "player_id" => arg.player_id.clone()
                    , "game_id" => arg.game_id.clone()
                    , "next_revision" => Uuid::new_v4().to_hyphenated().to_string()
                    , "score" => arg.score
                    , "moves" => arg.moves.clone()
                    , "revision" => arg.revision.clone()
                },
            )
            .await;

        if result.is_err() {
            drop(conn);
            return Err(RpcError::Other("Database error".to_owned()));
        }

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
            select row_count();
        ",
                Params::Empty,
            )
            .await;

        drop(conn);

        match result {
            Ok(Some(value)) => match value {
                1 => Ok(UpdatePlayerGameResponse {
                    message: None,
                    success: true,
                }),
                _ => Ok(UpdatePlayerGameResponse {
                    message: Some("Not found".to_owned()),
                    success: false,
                }),
            },
            _ => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn get_player(
        &self,
        _ctx: &Context,
        arg: &GetPlayerRequest,
    ) -> RpcResult<GetPlayerResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Option<(String, NaiveDateTime, NaiveDateTime)>, mysql_async::Error> =
            conn.exec_first(
                "
                select display_name, last_activity, last_change
                from players
                where id = uuid_to_bin(:id);
            ",
                params! {
                    "id" => arg.player_id.clone()
                },
            )
            .await;

        drop(conn);

        match result {
            Ok(option) => match option {
                Some((display_name, last_activity, last_change)) => Ok(GetPlayerResponse {
                    display_name: Some(display_name),
                    last_activity: Some(naivedatetime_to_timestamp(&last_activity)),
                    last_change: Some(naivedatetime_to_timestamp(&last_change)),
                    message: None,
                    success: true,
                }),
                None => Ok(GetPlayerResponse {
                    display_name: None,
                    last_activity: None,
                    last_change: None,
                    message: Some("Not found1".to_owned()),
                    success: false,
                }),
            },
            Err(_) => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn update_player(
        &self,
        _ctx: &Context,
        arg: &UpdatePlayerRequest,
    ) -> RpcResult<UpdatePlayerResponse> {
        let mut conn = self.get_conn().await?;

        let last_change = match DateTime::try_from(arg.last_change) {
            Ok(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            Err(_) => return Err(RpcError::Other("Error converting lastChange".to_owned())),
        };

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
                update players set
                display_name = :display_name
                , last_change = :last_change
                where id = uuid_to_bin(:player_id)
                and last_change <= :last_change;
            ",
                params! {
                    "player_id" => arg.player_id.clone()
                    , "display_name" => arg.display_name.clone()
                    , "last_change" => last_change.clone()
                },
            )
            .await;

        if result.is_err() {
            drop(conn);
            return Err(RpcError::Other("Database error".to_owned()));
        }

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
            select row_count();
        ",
                Params::Empty,
            )
            .await;

        drop(conn);

        match result {
            Ok(Some(value)) => match value {
                1 => Ok(UpdatePlayerResponse {
                    message: None,
                    success: true,
                }),
                _ => Ok(UpdatePlayerResponse {
                    message: Some("Not found".to_owned()),
                    success: false,
                }),
            },
            _ => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn create_player(
        &self,
        _ctx: &Context,
        arg: &CreatePlayerRequest,
    ) -> RpcResult<CreatePlayerResponse> {
        let mut conn = self.get_conn().await?;

        let last_change = match DateTime::try_from(arg.last_change) {
            Ok(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            Err(_) => return Err(RpcError::Other("Error converting lastChange".to_owned())),
        };

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
                insert ignore into players(id, display_name, last_change)
                values (uuid_to_bin(:player_id)
                , :display_name
                , :last_change
                );
            ",
                params! {
                    "player_id" => arg.player_id.clone()
                    , "display_name" => arg.display_name.clone()
                    , "last_change" => last_change.clone()
                },
            )
            .await;

        if result.is_err() {
            drop(conn);
            return Err(RpcError::Other(format!(
                "Database error1 {} {} {} {:?}",
                arg.player_id, arg.display_name, last_change, result
            )));
        }

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
            select row_count();
        ",
                Params::Empty,
            )
            .await;

        drop(conn);

        match result {
            Ok(Some(value)) => match value {
                1 => Ok(CreatePlayerResponse {
                    message: None,
                    success: true,
                }),
                _ => Ok(CreatePlayerResponse {
                    message: Some("Duplicate key".to_owned()),
                    success: false,
                }),
            },
            _ => Err(RpcError::Other("Database error2".to_owned())),
        }
    }

    async fn get_game(&self, _ctx: &Context, arg: &GetGameRequest) -> RpcResult<GetGameResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Option<(u64, u64, NaiveDateTime)>, mysql_async::Error> = conn
            .exec_first(
                "
                select seed, size, last_activity
                from games
                where id = uuid_to_bin(:game_id);
            ",
                params! {
                    "game_id" => arg.game_id.clone()
                },
            )
            .await;

        drop(conn);

        match result {
            Ok(option) => match option {
                Some((seed, size, last_activity)) => Ok(GetGameResponse {
                    seed: Some(seed),
                    size: Some(size),
                    last_activity: Some(naivedatetime_to_timestamp(&last_activity)),
                    message: None,
                    success: true,
                }),
                None => Ok(GetGameResponse {
                    seed: None,
                    size: None,
                    last_activity: None,
                    message: Some("Not found".to_owned()),
                    success: false,
                }),
            },
            Err(_) => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn create_game(
        &self,
        _ctx: &Context,
        arg: &CreateGameRequest,
    ) -> RpcResult<CreateGameResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
                insert ignore into games(id, seed, size)
                values (uuid_to_bin(:game_id)
                , :seed
                , :size
                );
            ",
                params! {
                    "game_id" => arg.game_id.clone()
                    , "seed" => arg.seed
                    , "size" => arg.size
                },
            )
            .await;

        if result.is_err() {
            drop(conn);
            return Err(RpcError::Other("Database error".to_owned()));
        }

        let result: Result<Option<usize>, mysql_async::Error> = conn
            .exec_first(
                "
            select row_count();
        ",
                Params::Empty,
            )
            .await;

        drop(conn);

        match result {
            Ok(Some(value)) => match value {
                1 => Ok(CreateGameResponse {
                    message: None,
                    success: true,
                }),
                _ => Ok(CreateGameResponse {
                    message: Some("Duplicate key".to_owned()),
                    success: false,
                }),
            },
            _ => Err(RpcError::Other("Database error".to_owned())),
        }
    }

    async fn get_leaderboard(
        &self,
        _ctx: &Context,
        arg: &GetLeaderboardRequest,
    ) -> RpcResult<GetLeaderboardResponse> {
        let mut conn = self.get_conn().await?;

        let result: Result<Vec<LeaderboardEntry>, mysql_async::Error> = conn.exec_map("
        select players.display_name, players_games.score, players.id = uuid_to_bin(:player_id) as requesting_player
        from players_games 
        inner join players on players_games.player_id = players.id
        where players_games.game_id = uuid_to_bin(:game_id)
        order by players_games.score desc;
            "
            , params!{"player_id" => arg.player_id.clone(), "game_id" => arg.game_id.clone()}
            , |(display_name, score, requesting_player)| {
                LeaderboardEntry{ 
                    display_name,
                    requesting_player,
                    score
                }})
        .await;

        drop(conn);

        match result {
            Ok(leaderboard) => Ok(GetLeaderboardResponse {
                leaderboard: Some(leaderboard),
                message: None,
                success: true,
            }),
            Err(_) => Err(RpcError::Other("Database error".to_owned())),
        }
    }
}
