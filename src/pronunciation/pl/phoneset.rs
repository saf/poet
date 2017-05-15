use pronunciation;
use pronunciation::phone;

use pronunciation::phone::VowelFrontness::*;
use pronunciation::phone::VowelHeight::*;
use pronunciation::phone::VowelRoundness::*;
use pronunciation::phone::VowelNasality::*;

use pronunciation::phone::ConsonantManner::*;
use pronunciation::phone::ConsonantPlace::*;
use pronunciation::phone::ConsonantPhonation::*;

#[derive(Debug, PartialEq)]
pub enum Phone {
    A, E, I, O, U, Y,

    P, B, T, D, K, G,
    
    F, V,
    S, Z, C, Dz,
    Sz, Zh, Cz, Dzh,
    Si, Zi, Ci, Dzi,
    H,
    
    M, N, Ni, Ng,
    R, L,
    J, W, Wx,
}

impl pronunciation::phone::Phone for Phone {
    fn ipa(&self) -> String {
        use self::Phone::*;
        match *self {
            A   => "ä".to_string(),
            E   => "ɛ".to_string(),
            O   => "ɔ".to_string(),
            Y   => "ɨ".to_string(),
            C   => "t͡s".to_string(),
            Dz  => "d͡z".to_string(),
            Sz  => "ʂ".to_string(),
            Zh  => "ʐ".to_string(),
            Cz  => "t͡ʂ".to_string(),
            Dzh => "d͡ʑ".to_string(),
            Si  => "ɕ".to_string(),
            Zi  => "ʑ".to_string(),
            Ci  => "t͡ɕ".to_string(),
            Dzi => "d͡ʑ".to_string(),
            H   => "x".to_string(),
            Ni  => "ɲ".to_string(),
            Ng  => "ŋ".to_string(),
            Wx  => "w̃".to_string(),
            _  => format!("{:?}", &self).to_lowercase()
        }
    }
    fn features(&self) -> phone::PhoneFeatures {
        use self::Phone::*;
        match *self {
            A   => vowel!(Center, Low,  Unrounded, Oral),
            E   => vowel!(Front,  Mid,  Unrounded, Oral),
            I   => vowel!(Front,  High, Unrounded, Oral),
            O   => vowel!(Back,   Mid,  Rounded,   Oral),
            U   => vowel!(Back,   High, Rounded,   Oral),
            Y   => vowel!(Center, High, Unrounded, Oral),
            
            P   => consonant!(Stop, Bilabial, Unvoiced),
            B   => consonant!(Stop, Bilabial, Voiced),
            T   => consonant!(Stop, Alveolar, Unvoiced),
            D   => consonant!(Stop, Alveolar, Voiced),
            K   => consonant!(Stop, Velar, Unvoiced),
            G   => consonant!(Stop, Velar, Voiced),
            F   => consonant!(Fricative, Labiodental, Unvoiced),
            V   => consonant!(Fricative, Labiodental, Voiced),
            S   => consonant!(Fricative, Alveolar, Unvoiced),
            Z   => consonant!(Fricative, Alveolar, Voiced),
            C   => consonant!(Affricate, Alveolar, Unvoiced),
            Dz  => consonant!(Affricate, Alveolar, Voiced),
            Sz  => consonant!(Fricative, Retroflex, Unvoiced),
            Zh  => consonant!(Fricative, Retroflex, Voiced),
            Cz  => consonant!(Affricate, Retroflex, Unvoiced),
            Dzh => consonant!(Affricate, Retroflex, Voiced),
            Si  => consonant!(Fricative, Alveolopalatal, Unvoiced),
            Zi  => consonant!(Fricative, Alveolopalatal, Voiced),
            Ci  => consonant!(Affricate, Alveolopalatal, Unvoiced),
            Dzi => consonant!(Affricate, Alveolopalatal, Voiced),
            H   => consonant!(Fricative, Velar, Unvoiced),
            M   => consonant!(Nasal, Bilabial, Voiced),
            N   => consonant!(Nasal, Dental, Voiced),
            Ni  => consonant!(Nasal, Palatal, Voiced),
            Ng  => consonant!(Nasal, Velar, Voiced),
            R   => consonant!(Trill, Alveolar, Voiced), 
            L   => consonant!(Lateral, Alveolar, Voiced), 
            J   => consonant!(Approximant, Palatal, Voiced), 
            W   => consonant!(Approximant, Labiovelar, Voiced),
            Wx  => consonant!(Nasal, Labiovelar, Voiced),
        }
    }
}
