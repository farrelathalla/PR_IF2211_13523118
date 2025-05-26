use anyhow::{Result, anyhow};

pub struct InputParser;

impl InputParser {
    pub fn new() -> Self {
        Self
    }
    
    /// Parse input file content
    pub fn parse(&mut self, content: &str) -> Result<(Vec<String>, Vec<Vec<f64>>)> {
        let lines: Vec<&str> = content.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();
        
        if lines.is_empty() {
            return Err(anyhow!("Empty input file"));
        }
        
        if self.is_matrix_format(&lines) {
            self.parse_matrix_format(&lines)
        } else {
            self.parse_list_format(&lines)
        }
    }
    
    /// Check input
    fn is_matrix_format(&self, lines: &[&str]) -> bool {
        if lines.is_empty() {
            return false;
        }
        
        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();

        first_line_parts.len() > 1 && 
        first_line_parts.iter().any(|part| part.parse::<f64>().is_err())
    }
    
    fn parse_matrix_format(&self, lines: &[&str]) -> Result<(Vec<String>, Vec<Vec<f64>>)> {
        if lines.len() < 2 {
            return Err(anyhow!("Matrix format requires at least 2 lines"));
        }
        
        let cities: Vec<String> = lines[0]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        let n = cities.len();
        if n == 0 {
            return Err(anyhow!("No cities found in first line"));
        }

        let mut matrix = Vec::with_capacity(n);
        
        for (i, line) in lines[1..].iter().enumerate() {
            if i >= n {
                break;
            }
            
            let row: Result<Vec<f64>, _> = line
                .split_whitespace()
                .map(|s| s.parse::<f64>())
                .collect();
            
            let row = row.map_err(|_| anyhow!("Invalid number in matrix row {}", i + 1))?;
            
            if row.len() != n {
                return Err(anyhow!(
                    "Matrix row {} has {} columns, expected {}", 
                    i + 1, row.len(), n
                ));
            }
            
            matrix.push(row);
        }
        
        if matrix.len() != n {
            return Err(anyhow!(
                "Matrix has {} rows, expected {}", 
                matrix.len(), n
            ));
        }
        
        Ok((cities, matrix))
    }

    fn parse_list_format(&self, lines: &[&str]) -> Result<(Vec<String>, Vec<Vec<f64>>)> {
        let mut matrix_start = 0;
        
        for (i, line) in lines.iter().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.iter().all(|part| part.parse::<f64>().is_ok()) {
                matrix_start = i;
                break;
            }
        }
        
        if matrix_start == 0 {
            return Err(anyhow!("Could not find distance matrix in input"));
        }
        
        let cities: Vec<String> = lines[..matrix_start].iter()
            .map(|line| line.to_string())
            .collect();
        
        let n = cities.len();
        if n == 0 {
            return Err(anyhow!("No cities found"));
        }
        
        // Parse matrix
        let matrix_lines = &lines[matrix_start..];
        let mut matrix = Vec::with_capacity(n);
        
        for (i, line) in matrix_lines.iter().enumerate() {
            if i >= n {
                break;
            }
            
            let row: Result<Vec<f64>, _> = line
                .split_whitespace()
                .map(|s| s.parse::<f64>())
                .collect();
            
            let row = row.map_err(|_| anyhow!("Invalid number in matrix row {}", i + 1))?;
            
            if row.len() != n {
                return Err(anyhow!(
                    "Matrix row {} has {} columns, expected {}", 
                    i + 1, row.len(), n
                ));
            }
            
            matrix.push(row);
        }
        
        if matrix.len() != n {
            return Err(anyhow!(
                "Matrix has {} rows, expected {}", 
                matrix.len(), n
            ));
        }
        
        Ok((cities, matrix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matrix_format() {
        let input = "A B C\n0 10 15\n10 0 20\n15 20 0";
        let mut parser = InputParser::new();
        let (cities, matrix) = parser.parse(input).unwrap();
        
        assert_eq!(cities, vec!["A", "B", "C"]);
        assert_eq!(matrix[0], vec![0.0, 10.0, 15.0]);
        assert_eq!(matrix[1], vec![10.0, 0.0, 20.0]);
        assert_eq!(matrix[2], vec![15.0, 20.0, 0.0]);
    }
    
    #[test]
    fn test_list_format() {
        let input = "A\nB\nC\n0 10 15\n10 0 20\n15 20 0";
        let mut parser = InputParser::new();
        let (cities, matrix) = parser.parse(input).unwrap();
        
        assert_eq!(cities, vec!["A", "B", "C"]);
        assert_eq!(matrix[0], vec![0.0, 10.0, 15.0]);
    }
    
    #[test]
    fn test_with_comments() {
        let input = "# TSP Input\nA B C\n# Distance matrix\n0 10 15\n10 0 20\n15 20 0";
        let mut parser = InputParser::new();
        let (cities, matrix) = parser.parse(input).unwrap();
        
        assert_eq!(cities.len(), 3);
        assert_eq!(matrix.len(), 3);
    }
}