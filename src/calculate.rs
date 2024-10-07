use crate::{clinning::CsvModel, utils};

pub struct CorrelationResult {
    pub column_x: String,
    pub column_y: String,
    pub correlation: f64,
}

/// Для расчета коэффициента корреляции между двумя столбцами
pub fn calculate_correlation(data: &CsvModel) -> Vec<CorrelationResult> {
    let mut results: Vec<CorrelationResult> = Vec::new();
    let headers = &data.headers;
    let headerc_counts = headers.len();
    let rows = &data.rows;

    // Проходим по всем парам столбцов
    for i in 0..headerc_counts {
        for j in (i + 1)..headerc_counts {
            // Извлекаем данные для двух столбцов
            let column_x: Vec<f64> = rows.iter().map(|row| row.get(&headers[i]).cloned().unwrap_or_default()).collect();
            let column_y: Vec<f64> = rows.iter().map(|row| row.get(&headers[j]).cloned().unwrap_or_default()).collect();

            // Рассчитываем коэффициент корреляции
            let x_mean = utils::mean(&column_x);
            let y_mean = utils::mean(&column_y);
            let sx = utils::std_dev(&column_x);
            let sy = utils::std_dev(&column_y);

            // Расчет корреляции r
            let numerator: f64 = column_x.iter()
                .zip(&column_y)
                .map(|(x, y)| (x - x_mean) * (y - y_mean))
                .sum();
            let denominator = (sx * sy) * (column_x.len() as f64);

            let correlation = if denominator != 0.0 {
                numerator / denominator
            } else {
                0.0
            };

            // Сохраняем результат
            results.push(CorrelationResult {
                column_x: headers[i].clone(),
                column_y: headers[j].clone(),
                correlation,
            });
        }
    }

    results
}