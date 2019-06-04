use std::fmt;

use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;

#[derive(Debug)]
pub struct Hand {
    // a hand consists of 13 tiles + 1 drawn tile
    // it can also have kan, which are groups of 4 tiles that behave as 3 tiles
    // so we should have a vector with 13 100% present tiles and 5 optional (4 from possible kans and 1 possible draw)
    tiles: Vec<Option<Tile>>
}

impl Hand {
    /// TODO
    pub fn random_hand(count: u8) -> Hand {
        if count < 13 || count > 14 {
            panic!("Only 13 or 14 tile hands allowed");
        } else {
            Hand {
                tiles: vec!(Option::Some(Tile::new(TileType::Number(1, TileColor::Manzu))))
            }
        }
    }

    pub fn from_text(representation: &str) -> Hand {
        if representation.len() % 2 != 0 {
            panic!("String representation of a hand must be even length");
        }

        let mut tiles : Vec<Option<Tile>> = Vec::with_capacity(representation.len());
        let mut iter = representation.chars();
        let mut pos = 0;
        let mut len;

        while pos < representation.len() {
            len = 0;
            for ch in iter.by_ref().take(2) {
                len += ch.len_utf8();
            }
            let tile_string = &representation[pos..pos + len];

            let tile = Tile::from_text(tile_string);

            tiles.push(Option::Some(tile));
            pos += len;
        }

        tiles.sort();

        if tiles.len() >= 13 {
            return Hand {
                tiles: tiles
            }
        }

        panic!("Couldn't parse hand representation.");
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for tile in self.tiles.iter() {
            match &tile {
                Option::Some(some_tile) => out.push_str(&some_tile.to_string()[..]),
                Option::None => ()
            }
        }
        write!(f, "{}", out)
    }
}