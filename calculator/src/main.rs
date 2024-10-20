use std::{fs::read_to_string, path::PathBuf};

use anyhow::Result;
use chrono::{NaiveDate, TimeDelta};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Details {
    author: String,
    company: String,
    signature_image: PathBuf,

    // Variables for calculations
    cent_per_km: usize,
    small_catering_money: usize,
    big_catering_money: usize,

    // These values change each month
    document_date: NaiveDate,
    month: String,
    entries: Vec<Entry>,

    #[serde(default)]
    totals: TotalValues,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(dead_code)]
struct TotalValues {
    travel_distance_km: usize,
    travel_money: usize,
    catering_money: usize,
    money: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Entry {
    day: usize,
    subject: String,
    #[serde(default)]
    start_time: Option<String>,
    #[serde(default)]
    end_time: Option<String>,
    #[serde(default)]
    traveled_km: usize,
    #[serde(default)]
    calculated: CalculatedValues,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(dead_code)]
struct CalculatedValues {
    hours: usize,
    catering_money: usize,
    travel_money: usize,
}

#[derive(Parser, Debug)]
#[command(name = "travel_expense_calculator")]
pub struct Arguments {
    pub path_in: PathBuf,
    pub path_out: PathBuf,
}

fn main() -> Result<()> {
    let opt = Arguments::parse();

    let yaml = read_to_string(opt.path_in)?;
    let mut details: Details = serde_yaml::from_str(&yaml)?;

    let mut total_catering_money = 0;
    let mut total_travel_money = 0;
    let mut total_travel_distance_km: usize = 0;
    let mut total_money = 0;

    for entry in &mut details.entries {
        // Insert some default values
        let start_time = entry.start_time.get_or_insert("00:00".to_string());
        let end_time = entry.end_time.get_or_insert("24:00".to_string());

        // Do manual parsing of the hours, as "24:00" isn't a valid number.
        let (start_hour, start_minute) = start_time.split_once(":").expect("No ':' in start time.");
        let (end_hour, end_minute) = end_time.split_once(":").expect("No ':' in end time.");
        let (start_hour, start_minute): (u32, u32) = (start_hour.parse()?, start_minute.parse()?);
        let (end_hour, end_minute): (u32, u32) = (end_hour.parse()?, end_minute.parse()?);

        // Calculate the total hours based on the start/end time.
        // For that, we just do some hour calculations on a day of which we know that it doesn't
        // have any time shifting shennanigans
        let clean_day = NaiveDate::from_ymd_opt(2024, 10, 19).unwrap();
        let start = clean_day.and_hms_opt(start_hour, start_minute, 0).unwrap();
        let end = if end_hour >= 24 {
            let mut day = clean_day.and_hms_opt(0, 0, 0).unwrap();
            day += TimeDelta::days(1);
            day
        } else {
            clean_day.and_hms_opt(end_hour, end_minute, 0).unwrap()
        };
        let total_hours = (end - start).num_hours().unsigned_abs() as usize;

        // Calculate the catering money based on the total hours
        let catering_money = if total_hours >= 24 {
            details.big_catering_money
        } else if total_hours > 8 {
            details.small_catering_money
        } else {
            0
        };

        let travel_money = entry.traveled_km * details.cent_per_km;

        total_travel_money += travel_money;
        total_catering_money += catering_money;
        total_travel_distance_km += entry.traveled_km;
        total_money += travel_money + catering_money;

        entry.calculated = CalculatedValues {
            hours: total_hours,
            catering_money,
            travel_money,
        };
    }

    details.totals = TotalValues {
        travel_distance_km: total_travel_distance_km,
        travel_money: total_travel_money,
        catering_money: total_catering_money,
        money: total_money,
    };

    let yaml = serde_yaml::to_string(&details)?;
    std::fs::write(opt.path_out, yaml)?;

    Ok(())
}
