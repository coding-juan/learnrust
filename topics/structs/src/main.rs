// ---------------------
//  - Vanilla Structs -
// ---------------------
struct GameMode {
    name: String,
    max_players: u32,
    game_time: f32,
    featured: bool
}

impl GameMode {
    fn new(name: &str, max_players: u32, game_time: f32, featured: bool) -> GameMode { // Here we don't use self in any way so it's an associated function (static method?)
        GameMode {
            name: String::from(name),
            max_players,
            game_time,
            featured
        }
    }

    fn get_gamemode_preview(&self) -> String { // This method borrows self immutably
        format!("{}: {} players, {} minutes", self.name, self.max_players, self.game_time)
    }

    fn destroy(self) { // Here we take ownership of self so it can no longer be used
        println!("You can't use this gamemode anymore");
    }
}

// ---------------------
//  - Tuple struct -
// ---------------------
struct Point(i32, i32, i32);

// ---------------------
//  - Unit-Like struct -
// ---------------------
struct AlwaysEqual;

// ---------------------
//  - App -
// ---------------------

fn main() {
    let mut gm = GameMode {
        name: String::from("Deathmatch"),
        max_players: 16,
        game_time: 20.0,
        featured: false
    };
    gm.featured = true;

    // Update sytax
    let gm2 = GameMode {
        name: String::from("Team Deathmatch"),
        ..gm
    }; // From here on we cannot longer use variable 'gm'

    let gm3 = GameMode::new("Capture the Flag", 10, 20.0, false);

    let spawn_point = Point(0, 2, 0);
    let height = spawn_point.1;
    println!("{}", height);

    let subjet = AlwaysEqual;
}
