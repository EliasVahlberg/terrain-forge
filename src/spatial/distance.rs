//! Distance transform algorithms for spatial analysis

use crate::{Cell, Grid};
use std::collections::VecDeque;

/// Distance metrics for spatial calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceMetric {
    /// Euclidean distance (straight line)
    Euclidean,
    /// Manhattan distance (taxicab)
    Manhattan,
    /// Chebyshev distance (chessboard)
    Chebyshev,
}

/// Distance transform result
#[derive(Debug, Clone)]
pub struct DistanceTransform {
    distances: Vec<f32>,
    width: usize,
    height: usize,
}

impl DistanceTransform {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            distances: vec![f32::INFINITY; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.distances[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, distance: f32) {
        self.distances[y * self.width + x] = distance;
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

/// Generate distance field from passable cells
pub fn distance_field<C: Cell>(grid: &Grid<C>, metric: DistanceMetric) -> DistanceTransform {
    let mut transform = DistanceTransform::new(grid.width(), grid.height());
    let mut queue = VecDeque::new();

    // Initialize with passable cells as distance 0
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(cell) = grid.get(x as i32, y as i32) {
                if cell.is_passable() {
                    transform.set(x, y, 0.0);
                    queue.push_back((x, y));
                }
            }
        }
    }

    // Propagate distances
    while let Some((x, y)) = queue.pop_front() {
        let current_dist = transform.get(x, y);

        for (dx, dy) in neighbors(metric) {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.width() && (ny as usize) < grid.height() {
                let nx = nx as usize;
                let ny = ny as usize;

                let step_dist = match metric {
                    DistanceMetric::Euclidean => ((dx * dx + dy * dy) as f32).sqrt(),
                    DistanceMetric::Manhattan => (dx.abs() + dy.abs()) as f32,
                    DistanceMetric::Chebyshev => dx.abs().max(dy.abs()) as f32,
                };

                let new_dist = current_dist + step_dist;
                if new_dist < transform.get(nx, ny) {
                    transform.set(nx, ny, new_dist);
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    transform
}

fn neighbors(metric: DistanceMetric) -> &'static [(i32, i32)] {
    match metric {
        DistanceMetric::Manhattan => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
        DistanceMetric::Euclidean | DistanceMetric::Chebyshev => &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
    }
}
