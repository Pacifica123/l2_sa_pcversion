pub fn mean(values: &Vec<f64>) -> f64{
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

pub fn mean_of_squares(values: &Vec<f64>) -> f64{
    let mut sum_sqr = 0.0;
    for v in values {
        sum_sqr += v.powi(2);
    }
    sum_sqr / values.len() as f64
}

/// Стандартое отклонение
pub fn std_dev(values: &Vec<f64>) -> f64{
    let mean = mean(values);
    // Вычисляем среднее значение квадратов
    let mean_of_squares = mean_of_squares(values);

    // Стандартное отклонение
    (mean_of_squares - mean.powi(2)).sqrt()
}

pub fn sum_xy_for_corr(x_values: &Vec<f64>, y_values: &Vec<f64>) -> f64{
    let x_mean = mean(&x_values);
    let y_mean = mean(&y_values);
    if x_values.len() != y_values.len() {
        println!("ВЫБОРКИ НЕ РАВНЫ ПО РАЗМЕРУ!")
    }
    let mut sum = 0.0;
    for i in 0..x_values.len(){
        sum += (x_values[i] - x_mean)*(y_values[i] - y_mean);
    }
    // deBug:
    // println!("sum_xy_for_corr : {}", sum);
    sum
}