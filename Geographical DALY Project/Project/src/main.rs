
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs;
fn parse_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|s| s.to_string()).collect());
    }

    Ok(data)
}
fn clean_data(data: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut cleaned_data = Vec::new();
    for row in data {
        if row.iter().all(|val| !val.trim().is_empty()) {
            cleaned_data.push(row);
        }
    }

    cleaned_data
}
fn normalize_data(data: &mut Vec<Vec<String>>) {
    if data.is_empty() {
        return;
    }
    let num_cols = data[0].len();
    let mut min_vals = vec![f64::MAX; num_cols];
    let mut max_vals = vec![f64::MIN; num_cols];
    for row in data.iter() {
        for (i, val) in row.iter().enumerate() {
            if let Ok(num) = val.parse::<f64>() {
                if num < min_vals[i] {
                    min_vals[i] = num;
                }
                if num > max_vals[i] {
                    max_vals[i] = num;
                }
            }
        }
    }
    for row in data.iter_mut() {
        for (i, val) in row.iter_mut().enumerate() {
            if let Ok(num) = val.parse::<f64>() {
                let range = max_vals[i] - min_vals[i];
                let normalized = if range == 0.0 { 0.0 } else { (num - min_vals[i]) / range };
                *val = normalized.to_string();
            }
        }
    }
}
fn write_cleaned_csv(data: Vec<Vec<String>>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_path(output_path)?;

    for row in data {
        wtr.write_record(row)?;
    }

    wtr.flush()?;
    Ok(())
}
fn build_graph(data: Vec<Vec<String>>, threshold: f64) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; data.len()];

    for i in 0..data.len() {
        for j in 0..data.len() {
            if i != j {
                let distance = calculate_distance(&data[i], &data[j]);
                if distance < threshold {
                    graph[i].push(j);
                }
            }
        }
    }

    graph
}

fn calculate_distance(row1: &Vec<String>, row2: &Vec<String>) -> f64 {
    row1.iter()
        .zip(row2.iter())
        .filter_map(|(val1, val2)| {
            if let (Ok(num1), Ok(num2)) = (val1.parse::<f64>(), val2.parse::<f64>()) {
                Some((num1 - num2).powi(2))
            } else {
                None
            }
        })
        .sum::<f64>()
        .sqrt()
}

fn main() -> Result<(), Box<dyn Error>> {

    let input_file = "life expectancy.csv";
    let output_file = "cleaned_life_expectancy.csv";


    let mut data = parse_csv(input_file)?;
    

    data = clean_data(data);

    normalize_data(&mut data);
    

    write_cleaned_csv(data.clone(), output_file)?;
    println!("Data cleaned and saved to {}", output_file);


    let threshold = 0.5; 
    let graph = build_graph(data, threshold);
    println!("Graph built with {} nodes.", graph.len());

    // Placeholder for K-Means clustering
    
    Ok(())
}
