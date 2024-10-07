use crate::{clinning::CsvModel, utils};

#[derive(Clone)]
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

/// фильтрация коэффициентов корреляции по целевому признаку и значимости
pub fn calculate_filter_by_target(
    corr_data: &Vec<CorrelationResult>, 
    target_column: &str
) -> Vec<CorrelationResult> {
    // Фильтруем по целевому признаку и значимости корреляции (значения > 0.5)
    let mut filtered_corr: Vec<CorrelationResult> = corr_data
        .iter() // Используем iter() вместо into_iter()
        .filter(|result| {
            (result.column_x == target_column || result.column_y == target_column) && result.correlation.abs() > 0.5
        })
        .cloned() // Клонируем элементы, чтобы создать Vec<CorrelationResult>
        .collect();
    
    // Сортируем по значению корреляции от большей к меньшей для удобства анализа
    filtered_corr.sort_by(|a, b| b.correlation.abs().partial_cmp(&a.correlation.abs()).unwrap());
    
    filtered_corr
}


/// фильтрация зависимых факторов
pub fn calculate_filter_independent_factors(
    relevant_corr: &Vec<CorrelationResult>, 
    all_corr_data: &Vec<CorrelationResult>,
    target: &str,
    mutual_threshold: f64
) -> Vec<CorrelationResult> {
    let mut final_factors = relevant_corr.clone();
    let mut factors_to_remove = Vec::new();  // Массив индексов на удаление

    // Проходим по каждому из отфильтрованных факторов
    let mut i = 0;
    while i < final_factors.len() {
        let factor_a = &final_factors[i];

        let mut j = i + 1;
        while j < final_factors.len() {
            let factor_b = &final_factors[j];

            // Ищем корреляцию между factor_a и factor_b в all_corr_data
            for corr in all_corr_data {
                let is_same_corr = (corr.column_x == factor_a.column_x && corr.column_y == factor_b.column_x)
                    || (corr.column_x == factor_a.column_y && corr.column_y == factor_b.column_y)
                    || (corr.column_x == factor_a.column_x && corr.column_y == factor_b.column_y)
                    || (corr.column_x == factor_a.column_y && corr.column_y == factor_b.column_x);

                if is_same_corr {
                    if corr.correlation.abs() > mutual_threshold {
                        if factor_a.correlation.abs() < factor_b.correlation.abs() {
                            if factor_a.column_x == target {
                                println!("Помечаем фактор {} на удаление", factor_a.column_y);    
                            } else {
                                println!("Помечаем фактор {} на удаление", factor_a.column_x);
                            }
                            
                            factors_to_remove.push(i);  // Помечаем фактор A на удаление
                            break;  // Прекращаем проверку этой пары
                        } else {
                            if factor_a.column_x == target {
                                println!("Помечаем фактор {} на удаление", factor_b.column_y);    
                            } else {
                                println!("Помечаем фактор {} на удаление", factor_b.column_x);
                            }
                            factors_to_remove.push(j);  // Помечаем фактор B на удаление
                            break;  // Прекращаем проверку этой пары
                        }
                    }
                }
            }

            j += 1;
        }

        i += 1;
    }

    // Удаляем все помеченные факторы, начиная с последнего индекса
    factors_to_remove.sort_by(|a, b| b.cmp(a));  // Сортируем индексы в обратном порядке
    factors_to_remove.dedup();  // Убираем дублирование индексов на удаление
    for idx in factors_to_remove {
        final_factors.remove(idx);
    }

    final_factors
}

