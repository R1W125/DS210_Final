use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
// use std::path::PathBuf;
use crate::data_processing::process_data;
use crate::k_means_cluster::{k_means, Point};
use crate::visualization::{
    plot_borough_boundaries,
    plot_all_points,
    plot_centroids_with_boroughs,
    plot_complete_visualization,
    plot_by_sex
};

mod data_processing;
mod k_means_cluster;
mod visualization;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the current directory
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("data").join("NYPD_Shooting_Incident_Data__Historic_.csv");
    
    let mut all_points = Vec::new();
    let mut male_points = Vec::new();
    let mut female_points = Vec::new();
    let mut race_counts = HashMap::new();

    // Read and process the CSV file
    let file = File::open(&file_path)?;
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(line) = line {
            if let Some(data) = process_data(&line) {
                let point = Point {
                    x: data.longitude,
                    y: data.latitude,
                };
                
                all_points.push(point.clone());
                
                if data.victim_sex == "M" {
                    male_points.push(point);
                } else if data.victim_sex == "F" {
                    female_points.push(point);
                }

                // Count races
                *race_counts.entry(data.victim_race).or_insert(0) += 1;
            }
        }
    }

    // Print race statistics
    println!("\nVictim Race Statistics:");
    println!("----------------------");
    for (race, count) in &race_counts {
        println!("{}: {}", race, count);
    }

    // Perform k-means clustering
    let clusters = k_means(&all_points, 5, 100);

    // Generate all visualizations
    plot_borough_boundaries()?;
    plot_all_points(&all_points)?;
    plot_centroids_with_boroughs(&clusters)?;
    plot_complete_visualization(&clusters)?;
    plot_by_sex(&male_points, "M")?;
    plot_by_sex(&female_points, "F")?;

    // Print cluster information
    println!("\nCluster Information:");
    println!("-------------------");
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}: {} points", i, cluster.points.len());
        println!("Centroid: ({:.6}, {:.6})", cluster.centroid.x, cluster.centroid.y);
    }

    Ok(())
}

#[cfg(test)]

use crate::k_means_cluster::Cluster;
mod tests {
    use super::*;

    #[test]
    fn test_k_means_clustering() {
        // Create test points
        let points = vec![
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.1, y: 1.1 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 5.1, y: 5.1 },
        ];
        
        let k = 2;
        let clusters = k_means(&points, k, 100);
        
        assert_eq!(clusters.len(), k);
        assert!(clusters.iter().all(|c| !c.points.is_empty()));
        
        // Check that points are assigned to the closest cluster
        for point in &points {
            let mut min_distance = f64::MAX;
            for cluster in &clusters {
                let distance = ((point.x - cluster.centroid.x).powi(2) + 
                              (point.y - cluster.centroid.y).powi(2)).sqrt();
                min_distance = min_distance.min(distance);
            }
            assert!(min_distance < 1.0);
        }
    }

    #[test]
    fn test_visualization_functions() {
        // Test that visualization functions don't panic
        assert!(plot_borough_boundaries().is_ok());
        
        let test_points = vec![
            Point { x: -73.99, y: 40.75 },
            Point { x: -73.98, y: 40.76 },
        ];
        
        assert!(plot_all_points(&test_points).is_ok());
        
        let test_clusters = vec![
            Cluster {
                centroid: Point { x: -73.99, y: 40.75 },
                points: test_points.clone(),
            }
        ];
        
        assert!(plot_centroids_with_boroughs(&test_clusters).is_ok());
        assert!(plot_complete_visualization(&test_clusters).is_ok());
        assert!(plot_by_sex(&test_points, "M").is_ok());
    }
}