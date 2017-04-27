use pronunciation::phone::*;

use pronunciation::phone::PhoneFeatures::*;

use pronunciation::phone::VowelFrontness::*;
use pronunciation::phone::VowelHeight::*;
use pronunciation::phone::VowelRoundness::*;
use pronunciation::phone::VowelNasality::*;

pub const PHONES: [Phone; 1] = [
    vowel!("a", "a", Front, Low, Unrounded, Oral),
];
