use std::collections::HashMap;
use anyhow::Result;

pub struct TSPSolver {
    distance_matrix: Vec<Vec<f64>>,
    n: usize,
    memo: HashMap<(usize, usize), f64>, // (mask, current_city) -> min_cost
    parent: HashMap<(usize, usize), usize>, // Path reconstruction
}

impl TSPSolver {
    pub fn new(distance_matrix: Vec<Vec<f64>>) -> Self {
        let n = distance_matrix.len();
        Self {
            distance_matrix,
            n,
            memo: HashMap::new(),
            parent: HashMap::new(),
        }
    }
    
    pub fn solve(&mut self, verbose: bool) -> Result<(f64, Vec<usize>)> {
        if self.n == 0 {
            return Ok((0.0, vec![]));
        }
        
        if self.n == 1 {
            return Ok((0.0, vec![0]));
        }
        
        println!("  • Initializing DP table for {} cities", self.n);
        
        // Mulai dari city 0
        let start_mask = 1;
        
        let min_cost = self.dp(start_mask, 0, verbose)?;
        let path = self.reconstruct_path(start_mask, 0)?;
        
        Ok((min_cost, path))
    }
    
    /// Dynamic Programming
    /// Mask: bitmask visited city
    fn dp(&mut self, mask: usize, current: usize, verbose: bool) -> Result<f64> {
        // Base case: Semua city visited
        if mask == (1 << self.n) - 1 {
            return Ok(self.distance_matrix[current][0]);
        }
        
        // Cek memoization
        if let Some(&cached_result) = self.memo.get(&(mask, current)) {
            return Ok(cached_result);
        }
        
        let mut min_cost = f64::INFINITY;
        let mut best_next = 0;
        
        // Visit unvisited city
        for next in 0..self.n {
            if mask & (1 << next) == 0 { // City not visited
                let new_mask = mask | (1 << next);
                let cost = self.distance_matrix[current][next] + 
                          self.dp(new_mask, next, verbose)?;
                
                if cost < min_cost {
                    min_cost = cost;
                    best_next = next;
                }
            }
        }
        
        // Memoize result
        self.memo.insert((mask, current), min_cost);
        self.parent.insert((mask, current), best_next);
        
        if verbose && self.count_bits(mask) <= 3 {
            println!("    • DP({:0width$b}, {}) = {:.1}", 
                    mask, current, min_cost, width = self.n);
        }
        
        Ok(min_cost)
    }
    
    /// Reconstruct optimal path
    fn reconstruct_path(&self, start_mask: usize, start_city: usize) -> Result<Vec<usize>> {
        let mut path = vec![start_city];
        let mut current_mask = start_mask;
        let mut current_city = start_city;
        
        while current_mask != (1 << self.n) - 1 {
            if let Some(&next_city) = self.parent.get(&(current_mask, current_city)) {
                path.push(next_city);
                current_mask |= 1 << next_city;
                current_city = next_city;
            } else {
                break;
            }
        }
        
        Ok(path)
    }
    
    /// Count number of set bits
    fn count_bits(&self, mut mask: usize) -> usize {
        let mut count = 0;
        while mask > 0 {
            count += mask & 1;
            mask >>= 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_small_tsp() {
        // Simple 3-city TSP
        let matrix = vec![
            vec![0.0, 10.0, 15.0],
            vec![10.0, 0.0, 20.0],
            vec![15.0, 20.0, 0.0],
        ];
        
        let mut solver = TSPSolver::new(matrix);
        let (cost, path) = solver.solve(false).unwrap();
    
        assert_eq!(cost, 45.0);
        assert_eq!(path, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_single_city() {
        let matrix = vec![vec![0.0]];
        let mut solver = TSPSolver::new(matrix);
        let (cost, path) = solver.solve(false).unwrap();
        
        assert_eq!(cost, 0.0);
        assert_eq!(path, vec![0]);
    }
}