pub struct Bonfire {
    pub name: &'static str,
    pub main_area: &'static str,
    pub bonfire_id: i32,
}

pub static BONFIRES: &[Bonfire; 77] = &[
    Bonfire {
        name: "Fire Keeper's Dwelling",
        main_area: "Things Betwixt",
        bonfire_id: 2650,
    },
    Bonfire {
        name: "Far Fire",
        main_area: "Majula",
        bonfire_id: 4650,
    },
    Bonfire {
        name: "Crestfallen's Retreat",
        main_area: "Forest of Fallen Giants",
        bonfire_id: 10670,
    },
    Bonfire {
        name: "Cardinal Tower",
        main_area: "Forest of Fallen Giants",
        bonfire_id: 10655,
    },
    Bonfire {
        name: "Soldier's Rest",
        main_area: "Forest of Fallen Giants",
        bonfire_id: 10660,
    },
    Bonfire {
        name: "The Place Unbeknownst",
        main_area: "Forest of Fallen Giants",
        bonfire_id: 10675,
    },
    Bonfire {
        name: "Heide's Ruin",
        main_area: "Heide's Tower of Flame",
        bonfire_id: 31655,
    },
    Bonfire {
        name: "Tower of Flame",
        main_area: "Heide's Tower of Flame",
        bonfire_id: 31650,
    },
    Bonfire {
        name: "Cathedral of Blue",
        main_area: "Heide's Tower of Flame",
        bonfire_id: 31660,
    },
    Bonfire {
        name: "Unseen Path to Heide's",
        main_area: "No-man's Wharf",
        bonfire_id: 18650,
    },
    Bonfire {
        name: "Exile Holding Cells",
        main_area: "The Lost Bastille",
        bonfire_id: 16655,
    },
    Bonfire {
        name: "McDuff's Workshop",
        main_area: "The Lost Bastille",
        bonfire_id: 16670,
    },
    Bonfire {
        name: "Servant's Quarters",
        main_area: "The Lost Bastille",
        bonfire_id: 16675,
    },
    Bonfire {
        name: "Straid's Cell",
        main_area: "The Lost Bastille",
        bonfire_id: 16650,
    },
    Bonfire {
        name: "The Tower Apart",
        main_area: "The Lost Bastille",
        bonfire_id: 16660,
    },
    Bonfire {
        name: "The Saltfort",
        main_area: "Sinners' Rise",
        bonfire_id: 16685,
    },
    Bonfire {
        name: "Upper Ramparts",
        main_area: "Belfry Luna",
        bonfire_id: 16665,
    },
    Bonfire {
        name: "Undead Refuge",
        main_area: "Huntsman's Copse",
        bonfire_id: 23650,
    },
    Bonfire {
        name: "Bridge Approach",
        main_area: "Huntsman's Copse",
        bonfire_id: 23655,
    },
    Bonfire {
        name: "Undead Lockaway",
        main_area: "Huntsman's Copse",
        bonfire_id: 23660,
    },
    Bonfire {
        name: "Undead Purgatory",
        main_area: "Undead Purgatory",
        bonfire_id: 23665,
    },
    Bonfire {
        name: "Poison Pool",
        main_area: "Harvest Valley",
        bonfire_id: 17665,
    },
    Bonfire {
        name: "The Mines",
        main_area: "Harvest Valley",
        bonfire_id: 17650,
    },
    Bonfire {
        name: "Lower Earthen Peak",
        main_area: "Earthen Peak",
        bonfire_id: 17655,
    },
    Bonfire {
        name: "Central Earthen Peak",
        main_area: "Earthen Peak",
        bonfire_id: 17670,
    },
    Bonfire {
        name: "Upper Earthen Peak",
        main_area: "Earthen Peak",
        bonfire_id: 17675,
    },
    Bonfire {
        name: "Threshold Bridge",
        main_area: "Iron Keep",
        bonfire_id: 19655,
    },
    Bonfire {
        name: "Ironhearth Hall",
        main_area: "Iron Keep",
        bonfire_id: 19650,
    },
    Bonfire {
        name: "Eygil's Idol",
        main_area: "Iron Keep",
        bonfire_id: 19660,
    },
    Bonfire {
        name: "Belfry Sol Approach",
        main_area: "Belfry Sol",
        bonfire_id: 19665,
    },
    Bonfire {
        name: "Old Akelarre",
        main_area: "Shaded Woods",
        bonfire_id: 29650,
    },
    Bonfire {
        name: "Ruined Fork Road",
        main_area: "Shaded Woods",
        bonfire_id: 32655,
    },
    Bonfire {
        name: "Shaded Ruins",
        main_area: "Shaded Woods",
        bonfire_id: 32660,
    },
    Bonfire {
        name: "Gyrm's Respite",
        main_area: "Doors of Pharros",
        bonfire_id: 33655,
    },
    Bonfire {
        name: "Ordeal's End",
        main_area: "Doors of Pharros",
        bonfire_id: 33660,
    },
    Bonfire {
        name: "Royal Army Campsite",
        main_area: "Brightstone Cove Tseldora",
        bonfire_id: 14655,
    },
    Bonfire {
        name: "Chapel Threshold",
        main_area: "Brightstone Cove Tseldora",
        bonfire_id: 14660,
    },
    Bonfire {
        name: "Lower Brightstone Cove",
        main_area: "Brightstone Cove Tseldora",
        bonfire_id: 14650,
    },
    Bonfire {
        name: "Harvel's Resting Place",
        main_area: "Grave of Saints",
        bonfire_id: 34655,
    },
    Bonfire {
        name: "Grave Entrance",
        main_area: "Grave of Saints",
        bonfire_id: 34650,
    },
    Bonfire {
        name: "Upper Gutter",
        main_area: "The Gutter",
        bonfire_id: 25665,
    },
    Bonfire {
        name: "Central Gutter",
        main_area: "The Gutter",
        bonfire_id: 25655,
    },
    Bonfire {
        name: "Black Gulch Mouth",
        main_area: "Black Gulch",
        bonfire_id: 25650,
    },
    Bonfire {
        name: "Hidden Chamber",
        main_area: "Black Gulch",
        bonfire_id: 25660,
    },
    Bonfire {
        name: "King's Gate",
        main_area: "Drangleic Castle",
        bonfire_id: 21650,
    },
    Bonfire {
        name: "Forgotten Chamber",
        main_area: "Drangleic Castle",
        bonfire_id: 21660,
    },
    Bonfire {
        name: "Under Castle Drangleic",
        main_area: "Drangleic Castle",
        bonfire_id: 21665,
    },
    Bonfire {
        name: "Central Castle Drangleic",
        main_area: "Drangleic Castle",
        bonfire_id: 21655,
    },
    Bonfire {
        name: "Tower of Prayer",
        main_area: "Shrine of Amana",
        bonfire_id: 11650,
    },
    Bonfire {
        name: "Crumbled Ruins",
        main_area: "Shrine of Amana",
        bonfire_id: 11655,
    },
    Bonfire {
        name: "Rhoy's Resting Place",
        main_area: "Shrine of Amana",
        bonfire_id: 11660,
    },
    Bonfire {
        name: "Rise of the Dead",
        main_area: "Shrine of Amana",
        bonfire_id: 11670,
    },
    Bonfire {
        name: "Undead Crypt Entrance",
        main_area: "Undead Crypt",
        bonfire_id: 24655,
    },
    Bonfire {
        name: "Undead Ditch",
        main_area: "Undead Crypt",
        bonfire_id: 24650,
    },
    Bonfire {
        name: "Foregarden",
        main_area: "Aldia's Keep",
        bonfire_id: 15650,
    },
    Bonfire {
        name: "Ritual Site",
        main_area: "Aldia's Keep",
        bonfire_id: 15655,
    },
    Bonfire {
        name: "Dragon Aerie",
        main_area: "Dragon Aerie",
        bonfire_id: 27650,
    },
    Bonfire {
        name: "Shrine Entrance",
        main_area: "Dragon Shrine",
        bonfire_id: 27655,
    },
    Bonfire {
        name: "Sanctum Walk",
        main_area: "Shulva, Sanctum City",
        bonfire_id: 35650,
    },
    Bonfire {
        name: "Tower of Prayer",
        main_area: "Shulva, Sanctum City",
        bonfire_id: 35685,
    },
    Bonfire {
        name: "Priestess's Chamber",
        main_area: "Shulva, Sanctum City",
        bonfire_id: 35655,
    },
    Bonfire {
        name: "Hidden Sanctum Chamber",
        main_area: "Dragon's Sanctum",
        bonfire_id: 35670,
    },
    Bonfire {
        name: "Lair of the Imperfect",
        main_area: "Dragon's Sanctum",
        bonfire_id: 35675,
    },
    Bonfire {
        name: "Sanctum Interior",
        main_area: "Dragon's Sanctum",
        bonfire_id: 35680,
    },
    Bonfire {
        name: "Sanctum Nadir",
        main_area: "Dragon's Rest",
        bonfire_id: 35665,
    },
    Bonfire {
        name: "Throne Floor",
        main_area: "Brume Tower",
        bonfire_id: 36650,
    },
    Bonfire {
        name: "Upper Floor",
        main_area: "Brume Tower",
        bonfire_id: 36660,
    },
    Bonfire {
        name: "Foyer",
        main_area: "Brume Tower",
        bonfire_id: 36655,
    },
    Bonfire {
        name: "Lowermost Floor",
        main_area: "Brume Tower",
        bonfire_id: 36670,
    },
    Bonfire {
        name: "The Smelter Throne",
        main_area: "Brume Tower",
        bonfire_id: 36675,
    },
    Bonfire {
        name: "Iron Passage",
        main_area: "Iron Passage",
        bonfire_id: 36665,
    },
    Bonfire {
        name: "Outer Wall",
        main_area: "Frozen Eleum Loyce",
        bonfire_id: 37650,
    },
    Bonfire {
        name: "Abandoned Dwelling",
        main_area: "Frozen Eleum Loyce",
        bonfire_id: 37660,
    },
    Bonfire {
        name: "Inner Wall",
        main_area: "Frozen Eleum Loyce",
        bonfire_id: 37685,
    },
    Bonfire {
        name: "Lower Garrison",
        main_area: "Frozen Eleum Loyce",
        bonfire_id: 37665,
    },
    Bonfire {
        name: "Expulsion Chamber",
        main_area: "Frozen Eleum Loyce",
        bonfire_id: 37675,
    },
    Bonfire {
        name: "Grand Cathedral",
        main_area: "Grand Cathedral",
        bonfire_id: 37670,
    },
];
