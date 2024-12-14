mod load_and_clean_data;

use load_and_clean_data::load_and_clean_data;
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct HashableCountry {
    name: String,
    communicable: i64,
    non_communicable: i64,
    co2: i64,
}

fn convert_to_hashable(country: &Country) -> HashableCountry {
    HashableCountry {
        name: country.name.clone(),
        communicable: (country.communicable * 1000.0) as i64, // Scale for precision.
        non_communicable: (country.non_communicable * 1000.0) as i64,
        co2: (country.co2 * 1000.0) as i64,
    }
}

fn initialize_centroids(countries: &[Country], k: usize) -> Vec<Country> {
    let mut rng = rand::thread_rng();
    countries.choose_multiple(&mut rng, k).cloned().collect()
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

fn build_graph(countries: &[Country], threshold: f64) -> Vec<(String, Vec<String>)> {
    let mut adjacency_list = Vec::new();

    for country in countries {
        let mut neighbors = Vec::new();
        for other in countries {
            if country.name != other.name {
                let distance = euclidean_distance(country, other);
                if distance < threshold {
                    neighbors.push(other.name.clone());
                }
            }
        }
        adjacency_list.push((country.name.clone(), neighbors));
    }

    adjacency_list
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "life expectancy.csv";
    let countries = load_and_clean_data(file_path)?;

    if countries.is_empty() {
        println!("No valid data found.");
        return Ok(());
    }

    // Step 1: Perform K-Means Clustering
    let k = 5;
    let max_iterations = 100;
    let clustered_countries = kmeans(countries.clone(), k, max_iterations);

    // Step 2: Build Graph Based on Threshold
    let threshold = 0.5;
    let graph = build_graph(&clustered_countries, threshold);

    println!("Graph connections:");
    for (country, neighbors) in graph {
        println!("{} -> {:?}", country, neighbors);
    }

    println!("\nClustered Results:");
    for country in clustered_countries {
        println!("{} - Cluster: {}", country.name, country.cluster.unwrap_or_default());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_and_clean_data() {
        let file_path = "life expectancy.csv";
        let result = load_and_clean_data(file_path);
        assert!(result.is_ok());
        let countries = result.unwrap();
        assert!(countries.len() > 0);
    }

    #[test]
    fn test_euclidean_distance() {
        let country1 = Country {
            name: "CountryA".to_string(),
            communicable: 10.0,
            non_communicable: 20.0,
            co2: 5.0,
            cluster: None,
        };

        let country2 = Country {
            name: "CountryB".to_string(),
            communicable: 15.0,
            non_communicable: 25.0,
            co2: 10.0,
            cluster: None,
        };

        let distance = euclidean_distance(&country1, &country2);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_kmeans() {
        let countries = vec![
            Country {
                name: "CountryA".to_string(),
                communicable: 10.0,
                non_communicable: 20.0,
                co2: 5.0,
                cluster: None,
            },
            Country {
                name: "CountryB".to_string(),
                communicable: 15.0,
                non_communicable: 25.0,
                co2: 10.0,
                cluster: None,
            },
            Country {
                name: "CountryC".to_string(),
                communicable: 30.0,
                non_communicable: 35.0,
                co2: 20.0,
                cluster: None,
            },
        ];

        let clustered_countries = kmeans(countries, 2, 10);
        assert!(clustered_countries.len() > 0);
        assert!(clustered_countries.iter().all(|c| c.cluster.is_some()));
    }

    #[test]
    fn test_build_graph() {
        let countries = vec![
            Country {
                name: "CountryA".to_string(),
                communicable: 10.0,
                non_communicable: 20.0,
                co2: 5.0,
                cluster: None,
            },
            Country {
                name: "CountryB".to_string(),
                communicable: 15.0,
                non_communicable: 25.0,
                co2: 10.0,
                cluster: None,
            },
        ];

        let graph = build_graph(&countries, 10.0);
        assert_eq!(graph.len(), 2);
    }
}