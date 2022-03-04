// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateGameRequest {
    #[serde(rename = "gameId")]
    #[serde(default)]
    pub game_id: String,
    pub seed: u64,
    pub size: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateGameResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatePlayerGameRequest {
    #[serde(rename = "gameId")]
    #[serde(default)]
    pub game_id: String,
    #[serde(default)]
    pub moves: String,
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
    pub score: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatePlayerGameResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatePlayerRequest {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "lastChange")]
    #[serde(default)]
    pub last_change: Timestamp,
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatePlayerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

pub type GameIdList = Vec<String>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetGameRequest {
    #[serde(rename = "gameId")]
    #[serde(default)]
    pub game_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetGameResponse {
    #[serde(rename = "lastActivity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetLeaderboardRequest {
    #[serde(rename = "gameId")]
    #[serde(default)]
    pub game_id: String,
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetLeaderboardResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaderboard: Option<Leaderboard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPlayerGameRequest {
    #[serde(rename = "gameId")]
    #[serde(default)]
    pub game_id: String,
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPlayerGameResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moves: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<u64>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPlayerGamesRequest {
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPlayerGamesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<GameIdList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPlayerRequest {
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPlayerResponse {
    #[serde(rename = "displayName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastActivity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<Timestamp>,
    #[serde(rename = "lastChange")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_change: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

pub type Leaderboard = Vec<LeaderboardEntry>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LeaderboardEntry {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "requestingPlayer")]
    #[serde(default)]
    pub requesting_player: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdatePlayerGameRequest {
    #[serde(rename = "gameId")]
    #[serde(default)]
    pub game_id: String,
    #[serde(default)]
    pub moves: String,
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
    #[serde(default)]
    pub revision: String,
    pub score: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdatePlayerGameResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdatePlayerRequest {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "lastChange")]
    #[serde(default)]
    pub last_change: Timestamp,
    #[serde(rename = "playerId")]
    #[serde(default)]
    pub player_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdatePlayerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

/// wasmbus.contractId: pwatrip:twenty48:state
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait State {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "pwatrip:twenty48:state"
    }
    /// Get the state of a game
    async fn get_player_game(
        &self,
        ctx: &Context,
        arg: &GetPlayerGameRequest,
    ) -> RpcResult<GetPlayerGameResponse>;
    async fn create_player_game(
        &self,
        ctx: &Context,
        arg: &CreatePlayerGameRequest,
    ) -> RpcResult<CreatePlayerGameResponse>;
    async fn update_player_game(
        &self,
        ctx: &Context,
        arg: &UpdatePlayerGameRequest,
    ) -> RpcResult<UpdatePlayerGameResponse>;
    async fn get_player_games(
        &self,
        ctx: &Context,
        arg: &GetPlayerGamesRequest,
    ) -> RpcResult<GetPlayerGamesResponse>;
    async fn get_player(
        &self,
        ctx: &Context,
        arg: &GetPlayerRequest,
    ) -> RpcResult<GetPlayerResponse>;
    async fn create_player(
        &self,
        ctx: &Context,
        arg: &CreatePlayerRequest,
    ) -> RpcResult<CreatePlayerResponse>;
    async fn update_player(
        &self,
        ctx: &Context,
        arg: &UpdatePlayerRequest,
    ) -> RpcResult<UpdatePlayerResponse>;
    async fn get_game(&self, ctx: &Context, arg: &GetGameRequest) -> RpcResult<GetGameResponse>;
    async fn create_game(
        &self,
        ctx: &Context,
        arg: &CreateGameRequest,
    ) -> RpcResult<CreateGameResponse>;
    async fn get_leaderboard(
        &self,
        ctx: &Context,
        arg: &GetLeaderboardRequest,
    ) -> RpcResult<GetLeaderboardResponse>;
}

/// StateReceiver receives messages defined in the State service trait
#[doc(hidden)]
#[async_trait]
pub trait StateReceiver: MessageDispatch + State {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "GetPlayerGame" => {
                let value: GetPlayerGameRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::get_player_game(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.GetPlayerGame",
                    arg: Cow::Owned(buf),
                })
            }
            "CreatePlayerGame" => {
                let value: CreatePlayerGameRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::create_player_game(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.CreatePlayerGame",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdatePlayerGame" => {
                let value: UpdatePlayerGameRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::update_player_game(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.UpdatePlayerGame",
                    arg: Cow::Owned(buf),
                })
            }
            "GetPlayerGames" => {
                let value: GetPlayerGamesRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::get_player_games(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.GetPlayerGames",
                    arg: Cow::Owned(buf),
                })
            }
            "GetPlayer" => {
                let value: GetPlayerRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::get_player(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.GetPlayer",
                    arg: Cow::Owned(buf),
                })
            }
            "CreatePlayer" => {
                let value: CreatePlayerRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::create_player(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.CreatePlayer",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdatePlayer" => {
                let value: UpdatePlayerRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::update_player(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.UpdatePlayer",
                    arg: Cow::Owned(buf),
                })
            }
            "GetGame" => {
                let value: GetGameRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::get_game(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.GetGame",
                    arg: Cow::Owned(buf),
                })
            }
            "CreateGame" => {
                let value: CreateGameRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::create_game(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.CreateGame",
                    arg: Cow::Owned(buf),
                })
            }
            "GetLeaderboard" => {
                let value: GetLeaderboardRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = State::get_leaderboard(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "State.GetLeaderboard",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "State::{}",
                message.method
            ))),
        }
    }
}

/// StateSender sends messages to a State service
/// client for sending State messages
#[derive(Debug)]
pub struct StateSender<T: Transport> {
    transport: T,
}

impl<T: Transport> StateSender<T> {
    /// Constructs a StateSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> StateSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl StateSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl StateSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a State provider
    /// implementing the 'pwatrip:twenty48:state' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("pwatrip:twenty48:state", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a State provider
    /// implementing the 'pwatrip:twenty48:state' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "pwatrip:twenty48:state",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> State for StateSender<T> {
    #[allow(unused)]
    /// Get the state of a game
    async fn get_player_game(
        &self,
        ctx: &Context,
        arg: &GetPlayerGameRequest,
    ) -> RpcResult<GetPlayerGameResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.GetPlayerGame",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetPlayerGame", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn create_player_game(
        &self,
        ctx: &Context,
        arg: &CreatePlayerGameRequest,
    ) -> RpcResult<CreatePlayerGameResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.CreatePlayerGame",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "CreatePlayerGame", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_player_game(
        &self,
        ctx: &Context,
        arg: &UpdatePlayerGameRequest,
    ) -> RpcResult<UpdatePlayerGameResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.UpdatePlayerGame",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "UpdatePlayerGame", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn get_player_games(
        &self,
        ctx: &Context,
        arg: &GetPlayerGamesRequest,
    ) -> RpcResult<GetPlayerGamesResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.GetPlayerGames",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetPlayerGames", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn get_player(
        &self,
        ctx: &Context,
        arg: &GetPlayerRequest,
    ) -> RpcResult<GetPlayerResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.GetPlayer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetPlayer", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn create_player(
        &self,
        ctx: &Context,
        arg: &CreatePlayerRequest,
    ) -> RpcResult<CreatePlayerResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.CreatePlayer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "CreatePlayer", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_player(
        &self,
        ctx: &Context,
        arg: &UpdatePlayerRequest,
    ) -> RpcResult<UpdatePlayerResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.UpdatePlayer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "UpdatePlayer", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn get_game(&self, ctx: &Context, arg: &GetGameRequest) -> RpcResult<GetGameResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.GetGame",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetGame", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn create_game(
        &self,
        ctx: &Context,
        arg: &CreateGameRequest,
    ) -> RpcResult<CreateGameResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.CreateGame",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "CreateGame", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn get_leaderboard(
        &self,
        ctx: &Context,
        arg: &GetLeaderboardRequest,
    ) -> RpcResult<GetLeaderboardResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "State.GetLeaderboard",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetLeaderboard", e)))?;
        Ok(value)
    }
}
