//! Advanced pathfinding algorithms

use crate::{Cell, Grid};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Dijkstra map for multi-goal pathfinding
#[derive(Debug, Clone)]
pub struct DijkstraMap {
    costs: Vec<f32>,
    width: usize,
    height: usize,
}

impl DijkstraMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            costs: vec![f32::INFINITY; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.costs[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, cost: f32) {
        self.costs[y * self.width + x] = cost;
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

/// Flow field for AI movement
#[derive(Debug, Clone)]
pub struct FlowField {
    directions: Vec<(i32, i32)>,
    width: usize,
    height: usize,
}

impl FlowField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            directions: vec![(0, 0); width * height],
            width,
            height,
        }
    }

    pub fn get_direction(&self, x: usize, y: usize) -> (i32, i32) {
        self.directions[y * self.width + x]
    }

    pub fn set_direction(&mut self, x: usize, y: usize, dir: (i32, i32)) {
        self.directions[y * self.width + x] = dir;
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

/// Pathfinding constraints
#[derive(Debug, Clone)]
pub struct PathfindingConstraints {
    pub movement_cost: HashMap<(i32, i32), f32>,
    pub blocked_cells: Vec<(usize, usize)>,
}

impl Default for PathfindingConstraints {
    fn default() -> Self {
        let mut movement_cost = HashMap::new();
        // Standard 8-directional movement
        movement_cost.insert((-1, 0), 1.0);
        movement_cost.insert((1, 0), 1.0);
        movement_cost.insert((0, -1), 1.0);
        movement_cost.insert((0, 1), 1.0);
        movement_cost.insert((-1, -1), 1.414);
        movement_cost.insert((-1, 1), 1.414);
        movement_cost.insert((1, -1), 1.414);
        movement_cost.insert((1, 1), 1.414);

        Self {
            movement_cost,
            blocked_cells: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    cost: f32,
    x: usize,
    y: usize,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Generate Dijkstra map from multiple goals
pub fn dijkstra_map<C: Cell>(
    grid: &Grid<C>,
    goals: &[(usize, usize)],
    constraints: &PathfindingConstraints,
) -> DijkstraMap {
    let mut map = DijkstraMap::new(grid.width(), grid.height());
    let mut heap = BinaryHeap::new();

    // Initialize goals with cost 0
    for &(x, y) in goals {
        map.set(x, y, 0.0);
        heap.push(Node { cost: 0.0, x, y });
    }

    while let Some(Node { cost, x, y }) = heap.pop() {
        if cost > map.get(x, y) {
            continue;
        }

        for (&(dx, dy), &move_cost) in &constraints.movement_cost {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.width() && (ny as usize) < grid.height() {
                let nx = nx as usize;
                let ny = ny as usize;

                if constraints.blocked_cells.contains(&(nx, ny)) {
                    continue;
                }

                if let Some(cell) = grid.get(nx as i32, ny as i32) {
                    if !cell.is_passable() {
                        continue;
                    }
                }

                let new_cost = cost + move_cost;
                if new_cost < map.get(nx, ny) {
                    map.set(nx, ny, new_cost);
                    heap.push(Node {
                        cost: new_cost,
                        x: nx,
                        y: ny,
                    });
                }
            }
        }
    }

    map
}

/// Find a shortest path between two points using a Dijkstra cost map.
pub fn shortest_path<C: Cell>(
    grid: &Grid<C>,
    start: (usize, usize),
    end: (usize, usize),
    constraints: &PathfindingConstraints,
) -> Option<Vec<(usize, usize)>> {
    if !grid.in_bounds(start.0 as i32, start.1 as i32)
        || !grid.in_bounds(end.0 as i32, end.1 as i32)
    {
        return None;
    }

    if constraints.blocked_cells.contains(&start) || constraints.blocked_cells.contains(&end) {
        return None;
    }

    if !grid
        .get(start.0 as i32, start.1 as i32)
        .is_some_and(|cell| cell.is_passable())
    {
        return None;
    }

    if !grid
        .get(end.0 as i32, end.1 as i32)
        .is_some_and(|cell| cell.is_passable())
    {
        return None;
    }

    let dijkstra = dijkstra_map(grid, &[end], constraints);
    if dijkstra.get(start.0, start.1) == f32::INFINITY {
        return None;
    }

    if start == end {
        return Some(vec![start]);
    }

    let mut path = vec![start];
    let mut current = start;
    let mut remaining_steps = grid.width() * grid.height();

    while current != end && remaining_steps > 0 {
        let (x, y) = current;
        let current_cost = dijkstra.get(x, y);
        let mut best = None;
        let mut best_cost = current_cost;

        for &(dx, dy) in constraints.movement_cost.keys() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if !grid.in_bounds(nx, ny) {
                continue;
            }

            let npos = (nx as usize, ny as usize);
            if constraints.blocked_cells.contains(&npos) {
                continue;
            }

            if !grid.get(nx, ny).is_some_and(|cell| cell.is_passable()) {
                continue;
            }

            let cost = dijkstra.get(npos.0, npos.1);
            if cost < best_cost {
                best_cost = cost;
                best = Some(npos);
            }
        }

        let next = best?;
        path.push(next);
        current = next;
        remaining_steps = remaining_steps.saturating_sub(1);
    }

    if current == end {
        Some(path)
    } else {
        None
    }
}

/// Generate flow field from Dijkstra map
pub fn flow_field_from_dijkstra(dijkstra: &DijkstraMap) -> FlowField {
    let mut flow = FlowField::new(dijkstra.width(), dijkstra.height());

    for y in 0..dijkstra.height() {
        for x in 0..dijkstra.width() {
            let current_cost = dijkstra.get(x, y);
            if current_cost == f32::INFINITY {
                continue;
            }

            let mut best_dir = (0, 0);
            let mut best_cost = current_cost;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0
                        && ny >= 0
                        && (nx as usize) < dijkstra.width()
                        && (ny as usize) < dijkstra.height()
                    {
                        let neighbor_cost = dijkstra.get(nx as usize, ny as usize);
                        if neighbor_cost < best_cost {
                            best_cost = neighbor_cost;
                            best_dir = (dx, dy);
                        }
                    }
                }
            }

            flow.set_direction(x, y, best_dir);
        }
    }

    flow
}
