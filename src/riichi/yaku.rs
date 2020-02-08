use crate::riichi::hand::Hand;
use crate::riichi::shape_finder::ShapeFinder;
use crate::riichi::shapes::{Shape, ShapeType, CompleteShape};
use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;

#[derive(IntoEnumIterator, Debug)]
pub enum YakuType {
    // 1 han closed
    MenzenTsumo,
    Riichi,
    Ippatsu,
    Pinfu,
    Iipeikou,
    // 1 han
    Haitei,
    Houtei,
    Rinshan,
    Chankan,
    Tanyao,
    EastRound,
    EastSeat,
    SouthRound,
    SouthSeat,
    WestRound,
    WestSeat,
    NorthSeat,
    WhiteDragons,
    GreenDragons,
    RedDragons,
    // 2 han
    DoubleRiichi,
    Chanta,
    SanshokuDoujun,
    Ittsu,
    Toitoi,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Chiitoitsu,
    Honroutou,
    Shousangen,
    // 3 han
    Honitsu,
    Junchan,
    Ryanpeikou,
    // 6 han
    Chinitsu,
    // Yakuman
    Kazoe,
    Kokushi,
    Suuankou,
    Daisangen,
    Shousuushii,
    Daisuushii,
    Tsuuiisou,
    Chinroutou,
    Ryuuiisou,
    Chuuren,
    Suukantsu,
    // special
    Tenhou,
    Chiihou
}

pub struct YakuFinder {

}

impl YakuFinder {
    pub fn new() -> YakuFinder {
        YakuFinder {
            ..Default::default()
        }
    }

    pub fn find(&self, hand: &mut Hand) {
        // only complete hands
        if hand.shanten() != -1 {
            return;
        }

        let mut sf = ShapeFinder::new();
        let variants = sf.find(hand);
        let mut variant_yaku: HashMap<usize, Vec<YakuType>> = HashMap::new();

        for (i, variant) in variants.iter().enumerate() {
            let mut yakus: Vec<YakuType> = vec!();
            for yaku_type in YakuType::into_enum_iter() {
                if yaku_type.is_in_hand(hand, variant) {
                    yakus.push(yaku_type);
                }
            }

            variant_yaku.insert(i, yakus);
        }

        println!("{:#?}", variant_yaku);
    }
}

impl Default for YakuFinder {
    fn default() -> YakuFinder {
        YakuFinder {

        }
    }
}

////////////////

impl YakuType {
    fn get_name(&self) -> &str {
        match self {
            YakuType::MenzenTsumo =>    "Menzen tsumo",
            YakuType::Riichi =>         "Riichi",
            YakuType::Ippatsu =>        "Ippatsu",
            YakuType::Pinfu =>          "Pinfu",
            YakuType::Iipeikou =>       "Iipeikou",
            YakuType::Haitei =>         "Haitei raoyue",
            YakuType::Houtei =>         "Houtei raoyui",
            YakuType::Rinshan =>        "Rinshan kaihou",
            YakuType::Chankan =>        "Chankan",
            YakuType::Tanyao =>         "Tanyao",
            YakuType::EastRound =>      "East round winds",
            YakuType::EastSeat =>       "East seat winds",
            YakuType::SouthRound =>     "South round winds",
            YakuType::SouthSeat =>      "South seat winds",
            YakuType::WestRound =>      "West round winds",
            YakuType::WestSeat =>       "West seat winds",
            YakuType::NorthSeat =>      "North seat winds",
            YakuType::WhiteDragons =>   "White dragons",
            YakuType::GreenDragons =>   "Green dragons",
            YakuType::RedDragons =>     "Red dragons",
            YakuType::DoubleRiichi =>   "Double riichi",
            YakuType::Chanta =>         "Chantaiyao",
            YakuType::SanshokuDoujun => "Sanshoku doujun",
            YakuType::Ittsu =>          "Ittsu",
            YakuType::Toitoi =>         "Toitoi",
            YakuType::Sanankou =>       "Sanankou",
            YakuType::SanshokuDoukou => "Sanshoku doukou",
            YakuType::Sankantsu =>      "Sankantsu",
            YakuType::Chiitoitsu =>     "Chiitoitsu",
            YakuType::Honroutou =>      "Honroutou",
            YakuType::Shousangen =>     "Shousangen",
            YakuType::Honitsu =>        "Honitsu",
            YakuType::Junchan =>        "Junchan taiyao",
            YakuType::Ryanpeikou =>     "Ryanpeikou",
            YakuType::Chinitsu =>       "Chinitsu",
            YakuType::Kazoe =>          "Kazoe yakuman",
            YakuType::Kokushi =>        "Kokushi musou",
            YakuType::Suuankou =>       "Suuankou",
            YakuType::Daisangen =>      "Daisangen",
            YakuType::Shousuushii =>    "Shousuushii",
            YakuType::Daisuushii =>     "Daisuushii",
            YakuType::Tsuuiisou =>      "Tsuuiisou",
            YakuType::Chinroutou =>     "Chinroutou",
            YakuType::Ryuuiisou =>      "Ryuuiisou",
            YakuType::Chuuren =>        "Chuuren poutou",
            YakuType::Suukantsu =>      "Suukantsu",
            YakuType::Tenhou =>         "Tenhou",
            YakuType::Chiihou =>        "Chiihou"
        }
    }

    fn get_han(&self) -> u8 {
        match self {
            YakuType::MenzenTsumo =>    1,
            YakuType::Riichi =>         1,
            YakuType::Ippatsu =>        1,
            YakuType::Pinfu =>          1,
            YakuType::Iipeikou =>       1,
            YakuType::Haitei =>         1,
            YakuType::Houtei =>         1,
            YakuType::Rinshan =>        1,
            YakuType::Chankan =>        1,
            YakuType::Tanyao =>         1,
            YakuType::EastRound =>      1,
            YakuType::EastSeat =>       1,
            YakuType::SouthRound =>     1,
            YakuType::SouthSeat =>      1,
            YakuType::WestRound =>      1,
            YakuType::WestSeat =>       1,
            YakuType::NorthSeat =>      1,
            YakuType::WhiteDragons =>   1,
            YakuType::GreenDragons =>   1,
            YakuType::RedDragons =>     1,
            YakuType::DoubleRiichi =>   2,
            YakuType::Chanta =>         {
                // TODO closed / open difference
                return 2;
            },
            YakuType::SanshokuDoujun => {
                // TODO closed / open difference
                return 2;
            },
            YakuType::Ittsu =>          {
                // TODO closed / open difference
                return 2;
            },
            YakuType::Toitoi =>         2,
            YakuType::Sanankou =>       2,
            YakuType::SanshokuDoukou => 2,
            YakuType::Sankantsu =>      2,
            YakuType::Chiitoitsu =>     2,
            YakuType::Honroutou =>      2,
            YakuType::Shousangen =>     2,
            YakuType::Honitsu =>        {
                // TODO closed / open difference
                return 3;
            },
            YakuType::Junchan =>        {
                // TODO closed / open difference
                return 3;
            },
            YakuType::Ryanpeikou =>     {
                // TODO closed / open difference
                return 3;
            },
            YakuType::Chinitsu =>       {
                // TODO closed / open difference
                return 6;
            },
            YakuType::Kazoe =>          13,
            YakuType::Kokushi =>        13,
            YakuType::Suuankou =>       13,
            YakuType::Daisangen =>      13,
            YakuType::Shousuushii =>    13,
            YakuType::Daisuushii =>     13,
            YakuType::Tsuuiisou =>      13,
            YakuType::Chinroutou =>     13,
            YakuType::Ryuuiisou =>      13,
            YakuType::Chuuren =>        13,
            YakuType::Suukantsu =>      13,
            YakuType::Tenhou =>         13,
            YakuType::Chiihou =>        13
        }
    }

    /// 2x the same shuntsu shape
    fn is_in_hand(&self, hand: &mut Hand, variant: &Vec<Shape>) -> bool {
        match self {
            YakuType::MenzenTsumo => {},
            YakuType::Riichi => {},
            YakuType::Ippatsu => {},
            YakuType::Pinfu => {},
            YakuType::Iipeikou => {},
            YakuType::Haitei => {},
            YakuType::Houtei => {},
            YakuType::Rinshan => {},
            YakuType::Chankan => {},
            YakuType::Tanyao => {
                let array_34 = hand.get_34_array();
                // can't contain any terminals or honors
                for (i, count) in array_34.iter().enumerate() {
                    if ([1, 9, 10, 18, 19, 27].contains(&(i + 1)) || (i + 1) >= 28) && *count > 0 {
                        return false;
                    }
                }

                return true;
            },
            YakuType::EastRound => {
                // TODO check round wind
            }
            YakuType::EastSeat => {
                // TODO check my seat wind
            }
            YakuType::SouthRound => {}
            YakuType::SouthSeat => {}
            YakuType::WestRound => {}
            YakuType::WestSeat => {}
            YakuType::NorthSeat => {}
            YakuType::WhiteDragons => return self.find_yakuhai(variant, 33),
            YakuType::GreenDragons => return self.find_yakuhai(variant, 32),
            YakuType::RedDragons => return self.find_yakuhai(variant, 34),
            YakuType::DoubleRiichi => {},
            YakuType::Chanta => {},
            YakuType::SanshokuDoujun => {},
            YakuType::Ittsu => {},
            YakuType::Toitoi => {},
            YakuType::Sanankou => {},
            YakuType::SanshokuDoukou => {},
            YakuType::Sankantsu => {},
            YakuType::Chiitoitsu => {
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Shuntsu(_) => return false,
                                CompleteShape::Koutsu(_) => return false,
                                CompleteShape::Toitsu(_) => (),
                                CompleteShape::Single(_) => return false,
                            }
                        },
                        ShapeType::Incomplete(_) => return false,
                    }
                }

                return true;
            },
            YakuType::Honroutou => {},
            YakuType::Shousangen => {},
            YakuType::Honitsu => {},
            YakuType::Junchan => {},
            YakuType::Ryanpeikou => {},
            YakuType::Chinitsu => {},
            YakuType::Kazoe => {},
            YakuType::Kokushi => {},
            YakuType::Suuankou => {},
            YakuType::Daisangen => {},
            YakuType::Shousuushii => {},
            YakuType::Daisuushii => {},
            YakuType::Tsuuiisou => {},
            YakuType::Chinroutou => {},
            YakuType::Ryuuiisou => {},
            YakuType::Chuuren => {},
            YakuType::Suukantsu => {},
            YakuType::Tenhou => {},
            YakuType::Chiihou => {}
        }

        false
    }

    fn find_yakuhai(&self, variant: &Vec<Shape>, tile_id: u8) -> bool {
        for shape in variant.iter() {
            match shape.get_shape_type() {
                ShapeType::Complete(cs) => {
                    match cs {
                        CompleteShape::Shuntsu(_) => {},
                        CompleteShape::Koutsu(tiles) => {
                            if tiles[0].to_id() == tile_id {
                                return true;
                            }
                        },
                        CompleteShape::Toitsu(_) => {},
                        CompleteShape::Single(_) => {},
                    }
                },
                ShapeType::Incomplete(_) => (),
            }
        }

        return false;
    }
}

mod tests {
    use super::*;

    #[test]
    fn find_tanyao() {
        let mut hand = Hand::from_text("234567m234567s88p", false).unwrap();
        hand.yaku();
    }

    #[test]
    fn find_tanyao_chiitoi() {
        let mut hand = Hand::from_text("224466m4477s3388p", false).unwrap();
        hand.yaku();
    }

    #[test]
    fn find_white_dragons() {
        let mut hand = Hand::from_text("123m234s67888p666z", false).unwrap();
        hand.yaku();
    }
}
