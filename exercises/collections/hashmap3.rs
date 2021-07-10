// hashmap3.rs

// A list of scores (one per line) of a match is given. Each line is
// of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute the command `rustlings hint hashmap3` if you need
// hints.

// I AM NOT DONE

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // name of the team is the key and corresponding team is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1,
        // will be number of goals conceded by the team_2, similarly
        // goals scored by team_2 will be number of goals conceded by
        // team_1.
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
