use std::collections::HashMap;
fn main() {
    let teams: [(&str, i32); 3] = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1: HashMap<&str, i32> = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT team_map2 in two ways
    // Tips: one of the approaches is to use `collect` method
    // One Approach:
    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
    // Second Approach:
    // let teams_map2: HashMap<&str, i32> = teams.into_iter().collect(); // does not compile in new
    // rust version

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}
