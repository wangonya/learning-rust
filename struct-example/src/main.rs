enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Golie,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

fn main() {
    let player = HockeyPlayer {
        name: String::from("John Doe"),
        number: 17,
        position: HockeyPosition::Wing,
        goals_ytd: 8,
    };

    println!("Player -> {}", player.name)
}
