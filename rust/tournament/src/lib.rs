use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Not;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

struct InvalidValue;

impl FromStr for MatchResult {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "win" => Ok(MatchResult::Win),
            "draw" => Ok(MatchResult::Draw),
            "loss" => Ok(MatchResult::Loss),
            _ => Err(InvalidValue),
        }
    }
}

impl Not for MatchResult {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
}

#[derive(Debug, Default)]
struct Stats {
    total_matches: u32,
    won_matches: u32,
    drawn_matches: u32,
    lost_matches: u32,
    points: u32,
}

fn update_stats(entry: &mut Stats, result: MatchResult) {
    entry.total_matches += 1;
    match result {
        MatchResult::Win => {
            entry.won_matches += 1;
            entry.points += 3;
        }
        MatchResult::Draw => {
            entry.drawn_matches += 1;
            entry.points += 1;
        }
        MatchResult::Loss => {
            entry.lost_matches += 1;
        }
    }
}

fn parse_line(s: &str) -> Option<(&str, &str, MatchResult)> {
    let fields: Vec<_> = s.split(';').collect();
    if fields.len() != 3 {
        return None;
    }
    match fields[2].parse() {
        Ok(result) => Some((fields[0], fields[1], result)),
        Err(_) => None,
    }
}

fn gather_stats(competition_results: &str) -> HashMap<&str, Stats> {
    let mut stats = HashMap::new();
    for line in competition_results.split('\n') {
        if let Some((team_a, team_b, result)) = parse_line(line) {
            {
                let team_a = stats.entry(team_a).or_insert(Stats::default());
                update_stats(team_a, result);
            }
            {
                let team_b = stats.entry(team_b).or_insert(Stats::default());
                update_stats(team_b, !result);
            }
        }
    }
    stats
}

pub fn tally(competition_results: &str) -> String {
    let stats = gather_stats(competition_results);
    let mut stats: Vec<_> = stats.iter().collect();
    stats.sort_by(|&(name_a, ref a), &(name_b, ref b)| {
        match b.points.cmp(&a.points) {
            Ordering::Equal => {
                match b.won_matches.cmp(&a.won_matches) {
                    Ordering::Equal => name_a.cmp(&name_b),
                    other => other,
                }
            }
            other => other,
        }
    });

    let mut output = format!("{:31}| MP |  W |  D |  L |  P", "Team");
    for (team, stat) in stats {
        output += &format!("\n{:31}| {:2} | {:2} | {:2} | {:2} | {:2}",
                           team,
                           stat.total_matches,
                           stat.won_matches,
                           stat.drawn_matches,
                           stat.lost_matches,
                           stat.points);
    }
    output
}
