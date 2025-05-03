//This module is used to perform k-means clustering

// Point struct contains the latitude and longitude
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Cluster struct contains the centroid and the points
#[derive(Debug)]
pub struct Cluster {
    pub centroid: Point,
    pub points: Vec<Point>,
}

// Calculates the distance between two points
impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Performs k-means clustering
pub fn k_means(points: &[Point], k: usize, max_iterations: usize) -> Vec<Cluster> {
    if points.is_empty() || k == 0 {
        return Vec::new();
    }

    // Initialize centroids randomly
    let mut centroids: Vec<Point> = points
        .iter()
        .take(k)
        .cloned()
        .collect();

    let mut clusters: Vec<Cluster> = Vec::new();

    for _ in 0..max_iterations {
        // Clear previous clusters
        clusters.clear();

        // Initialize empty clusters
        for centroid in &centroids {
            clusters.push(Cluster {
                centroid: centroid.clone(),
                points: Vec::new(),
            });
        }

        // Assign points to nearest centroid
        for point in points {
            let mut min_distance = f64::INFINITY;
            let mut nearest_cluster = 0;

            for (i, centroid) in centroids.iter().enumerate() {
                let distance = point.distance(centroid);
                if distance < min_distance {
                    min_distance = distance;
                    nearest_cluster = i;
                }
            }

            clusters[nearest_cluster].points.push(point.clone());
        }

        // Update centroids
        let mut converged = true;
        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.points.is_empty() {
                continue;
            }

            let sum_x: f64 = cluster.points.iter().map(|p| p.x).sum();
            let sum_y: f64 = cluster.points.iter().map(|p| p.y).sum();
            let count = cluster.points.len() as f64;

            let new_centroid = Point {
                x: sum_x / count,
                y: sum_y / count,
            };

            if new_centroid.distance(&centroids[i]) > 0.0001 {
                converged = false;
            }

            centroids[i] = new_centroid;
        }

        if converged {
            break;
        }
    }

    // Create final clusters with updated centroids
    clusters.clear();
    for centroid in centroids {
        clusters.push(Cluster {
            centroid,
            points: Vec::new(),
        });
    }

    // Final assignment of points
    for point in points {
        let mut min_distance = f64::INFINITY;
        let mut nearest_cluster = 0;

        for (i, cluster) in clusters.iter().enumerate() {
            let distance = point.distance(&cluster.centroid);
            if distance < min_distance {
                min_distance = distance;
                nearest_cluster = i;
            }
        }

        clusters[nearest_cluster].points.push(point.clone());
    }

    clusters
} 