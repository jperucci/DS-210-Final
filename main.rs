use csv::Reader;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::error::Error;
#[derive(Debug, Clone, PartialEq)]
struct Country {
    name: String,
    communicable: f64,
    non_communicable: f64,
    co2: f64,
    cluster: Option<usize>,
}
fn load_data(file_path: &str) -> Result<Vec<Country>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut countries = Vec::new();

    for result in reader.records() {
        let record = result?;
        let name = record.get(0).unwrap_or_default().to_string();
        let communicable = record.get(8).unwrap_or("0.0").parse().unwrap_or(0.0);
        let non_communicable = record.get(9).unwrap_or("0.0").parse().unwrap_or(0.0);
        let co2 = record.get(12).unwrap_or("0.0").parse().unwrap_or(0.0);

        if communicable != 0.0 || non_communicable != 0.0 || co2 != 0.0 {
            countries.push(Country {
                name,
                communicable,
                non_communicable,
                co2,
                cluster: None,
            });
        }
    }

    Ok(countries)
}

fn initialize_centroids(countries: &[Country], k: usize) -> Vec<Country> {
    let mut rng = rand::thread_rng();
    let centroids: Vec<Country> = countries.choose_multiple(&mut rng, k).cloned().collect();
    centroids
}

fn assign_clusters(countries: &mut [Country], centroids: &[Country]) {
    for country in countries.iter_mut() {
        let mut min_distance = f64::MAX;
        let mut assigned_cluster = 0;

        for (i, centroid) in centroids.iter().enumerate() {
            let distance = euclidean_distance(country, centroid);
            if distance < min_distance {
                min_distance = distance;
                assigned_cluster = i;
            }
        }

        country.cluster = Some(assigned_cluster);
    }
}

fn update_centroids(countries: &[Country], k: usize) -> Vec<Country> {
    let mut new_centroids = vec![
        Country {
            name: String::new(),
            communicable: 0.0,
            non_communicable: 0.0,
            co2: 0.0,
            cluster: None,
        };
        k
    ];
    let mut counts = vec![0; k];

    for country in countries {
        if let Some(cluster) = country.cluster {
            new_centroids[cluster].communicable += country.communicable;
            new_centroids[cluster].non_communicable += country.non_communicable;
            new_centroids[cluster].co2 += country.co2;
            counts[cluster] += 1;
        }
    }

    for (i, centroid) in new_centroids.iter_mut().enumerate() {
        if counts[i] > 0 {
            centroid.communicable /= counts[i] as f64;
            centroid.non_communicable /= counts[i] as f64;
            centroid.co2 /= counts[i] as f64;
        }
    }

    new_centroids
}

fn euclidean_distance(country: &Country, centroid: &Country) -> f64 {
    ((country.communicable - centroid.communicable).powi(2)
        + (country.non_communicable - centroid.non_communicable).powi(2)
        + (country.co2 - centroid.co2).powi(2))
    .sqrt()
}

fn kmeans(mut countries: Vec<Country>, k: usize, max_iterations: usize) -> Vec<Country> {
    let mut centroids = initialize_centroids(&countries, k);

    for _ in 0..max_iterations {
        assign_clusters(&mut countries, &centroids);
        let new_centroids = update_centroids(&countries, k);

        if centroids == new_centroids {
            break;
        }

        centroids = new_centroids;
    }

    countries
}
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "life expectancy.csv";
    let countries = load_data(file_path)?;

    if countries.is_empty() {
        println!("No valid data found in the file.");
        return Ok(());
    }

    let k = 5;
    let max_iterations = 100;
    let clustered_countries = kmeans(countries, k, max_iterations);

    // Use a HashSet to filter out duplicates
    let mut unique_results = HashSet::new();
    for country in &clustered_countries {
        unique_results.insert((country.name.clone(), country.cluster.unwrap_or_default()));
    }

    // Print unique results
    for (name, cluster) in unique_results {
        println!("{} - Cluster: {}", name, cluster);
    }

    Ok(())
}
