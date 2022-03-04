// state.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "eu.pwatrip.twenty48.state", crate: "pwatrip_twenty_48_state" } ]

namespace eu.pwatrip.twenty48.state

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

@wasmbus(
    contractId: "pwatrip:twenty48:state",
    actorReceive: true,
    providerReceive: true )
service State {
  version: "0.1",
  operations: [ 
    GetPlayerGame
    , CreatePlayerGame
    , UpdatePlayerGame
    , GetPlayerGames
    , GetPlayer
    , CreatePlayer
    , UpdatePlayer
    , GetGame
    , CreateGame
    , GetLeaderboard
  ]
}

/// Get the state of a game
operation GetPlayerGame {
  input: GetPlayerGameRequest,
  output: GetPlayerGameResponse
}

structure GetPlayerGameRequest {
  @required
  playerId: String,

  @required
  gameId: String,
}

structure GetPlayerGameResponse {
  @required
  success: Boolean,
  message: String,
  revision: String,
  score: U64,
  moves: String,
}

operation CreatePlayerGame {
  input: CreatePlayerGameRequest,
  output: CreatePlayerGameResponse
}

structure CreatePlayerGameRequest {
  @required
  playerId: String,
  @required
  gameId: String,
  @required
  score: U64,
  @required
  moves: String,
}

structure CreatePlayerGameResponse {
  @required
  success: Boolean,
  message: String,
}

operation UpdatePlayerGame {
  input: UpdatePlayerGameRequest,
  output: UpdatePlayerGameResponse
}

structure UpdatePlayerGameRequest {
  @required
  playerId: String,
  @required
  gameId: String,
  @required
  revision: String,
  @required
  score: U64,
  @required
  moves: String,
}

structure UpdatePlayerGameResponse {
  @required
  success: Boolean,
  message: String,
}

operation GetPlayerGames {
  input: GetPlayerGamesRequest,
  output: GetPlayerGamesResponse,
}

structure GetPlayerGamesRequest {
  @required
  playerId: String,
}

structure GetPlayerGamesResponse {
  @required
  success: Boolean,
  message: String,
  ids: GameIdList,
}

list GameIdList {
  member: String
}

operation GetPlayer {
  input: GetPlayerRequest,
  output: GetPlayerResponse,
}

structure GetPlayerRequest {
  @required
  playerId: String,
}

structure GetPlayerResponse {
  @required
  success: Boolean,
  message: String,
  displayName: String,
  lastActivity: Timestamp,
  lastChange: Timestamp,
}

operation CreatePlayer {
  input: CreatePlayerRequest,
  output: CreatePlayerResponse,
}

structure CreatePlayerRequest {
  @required
  playerId: String,
  @required
  displayName: String,
  @required
  lastChange: Timestamp,
}

structure CreatePlayerResponse {
  @required
  success: Boolean,
  message: String,
}

operation UpdatePlayer {
  input: UpdatePlayerRequest,
  output: UpdatePlayerResponse,
}

structure UpdatePlayerRequest {
  @required
  playerId: String,
  @required
  displayName: String,
  @required
  lastChange: Timestamp,
}

structure UpdatePlayerResponse {
  @required
  success: Boolean,
  message: String,
}

operation GetGame {
  input: GetGameRequest,
  output: GetGameResponse,
}

structure GetGameRequest {
  @required
  gameId: String,
}

structure GetGameResponse {
  @required
  success: Boolean,
  message: String,
  seed: U64,
  size: U64,
  lastActivity: Timestamp,
}

operation CreateGame {
  input: CreateGameRequest,
  output: CreateGameResponse,
}

structure CreateGameRequest {
  @required
  gameId: String,
  @required
  seed: U64,
  @required
  size: U64,
}

structure CreateGameResponse {
  @required
  success: Boolean,
  message: String,
}

operation GetLeaderboard {
  input: GetLeaderboardRequest,
  output: GetLeaderboardResponse,
}

structure GetLeaderboardRequest {
  @required
  gameId: String,
  @required
  playerId: String,
}

structure GetLeaderboardResponse {
  @required
  success: Boolean,
  message: String,
  leaderboard: Leaderboard,
}

list Leaderboard {
  member: LeaderboardEntry
}

structure LeaderboardEntry {
  @required
  displayName: String,
  score: U64,
  requestingPlayer: Boolean,
}
