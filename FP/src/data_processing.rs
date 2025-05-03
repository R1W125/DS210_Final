//This module is used to process the data

use std::str::FromStr;

#[derive(Debug, Clone)]
// CrimeData struct contains the latitude, longitude, victim sex, and victim race
pub struct CrimeData {
    pub latitude: f64,
    pub longitude: f64,
    pub victim_sex: String,
    pub victim_race: String,
}

// Takes in a line of data and returns a CrimeData struct
pub fn process_data(line: &str) -> Option<CrimeData> {
    let parts: Vec<&str> = line.split(',').collect();
    
    // Skip header row
    if parts[0] == "INCIDENT_KEY" {
        return None;
    }

    // Check num columns
    if parts.len() < 21 {
        return None;
    }

    // Latitude is at index 18, Longitude at 19, VIC_SEX at index 14, VIC_RACE at index 15, BORO at index 3
    let latitude = f64::from_str(parts[18].trim()).ok()?;
    let longitude = f64::from_str(parts[19].trim()).ok()?;
    let victim_sex = parts[14].trim().to_string();
    let victim_race = parts[15].trim().to_string();

    // Skip if any required field is empty
    if victim_sex.is_empty() || victim_race.is_empty() {
        return None;
    }

    Some(CrimeData {
        latitude,
        longitude,
        victim_sex,
        victim_race,
    })
}
