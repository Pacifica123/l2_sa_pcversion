// Подключаем нужные библиотеки
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

// Структура для модели данных с динамическими числовыми полями
#[derive(Debug)]
pub struct CsvModel {
    pub headers: Vec<String>,         // Заголовки столбцов
    pub rows: Vec<HashMap<String, f64>>, // Значения в виде числового словаря {колонка: значение}
}

// // Функция для фильтрации по типу продукции
// pub fn filter_by_category(
//     filepath: &str,
//     category_column: &str,
//     category_value: &str
// ) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
//     let file = File::open(filepath)?;
//     let mut rdr = ReaderBuilder::new().from_reader(file);

//     let headers = rdr.headers()?.clone();

//     let mut filtered_data = Vec::new();

//     // Проходим по строкам и фильтруем их
//     for result in rdr.records() {
//         let record = result?;
//         if record.get(headers.iter().position(|h| h == category_column).unwrap()) == Some(category_value) {
//             let mut record_map = HashMap::new();
//             for (i, field) in record.iter().enumerate() {
//                 record_map.insert(headers[i].to_string(), field.to_string());
//             }
//             filtered_data.push(record_map);
//         }
//     }

//     Ok(filtered_data)
// }
pub fn filter_by_category(
    filepath: &str,
    category_column: &str,
    category_value: &str
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    let headers = rdr.headers()?.clone();
    
    let category_index = headers.iter().position(|h| h == category_column)
        .ok_or("Category column not found")?;
    
    let filtered_data: Vec<HashMap<String, String>> = rdr.records()
        .filter_map(|result| {
            let record = result.ok()?;
            if record.get(category_index) == Some(category_value) {
                let record_map: HashMap<String, String> = headers.iter()
                    .enumerate()                
                    .filter_map(|(i, header)| {
                        record.get(i).map(|field| (header.to_string(), field.to_string()))
                    })
                    .collect();
                Some(record_map)
            } else {
                None
            }
        })
        .collect();

    Ok(filtered_data)
}

/// Функция для удаления строковых столбцов
pub fn remove_string_columns(filtered_data: Vec<HashMap<String, String>>) -> CsvModel {
    let mut headers = Vec::new();
    let mut rows = Vec::new();

    for record in filtered_data {
        let mut numerical_record = HashMap::new();
        for (key, value) in record {
            if let Ok(parsed_value) = value.parse::<f64>() {
                numerical_record.insert(key.clone(), parsed_value);
            }
        }
        if !numerical_record.is_empty() {
            rows.push(numerical_record);
        }
    }

    if let Some(first_row) = rows.get(0) {
        headers = first_row.keys().cloned().collect();  // Формируем заголовки только по числовым значениям
    }

    CsvModel { headers, rows }
}


/// Функция для удаления абсолютных (статических) значений
pub fn remove_absolute_columns(csv_model: CsvModel) -> CsvModel {
    let mut clean_rows = Vec::new();
    let mut headers = Vec::new();

    if let Some(first_row) = csv_model.rows.get(0) {
        let mut non_static_columns = Vec::new();

        for header in &csv_model.headers {
            let first_value = first_row.get(header).cloned();
            if csv_model.rows.iter().any(|row| row.get(header) != first_value.as_ref()) {
                non_static_columns.push(header.clone());
            }
        }

        headers = non_static_columns.clone();

        for row in csv_model.rows {
            let mut cleaned_row = HashMap::new();
            for header in &non_static_columns {
                if let Some(value) = row.get(header) {
                    cleaned_row.insert(header.clone(), *value);
                }
            }
            clean_rows.push(cleaned_row);
        }
    }

    CsvModel {
        headers,
        rows: clean_rows,
    }
}


// Основная функция, которая объединяет все этапы очистки
pub fn clean_csv(filepath: &str, category_column: &str, category_value: &str) -> Result<CsvModel, Box<dyn Error>> {
    let filtered_data = filter_by_category(filepath, category_column, category_value)?;
    let csv_model = remove_string_columns(filtered_data);
    let cleaned_model = remove_absolute_columns(csv_model);
    
    Ok(cleaned_model)
}
