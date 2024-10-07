pub fn mean(values: &Vec<f64>) -> f64{
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

/// Стандартое отклонение
pub fn std_dev(values: &Vec<f64>) -> f64{
    let mean = mean(values);
    let var: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    var.sqrt()
}