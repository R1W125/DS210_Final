//This module is used to visualize the data
// It will be used to generate all the graphs


use plotters::prelude::*;
use plotters::coord::types::RangedCoordf64;
use crate::k_means_cluster::{Cluster, Point};

const PLOT_RES: (u32, u32) = (1200, 800);

// Exact NYC borough boundaries
const BOROUGH_BOUNDARIES: &[&[(f64, f64)]] = &[
    // Manhattan
    &[
        (40.877820, -73.926106),
        (40.872528, -73.909369),
        (40.833169, -73.935539),
        (40.808328, -73.934627),
        (40.795398, -73.914542),
        (40.738806, -73.978413),
        (40.711648, -73.978413),
        (40.702274, -74.016015),
        (40.751425, -74.009706),
        (40.877820, -73.926106),
    ],
    // Brooklyn
    &[
        (40.738819, -73.957306),
        (40.684611, -73.896915),
        (40.695155, -73.869104),
        (40.647242, -73.854801),
        (40.576967, -73.933467),
        (40.576364, -74.012134),
        (40.625232, -74.043124),
        (40.679790, -74.019683),
        (40.738819, -73.957306),
    ],
    // Queens
    &[
        (40.741221, -73.961788),
        (40.802582, -73.821026),
        (40.743822, -73.699489),
        (40.624062, -73.772274),
        (40.685011, -73.895183),
        (40.741221, -73.961788),
    ],
    // Bronx
    &[
        (40.879671, -73.924364),
        (40.913354, -73.910288),
        (40.882916, -73.793358),
        (40.804728, -73.791762),
        (40.797152, -73.912367),
        (40.811720, -73.932639),
        (40.871628, -73.908951),
        (40.879671, -73.924364),
    ],
    // Staten Island
    &[
        (40.643235, -74.184901),
        (40.648125, -74.081784),
        (40.602199, -74.055646),
        (40.500371, -74.252328),
        (40.546648, -74.242802),
        (40.643235, -74.184901),
    ],
];

// Helper function to draw borough boundaries
fn draw_boroughs<DB: DrawingBackend>(chart: &mut ChartContext<DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    // Draw borough boundaries
    for boundary in BOROUGH_BOUNDARIES {
        chart.draw_series(LineSeries::new(
            boundary.iter().map(|&(lat, lon)| (lon, lat)),
            BLACK,
        ))?;
    }
    Ok(())
}

// Plots the borough boundaries
pub fn plot_borough_boundaries() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("borough_boundaries.png", PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("NYC Borough Boundaries", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3f64..-73.7f64, 40.4f64..40.9f64)?;

    chart.configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    draw_boroughs(&mut chart)?;

    root.present()?;
    println!("Borough boundaries plot saved to borough_boundaries.png");
    Ok(())
}

// Plots all the points
pub fn plot_all_points(points: &[Point]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("all_points.png", PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("All Crime Locations", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3f64..-73.7f64, 40.4f64..40.9f64)?;

    chart.configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    draw_boroughs(&mut chart)?;

    // Plot all points
    chart.draw_series(points.iter().map(|point| {
        Circle::new((point.x, point.y), 2, BLACK.filled())
    }))?;

    root.present()?;
    println!("All points plot saved to all_points.png");
    Ok(())
}

// Plots the centroids with the borough boundaries
pub fn plot_centroids_with_boroughs(clusters: &[Cluster]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("centroids.png", PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Crime Centroids", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3f64..-73.7f64, 40.4f64..40.9f64)?;

    chart.configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    draw_boroughs(&mut chart)?;

    // Plot centroids
    for (i, cluster) in clusters.iter().enumerate() {
        chart.draw_series(std::iter::once(
            Circle::new(
                (cluster.centroid.x, cluster.centroid.y),
                10,
                RED.filled(),
            )
        ))?;

        chart.draw_series(std::iter::once(
            Text::new(
                format!("Cluster {}", i),
                (cluster.centroid.x, cluster.centroid.y + 0.01),
                ("sans-serif", 15).into_font().color(&RED),
            )
        ))?;
    }

    root.present()?;
    println!("Centroids plot saved to centroids.png");
    Ok(())
}

// Plots the complete visualization
pub fn plot_complete_visualization(clusters: &[Cluster]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("complete_visualization.png", PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Complete Crime Analysis", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3f64..-73.7f64, 40.4f64..40.9f64)?;

    chart.configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    draw_boroughs(&mut chart)?;

    // Plot all points with cluster colors
    let colors = [RED, BLUE, GREEN, MAGENTA, CYAN, YELLOW];
    for (i, cluster) in clusters.iter().enumerate() {
        let color = colors[i % colors.len()];
        chart.draw_series(cluster.points.iter().map(|point| {
            Circle::new((point.x, point.y), 2, color.filled())
        }))?;

        // Plot centroid
        chart.draw_series(std::iter::once(
            Circle::new(
                (cluster.centroid.x, cluster.centroid.y),
                10,
                color.filled(),
            )
        ))?;

        // Add cluster label
        chart.draw_series(std::iter::once(
            Text::new(
                format!("Cluster {} ({})", i, cluster.points.len()),
                (cluster.centroid.x, cluster.centroid.y + 0.05),
                ("sans-serif", 15).into_font().color(&color),
            )
        ))?;
    }

    root.present()?;
    println!("Complete visualization saved to complete_visualization.png");
    Ok(())
}

// Plots the points by sex  
pub fn plot_by_sex(points: &[Point], victim_sex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}_victims.png", victim_sex.to_lowercase());
    let root = BitMapBackend::new(&filename, PLOT_RES).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} Crime Victims", victim_sex), ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-74.3f64..-73.7f64, 40.4f64..40.9f64)?;

    chart.configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    draw_boroughs(&mut chart)?;

    // Plot points
    let color = if victim_sex == "M" { BLUE } else { RED };
    chart.draw_series(points.iter().map(|point| {
        Circle::new((point.x, point.y), 2, color.filled())
    }))?;

    root.present()?;
    println!("{} victims plot saved to {}", victim_sex, filename);
    Ok(())
} 