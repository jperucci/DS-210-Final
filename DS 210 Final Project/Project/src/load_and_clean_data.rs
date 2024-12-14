use csv::ReaderBuilder;
use std::collections::HashSet;
use std::error::Error;
use crate::Country;

pub fn load_and_clean_data(file_path: &str) -> Result<Vec<Country>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut seen_countries = HashSet::new();
    let mut countries = Vec::new();

    for result in reader.records() {
        let record = result?;
        let name = record.get(0).unwrap_or_default().to_string();
        let communicable = record.get(8).unwrap_or("0.0").parse().unwrap_or(0.0);
        let non_communicable = record.get(9).unwrap_or("0.0").parse().unwrap_or(0.0);
        let co2 = record.get(12).unwrap_or("0.0").parse().unwrap_or(0.0);

        if (communicable != 0.0 || non_communicable != 0.0 || co2 != 0.0)
            && !seen_countries.contains(&name)
        {
            seen_countries.insert(name.clone());
            countries.push(Country {
                name,
                communicable,
                non_communicable,
                co2,
                cluster: None,
            });
        }
    }

    println!("Loaded and cleaned {} unique countries.", countries.len());
    Ok(countries)
}