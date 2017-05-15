use super::phoneset;
use super::phoneset::Phone::*;

use pronunciation::phone::Phone;
use pronunciation::phone::PhoneFeatures::*;

use pronunciation::phone::ConsonantManner::*;
use pronunciation::phone::ConsonantPhonation::*;
use pronunciation::phone::ConsonantPlace::*;

fn modifies_nasals(p: &phoneset::Phone) -> bool {
    match p.features() {
        Consonant { manner: Stop, .. } => true,
        Consonant { manner: Affricate, .. } => true,
        _ => false,
    }
}

fn modifies_voicing(p: &phoneset::Phone) -> bool {
    match p.features() {
        Consonant { manner: Stop, .. } => true,
        Consonant { manner: Fricative, .. } => true,
        Consonant { manner: Affricate, .. } => true,
        _ => false,
    }
}

fn devoices(p: &phoneset::Phone) -> bool {
    modifies_voicing(p) && match p.features() {
        Consonant { phonation: Unvoiced, .. } => true,
        _ => false,
    }   
}

fn envoices(p: &phoneset::Phone) -> bool {
    modifies_voicing(p) && match p.features() {
        Consonant { phonation: Voiced, manner: ref m, .. } if *m != Nasal => true,
        _ => false,
    }   
}

fn transcribe_char(prev_chars: &[char],
                   chars: &[char],
                   next_phones: &[phoneset::Phone]) -> Option<Vec<phoneset::Phone>> {
    assert!(chars.len() > 0);
    let (this_char, next_chars) = chars.split_first().unwrap();
    let next_phone = next_phones.first();
    let devoiced = match next_phone { None => false, Some(ref p) => devoices(p) };
    let envoiced = match next_phone { None => false, Some(ref p) => envoices(p) };

    let no_char = '#';
    let prev_char = prev_chars.last().unwrap_or(&no_char);
    let next_char = next_chars.first().unwrap_or(&no_char);
    let nnext_char = next_chars.get(1).unwrap_or(&no_char);

    match *this_char {
        'a' => tr![A],
        'ą' =>
            match next_phone {
                None => tr![O Wx],
                Some(ref p) => match p.features() {
                    Consonant { place: Bilabial, .. }       if modifies_nasals(p) => tr![O M],
                    Consonant { place: Alveolar, .. }       if modifies_nasals(p) => tr![O N],
                    Consonant { place: Alveolopalatal, .. } if modifies_nasals(p) => tr![O N],
                    Consonant { place: Velar, .. }          if modifies_nasals(p) => tr![O Ng],
                    _ => tr![O Wx],
                }
            },
        'b' => if devoiced { tr![P] } else { tr![B] },
        'c' =>
            match *next_char {
                'z' => if envoiced { tr![Dzh] } else { tr![Cz] },
                'i' => tr![Ci],
                'h' => tr![H],
                _ => if envoiced { tr![Dz] } else { tr![C] },
            },
        'ć' => if envoiced { tr![Dzi] } else { tr![Ci] },
        'd' =>
            match *next_char {
                'z' =>
                    if *nnext_char == 'i' {
                        tr![Dzi]
                    } else {                      
                        if devoiced { tr![C] } else { tr![Dz] }
                    },
                'ź' => if devoiced { tr![Ci] } else { tr![Dzi] },
                'ż' => if devoiced { tr![Cz] } else { tr![Dzh] },
                _ => if devoiced { tr![T] } else { tr![D] },
            },
        'e' => tr![E],
        'ę' =>
            match next_phone {
                None => tr![E],
                Some(ref p) => match p.features() {
                    Consonant { place: Bilabial, .. }       if modifies_nasals(p) => tr![E M],
                    Consonant { place: Alveolar, .. }       if modifies_nasals(p) => tr![E N],
                    Consonant { place: Alveolopalatal, .. } if modifies_nasals(p) => tr![E N],
                    Consonant { place: Velar, .. }          if modifies_nasals(p) => tr![E Ng],
                    _ => tr![E Wx],
                }
            },
        'f' => if envoiced { tr![V] } else { tr![F] },
        'g' => if devoiced { tr![K] } else { tr![G] },
        'h' => if *prev_char == 'c' { tr![] } else { tr![H] },
        'i' =>
            if let Some(p) = next_phone {
                match p.features() {
                    Consonant { .. } => tr![I],
                    Vowel     { .. } =>
                        match *prev_char {
                            'c' | 's' | 'z' | 'n' => tr![],
                            _ => tr![J],
                        }
                }
            } else {
                tr![I]
            },
        'j' => tr![J],
        'k' => if envoiced { tr![G] } else { tr![K] },
        'l' => tr![L],
        'ł' => tr![W],
        'm' => tr![M],
        'n' =>
            if *next_char == 'i' {
                tr![Ni]
            } else {
                match next_phone {
                    None => tr![N],
                    Some(p) => match p.features() {
                        Consonant { place: Velar, .. } => tr![Ng],
                        _ => tr![N],
                    }
                }
            },
        'ń' => tr![Ni],
        'o' => tr![O],
        'ó' => tr![U],
        'p' => if envoiced { tr![B] } else { tr![P] },
        'r' =>
            if *next_char == 'z' {
                if devoiced { tr![Sz] } else { tr![Zh] }
            } else {
                tr![R]
            },
        's' =>
            match *next_char {
                'i' => if envoiced { tr![Zi] } else { tr![Si] },
                'z' => if envoiced { tr![Zh] } else { tr![Sz] },
                _   => if envoiced { tr![Z]  } else { tr![S]  },
            },
        'ś' => if envoiced { tr![Zi] } else { tr![Si] },
        't' => if envoiced { tr![D] } else { tr![T] },
        'u' => tr![U],
        'w' => if devoiced { tr![F] } else { tr![V] },
        'y' => tr![Y],
        'z' =>
            match *prev_char {
                'c' | 'd' | 'r' | 's' => tr![],
                _ => if devoiced { tr![S] } else { tr![Z] },
            },
        'ź' =>
            match *prev_char {
                'd' => tr![],
                _ => if devoiced { tr![Si] } else { tr![Zi] },
            },
        'ż' =>
            match *prev_char {
                'd' => tr![],
                _ => if devoiced { tr![Sz] } else { tr![Zh] },
            },
        _   => None
    }
}

pub fn transcribe(word: &str) -> Option<Vec<phoneset::Phone>> {
    let mut word_phones = vec![];
    let word_chars: Vec<char> = word.chars().collect();
    for (i, _) in word_chars.iter().enumerate().rev() {
        let (prev_chars, next_chars) = word_chars.split_at(i);
        if let Some(mut char_phones) = transcribe_char(prev_chars, next_chars, &word_phones) {
            char_phones.append(&mut word_phones);
            word_phones = char_phones;
        } else {
            return None;
        }

    }
    return Some(word_phones);
}

#[test]
fn test_envoicing() {
    assert!(envoices(&B));
    assert!(envoices(&Dzi));
    assert!(envoices(&V));
}

#[test]
fn test_letter_ax() {
    check_word_transcriptions!(
        "idą" => [I D O Wx],
        "dąb" => [D O M B],
        "bądź" => [B O N Dzi],
        "dąć" => [D O N Ci],
        "bąk" => [B O Ng K],
        "wąs" => [V O Wx S]
    );
}

#[test]
fn test_letter_b() {
    check_word_transcriptions!(
        "bar" => [B A R],
        "karb" => [K A R B],
        "babka" => [B A P K A]
    );
}

#[test]
fn test_letter_c() {
    check_word_transcriptions!(
        "cap" => [C A P],
        "pac" => [P A C],
        "paca" => [P A C A],
        "tancbuda" => [T A N Dz B U D A],
        
        "ciasto" => [Ci A S T O],
        "koci" => [K O Ci I],
        "kocia" => [K O Ci A],
        
        "czy" => [Cz Y],
        "bacz" => [B A Cz],
        "baczże" => [B A Dzh Zh E],
        
        "bach" => [B A H],
        "cham" => [H A M]
    );
}

#[test]
fn test_letter_ci() {
    check_word_transcriptions!(
        "myć" => [M Y Ci],
        "ćma" => [Ci M A],
        "ćwierćinteligent" => [Ci F I E R Ci I N T E L I G E N T]
    );
}

#[test]
fn test_letter_d() {
    check_word_transcriptions!(
        "dama" => [D A M A],
        "trud" => [T R U D],
        "kładka" => [K W A T K A],
        
        "widz" => [V I Dz],
        "dzwon" => [Dz V O N],
        "łódzki" => [W U C K I],
        
        "dziad" => [Dzi A D],
        "młodzi" => [M W O Dzi I],
        
        "łódź" => [W U Dzi],
        "dźwig" => [Dzi V I G],
        "pójdźka" => [P U J Ci K A],
        
        "dżem" => [Dzh E M],
        "dżdżownica" => [Dzh Dzh O V Ni I C A],
        "różdżka" => [R U Sz Cz K A]
    );
}

#[test]
fn test_letter_ex() {
    check_word_transcriptions!(
        "więcej" => [V J E N C E J],
        "dźwięk" => [Dzi V J E Ng K],
        "kurczę" => [K U R Cz E],
        "kęs" => [K E Wx S],
        "dębu" => [D E M B U]
    );
}

#[test]
fn test_letter_f() {
    check_word_transcriptions!(
        "fala" => [F A L A],
        "klif" => [K L I F],
        "afgan" => [A V G A N]
    );
}

#[test]
fn test_letter_g() {
    check_word_transcriptions!(
        "gęś" => [G E Wx Si],
        "magia" => [M A G J A],
        "dog" => [D O G],
        "angst" => [A Ng K S T],
        "gwiazda" => [G V J A Z D A]
            
    );
}

#[test]
fn test_letter_h() {
    check_word_transcriptions!(
        "habit" => [H A B I T],
        "druh" => [D R U H],
        "chata" => [H A T A]
    );
}

#[test]
fn test_letter_i() {
    check_word_transcriptions!(
        "ile"   => [I L E],
        "cisza" => [Ci I Sz A],
        "cierń" => [Ci E R Ni],
        "sień"  => [Si E Ni],
        "wieża" => [V J E Zh A],
        "koci"  => [K O Ci I],
        "wije"  => [V I J E],
        "kiedy" => [K J E D Y],
        "siad"  => [Si A D]
    );
}

#[test]
fn test_letter_j() {
    check_word_transcriptions!(
        "kij" => [K I J],
        "jak" => [J A K]
    );
}

#[test]
fn test_letter_k() {
    check_word_transcriptions!(
        "kod" => [K O D],
        "dok" => [D O K],
        "także" => [T A G Zh E],
        "kwiat" => [K F J A T]
    );
}

#[test]
fn test_letter_l() {
    check_word_transcriptions!(
        "lama" => [L A M A],
        "halka" => [H A L K A],
        "bal" => [B A L]
    );
}

#[test]
fn test_letter_ll() {
    check_word_transcriptions!(
        "łamy" => [W A M Y],
        "bułka" => [B U W K A],
        "pół" => [P U W]
    );
}

#[test]
fn test_letter_m() {
    check_word_transcriptions!(
        "mamka" => [M A M K A],
        "wyłom" => [V Y W O M]
    );
}

#[test]
fn test_letter_n() {
    check_word_transcriptions!(
        "najmniej" => [N A J M Ni E J],
        "mnie" => [M Ni E],
        "nas" => [N A S],
        "sanna" => [S A N N A],
        "pani" => [P A Ni I]
    );
}

#[test]
fn test_letter_o() {
    check_word_transcriptions!(
        "polo" => [P O L O],
        "olcha" => [O L H A]
    );
}

#[test]
fn test_failed_transcriptions() {
    assert_eq!(transcribe("beyoncé"), None);
    assert_eq!(transcribe("übermensch"), None);
}
