#[cfg(feature = "random")]
use nanorand::{Rng, WyRand};
use oorandom::Rand64;
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::TryFrom;
#[cfg(feature = "random")]
use uuid::Uuid;
#[cfg(feature = "bindgen")]
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tile {
    pub id: usize,
    pub value: usize,
    pub merged_with: Option<usize>,
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
#[derive(Clone, Serialize, Deserialize)]
pub struct GameExchange {
    player: String,
    id: String,
    score: usize,
    seed: String,
    size: usize,
    moves: Vec<Direction>,
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    id: String,
    score: usize,
    game_over: bool,
    seed: u64,
    rng: Rand64,
    size: usize,
    next_tile_id: usize,
    tiles: Vec<Option<Tile>>,
    moves: Vec<Direction>,
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Clone)]
struct Cursor {
    row: usize,
    col: usize,
    size: usize,
    prev_along_f: fn(&Self) -> Option<Self>,
    next_across_f: fn(&Self) -> Option<Self>,
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
pub fn rng_test(seed: u64) -> bool {
    let mut rng = Rand64::new(seed as u128);
    let samples = (0..20)
        .into_iter()
        .map(|_| rng.rand_u64())
        .collect::<Vec<_>>();

    println!("{} {:?}", seed, samples);
    samples
        == [
            824868410376368605,
            15477559782269922615,
            5081633107576607391,
            8856448160376592522,
            5905815615133831151,
            13140293514862084767,
            7257284078658714687,
            6159118809728095634,
            4376305407353685941,
            7130165131050712251,
            5693131055793436498,
            18126237701427793392,
            7377923697696518388,
            11738103520457482240,
            7314144700598124327,
            4353691544780060151,
            16597648435527420477,
            5658763766874107292,
            14175692770654253831,
            14745658191250406569,
        ]
}

impl Cursor {
    fn row(&self) -> usize {
        self.row
    }

    fn col(&self) -> usize {
        self.col
    }

    fn incr_row(&self) -> Option<Self> {
        if self.row < self.size - 1 {
            Some(Cursor {
                row: self.row + 1,
                ..*self
            })
        } else {
            None
        }
    }

    fn decr_row(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Cursor {
                row: self.row - 1,
                ..*self
            })
        } else {
            None
        }
    }

    fn incr_col(&self) -> Option<Self> {
        if self.col < self.size - 1 {
            Some(Cursor {
                col: self.col + 1,
                ..*self
            })
        } else {
            None
        }
    }

    fn decr_col(&self) -> Option<Self> {
        if self.col > 0 {
            Some(Cursor {
                col: self.col - 1,
                ..*self
            })
        } else {
            None
        }
    }

    pub fn new(size: usize, d: Direction) -> Self {
        match d {
            Direction::Right => Cursor {
                row: 0,
                col: size - 1,
                size,
                prev_along_f: Cursor::decr_col,
                next_across_f: Cursor::incr_row,
            },
            Direction::Up => Cursor {
                row: 0,
                col: 0,
                size,
                prev_along_f: Cursor::incr_row,
                next_across_f: Cursor::incr_col,
            },
            Direction::Left => Cursor {
                row: 0,
                col: 0,
                size,
                prev_along_f: Cursor::incr_col,
                next_across_f: Cursor::incr_row,
            },
            Direction::Down => Cursor {
                row: size - 1,
                col: 0,
                size,
                prev_along_f: Cursor::decr_row,
                next_across_f: Cursor::incr_col,
            },
        }
    }

    pub fn prev_along(&self) -> Option<Self> {
        (self.prev_along_f)(self)
    }

    pub fn next_across(&self) -> Option<Self> {
        (self.next_across_f)(self)
    }
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
impl GameExchange {

    pub fn new(player: String, id: String, score: usize, seed: String, size: usize, moves_str: &str) -> Result<GameExchange, String> {
        let moves = match serde_json::from_str(moves_str) {
            Ok(m) => Ok(m),
            Err(_) => Err("Error parsing moves".to_owned()),
        }?;

        Ok(Self{
            player,
            id,
            score,
            seed,
            size,
            moves
        })
    }
        
    pub fn from_json(json: String) -> Option<GameExchange> {
        let parsed: serde_json::Result<GameExchange> = serde_json::from_str(&json);
        match parsed {
            Ok(game) => Some(game),
            _ => None,
        }
    }

    pub fn to_game(&self) -> Option<Game> {
        match Game::try_from(self) {
            Ok(game) => Some(game),
            _ => None,
        }
    }

    pub fn from_game(g: &Game) -> GameExchange {
        g.into()
    }

    pub fn get_player(&self) -> String {
        self.player.clone()
    }

    pub fn set_player(&mut self, name: String) {
        self.player = name;
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_moves_str(&self) -> String {
        serde_json::to_string(&self.moves).unwrap()
    }

    pub fn to_json(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(json) => Some(json),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "bindgen", wasm_bindgen)]
impl Game {
    #[cfg(feature = "random")]
    pub fn new(size: usize) -> Self {
        Self::new_from_seed(size, WyRand::new().generate(), &Uuid::new_v4().to_hyphenated().to_string())
    }

    pub fn new_from_seed(size: usize, seed: u64, id: &str) -> Self {
        let rng = Rand64::new(seed as u128);
        Game {
            id: id.to_owned(),
            score: 0,
            game_over: false,
            seed,
            rng,
            size,
            next_tile_id: 0,
            tiles: vec![None; size * size],
            moves: vec![],
        }
        .add_tile()
        .unwrap()
        .add_tile()
        .unwrap()
    }

    pub fn from_exchange(gx: &GameExchange) -> Option<Game> {
        match Game::try_from(gx) {
            Ok(game) => Some(game),
            _ => None,
        }
    }

    pub fn to_exchange(&self) -> GameExchange {
        GameExchange::from(self)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    pub fn debug(&self) -> String {
        format!("{:?}", self)
    }

    fn can_move(&self, d: Direction) -> bool {
        let mut across_cursor_option = Some(Cursor::new(self.size, d));

        // Outer loop over the across direction
        while let Some(across_cursor) = across_cursor_option {
            let along_cursor = across_cursor.clone();
            let mut prev_cursor_option = along_cursor.prev_along();

            let mut along_tile = self
                .get_tile(along_cursor.row(), along_cursor.col())
                .unwrap();

            // Inner loop over the along direction
            while let Some(prev_cursor) = prev_cursor_option {
                let prev_tile = self.get_tile(prev_cursor.row(), prev_cursor.col()).unwrap();

                if along_tile.value == prev_tile.value {
                    return true;
                }

                along_tile = prev_tile;
                prev_cursor_option = prev_cursor.prev_along();
            }

            across_cursor_option = across_cursor.next_across();
        }
        false
    }

    fn update_game_over(&mut self) {
        if self.tiles.iter().all(|t| t.is_some())
            && !self.can_move(Direction::Down)
            && !self.can_move(Direction::Right)
        {
            self.game_over = true;
        }
    }

    fn add_tile(&self) -> Option<Self> {
        if self.game_over {
            return None;
        }
        let mut rng = self.rng.clone();
        let empty_indices = self
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| if tile.is_none() { Some(i) } else { None })
            .collect::<Vec<_>>();
        if empty_indices.len() == 0 {
            return None;
        }
        let index = empty_indices[rng.rand_range(0..empty_indices.len() as u64) as usize];
        let value = if rng.rand_range(0..9) == 0 { 4 } else { 2 };
        let mut tiles = self.tiles.clone();
        tiles[index] = Some(Tile {
            id: self.next_tile_id,
            value,
            merged_with: None,
        });
        let mut rv = Game {
            id: self.id.clone(),
            rng,
            next_tile_id: self.next_tile_id + 1,
            tiles,
            moves: self.moves.clone(),
            ..*self
        };
        rv.update_game_over();
        Some(rv)
    }

    pub fn get_tile(&self, row: usize, col: usize) -> Option<Tile> {
        self.tiles[row * self.size + col].clone()
    }

    fn set_tile(&mut self, row: usize, col: usize, tile: Option<Tile>) {
        self.tiles[row * self.size + col] = tile;
    }

    fn slide(&self, d: Direction) -> Option<Self> {
        if self.game_over {
            return None;
        }
        let mut changed = false;
        let mut rv = self.clone();
        let mut across_cursor_option = Some(Cursor::new(self.size, d.clone()));

        for t in &mut rv.tiles {
            if let Some(ref mut tile) = t {
                tile.merged_with = None;
            }
        }
        // Outer loop over the across direction
        while let Some(across_cursor) = across_cursor_option {
            let mut dst_cursor = across_cursor.clone();
            let mut src_cursor_option = dst_cursor.prev_along();

            // Inner loop over the along direction
            while let Some(ref src_cursor) = src_cursor_option {
                let src_row = src_cursor.row();
                let src_col = src_cursor.col();
                match rv.get_tile(src_row, src_col) {
                    Some(src_tile) => {
                        // The source tile contains a tile
                        let dst_row = dst_cursor.row();
                        let dst_col = dst_cursor.col();

                        if src_row == dst_row && src_col == dst_col {
                            // Source and destination are the same - step the source
                            src_cursor_option = src_cursor.prev_along();
                        } else {
                            match rv.get_tile(dst_row, dst_col) {
                                Some(dst_tile) => {
                                    // The destination contains a tile
                                    if src_tile.value == dst_tile.value {
                                        // Merge tiles of equal value
                                        let new_value = src_tile.value + dst_tile.value;
                                        rv.set_tile(
                                            dst_row,
                                            dst_col,
                                            Some(Tile {
                                                id: src_tile.id,
                                                value: new_value,
                                                merged_with: Some(dst_tile.id),
                                            }),
                                        );
                                        rv.set_tile(src_row, src_col, None);
                                        rv.score += new_value;
                                        // Step the source
                                        src_cursor_option = src_cursor.prev_along();
                                        changed = true;
                                    }
                                    // Step the destination in any case, whether merged or not
                                    dst_cursor = dst_cursor.prev_along().unwrap();
                                }
                                None => {
                                    // No tile in destination - move the source tile
                                    rv.set_tile(dst_row, dst_col, Some(src_tile));
                                    rv.set_tile(src_row, src_col, None);
                                    // Step the source
                                    src_cursor_option = src_cursor.prev_along();
                                    changed = true;
                                }
                            }
                        }
                    }
                    None => {
                        // No tile in source - step the source
                        src_cursor_option = src_cursor.prev_along();
                    }
                }
            }

            across_cursor_option = across_cursor.next_across();
        }
        if changed {
            rv.moves.push(d);
            rv.update_game_over();
            Some(rv)
        } else {
            None
        }
    }

    pub fn make_move(&self, d: Direction) -> Option<Game> {
        let game = self.slide(d)?;
        let game = game.add_tile()?;
        Some(game)
    }

    pub fn is_ancestor(&self, other: &Game) -> bool {
        if self.id != other.id {
            return false;
        }
        if self.seed != other.seed {
            return false;
        }
        if self.size != other.size {
            return false;
        }
        if self.moves.len() > other.moves.len() {
            return false;
        }

        if !(self.moves == other.moves[..self.moves.len()]) {
            return false;
        }

        if self.moves.len() == other.moves.len() {
            return self == other;
        }

        let mut g = Some(self.clone());
        for d in other.moves[self.moves.len()..].iter() {
            g = g.unwrap().make_move(d.clone());
            if g.is_none() {
                return false;
            }
        }

        g.unwrap() == *other
    }
}

impl From<&Game> for GameExchange {
    fn from(g: &Game) -> Self {
        GameExchange {
            player: "".into(),
            id: g.id.clone(),
            score: g.score,
            seed: g.seed.to_string(),
            size: g.size,
            moves: g.moves.clone(),
        }
    }
}

impl TryFrom<&GameExchange> for Game {
    type Error = &'static str;

    fn try_from(gx: &GameExchange) -> Result<Self, Self::Error> {
        let seed = match gx.seed.parse() {
            Ok(s) => Ok(s),
            Err(_) => Err("Invalid seed"),
        }?;
        let mut g = Game::new_from_seed(gx.size, seed, &gx.id);
        for d in &gx.moves {
            match g.make_move(d.clone()) {
                Some(new_g) => g = new_g,
                None => return Err("Invalid move"),
            }
        }
        if g.score != gx.score {
            return Err("Invalid score");
        }
        Ok(g)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn slide_test() {
        let game = Game {
            id: "".to_owned(),
            score: 0,
            game_over: false,
            seed: 0,
            rng: Rand64::new(0),
            size: 4,
            next_tile_id: 0,
            moves: vec![],
            tiles: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Tile {
                    id: 0,
                    value: 2,
                    merged_with: None,
                }),
                None,
                Some(Tile {
                    id: 1,
                    value: 2,
                    merged_with: None,
                }),
            ]
            .to_vec(),
        };
        let game = game.slide(Direction::Left);
        assert!(
            game.unwrap().tiles
                == [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Tile {
                        id: 1,
                        value: 4,
                        merged_with: Some(0)
                    }),
                    None,
                    None,
                    None,
                ]
        );
    }

    #[test]
    fn is_ancestor_test() {
        let game1 = Game::new_from_seed(4, 0, "");
        let game2 = game1.make_move(Direction::Down).unwrap();
        assert!(game1.is_ancestor(&game2));
        assert!(!game2.is_ancestor(&game1));
    }

    #[test]
    fn exchange_test() {
        let game1 = Game::new_from_seed(4, 0, "")
            .make_move(Direction::Down)
            .unwrap();
        let json = game1.to_exchange().to_json().unwrap();
        println!("{}", json);
        //assert!(json == "");
        let game2 = GameExchange::from_json(json).unwrap().to_game().unwrap();
        assert!(game1 == game2);
    }

    #[test]
    fn rng_test() {
        assert!(super::rng_test(u64::MAX))
    }
}
