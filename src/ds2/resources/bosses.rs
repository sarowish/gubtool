pub struct Boss {
    pub name: &'static str,
    pub bonfire_id: i32,
    pub coordinates: &'static [f32; 16],
    pub event_object_id: Option<i32>,
    pub death_flag: u32,
}

pub static BOSSES: &[Boss; 41] = &[
    Boss {
        name: "The Last Giant",
        bonfire_id: 10655,
        coordinates: &[
            92.18, -40.43, -147.47, 0.00, 92.18, -40.43, -147.47, 55.84, 92.18, -40.43, -147.47,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100971,
    },
    Boss {
        name: "The Pursuer",
        bonfire_id: 10655,
        coordinates: &[
            139.12, 10.05, -217.90, 0.00, 139.12, 10.05, -217.90, 58.79, 139.12, 10.05, -217.90,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100968,
    },
    Boss {
        name: "Dragonrider",
        bonfire_id: 31650,
        coordinates: &[
            -6.91, -14.66, 283.71, 0.00, -6.91, -14.66, 283.71, 36.76, -6.91, -14.66, 283.71, 0.00,
            0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100959,
    },
    Boss {
        name: "Old Dragonslayer",
        bonfire_id: 31660,
        coordinates: &[
            -144.09, 8.03, 170.72, 0.00, -144.09, 8.03, 170.72, 45.42, -144.09, 8.03, 170.72, 0.00,
            0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100960,
    },
    Boss {
        name: "Flexile Sentry",
        bonfire_id: 18650,
        coordinates: &[
            2.91, -65.35, 514.14, 0.00, 2.91, -65.35, 514.14, 44.90, 2.91, -65.35, 514.14, 0.00,
            0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100961,
    },
    Boss {
        name: "Ruin Sentinels",
        bonfire_id: 16660,
        coordinates: &[
            -142.18, 11.41, 538.97, 0.00, -142.18, 11.41, 538.97, 48.82, -142.18, 11.41, 538.97,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100962,
    },
    Boss {
        name: "Belfry Gargoyles",
        bonfire_id: 16675,
        coordinates: &[
            -185.25, 14.93, 518.61, 0.00, -185.25, 14.93, 518.61, 58.12, -185.25, 14.93, 518.61,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101001,
    },
    Boss {
        name: "The Lost Sinner",
        bonfire_id: 16685,
        coordinates: &[
            -122.87, -76.97, 563.96, 0.00, -122.87, -76.97, 563.96, 47.41, -122.87, -76.97, 563.96,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100963,
    },
    Boss {
        name: "Executioner's Chariot",
        bonfire_id: 23655,
        coordinates: &[
            -249.67, 47.70, -30.20, 0.00, -249.67, 47.70, -30.20, 36.29, -249.67, 47.70, -30.20,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100953,
    },
    Boss {
        name: "Skeleton Lords",
        bonfire_id: 23660,
        coordinates: &[
            -404.14, 35.35, 225.87, 0.00, -404.14, 35.35, 225.87, 44.99, -404.14, 35.35, 225.87,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100954,
    },
    Boss {
        name: "Covetous Demon",
        bonfire_id: 17650,
        coordinates: &[
            -537.04, 52.61, 525.46, 0.00, -537.04, 52.61, 525.46, 60.89, -537.04, 52.61, 525.46,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100955,
    },
    Boss {
        name: "Mytha, the Baneful Queen",
        bonfire_id: 17675,
        coordinates: &[
            -562.06, 86.06, 530.41, 0.00, -562.06, 86.06, 530.41, 41.60, -562.06, 86.06, 530.41,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100956,
    },
    Boss {
        name: "Smelter Demon (Red)",
        bonfire_id: 19650,
        coordinates: &[
            -706.58, 176.36, 650.18, 0.00, -706.58, 176.36, 650.18, 40.83, -706.58, 176.36, 650.18,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100964,
    },
    Boss {
        name: "Old Iron King",
        bonfire_id: 19660,
        coordinates: &[
            -650.67, 166.55, 721.19, 0.00, -650.67, 166.55, 721.19, 34.74, -650.67, 166.55, 721.19,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100952,
    },
    Boss {
        name: "Royal Rat Vanguard",
        bonfire_id: 34650,
        coordinates: &[
            -122.89, -29.97, 34.62, 0.00, -122.89, -29.97, 34.62, 47.42, -122.89, -29.97, 34.62,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100965,
    },
    Boss {
        name: "The Rotten",
        bonfire_id: 25660,
        coordinates: &[
            -242.77, -226.56, -97.95, 0.00, -242.77, -226.56, -97.95, 58.82, -242.77, -226.56,
            -97.95, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100966,
    },
    Boss {
        name: "Scorpioness Najka",
        bonfire_id: 32660,
        coordinates: &[
            -460.01, 84.05, -285.30, 0.00, -460.01, 84.05, -285.30, 47.39, -460.01, 84.05, -285.30,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100957,
    },
    Boss {
        name: "Royal Rat Authority",
        bonfire_id: 33660,
        coordinates: &[
            -541.01, 107.65, -176.01, 0.00, -541.01, 107.65, -176.01, 60.74, -541.01, 107.65,
            -176.01, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100967,
    },
    Boss {
        name: "Prowling Magus & Congregation",
        bonfire_id: 14660,
        coordinates: &[
            -625.40, 116.23, -53.77, 0.00, -625.40, 116.23, -53.77, 58.04, -625.40, 116.23, -53.77,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101000,
    },
    Boss {
        name: "The Duke's Dear Freja",
        bonfire_id: 14655,
        coordinates: &[
            -581.67, 75.09, -139.77, 0.00, -581.67, 75.09, -139.77, 48.42, -581.67, 75.09, -139.77,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100951,
    },
    Boss {
        name: "Twin Dragonriders",
        bonfire_id: 21660,
        coordinates: &[
            -463.06, 108.87, -363.38, 0.00, -463.06, 108.87, -363.38, 61.22, -463.06, 108.87,
            -363.38, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100980,
    },
    Boss {
        name: "Looking Glass Knight",
        bonfire_id: 21655,
        coordinates: &[
            -632.53, 103.20, -342.16, 0.00, -632.53, 103.20, -342.16, 56.00, -632.53, 103.20,
            -342.16, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100958,
    },
    Boss {
        name: "Demon of Song",
        bonfire_id: 11670,
        coordinates: &[
            -1066.01, -31.48, -170.87, 0.00, -1066.01, -31.48, -170.87, 44.17, -1066.01, -31.48,
            -170.87, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100950,
    },
    Boss {
        name: "Velstadt, the Royal Aegis",
        bonfire_id: 24655,
        coordinates: &[
            -1004.70, -132.63, -72.88, 0.00, -1004.70, -132.63, -72.88, 35.29, -1004.70, -132.63,
            -72.88, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100975,
    },
    Boss {
        name: "Guardian Dragon",
        bonfire_id: 15655,
        coordinates: &[
            -746.30, 80.42, -249.34, 0.00, -746.30, 80.42, -249.34, 62.29, -746.30, 80.42, -249.34,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100970,
    },
    Boss {
        name: "Ancient Dragon",
        bonfire_id: 27655,
        coordinates: &[
            -862.93, 336.15, -670.19, 0.00, -862.93, 336.15, -670.19, 33.42, -862.93, 336.15,
            -670.19, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100977,
    },
    Boss {
        name: "Giant Lord",
        bonfire_id: 10675,
        coordinates: &[
            44.59, -11.07, -152.36, 0.00, 44.59, -11.07, -152.36, 58.43, 44.59, -11.07, -152.36,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100972,
    },
    Boss {
        name: "Vendrick",
        bonfire_id: 24655,
        coordinates: &[
            -978.03, -136.16, -28.82, 0.00, -978.03, -136.16, -28.82, 33.60, -978.03, -136.16,
            -28.82, 0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100978,
    },
    /*
        Boss {
            name: "Darklurker",
            bonfire_id: 671285248,
            coordinates:
            event_object_id: Some(400004,
            death_flag: 100979,
        },
    */
    Boss {
        name: "Darklurker",
        bonfire_id: 671285248,
        coordinates: &[
            -981.69775, 15.755093, 301.79767, 0.0, -981.69775, 15.755093, 301.79767, 45.341534,
            -981.69775, 15.755093, 301.79767, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        event_object_id: Some(400020),
        death_flag: 100979,
    },
    Boss {
        name: "Throne Watcher & Throne Defender",
        bonfire_id: 21650,
        coordinates: &[
            -697.18, -5.90, -259.90, 0.00, -697.18, -5.90, -259.90, 43.69, -697.18, -5.90, -259.90,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100974,
    },
    Boss {
        name: "Nashandra",
        bonfire_id: 21650,
        coordinates: &[
            -697.18, -5.90, -259.90, 0.00, -697.18, -5.90, -259.90, 43.69, -697.18, -5.90, -259.90,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 100973,
    },
    Boss {
        name: "Aldia, Scholar of the First Sin",
        bonfire_id: 21650,
        coordinates: &[
            -697.18, -5.90, -259.90, 0.00, -697.18, -5.90, -259.90, 43.69, -697.18, -5.90, -259.90,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101080,
    },
    Boss {
        name: "Worshippers of the Dead",
        bonfire_id: 35655,
        coordinates: &[
            -7.41, 18.61, 115.72, 0.00, -7.41, 18.61, 115.72, 33.32, -7.41, 18.61, 115.72, 0.00,
            0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101053,
    },
    Boss {
        name: "Elana, the Squalid Queen",
        bonfire_id: 35680,
        coordinates: &[
            -89.85, -71.60, -34.32, 0.00, -89.85, -71.60, -34.32, 51.99, -89.85, -71.60, -34.32,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101050,
    },
    Boss {
        name: "Sinh, the Slumbering Dragon",
        bonfire_id: 35665,
        coordinates: &[
            -173.86, -79.51, -3.73, 0.00, -173.86, -79.51, -3.73, 48.41, -173.86, -79.51, -3.73,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101051,
    },
    Boss {
        name: "Fume Knight",
        bonfire_id: 36670,
        coordinates: &[
            -167.46, -59.14, 385.48, 0.00, -167.46, -59.14, 385.48, 56.50, -167.46, -59.14, 385.48,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101061,
    },
    Boss {
        name: "Smelter Demon (Blue)",
        bonfire_id: 36665,
        coordinates: &[
            -253.99, -26.80, 461.00, 0.00, -253.99, -26.80, 461.00, 40.69, -253.99, -26.80, 461.00,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101063,
    },
    Boss {
        name: "Sir Alonne",
        bonfire_id: 841220096,
        coordinates: &[
            -100.11, 52.61, 693.86, 0.00, -100.11, 52.61, 693.86, 34.74, -100.11, 52.61, 693.86,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: Some(5300000),
        death_flag: 101062,
    },
    Boss {
        name: "Aava, the King's Pet",
        bonfire_id: 37650,
        coordinates: &[
            -89.58, -22.25, -31.02, 0.00, -89.58, -22.25, -31.02, 37.00, -89.58, -22.25, -31.02,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101071,
    },
    Boss {
        name: "Lud & Zallen, the King's Pets",
        bonfire_id: 841285632,
        coordinates: &[
            -380.34, -69.53, 357.43, 0.00, -380.34, -69.53, 357.43, 38.63, -380.34, -69.53, 357.43,
            0.00, 0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: Some(400000),
        death_flag: 101072,
    },
    Boss {
        name: "Burnt Ivory King",
        bonfire_id: 37670,
        coordinates: &[
            203.59, -6.07, -53.48, 0.00, 203.59, -6.07, -53.48, 56.08, 203.59, -6.07, -53.48, 0.00,
            0.00, 0.00, 0.00, 1.00,
        ],
        event_object_id: None,
        death_flag: 101070,
    },
/*
    Boss {
        name: "Dark Chasm of Old (Shaded Woods)",
        bonfire_id: 671285248,
        coordinates: None,
        event_object_id: Some(400010),
        death_flag: 0,
    },
    Boss {
        name: "Dark Chasm of Old (Black Gulch)",
        bonfire_id: 671285248,
        coordinates: None,
        event_object_id: Some(400030),
        death_flag: 0,
    },
    Boss {
        name: "Dark Chasm of Old (Drangleic Castle)",
        bonfire_id: 671285248,
        coordinates: None,
        event_object_id: Some(400020),
        death_flag: 0,
    },
*/
];