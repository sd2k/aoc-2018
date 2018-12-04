use std::str::FromStr;

use chrono::{prelude::*, Duration, NaiveDateTime};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::info;
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct GuardID(u32);
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Minute(u32);

struct Guard {
    id: GuardID,
    minutes_asleep: HashMap<Minute, u32>,
}

impl Guard {
    fn new(id: u32) -> Self {
        Guard {
            id: GuardID(id),
            minutes_asleep: HashMap::new(),
        }
    }
    fn add_sleep_window(&mut self, start: &NaiveDateTime, end: &NaiveDateTime) {
        let total_minutes_sleeping = end.signed_duration_since(*start).num_minutes();
        for minute in 0..total_minutes_sleeping {
            let timestamp = start
                .checked_add_signed(Duration::minutes(minute))
                .expect("adding timestamps to work");
            self.minutes_asleep
                .entry(Minute(timestamp.minute()))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    fn most_common(&self) -> Minute {
        let mut mins_vec: Vec<(&Minute, &u32)> = self.minutes_asleep.iter().collect();
        mins_vec.sort_unstable_by_key(|el| el.1);
        *mins_vec.iter().last().unwrap().0
    }
    fn total_minutes_sleeping(&self) -> u32 {
        self.minutes_asleep.values().sum()
    }
}

enum Action {
    StartsShift(u32),
    FallsAsleep,
    WakesUp,
}

const LOG_ENTRY_REGEX: &str = r".+Guard #(?P<id>\d+) begins shift";

struct LogEntry {
    time: NaiveDateTime,
    action: Action,
}

impl FromStr for LogEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(LOG_ENTRY_REGEX).expect("failed to create regex");
        }
        let mut s = s.split(']');
        let time = NaiveDateTime::parse_from_str(
            s.next().unwrap().trim_left_matches('['),
            "%Y-%m-%d %H:%M",
        )
        .expect("failed to parse timestamp");
        let action = match s.next().unwrap() {
            " wakes up" => Action::WakesUp,
            " falls asleep" => Action::FallsAsleep,
            other => Action::StartsShift(
                RE.captures(other).expect("no regex match found")["id"]
                    .parse()
                    .expect("failed to parse guard ID"),
            ),
        };
        Ok(LogEntry { time, action })
    }
}

fn build_guard_map(entries: &[LogEntry]) -> HashMap<GuardID, Guard> {
    let mut guards: HashMap<GuardID, Guard> = HashMap::new();
    let mut iter = entries.iter();
    info!("Looking for guard actions");
    loop {
        let entry = iter.next();

        if entry.is_none() {
            // We're out of log entries
            break;
        };

        match &entry.unwrap() {
            LogEntry {
                time,
                action: Action::StartsShift(guard_id),
            } => {
                info!("Guard {} started shift at {}", guard_id, time);
                // Assume we'll always see a 'falls asleep' before 'wakes up'.
                // Otherwise we'll panic.
                let mut start_time = None;
                let mut guard_iter = iter.clone();
                loop {
                    match guard_iter.next() {
                        Some(LogEntry {
                            time: start,
                            action: Action::FallsAsleep,
                        }) => {
                            info!("Guard {} fell asleep at {}", guard_id, start);
                            start_time = Some(start);
                        }
                        Some(LogEntry {
                            time: end,
                            action: Action::WakesUp,
                        }) => {
                            info!("Guard {} woke up at {}", guard_id, end);
                            guards
                                .entry(GuardID(*guard_id))
                                .and_modify(|guard| {
                                    guard.add_sleep_window(
                                        start_time.expect("Woke up before falling asleep"),
                                        end,
                                    )
                                })
                                .or_insert({
                                    let mut guard = Guard::new(*guard_id);
                                    guard.add_sleep_window(
                                        start_time.expect("Woke up before falling asleep"),
                                        end,
                                    );
                                    guard
                                });
                        }
                        _ => {
                            info!("Moving to next guard");
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    guards
}

pub fn part1(input: &[&str]) -> u32 {
    info!("Parsing log entries");
    let mut entries: Vec<LogEntry> = input
        .iter()
        .map(|x| LogEntry::from_str(x).expect("could not parse log entry"))
        .collect();
    entries.sort_unstable_by_key(|el| el.time);
    let guards = build_guard_map(&entries);
    let mut total_minutes_sleeping: Vec<(GuardID, u32)> = guards
        .values()
        .map(|guard| (guard.id, guard.total_minutes_sleeping()))
        .collect();
    total_minutes_sleeping.sort_unstable_by_key(|el| el.1);
    let sleepiest_guard = total_minutes_sleeping.iter().last().unwrap().0;
    let most_common = guards[&sleepiest_guard].most_common();
    most_common.0 * sleepiest_guard.0
}

pub fn part2(input: &[&str]) -> u32 {
    info!("Parsing log entries");
    let mut entries: Vec<LogEntry> = input
        .iter()
        .map(|x| LogEntry::from_str(x).expect("could not parse log entry"))
        .collect();
    entries.sort_unstable_by_key(|el| el.time);
    let guards = build_guard_map(&entries);
    let mut minutes_per_minute: Vec<(&GuardID, &Minute, &u32)> = guards
        .iter()
        .flat_map(|(guard_id, guard)| {
            guard
                .minutes_asleep
                .iter()
                .map(|(k, v)| (guard_id, k, v))
                .collect::<Vec<(&GuardID, &Minute, &u32)>>()
        })
        .collect();
    minutes_per_minute.sort_unstable_by_key(|el| el.2);
    let most = minutes_per_minute.iter().last().unwrap();
    (most.0).0 * (most.1).0
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = &[
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ];
        assert_eq!(part1(input), 240);
    }

    #[test]
    fn test_part2() {
        let input = &[
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ];
        assert_eq!(part2(input), 4455);
    }
}
