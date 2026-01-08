use crate::Rng;

/// Poisson disk sampling for even point distribution
pub struct PoissonDisk;

impl PoissonDisk {
    /// Generate evenly distributed points within bounds
    pub fn sample(width: usize, height: usize, min_dist: f64, seed: u64) -> Vec<(usize, usize)> {
        let mut rng = Rng::new(seed);
        let cell_size = min_dist / std::f64::consts::SQRT_2;
        let grid_w = (width as f64 / cell_size).ceil() as usize + 1;
        let grid_h = (height as f64 / cell_size).ceil() as usize + 1;
        
        let mut grid: Vec<Option<(f64, f64)>> = vec![None; grid_w * grid_h];
        let mut points = Vec::new();
        let mut active = Vec::new();
        
        // Start with random point
        let start = (rng.random() * width as f64, rng.random() * height as f64);
        let gi = (start.0 / cell_size) as usize;
        let gj = (start.1 / cell_size) as usize;
        grid[gj * grid_w + gi] = Some(start);
        points.push(start);
        active.push(start);
        
        while !active.is_empty() {
            let idx = rng.range_usize(0, active.len());
            let point = active[idx];
            let mut found = false;
            
            for _ in 0..30 {
                let angle = rng.random() * std::f64::consts::TAU;
                let dist = min_dist + rng.random() * min_dist;
                let nx = point.0 + angle.cos() * dist;
                let ny = point.1 + angle.sin() * dist;
                
                if nx < 0.0 || nx >= width as f64 || ny < 0.0 || ny >= height as f64 {
                    continue;
                }
                
                let gi = (nx / cell_size) as usize;
                let gj = (ny / cell_size) as usize;
                
                let mut valid = true;
                'outer: for dy in 0..=2 {
                    for dx in 0..=2 {
                        let ci = gi.saturating_sub(1) + dx;
                        let cj = gj.saturating_sub(1) + dy;
                        if ci < grid_w && cj < grid_h {
                            if let Some(p) = grid[cj * grid_w + ci] {
                                let d = ((nx - p.0).powi(2) + (ny - p.1).powi(2)).sqrt();
                                if d < min_dist {
                                    valid = false;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                
                if valid {
                    grid[gj * grid_w + gi] = Some((nx, ny));
                    points.push((nx, ny));
                    active.push((nx, ny));
                    found = true;
                    break;
                }
            }
            
            if !found {
                active.swap_remove(idx);
            }
        }
        
        points.into_iter().map(|(x, y)| (x as usize, y as usize)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poisson_generates_points() {
        let points = PoissonDisk::sample(50, 50, 5.0, 12345);
        assert!(!points.is_empty());
    }

    #[test]
    fn poisson_respects_min_distance() {
        let points = PoissonDisk::sample(50, 50, 8.0, 12345);
        for (i, &p1) in points.iter().enumerate() {
            for &p2 in points.iter().skip(i + 1) {
                let d = (((p1.0 as f64 - p2.0 as f64).powi(2) 
                    + (p1.1 as f64 - p2.1 as f64).powi(2)).sqrt());
                assert!(d >= 7.0, "Points too close: {:?} {:?} dist {}", p1, p2, d);
            }
        }
    }

    #[test]
    fn poisson_deterministic() {
        let p1 = PoissonDisk::sample(50, 50, 5.0, 12345);
        let p2 = PoissonDisk::sample(50, 50, 5.0, 12345);
        assert_eq!(p1, p2);
    }
}
