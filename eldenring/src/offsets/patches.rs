use engine::{Version, module_handle, version};

pub fn no_logo() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0xAAAD4A,
        Version::ER1_2_1 => 0xAAADCA,
        Version::ER1_2_2 => 0xAAAE3A,
        Version::ER1_2_3 => 0xAAAF1A,
        Version::ER1_3_0 => 0xAB021D,
        Version::ER1_3_1 => 0xAB022D,
        Version::ER1_3_2 => 0xAB020D,
        Version::ER1_4_0 => 0xA8FB6D,
        Version::ER1_4_1 => 0xA8FA7D,
        Version::ER1_5_0 => 0xA9417D,
        Version::ER1_6_0 => 0xA9807D,
        Version::ER1_7_0 => 0xA9972D,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0xADB0FD,
        Version::ER1_9_0 => 0xADDC8D,
        Version::ER1_9_1 => 0xADDCED,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0xADDF7D,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0xB0BD7D,
        Version::ER2_3_0 => 0xB0C0ED,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0xB0C26D,
        Version::ER2_6_0 => 0xB0C3ED,
        Version::ER2_6_1 => 0xB0C44D,
        _ => 0x0,
    }
}

pub fn fps_cap() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0xDFEF5F,
        Version::ER1_2_1 => 0xDFEFAF,
        Version::ER1_2_2 => 0xDFF3BF,
        Version::ER1_2_3 => 0xDFFE9F,
        Version::ER1_3_0 => 0xE081CF,
        Version::ER1_3_1 => 0xE07F6F,
        Version::ER1_3_2 => 0xE07F4F,
        Version::ER1_4_0 => 0xDE8C5F,
        Version::ER1_4_1 => 0xDE8B6F,
        Version::ER1_5_0 => 0xDF094F,
        Version::ER1_6_0 => 0xDF6F7F,
        Version::ER1_7_0 => 0xDFA6BF,
        Version::ER1_8_0 => 0xE4167F,
        Version::ER1_8_1 => 0xE4165F,
        Version::ER1_9_0 => 0xE4438F,
        Version::ER1_9_1 => 0xE4449F,
        Version::ER2_0_0 => 0xE4474F,
        Version::ER2_0_1 => 0xE4482F,
        Version::ER2_2_0 => 0xE826BD,
        Version::ER2_2_3 => 0xE826DD,
        Version::ER2_3_0 => 0xE82BCD,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0xE82B2D,
        Version::ER2_6_0 => 0xE82AFD,
        Version::ER2_6_1 => 0xE82B5D,
        _ => 0x0,
    }
}

pub fn mute_music() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0xD4A95A,
        Version::ER1_2_1 => 0xD4A9DA,
        Version::ER1_2_2 => 0xD4AB3A,
        Version::ER1_2_3 => 0xD4AC2A,
        Version::ER1_3_0 => 0xD531BA,
        Version::ER1_3_1 => 0xD531CA,
        Version::ER1_3_2 => 0xD531AA,
        Version::ER1_4_0 => 0xD3343A,
        Version::ER1_4_1 => 0xD3334A,
        Version::ER1_5_0 => 0xD38A6A,
        Version::ER1_6_0 => 0xD3EDBA,
        Version::ER1_7_0 => 0xD424CA,
        Version::ER1_8_0 => 0xD88D0A,
        Version::ER1_8_1 => 0xD88CEA,
        Version::ER1_9_0 => 0xD8BA1A,
        Version::ER1_9_1 => 0xD8BB2A,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0xD8BDDA,
        Version::ER2_2_0 => 0xDC3E7A,
        Version::ER2_2_3 => 0xDC3E9A,
        Version::ER2_3_0 => 0xDC438A,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0xDC475A,
        Version::ER2_6_0 => 0xDC48DA,
        Version::ER2_6_1 => 0xDC493A,
        _ => 0x0,
    }
}

pub fn pause_world() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0xA98175,
        Version::ER1_2_1 => 0xA981F5,
        Version::ER1_2_2 => 0xA98265,
        Version::ER1_2_3 => 0xA98345,
        Version::ER1_3_0 => 0xA9D555,
        Version::ER1_3_1 => 0xA9D565,
        Version::ER1_3_2 => 0xA9D545,
        Version::ER1_4_0 => 0xA7CDC5,
        Version::ER1_4_1 => 0xA7CCD5,
        Version::ER1_5_0 => 0xA81335,
        Version::ER1_6_0 => 0xA851E5,
        Version::ER1_7_0 => 0xA868F5,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0xAC78E5,
        Version::ER1_9_0 => 0xACA475,
        Version::ER1_9_1 => 0xACA4D5,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0xACA765,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0xAF7E05,
        Version::ER2_3_0 => 0xAF8175,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0xAF8335,
        Version::ER2_6_0 => 0xAF84B5,
        Version::ER2_6_1 => 0xAF8515,
        _ => 0x0,
    }
}

pub fn torrent_disabled_underworld() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0xC81F8A,
        Version::ER1_2_1 => 0xC8200A,
        Version::ER1_2_2 => 0xC8216A,
        Version::ER1_2_3 => 0xC8225A,
        Version::ER1_3_0 => 0xC891FA,
        Version::ER1_3_1 => 0xC8920A,
        Version::ER1_3_2 => 0xC891EA,
        Version::ER1_4_0 => 0xC69A7A,
        Version::ER1_4_1 => 0xC6998A,
        Version::ER1_5_0 => 0xC6E77A,
        Version::ER1_6_0 => 0xC730EA,
        Version::ER1_7_0 => 0xC74DFA,
        Version::ER1_8_0 => 0xCBA2FA,
        Version::ER1_8_1 => 0xCBA2DA,
        Version::ER1_9_0 => 0xCBCF1A,
        Version::ER1_9_1 => 0xCBCF7A,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0xCBD20A,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0xCEF0EA,
        Version::ER2_3_0 => 0xCEF5DA,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0xCEF9AA,
        Version::ER2_6_0 => 0xCEFB2A,
        Version::ER2_6_1 => 0xCEFB8A,
        _ => 0x0,
    }
}

pub fn whistle_disabled() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x6DFF7F,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x6DFFEF,
        Version::ER1_2_3 => 0x6E010F,
        Version::ER1_3_0 |
        Version::ER1_3_1 => 0x6E150F,
        Version::ER1_3_2 => 0x6E14EF,
        Version::ER1_4_0 => 0x6E51AF,
        Version::ER1_4_1 => 0x6E50BF,
        Version::ER1_5_0 => 0x6E5F6F,
        Version::ER1_6_0 => 0xC730EA,
        Version::ER1_7_0 => 0x6E92DF,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x6F6ABF,
        Version::ER1_9_0 => 0x6F7B5F,
        Version::ER1_9_1 => 0x6F7BBF,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x6F7E4F,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x705E9F,
        Version::ER2_3_0 => 0x70608F,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x7060EF,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x70626F,
        _ => 0x0,
    }
}

pub fn open_map() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x7C4CDA,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x7C4D4A,
        Version::ER1_2_3 => 0x7C4E1A,
        Version::ER1_3_0 |
        Version::ER1_3_1 => 0x7C6E1A,
        Version::ER1_3_2 => 0x7C6DFA,
        Version::ER1_4_0 => 0x7C777A,
        Version::ER1_4_1 => 0x7C768A,
        Version::ER1_5_0 => 0x7C94DA,
        Version::ER1_6_0 => 0x7CB4DA,
        Version::ER1_7_0 => 0x7CCC1A,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x7DBEBA,
        Version::ER1_9_0 => 0x7DCFBA,
        Version::ER1_9_1 => 0x7DD01A,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x7DD2AA,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x7EE97A,
        Version::ER2_3_0 => 0x7EEB6A,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x7EEBCA,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x7EED4A,
        _ => 0x0,
    }
}

pub fn close_map() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x990BA5,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x990C25,
        Version::ER1_2_3 => 0x990D15,
        Version::ER1_3_0 => 0x99593A,
        Version::ER1_3_1 => 0x99594A,
        Version::ER1_3_2 => 0x99592A,
        Version::ER1_4_0 => 0x973A0A,
        Version::ER1_4_1 => 0x97391A,
        Version::ER1_5_0 => 0x977ADA,
        Version::ER1_6_0 => 0x979B2A,
        Version::ER1_7_0 => 0x97B26A,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x9A14FA,
        Version::ER1_9_0 => 0x9A354A,
        Version::ER1_9_1 => 0x9A35AA,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x9A383A,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x9C35AE,
        Version::ER2_3_0 => 0x9C374E,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x9C37AE,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x9C392E,
        _ => 0x0,
    }
}

pub fn can_fast_travel() -> u64 {
    module_handle() + match version() {
        Version::ER1_2_0 => 0x798300,
        Version::ER1_2_1 |
        Version::ER1_2_2 => 0x798370,
        Version::ER1_2_3 => 0x798440,
        Version::ER1_3_0 |
        Version::ER1_3_1 => 0x79A320,
        Version::ER1_3_2 => 0x79A300,
        Version::ER1_4_0 => 0x79DD60,
        Version::ER1_4_1 => 0x79DC70,
        Version::ER1_5_0 => 0x79FAA0,
        Version::ER1_6_0 => 0x7A1AA0,
        Version::ER1_7_0 => 0x7A31E0,
        Version::ER1_8_0 |
        Version::ER1_8_1 => 0x7B2110,
        Version::ER1_9_0 => 0x7B31B0,
        Version::ER1_9_1 => 0x7B3210,
        Version::ER2_0_0 |
        Version::ER2_0_1 => 0x7B34A0,
        Version::ER2_2_0 |
        Version::ER2_2_3 => 0x7C4A90,
        Version::ER2_3_0 => 0x7C4C80,
        Version::ER2_4_0 |
        Version::ER2_5_0 => 0x7C4CE0,
        Version::ER2_6_0 |
        Version::ER2_6_1 => 0x7C4E60,
        _ => 0x0,
    }
}