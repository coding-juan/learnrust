struct Player {
    title: Option<String>,
}

fn main() {
    // Closures
    let player1 = Player { title: Some("Dragon Slayer".to_string()) };
    let player2 = Player { title: None };

    get_player_title(player1);
    get_player_title(player2);

    let mut player1 = Player { title: Some("Dragon Slayer".to_string()) };
    let mut player2 = Player { title: None };

    set_player_title_if_empty(&mut player1);
    set_player_title_if_empty(&mut player2);
    get_player_title(player1);
    get_player_title(player2);

    // Iterators
    iterator_demonstration();
}

// Closures
fn get_player_title(player: Player) {
    println!("{}", player.title.unwrap_or_else(|| "Noob".to_string()));
}

fn set_player_title_if_empty(player: &mut Player) {
    let new = player.title.as_deref().unwrap_or_else(|| "Epic Founder");
    player.title = Some(new.to_string());
}

// Iterators
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    iterator_sum();

    println!("Iterator tests passed");
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}