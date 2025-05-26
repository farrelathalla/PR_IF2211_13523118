use clap::Parser;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use thiserror::Error;

mod tsp_solver;
mod input_parser;
mod visualizer;

use tsp_solver::TSPSolver;
use input_parser::InputParser;
use visualizer::Visualizer;

#[derive(Parser)]
#[command(name = "tsp-solver")]
#[command(about = "A TSP solver using dynamic programming")]
struct Args {
    /// Input file name
    #[arg(short, long)]
    input: String,
    
    /// Output file name
    #[arg(short, long, default_value = "tsp_solution")]
    output: String,
    
    /// Show steps
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Error, Debug)]
pub enum TSPError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid input format: {0}")]
    InvalidFormat(String),
    
    #[error("Graph validation error: {0}")]
    GraphValidation(String),
    
    #[error("Solver error: {0}")]
    SolverError(String),
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("TSP Solver with Dynamic Programming");
    println!("=====================================");
    
    // Validate input file path
    let input_path = format!("input/{}", args.input);
    if !Path::new(&input_path).exists() {
        return Err(TSPError::FileNotFound(input_path.clone()).into());
    }
    
    println!("Reading input file: {}", input_path);
    
    // Parse input file
    let content = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read file: {}", input_path))?;
    
    let mut parser = InputParser::new();
    let (cities, distance_matrix) = parser.parse(&content)
        .map_err(|e| TSPError::InvalidFormat(e.to_string()))?;
    
    println!("Successfully parsed {} cities", cities.len());
    
    // Validate input
    validate_input(&cities, &distance_matrix)?;
    
    if args.verbose {
        print_input_summary(&cities, &distance_matrix);
    }
    
    // Solve TSP using dynamic programming
    println!("Solving TSP using Dynamic Programming...");
    let mut solver = TSPSolver::new(distance_matrix);
    let (min_cost, path) = solver.solve(args.verbose)?;
    
    // Display results
    println!("\nSolution Found!");
    println!("==================");
    println!("Minimum cost: {}", min_cost);
    print!("Optimal path: ");
    for (i, &city_idx) in path.iter().enumerate() {
        if i > 0 { print!(" -> "); }
        print!("{}", cities[city_idx]);
    }
    print!(" -> {}", cities[path[0]]); // Return to start
    println!();
    
    // Generate visualization
    println!("\nGenerating visualization...");
    
    // Generate filename
    let output_filename = generate_unique_filename(&args.output)?;
    
    let visualizer = Visualizer::new();
    visualizer.create_visualization(&cities, &path, min_cost, &output_filename)
        .with_context(|| "Failed to create visualization")?;
    
    println!("Visualization saved to: {}", output_filename);
    println!("\nTSP solving completed successfully!");
    
    Ok(())
}

fn generate_unique_filename(base_name: &str) -> Result<String> {
    let output_dir = "output";
    
    // Check if output directory exists
    if !Path::new(output_dir).exists() {
        return Err(TSPError::FileNotFound(
            "Output directory not found. Please create 'output' folder first.".to_string()
        ).into());
    }
    
    // Try the base name first
    let first_attempt = format!("{}/{}.png", output_dir, base_name);
    if !Path::new(&first_attempt).exists() {
        return Ok(first_attempt);
    }
    
    // If base name exists, try with incrementing numbers
    let mut counter = 1;
    loop {
        let filename = format!("{}/{}_{}.png", output_dir, base_name, counter);
        if !Path::new(&filename).exists() {
            return Ok(filename);
        }
        counter += 1;
        
        // Safety check to prevent infinite loop
        if counter > 9999 {
            return Err(TSPError::SolverError(
                "Too many output files, please clean up the output directory".to_string()
            ).into());
        }
    }
}

fn validate_input(cities: &[String], matrix: &[Vec<f64>]) -> Result<()> {
    let n = cities.len();
    
    // Check minimum number of cities
    if n < 2 {
        return Err(TSPError::GraphValidation(
            "At least 2 cities are required".to_string()
        ).into());
    }
    
    // Check maximum number of cities (for performance)
    if n > 20 {
        return Err(TSPError::GraphValidation(
            "Maximum 20 cities supported (due to exponential complexity)".to_string()
        ).into());
    }
    
    // Check matrix dimensions
    if matrix.len() != n {
        return Err(TSPError::GraphValidation(
            format!("Distance matrix rows ({}) don't match cities count ({})", 
                   matrix.len(), n)
        ).into());
    }
    
    for (i, row) in matrix.iter().enumerate() {
        if row.len() != n {
            return Err(TSPError::GraphValidation(
                format!("Distance matrix row {} has {} columns, expected {}", 
                       i, row.len(), n)
            ).into());
        }
        
        // Check diagonal is zero
        if matrix[i][i] != 0.0 {
            return Err(TSPError::GraphValidation(
                format!("Distance from city {} to itself should be 0", i)
            ).into());
        }
        
        // Check for negative distances
        for (j, &dist) in row.iter().enumerate() {
            if dist < 0.0 {
                return Err(TSPError::GraphValidation(
                    format!("Negative distance found between cities {} and {}", i, j)
                ).into());
            }
        }
    }
    
    println!("✅ Input validation passed");
    Ok(())
}

fn print_input_summary(cities: &[String], matrix: &[Vec<f64>]) {
    println!("\n📋 Input Summary:");
    println!("Cities: {:?}", cities);
    println!("Distance Matrix:");
    for (i, row) in matrix.iter().enumerate() {
        print!("  {}: ", cities[i]);
        for (j, &dist) in row.iter().enumerate() {
            if j > 0 { print!(", "); }
            print!("{:6.1}", dist);
        }
        println!();
    }
}