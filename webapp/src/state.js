import Dexie from "dexie";
import { v4 as uuidv4 } from 'uuid';
import { Game, GameExchange } from '../../engine/pkg/engine';

const SyncState = {
    NEW: 1,
    DIRTY: 2,
    CLEAN: 3,
};

const Modes = {
    PLAY: 1,
    SWITCH: 2,
    LEADERBOARD: 3,
    SHARE: 4,
};

const STATUS_CODES = {
    OK: 200,
    NOT_FOUND: 404,
};

const BASE_URL = window.location.origin; 
const API_URL = `${BASE_URL}/api/`;

let hidden, visibilityChange;
if (typeof document.hidden !== "undefined") { // Opera 12.10 and Firefox 18 and later support
  hidden = "hidden";
  visibilityChange = "visibilitychange";
} else if (typeof document.msHidden !== "undefined") {
  hidden = "msHidden";
  visibilityChange = "msvisibilitychange";
} else if (typeof document.webkitHidden !== "undefined") {
  hidden = "webkitHidden";
  visibilityChange = "webkitvisibilitychange";
}

function get_gx_js_object(game) {
    const gx = GameExchange.from_game(game);
    const gxjs = JSON.parse(gx.to_json());
    gx.free();
    return gxjs;
}

function gxjson_to_game(gxjson) {
    const gx = GameExchange.from_json(gxjson);
    if (gx != undefined) {
        const game = Game.from_exchange(gx);
        gx.free();
        if (game != undefined) {
            return game;
        }
    }
    return undefined;
}

function gxjs_to_game(gxjs) {
    if (gxjs != undefined) {
        const gxjson = JSON.stringify(gxjs);
        return gxjson_to_game(gxjson);
    }
    return undefined;
}

class State {
    constructor() {
        this.db = new Dexie("db");
        this.db.version(1).stores({
            player: "idx",
            games: "id, lastChange, syncState",
        });
        this.sync_in_flight = false;
        this.sync_in_queue = false;
        this.mode = Modes.PLAY;
        this.playerObservers = [];
        this.leaderboardObservers = [];
        this.gameObservers = [];
    }

    async initialize(gameId) {
        // ensure that player is created
        const player = await this.get_player();
        let game = undefined;
        if (player.startupGameId) {
            await this.db.player.update(1, { startupGameId: null });
            game = await this.newGameFromRemote(player.startupGameId);
        }
        if (game == undefined) {
            const gxjs = await this.db.games.orderBy("lastChange").reverse().first();
            game = gxjs_to_game(gxjs);
        }
        if (game == undefined) {
            await this.new_game();
        } else {
            this.current_game = game;
        }
        this.leaderboard = [];
        this.start_sync();
        setInterval(() => { this.refreshLeaderboard() }, 5000);
    }

    get_mode() {
        return this.mode;
    }

    set_mode(mode) {
        this.mode = mode;
    }

    add_player_observer(callback) {
        this.playerObservers.push(callback);
    }

    notify_player_observers() {
        for (const observer of this.playerObservers) {
            observer();
        }
    }

    add_leaderboard_observer(callback) {
        this.leaderboardObservers.push(callback);
    }

    notify_leaderboard_observers() {
        for (const observer of this.leaderboardObservers) {
            observer();
        }
    }

    add_game_observer(callback) {
        this.gameObservers.push(callback);
    }

    notify_game_observers() {
        for (const observer of this.gameObservers) {
            observer();
        }
    }

    async update_player_from_remote(name, lastChange) {
        const playerEntry = await this.get_player();
        if (lastChange > new Date(playerEntry.lastChange)) {
            await this.db.player.update(1, { name, syncState: SyncState.CLEAN, lastChange });
            this.notify_player_observers();
        } else {
            await this.db.player.update(1, { syncState: SyncState.CLEAN });
        }
    }

    async sync_player() {
        const playerEntry = await this.get_player();

        const [remotePlayerState, remotePlayerEntry] = await (async function () {
            const response = await fetch(API_URL + 'players/' + playerEntry.id);
            if (response.status == STATUS_CODES.OK) {
                return [response.status, await response.json()];
            } else {
                return [response.status, {}];
            }
        })();

        if ((remotePlayerState == STATUS_CODES.OK && Date.parse(remotePlayerEntry.lastChange) <= new Date(playerEntry.lastChange))
            || remotePlayerState == STATUS_CODES.NOT_FOUND) {
            const body = JSON.stringify({
                displayName: playerEntry.name,
                lastChange: (new Date(playerEntry.lastChange)).toISOString(),
            });
            const response = await fetch(API_URL + 'players/' + playerEntry.id, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json'
                },
                body
            });
            if (response.status == STATUS_CODES.OK) {
                const resp = await response.json();

                await this.update_player_from_remote(resp.displayName, Date.parse(resp.lastChange));
            }
        } else if (remotePlayerState == STATUS_CODES.OK) {
            await this.update_player_from_remote(remotePlayerEntry.displayName, Date.parse(remotePlayerEntry.lastChange));
        }

    }

    async push_new_games() {
        const playerEntry = await this.get_player();
        const gxjss = await this.db.games.where("syncState").equals(SyncState.NEW).toArray();
        for (const gxjs of gxjss) {
            const body = JSON.stringify(gxjs);
            const response = await fetch(API_URL + 'players/' + playerEntry.id + "/games/" + gxjs.id, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json'
                },
                body
            });
            if (response.status == STATUS_CODES.OK) {
                // game successfully pushed, update DB status to clean
                await this.db.games.update(gxjs.id, { syncState: SyncState.CLEAN });
            }
        }
    }

    async getRemoteGameIds() {
        const playerEntry = await this.get_player();
        const response = await fetch(API_URL + 'players/' + playerEntry.id + "/games");
        if (response.status == STATUS_CODES.OK) {
            const games = await response.json();
            return games.ids;
        }
        return undefined;
    }

    async removeStaleGames(remoteIds) {
        if (remoteIds != undefined) {
            const currentId = this.current_game.get_id();
            const localIds = (await this.db.games.toArray()).map((entry) => entry.id);
            const staleIds = localIds.filter(id => !(remoteIds.includes(id) || id == currentId));
            this.db.games.bulkDelete(staleIds);
        }
    }

    async getNewRemoteGames(remoteIds) {
        if (remoteIds != undefined) {
            const playerEntry = await this.get_player();
            const currentId = this.current_game.get_id();
            const localIds = (await this.db.games.toArray()).map((entry) => entry.id);
            const newIds = remoteIds.filter(id => !localIds.includes(id));
            for (newId of newIds) {
                const response = await fetch(API_URL + 'players/' + playerEntry.id + "/games/" + newId);
                if (response.status == STATUS_CODES.OK) {
                    const gxjs = response.json();
                    gxjs.syncState = SyncState.CLEAN;
                    await this.db.games.put(gxjs);
                }
            }
        }
    }

    async replaceGameWithRemote(remoteGxjs) {
        await this.db.games.update(remoteGxjs.id, { moves: remoteGxjs.moves, score: remoteGxjs.score, seed: remoteGxjs.seed, size: remoteGxjs.size, syncState: SyncState.CLEAN });
        const currentId = this.current_game.get_id();
        if (remoteGxjs.id == currentId) {
            const newGame = gxjs_to_game(remoteGxjs);
            this.switch_game(newGame);
            this.notify_game_observers();
        }
    }

    async refreshGames() {
        const localGxjss = await this.db.games.toArray();
        const playerEntry = await this.get_player();
        for (const localGxjs of localGxjss) {
            const response = await fetch(API_URL + 'players/' + playerEntry.id + "/games/" + localGxjs.id);
            if (response.status == STATUS_CODES.OK) {
                const remoteGxjs = await response.json();
                const remoteGame = gxjs_to_game(remoteGxjs);
                if (remoteGame == undefined) {
                    continue;
                }
                const localGame = gxjs_to_game(localGxjs);
                if (localGame == undefined) {
                    await this.replaceGameWithRemote(remoteGxjs);
                    remoteGame.free();
                    continue;
                }
                if (remoteGame.is_ancestor(localGame)) {
                    remoteGame.free();
                    localGame.free();
                    if (remoteGxjs.moves.length < localGxjs.moves.length) {
                        const body = JSON.stringify(localGxjs);
                        const response = await fetch(API_URL + 'players/' + playerEntry.id + "/games/" + localGxjs.id, {
                            method: 'PUT',
                            headers: {
                                'Content-Type': 'application/json'
                            },
                            body
                        });
                        if (response.status == STATUS_CODES.OK) {
                            // game successfully updated, update DB status to clean
                            await this.db.games.update(localGxjs.id, { syncState: SyncState.CLEAN });
                        }
                    }
                } else {
                    remoteGame.free();
                    localGame.free();
                    await this.replaceGameWithRemote(remoteGxjs);
                }
            }
        }
    }

    async fetchLeaderboard() {
        const playerEntry = await this.get_player();
        const gameId = this.current_game.get_id();
        const response = await fetch(API_URL + 'players/' + playerEntry.id + '/games/' + gameId + '/leaderboard');
        if (response.status == STATUS_CODES.OK) {
            this.leaderboard  = await response.json();
            this.notify_leaderboard_observers();
        }
    }

    async refreshLeaderboard() {
        if (!document[hidden] && !this.sync_in_flight) {
            await this.fetchLeaderboard();
        }
    }

    async storeStartupGameId(gameId) {
        await this.db.player.update(1, { startupGameId: gameId });
    }

    async newGameFromRemote(gameId) {
        const response = await fetch(API_URL + 'games/' + gameId);
        if (response.status == STATUS_CODES.OK) {
            const gameParams  = await response.json();
            const game = Game.new_from_seed(gameParams.size, gameParams.seed, gameId);
            if (game != undefined) {
                await this.store_game(game);
            }
            return game;
        }
        return undefined;
    }

    async sync_games() {
        const remoteGameIds = await this.getRemoteGameIds();
        await this.push_new_games(remoteGameIds);
        await this.getNewRemoteGames(remoteGameIds);
        await this.removeStaleGames();
        await this.refreshGames();
        await this.fetchLeaderboard();
    }

    async perform_sync() {
        try {
            if (this.sync_in_flight) {
                this.sync_in_queue = true;
                return;
            }
            this.sync_in_flight = true;

            await this.sync_player();
            await this.sync_games();
        } catch (error) {
            console.log(error);
        }
        this.sync_in_flight = false;
        if (this.sync_in_queue) {
            this.sync_in_queue = false;
            this.perform_sync();
        }
    }

    async start_sync() {
        clearTimeout(this.syncTimeout);
        this.syncTimeout = setTimeout(() => { this.perform_sync() }, 3000);
    }

    async get_player() {
        let entry = await this.db.player.where("idx").equals(1).first();
        if (entry == undefined) {
            entry = {
                idx: 1,
                id: uuidv4(),
                name: "",
                syncState: SyncState.NEW,
                lastChange: Date.now(),
            }
            this.db.player.put(entry);
            this.db.games.clear();
            this.start_sync();
        }
        return entry;
    }

    async get_player_name() {
        return (await this.get_player()).name;
    }

    get_leaderboard() {
        return this.leaderboard;
    }

    async set_player_name(name) {
        // ensure that there is a record to modify
        let entry = await this.get_player();
        if (entry.syncState == SyncState.CLEAN) {
            entry.syncState = SyncState.DIRTY;
        }
        entry.lastChange = Date.now();
        await this.db.player.update(1, { name, syncState: entry.syncState, lastChange: entry.lastChange });
        this.start_sync();
    }

    async store_game(game) {
        const gxjs = get_gx_js_object(game);
        gxjs.lastChange = Date.now();
        const old = await this.db.games.where("id").equals(gxjs.id).first();
        if (old == undefined || old.syncState == SyncState.NEW) {
            gxjs.syncState = SyncState.NEW;
        } else {
            gxjs.syncState = SyncState.DIRTY;
        }
        await this.db.games.put(gxjs);
        this.start_sync();
    }

    async switch_game(game) {
        if (game != undefined) {
            if (this.current_game != undefined) {
                this.current_game.free();
            }
            this.current_game = game;
        }
        return this.current_game;
    }

    async new_game() {
        const game = Game.new(4);
        await this.store_game(game)
        return await this.switch_game(game);
    }

    get_current_game() {
        return this.current_game;
    }

    async get_all_games() {
        const gxjss = await this.db.games.orderBy("lastChange").reverse().toArray();
        return gxjss.map((gxjs) => {
            return {
                id: gxjs.id,
                game: gxjs_to_game(gxjs),
            }
        });
    }

    async load_game(id) {
        const game = gxjs_to_game(await this.db.games.where("id").equals(id).first());
        return this.switch_game(game);
    }

    async make_move(direction) {
        const newGame = this.current_game.make_move(direction);
        if (newGame != undefined) {
            this.current_game.free();
            this.current_game = newGame;
            await this.store_game(newGame);
        }
        return newGame;
    }
}

export default State;
export { Modes }