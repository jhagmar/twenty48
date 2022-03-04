mod comm;

use chrono::{DateTime, Utc};
use comm::Player;
use engine::{Game, GameExchange};
use once_cell::sync::Lazy;
use pwatrip_twenty48_state::{
    CreateGameRequest, CreatePlayerGameRequest, CreatePlayerRequest, GetGameRequest,
    GetLeaderboardRequest, GetPlayerGameRequest, GetPlayerGamesRequest, GetPlayerRequest, State,
    StateSender, UpdatePlayerGameRequest, UpdatePlayerRequest,
};
use route_recognizer::{Params, Router};
use wasmbus_rpc::{actor::prelude::*, Timestamp};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct Twenty48BackendActor {}

const PLAYER_ID_KEY: &'static str = "player_id";
const GAME_ID_KEY: &'static str = "game_id";
static GET_PLAYER_ROUTE: Lazy<String> = Lazy::new(|| "GET/players/:".to_owned() + PLAYER_ID_KEY);
static UPDATE_PLAYER_ROUTE: Lazy<String> = Lazy::new(|| "PUT/players/:".to_owned() + PLAYER_ID_KEY);
static GET_PLAYER_GAME_ROUTE: Lazy<String> =
    Lazy::new(|| "GET/players/:".to_owned() + PLAYER_ID_KEY + "/games/:" + GAME_ID_KEY);
static PUT_PLAYER_GAME_ROUTE: Lazy<String> =
    Lazy::new(|| "PUT/players/:".to_owned() + PLAYER_ID_KEY + "/games/:" + GAME_ID_KEY);
static GET_PLAYER_GAMES_ROUTE: Lazy<String> =
    Lazy::new(|| "GET/players/:".to_owned() + PLAYER_ID_KEY + "/games");
static GET_PLAYER_GAME_LEADERBOARD_ROUTE: Lazy<String> = Lazy::new(|| {
    "GET/players/:".to_owned() + PLAYER_ID_KEY + "/games/:" + GAME_ID_KEY + "/leaderboard"
});
static GET_GAME_ROUTE: Lazy<String> = Lazy::new(|| "GET/games/:".to_owned() + GAME_ID_KEY);

enum HttpResponseCodes {
    //Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

fn rpc_error_to_http_response(err: RpcError) -> Result<HttpResponse, RpcError> {
    Ok(HttpResponse {
        status_code: HttpResponseCodes::InternalServerError as u16,
        body: format!("{:?}", err).as_bytes().to_vec(),
        ..Default::default()
    })
}

fn failure_to_http_response(message: &str) -> Result<HttpResponse, RpcError> {
    Ok(HttpResponse {
        status_code: HttpResponseCodes::NotFound as u16,
        body: message.as_bytes().to_vec(),
        ..Default::default()
    })
}

async fn handle_get_player_game(
    ctx: &Context,
    params: &Params,
) -> std::result::Result<HttpResponse, RpcError> {
    let game_id = params.find(GAME_ID_KEY).unwrap();
    let player_id = params.find(PLAYER_ID_KEY).unwrap();

    let sender = StateSender::new();

    let request = GetPlayerGameRequest {
        game_id: game_id.to_owned(),
        player_id: player_id.to_owned(),
    };

    let (moves, score) = match sender.get_player_game(ctx, &request).await {
        Ok(resp) => match resp.success {
            true => (resp.moves.unwrap(), resp.score.unwrap() as usize),
            false => return failure_to_http_response(&resp.message.unwrap()),
        },
        Err(err) => return rpc_error_to_http_response(err),
    };

    let request = GetGameRequest {
        game_id: game_id.to_owned(),
    };

    let (seed, size) = match sender.get_game(ctx, &request).await {
        Ok(resp) => match resp.success {
            true => (resp.seed.unwrap().to_string(), resp.size.unwrap() as usize),
            false => return failure_to_http_response(&resp.message.unwrap()),
        },
        Err(err) => return rpc_error_to_http_response(err),
    };

    let gx = match GameExchange::new(String::new(), game_id.to_owned(), score, seed, size, &moves) {
        Ok(gx) => gx,
        Err(_) => {
            return Ok(HttpResponse {
                status_code: HttpResponseCodes::InternalServerError as u16,
                body: "Error creating game exchange response".as_bytes().to_vec(),
                ..Default::default()
            })
        }
    };

    Ok(HttpResponse {
        body: serde_json::to_string(&gx).unwrap().as_bytes().to_vec(),
        ..Default::default()
    })
}

async fn handle_update_player_game(
    ctx: &Context,
    params: &Params,
    body: &Vec<u8>,
) -> std::result::Result<HttpResponse, RpcError> {
    let game_id = params.find(GAME_ID_KEY).unwrap();
    let player_id = params.find(PLAYER_ID_KEY).unwrap();

    let body_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => {
            return Ok(HttpResponse {
                status_code: HttpResponseCodes::BadRequest as u16,
                body: "Error parsing request body as string".as_bytes().to_vec(),
                ..Default::default()
            })
        }
    };

    let new_gx = match GameExchange::from_json(body_str.to_owned()) {
        Some(gx) => gx,
        None => {
            return Ok(HttpResponse {
                status_code: HttpResponseCodes::BadRequest as u16,
                body: "Error parsing request body".as_bytes().to_vec(),
                ..Default::default()
            })
        }
    };

    if new_gx.get_size() != 4 {
        return Ok(HttpResponse {
            status_code: HttpResponseCodes::BadRequest as u16,
            body: "Only games of size 4 are allowed".as_bytes().to_vec(),
            ..Default::default()
        });
    }

    let new_game = match Game::try_from(&new_gx) {
        Ok(game) => game,
        Err(_) => {
            return Ok(HttpResponse {
                status_code: HttpResponseCodes::BadRequest as u16,
                body: "Error parsing request body as game".as_bytes().to_vec(),
                ..Default::default()
            })
        }
    };

    let sender = StateSender::new();

    let (seed, size, game_existed) = {
        let seed;
        let size;
        let game_existed;

        loop {
            let request = GetGameRequest {
                game_id: game_id.to_owned(),
            };

            let game_resp = match sender.get_game(ctx, &request).await {
                Ok(resp) => match resp.success {
                    true => Some((resp.seed.unwrap().to_string(), resp.size.unwrap() as usize)),
                    false => None,
                },
                Err(err) => return rpc_error_to_http_response(err),
            };

            match game_resp {
                Some((sed, siz)) => {
                    seed = sed;
                    size = siz;
                    game_existed = true;
                    break;
                }
                None => {
                    let request = CreateGameRequest {
                        game_id: game_id.to_owned(),
                        seed: new_game.get_seed(),
                        size: new_game.get_size() as u64,
                    };
                    match sender.create_game(ctx, &request).await {
                        Ok(resp) => match resp.success {
                            true => {
                                seed = new_game.get_seed().to_string();
                                size = new_game.get_size();
                                game_existed = false;
                                break;
                            }
                            false => (),
                        },
                        Err(err) => return rpc_error_to_http_response(err),
                    }
                }
            };
        }

        (seed, size, game_existed)
    };

    loop {
        let request = GetPlayerRequest {
            player_id: player_id.to_owned(),
        };

        match sender.get_player(ctx, &request).await {
            Ok(resp) => match resp.success {
                true => break,
                false => {
                    let request = CreatePlayerRequest {
                        display_name: new_gx.get_player().to_owned(),
                        last_change: Timestamp::from(Utc::now()),
                        player_id: player_id.to_owned(),
                    };

                    match sender.create_player(ctx, &request).await {
                        Ok(resp) => match resp.success {
                            true => break,
                            false => (),
                        },
                        Err(err) => return rpc_error_to_http_response(err),
                    }
                }
            },
            Err(err) => return rpc_error_to_http_response(err),
        }
    }

    loop {
        let player_game_data = match game_existed {
            true => {
                let request = GetPlayerGameRequest {
                    game_id: game_id.to_owned(),
                    player_id: player_id.to_owned(),
                };

                match sender.get_player_game(ctx, &request).await {
                    Ok(resp) => match resp.success {
                        true => Some((
                            resp.moves.unwrap(),
                            resp.score.unwrap() as usize,
                            resp.revision.unwrap(),
                        )),
                        false => None,
                    },
                    Err(err) => return rpc_error_to_http_response(err),
                }
            }
            false => None,
        };

        let old_game_option = match &player_game_data {
            Some((moves, score, _)) => {
                match GameExchange::new(
                    String::new(),
                    game_id.to_owned(),
                    *score,
                    seed.clone(),
                    size,
                    &moves,
                ) {
                    Ok(gx) => match Game::try_from(&gx) {
                        Ok(game) => Some(game),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            }
            None => None,
        };

        if let Some(old_game) = old_game_option {
            if old_game == new_game {
                return Ok(HttpResponse {
                    body: body.clone(),
                    ..Default::default()
                });
            }

            if !old_game.is_ancestor(&new_game) {
                return Ok(HttpResponse {
                    status_code: HttpResponseCodes::BadRequest as u16,
                    body: "Submitted game is not a descendant of the stored game"
                        .as_bytes()
                        .to_vec(),
                    ..Default::default()
                });
            }

            let (_, _, revision) = player_game_data.unwrap();

            let request = UpdatePlayerGameRequest {
                game_id: game_id.to_owned(),
                moves: new_gx.get_moves_str(),
                player_id: player_id.to_owned(),
                revision,
                score: new_game.get_score() as u64,
            };

            match sender.update_player_game(ctx, &request).await {
                Ok(resp) => match resp.success {
                    true => {
                        return Ok(HttpResponse {
                            body: new_gx.to_json().unwrap().as_bytes().to_vec(),
                            ..Default::default()
                        })
                    }
                    false => (),
                },
                Err(err) => return rpc_error_to_http_response(err),
            }
        } else {
            let request = CreatePlayerGameRequest {
                game_id: game_id.to_owned(),
                moves: new_gx.get_moves_str(),
                player_id: player_id.to_owned(),
                score: new_game.get_score() as u64,
            };

            match sender.create_player_game(ctx, &request).await {
                Ok(resp) => match resp.success {
                    true => {
                        return Ok(HttpResponse {
                            body: new_gx.to_json().unwrap().as_bytes().to_vec(),
                            ..Default::default()
                        })
                    }
                    false => (),
                },
                Err(err) => return rpc_error_to_http_response(err),
            }
        }
    }
}

async fn handle_get_player_games(
    ctx: &Context,
    params: &Params,
) -> std::result::Result<HttpResponse, RpcError> {
    let sender = StateSender::new();
    let request = GetPlayerGamesRequest {
        player_id: params.find(PLAYER_ID_KEY).unwrap().to_owned(),
    };
    let result = sender.get_player_games(ctx, &request).await;

    match result {
        Ok(resp) => match resp.success {
            true => {
                let json_string = serde_json::to_string(&comm::GameIdList {
                    ids: resp.ids.unwrap(),
                })
                .unwrap();
                Ok(HttpResponse {
                    body: json_string.as_bytes().to_vec(),
                    ..Default::default()
                })
            }
            false => failure_to_http_response(&resp.message.unwrap()),
        },
        Err(err) => rpc_error_to_http_response(err),
    }
}

async fn handle_get_player(
    ctx: &Context,
    params: &Params,
) -> std::result::Result<HttpResponse, RpcError> {
    let player_id = params.find(PLAYER_ID_KEY).unwrap();

    let sender = StateSender::new();

    let request = GetPlayerRequest {
        player_id: player_id.to_owned(),
    };

    match sender.get_player(ctx, &request).await {
        Ok(resp) => match resp.success {
            true => {
                let player = comm::Player {
                    display_name: resp.display_name.unwrap(),
                    last_change: DateTime::<Utc>::try_from(resp.last_change.unwrap()).unwrap(),
                };
                Ok(HttpResponse {
                    body: serde_json::to_string(&player).unwrap().as_bytes().to_vec(),
                    ..Default::default()
                })
            }
            false => failure_to_http_response(&resp.message.unwrap()),
        },
        Err(err) => rpc_error_to_http_response(err),
    }
}

async fn handle_update_player(
    ctx: &Context,
    params: &Params,
    body: &Vec<u8>,
) -> std::result::Result<HttpResponse, RpcError> {
    let player_id = params.find(PLAYER_ID_KEY).unwrap();

    let body_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => {
            return Ok(HttpResponse {
                status_code: HttpResponseCodes::BadRequest as u16,
                body: "Error parsing request body as string".as_bytes().to_vec(),
                ..Default::default()
            })
        }
    };

    let player = match serde_json::from_str::<comm::Player>(body_str) {
        Ok(player) => player,
        Err(_) => {
            return Ok(HttpResponse {
                status_code: HttpResponseCodes::BadRequest as u16,
                body: "Error parsing request body as player".as_bytes().to_vec(),
                ..Default::default()
            })
        }
    };

    let sender = StateSender::new();

    loop {
        let request = GetPlayerRequest {
            player_id: player_id.to_owned(),
        };

        match sender.get_player(ctx, &request).await {
            Ok(gp_resp) => match gp_resp.success {
                true => {
                    let request = UpdatePlayerRequest {
                        display_name: player.display_name.clone(),
                        last_change: Timestamp::from(player.last_change),
                        player_id: player_id.to_owned(),
                    };

                    match sender.update_player(ctx, &request).await {
                        Ok(resp) => match resp.success {
                            true => {
                                return Ok(HttpResponse {
                                    body: body.clone(),
                                    ..Default::default()
                                })
                            }
                            false => {
                                let player = Player {
                                    display_name: gp_resp.display_name.unwrap().clone(),
                                    last_change: DateTime::<Utc>::try_from(
                                        gp_resp.last_change.unwrap(),
                                    )
                                    .unwrap(),
                                };
                                return Ok(HttpResponse {
                                    body: serde_json::to_string(&player)
                                        .unwrap()
                                        .as_bytes()
                                        .to_vec(),
                                    ..Default::default()
                                });
                            }
                        },
                        Err(err) => return rpc_error_to_http_response(err),
                    }
                }
                false => {
                    let request = CreatePlayerRequest {
                        display_name: player.display_name.clone(),
                        last_change: Timestamp::from(player.last_change),
                        player_id: player_id.to_owned(),
                    };

                    match sender.create_player(ctx, &request).await {
                        Ok(resp) => match resp.success {
                            true => {
                                return Ok(HttpResponse {
                                    body: body.clone(),
                                    ..Default::default()
                                })
                            }
                            false => (),
                        },
                        Err(err) => return rpc_error_to_http_response(err),
                    }
                }
            },
            Err(err) => return rpc_error_to_http_response(err),
        }
    }
}

async fn handle_get_player_game_leaderboard(
    ctx: &Context,
    params: &Params,
) -> std::result::Result<HttpResponse, RpcError> {
    let game_id = params.find(GAME_ID_KEY).unwrap();
    let player_id = params.find(PLAYER_ID_KEY).unwrap();

    let sender = StateSender::new();

    let request = GetLeaderboardRequest {
        game_id: game_id.to_owned(),
        player_id: player_id.to_owned(),
    };

    let leaderboard = match sender.get_leaderboard(ctx, &request).await {
        Ok(resp) => match resp.success {
            true => resp.leaderboard.unwrap(),
            false => return failure_to_http_response(&resp.message.unwrap()),
        },
        Err(err) => return rpc_error_to_http_response(err),
    };

    Ok(HttpResponse {
        body: serde_json::to_string(&leaderboard)
            .unwrap()
            .as_bytes()
            .to_vec(),
        ..Default::default()
    })
}

async fn handle_get_game(
    ctx: &Context,
    params: &Params,
) -> std::result::Result<HttpResponse, RpcError> {
    let game_id = params.find(GAME_ID_KEY).unwrap();

    let sender = StateSender::new();

    let request = GetGameRequest {
        game_id: game_id.to_owned(),
    };

    let game = match sender.get_game(ctx, &request).await {
        Ok(resp) => match resp.success {
            true => comm::Game {
                seed: resp.seed.unwrap().to_string(),
                size: resp.size.unwrap(),
            },
            false => return failure_to_http_response(&resp.message.unwrap()),
        },
        Err(err) => return rpc_error_to_http_response(err),
    };

    Ok(HttpResponse {
        body: serde_json::to_string(&game).unwrap().as_bytes().to_vec(),
        ..Default::default()
    })
}

enum Requests {
    GetPlayer,
    UpdatePlayer,
    GetPlayerGame,
    UpdatePlayerGame,
    GetPlayerGames,
    GetPlayerGameLeaderboard,
    GetGame,
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for Twenty48BackendActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let mut router = Router::<Requests>::new();

        router.add(&GET_PLAYER_GAME_ROUTE, Requests::GetPlayerGame);
        router.add(&PUT_PLAYER_GAME_ROUTE, Requests::UpdatePlayerGame);
        router.add(&GET_PLAYER_GAMES_ROUTE, Requests::GetPlayerGames);
        router.add(&GET_PLAYER_ROUTE, Requests::GetPlayer);
        router.add(&UPDATE_PLAYER_ROUTE, Requests::UpdatePlayer);
        router.add(
            &GET_PLAYER_GAME_LEADERBOARD_ROUTE,
            Requests::GetPlayerGameLeaderboard,
        );
        router.add(&GET_GAME_ROUTE, Requests::GetGame);

        let route = req.method.clone() + &req.path;

        match router.recognize(&route) {
            Ok(m) => match m.handler() {
                Requests::GetPlayerGame => handle_get_player_game(ctx, m.params()).await,
                Requests::UpdatePlayerGame => {
                    handle_update_player_game(ctx, m.params(), &req.body).await
                }
                Requests::GetPlayerGames => handle_get_player_games(ctx, m.params()).await,
                Requests::GetPlayer => handle_get_player(ctx, m.params()).await,
                Requests::UpdatePlayer => handle_update_player(ctx, m.params(), &req.body).await,
                Requests::GetPlayerGameLeaderboard => {
                    handle_get_player_game_leaderboard(ctx, m.params()).await
                }
                Requests::GetGame => handle_get_game(ctx, m.params()).await,
            },
            Err(_) => Ok(HttpResponse {
                status_code: 404,
                body: format!("{} not found", route).as_bytes().to_vec(),
                ..Default::default()
            }),
        }
    }
}
