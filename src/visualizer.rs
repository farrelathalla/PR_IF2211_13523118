use anyhow::Result;
use plotters::prelude::*;
use std::f64::consts::PI;

pub struct Visualizer;

impl Visualizer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn create_visualization(
        &self,
        cities: &[String],
        path: &[usize],
        min_cost: f64,
        output_file: &str,
    ) -> Result<()> {
        let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(
                &format!("TSP Solution - Total Distance: {:.1}", min_cost),
                ("Arial", 30).into_font(),
            )
            .margin(40)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(-1.2f64..1.2f64, -1.2f64..1.2f64)?;
        
        chart.configure_mesh()
            .x_desc("X Coordinate")
            .y_desc("Y Coordinate")
            .draw()?;
        
        // Membuat posisi 
        let city_positions = self.generate_city_positions(cities.len());
        
        // Gambar
        for (i, (x, y)) in city_positions.iter().enumerate() {
            chart.draw_series(PointSeries::of_element(
                vec![(*x, *y)],
                10,
                ShapeStyle::from(&BLUE).filled(),
                &|coord, size, style| {
                    EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                },
            ))?;
            
            // Label
            chart.draw_series(std::iter::once(Text::new(
                cities[i].clone(),
                (*x, *y + 0.15),
                ("Arial", 15).into_font(),
            )))?;
        }
        
        // Gambar path
        let mut path_points = Vec::new();
        for &city_idx in path {
            path_points.push(city_positions[city_idx]);
        }
        // Close loop
        if !path.is_empty() {
            path_points.push(city_positions[path[0]]);
        }
        
        chart.draw_series(LineSeries::new(path_points.clone(), RED.stroke_width(3)))?
            .label("Optimal Path")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], RED));
        
        // Gambar arrow
        for i in 0..path_points.len() - 1 {
            let (x1, y1) = path_points[i];
            let (x2, y2) = path_points[i + 1];
            
            // Posisi arrow
            let arrow_x = x1 + 0.75 * (x2 - x1);
            let arrow_y = y1 + 0.75 * (y2 - y1);
            
            // Arah arrow
            let dx = x2 - x1;
            let dy = y2 - y1;
            let length = (dx * dx + dy * dy).sqrt();
            
            if length > 0.01 {
                let unit_x = dx / length;
                let unit_y = dy / length;
                
                // Arrow head
                let arrow_length = 0.05;
                let arrow_angle: f64 = 0.5;
                
                let ax1 = arrow_x - arrow_length * (unit_x * arrow_angle.cos() - unit_y * arrow_angle.sin());
                let ay1 = arrow_y - arrow_length * (unit_x * arrow_angle.sin() + unit_y * arrow_angle.cos());
                
                let ax2 = arrow_x - arrow_length * (unit_x * arrow_angle.cos() + unit_y * arrow_angle.sin());
                let ay2 = arrow_y - arrow_length * (-unit_x * arrow_angle.sin() + unit_y * arrow_angle.cos());
                
                chart.draw_series(LineSeries::new(
                    vec![(arrow_x, arrow_y), (ax1, ay1)],
                    RED.stroke_width(2),
                ))?;
                
                chart.draw_series(LineSeries::new(
                    vec![(arrow_x, arrow_y), (ax2, ay2)],
                    RED.stroke_width(2),
                ))?;
            }
        }
        
        chart.configure_series_labels().draw()?;
        
        // Path information
        let path_text = format!(
            "Path: {} → {}",
            path.iter().map(|&i| cities[i].as_str()).collect::<Vec<_>>().join(" → "),
            cities[path[0]]
        );
        
        chart.draw_series(std::iter::once(Text::new(
            path_text,
            (-1.1, -1.1),
            ("Arial", 12).into_font().color(&BLACK),
        )))?;
        
        root.present()?;
        println!("  • Visualization created with {} cities", cities.len());
        
        Ok(())
    }
    
    /// Generate posisi
    fn generate_city_positions(&self, n: usize) -> Vec<(f64, f64)> {
        let mut positions = Vec::with_capacity(n);
        
        for i in 0..n {
            let angle = 2.0 * PI * (i as f64) / (n as f64) - PI / 2.0; // Start from top
            let x = angle.cos();
            let y = angle.sin();
            positions.push((x, y));
        }
        
        positions
    }
}