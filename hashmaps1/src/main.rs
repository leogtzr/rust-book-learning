use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 12);

    let team_name: String = String::from("blue");

    let res = scores.get("blue");

    // match res {
    //     None => {}
    //     Some(v) => {
    //         println!("We got: {}", *v);
    //     }
    // }

    if let Some(blue_score) = scores.get(&String::from("blue")) {
        println!("Blue score is: {}", blue_score);
    }

    for (team, score) in &scores {
        println!("Team: {}, score: {}", team, score);
    }

}
