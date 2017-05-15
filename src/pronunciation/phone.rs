//! Module definining the Phone structure with phone features.

#![macro_use]

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum VowelFrontness {
    Front,
    Center,
    Back,
}

#[derive(Debug, PartialEq)]
pub enum VowelHeight {
    High,
    Mid,
    Low,
}

#[derive(Debug, PartialEq)]
pub enum VowelRoundness {
    Rounded,
    Unrounded,
}

#[derive(Debug, PartialEq)]
pub enum VowelNasality {
    Oral,
    AsynchronousNasal,
    SynchronousNasal,
}

#[derive(Debug, PartialEq)]
pub enum ConsonantManner {
    Stop,
    Approximant,
    Fricative,
    Affricate,
    Trill,
    Nasal,
    Lateral,
}

#[derive(Debug, PartialEq)]
pub enum ConsonantPlace {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Palatoalveolar,
    Retroflex,
    Alveolopalatal,
    Palatal,
    Velar,
    Labiovelar,
    Uvular,
    Pharyngeal,
    Glottal,
}

#[derive(Debug, PartialEq)]
pub enum ConsonantPhonation {
    Voiced,
    Unvoiced,
}

#[derive(Debug, PartialEq)]
pub enum PhoneFeatures {
    Vowel { frontness: VowelFrontness,
            height: VowelHeight,
            roundness: VowelRoundness,
            nasality: VowelNasality,
    },
    Consonant { manner: ConsonantManner,
                place: ConsonantPlace,
                phonation: ConsonantPhonation,
    },
}

pub trait Phone : Debug + PartialEq {
    fn ipa(&self) -> String {
        return format!("{:?}", &self).to_lowercase();
    }
    fn name(&self) -> String {
        return format!("{:?}", &self).to_lowercase();
    }
    fn features(&self) -> PhoneFeatures;
}

macro_rules! vowel {
    ($front:expr, $height:expr, $round:expr, $nasal:expr) => (
        pronunciation::phone::PhoneFeatures::Vowel {
            frontness: $front,
            height: $height,
            roundness: $round,
            nasality: $nasal,
        }
    );
}

macro_rules! consonant {
    ($manner:expr, $place:expr, $phonation:expr) => (
        pronunciation::phone::PhoneFeatures::Consonant {
            manner: $manner,
            place: $place,
            phonation: $phonation,
        }
    );
}

macro_rules! check_word_transcriptions {
    ($( $word:expr => [ $( $phone:ident )+ ]),+) => {
        $( assert_eq!(transcribe($word), Some(vec![$($phone),+])); )+
    };
}

macro_rules! tr {
    ($( $phone:expr )*) => (Some(vec![$($phone),*]))
}
