/* By CKKitty on 2024-Mar-09
 * Defines the syllable struct
 */

pub enum VowelPitch {
    HIGH,
    LOW
}

pub struct Syllable {
    onset: String,
    nucleus: String,
    coda: String,
    pitch: VowelPitch,
    nasal: bool
}

pub struct SyllableSet {
    syllables: Vec<Syllable>,
}