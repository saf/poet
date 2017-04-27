//! Module definining the Phone structure with phone features.

pub enum VowelFrontness {
    Front,
    Mid,
    Back,
}

pub enum VowelHeight {
    High,
    Center,
    Low,
}

pub enum VowelRoundness {
    Rounded,
    Unrounded,
}

pub enum VowelNasality {
    Oral,
    AsynchronousNasal,
    SynchronousNasal,
}

pub enum ConsonantManner {
    Stop,
    Approximant,
    Fricative,
    Affricate,
    Trill,
    Nasal,
    Lateral,
}

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
    Uvular,
    Pharyngeal,
    Glottal,
}

pub enum ConsonantPhonation {
    Voiced,
    Unvoiced,
}

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

pub struct Phone {
    pub ipa: &'static str,
    pub name: &'static str,
    pub features: PhoneFeatures,
}

macro_rules! vowel {
    ($name:expr, $ipa:expr, $front:expr, $height:expr, $round:expr, $nasal:expr) => (
        Phone { ipa: $ipa, name: $name, features: PhoneFeatures::Vowel {
            frontness: $front,
            height: $height,
            roundness: $round,
            nasality: $nasal,
        } }
    );
};

