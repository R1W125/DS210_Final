// Make visualizations in here
use plotters::prelude::*;
use crate::k_means_cluster::{Cluster, Point};
//use std::ops::Range;
// use plotters::coord::types::RangedCoordf64;
// use plotters::style::text_anchor::{HPos, VPos};

const OUT_FILE_NAME: &str = "nyc_crime_analysis.png";
const ORIGINAL_POINTS_FILE: &str = "nyc_original_points.png";
const PLOT_RES: (u32, u32) = (1200, 800);

const BOROUGH_BOUNDARIES: &[(f64, f64)] = &[
    // Manhattan
    (-74.02, 40.70), (-73.99, 40.70), (-73.99, 40.80), (-74.02, 40.80), (-74.02, 40.70),
    // Brooklyn
    (-74.02, 40.60), (-73.99, 40.60), (-73.99, 40.70), (-74.02, 40.70), (-74.02, 40.60),
    // Queens
    (-73.99, 40.70), (-73.80, 40.70), (-73.80, 40.80), (-73.99, 40.80), (-73.99, 40.70),
    // Bronx
    (-73.99, 40.80), (-73.80, 40.80), (-73.80, 40.90), (-73.99, 40.90), (-73.99, 40.80),
    // Staten Island
    (-74.20, 40.50), (-74.02, 40.50), (-74.02, 40.60), (-74.20, 40.60), (-74.20, 40.50),
];

pub fn plot_clusters_with_boroughs(clusters: &[Cluster]) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area
    let root = BitMapBackend::new(OUT_FILE_NAME, PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    // Define NYC map boundaries
    let min_x = -74.3;
    let max_x = -73.7;
    let min_y = 40.5;
    let max_y = 40.9;

    // Create the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("NYC Crime Clusters", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    // Configure the chart
    chart
        .configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    // Colors for different clusters
    let cluster_colors = [
        RGBColor(255, 0, 0),    // Red
        RGBColor(0, 0, 255),    // Blue
        RGBColor(0, 255, 0),    // Green
        RGBColor(255, 0, 255),  // Magenta
        RGBColor(0, 255, 255),  // Cyan
        RGBColor(255, 255, 0),  // Yellow
        RGBColor(128, 0, 0),    // Dark Red
        RGBColor(0, 0, 128),    // Dark Blue
        RGBColor(0, 128, 0),    // Dark Green
        RGBColor(128, 0, 128),  // Dark Magenta
    ];

    // Plot each cluster centroid with a different color
    for (i, cluster) in clusters.iter().enumerate() {
        let color = &cluster_colors[i % cluster_colors.len()];

        // Plot centroid with a larger marker
        chart.draw_series(std::iter::once(
            Circle::new(
                (cluster.centroid.x, cluster.centroid.y),
                10,  // Increased size for better visibility
                color.filled(),
            )
        ))?;

        // Add cluster label with size information
        chart.draw_series(std::iter::once(
            Text::new(
                format!("Cluster {} ({})", i, cluster.points.len()),
                (cluster.centroid.x, cluster.centroid.y + 0.01),
                ("sans-serif", 15).into_font().color(color),
            )
        ))?;
    }

    root.present()?;

    println!("Plot has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

pub fn plot_borough_boundaries() -> Result<(), Box<dyn std::error::Error>> {
    // Approximate borough boundaries (simplified for visualization)
    let manhattan = vec![
        Point { x: -74.0479, y: 40.6829 },
        Point { x: -73.9067, y: 40.8783 },
        Point { x: -73.9261, y: 40.7914 },
        Point { x: -74.0196, y: 40.7024 },
    ];

    let brooklyn = vec![
        Point { x: -74.0421, y: 40.5698 },
        Point { x: -73.8334, y: 40.5819 },
        Point { x: -73.8890, y: 40.7391 },
        Point { x: -74.0421, y: 40.5698 },
    ];

    // Create a drawing area
    let root = BitMapBackend::new("borough_boundaries.png", PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("NYC Borough Boundaries", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.1f64..-73.7f64, 40.5f64..40.9f64)?;

    chart
        .configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    // Draw borough boundaries
    chart.draw_series(LineSeries::new(
        manhattan.iter().map(|p| (p.x, p.y)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        brooklyn.iter().map(|p| (p.x, p.y)),
        &BLUE,
    ))?;

    root.present()?;

    println!("Borough boundaries have been saved to borough_boundaries.png");
    Ok(())
}

pub fn plot_original_points_with_boroughs(clusters: &[Cluster]) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area
    let root = BitMapBackend::new(ORIGINAL_POINTS_FILE, PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    // Define NYC map boundaries
    let min_x = -74.3;
    let max_x = -73.7;
    let min_y = 40.5;
    let max_y = 40.9;

    // Create the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("NYC Crime Distribution", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    // Configure the chart
    chart
        .configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    // Plot all original points in black
    let all_points: Vec<Point> = clusters.iter()
        .flat_map(|cluster| cluster.points.iter().cloned())
        .collect();

    chart.draw_series(all_points.iter().map(|point| {
        Circle::new((point.x, point.y), 2, BLACK.filled())
    }))?;

    root.present()?;

    println!("Original points plot has been saved to {}", ORIGINAL_POINTS_FILE);
    Ok(())
}

pub fn plot_male_victims_with_boroughs(clusters: &[Cluster]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("nyc_male_victims.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("NYC Male Crime Victims", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3..-73.7, 40.4..40.9)?;

    chart.configure_mesh().draw()?;

    // Draw borough boundaries
    chart.draw_series(LineSeries::new(
        BOROUGH_BOUNDARIES.iter().map(|&(x, y)| (x, y)),
        BLACK,
    ))?;

    // Plot male victims
    for cluster in clusters {
        let points: Vec<(f64, f64)> = cluster.points.iter()
            .map(|p| (p.x, p.y))
            .collect();
        chart.draw_series(
            points.iter().map(|&(x, y)| Circle::new((x, y), 2, BLUE.filled())),
        )?;
    }

    Ok(())
}

pub fn plot_female_victims_with_boroughs(clusters: &[Cluster]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("nyc_female_victims.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("NYC Female Crime Victims", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3..-73.7, 40.4..40.9)?;

    chart.configure_mesh().draw()?;

    // Draw borough boundaries
    chart.draw_series(LineSeries::new(
        BOROUGH_BOUNDARIES.iter().map(|&(x, y)| (x, y)),
        BLACK,
    ))?;

    // Plot female victims
    for cluster in clusters {
        let points: Vec<(f64, f64)> = cluster.points.iter()
            .map(|p| (p.x, p.y))
            .collect();
        chart.draw_series(
            points.iter().map(|&(x, y)| Circle::new((x, y), 2, RED.filled())),
        )?;
    }

    Ok(())
}

