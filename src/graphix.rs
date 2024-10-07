

use crate::calculate::CorrelationResult;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};

pub fn write_corr_table_to_csv(correlations: &Vec<CorrelationResult>, filename: &str) -> io::Result<()> {
    // Извлекаем уникальные заголовки
    let mut headers_set: HashSet<String> = HashSet::new();
    for corr in correlations {
        headers_set.insert(corr.column_x.clone());
        headers_set.insert(corr.column_y.clone());
    }
    let headers: Vec<String> = headers_set.into_iter().collect();

    // Создаем файл
    let mut file = File::create(filename)?;

    // Записываем заголовки столбцов
    write!(file, ",")?; // Пустое место для заголовка строк
    for header in &headers {
        write!(file, "{},", header)?;
    }
    writeln!(file)?;

    // Записываем данные таблицы
    for (i, header_x) in headers.iter().enumerate() {
        write!(file, "{}", header_x)?; // Заголовки строк
        for (j, header_y) in headers.iter().enumerate() {
            if i >= j {
                let corr = correlations.iter().find(|&c| 
                    (c.column_x == *header_x && c.column_y == *header_y) || 
                    (c.column_x == *header_y && c.column_y == *header_x)
                );
                match corr {
                    Some(c) => write!(file, "{:.2},", c.correlation)?,
                    None => write!(file, "{},", "-")?,
                }
            } else {
                write!(file, "{},", "-")?;
            }
        }
        writeln!(file)?;
    }

    Ok(())
}
// pub fn write_corr_table_to_csv(correlations: &Vec<CorrelationResult>, filename: &str) -> io::Result<()> {
//     // Извлекаем уникальные заголовки
//     let mut headers: Vec<String> = Vec::new();
//     for corr in correlations {
//         if !headers.contains(&corr.column_x) {
//             headers.push(corr.column_x.clone());
//         }
//     }

//     // Создаем файл
//     let mut file = File::create(filename)?;

//     // Записываем заголовки столбцов
//     write!(file, ",")?; // Пустое место для заголовка строк
//     for header in &headers {
//         write!(file, "{},", header)?;
//     }
//     writeln!(file)?;

//     // Записываем данные таблицы
//     for (i, header_x) in headers.iter().enumerate() {
//         write!(file, "{}", header_x)?; // Заголовки строк
//         for (j, header_y) in headers.iter().enumerate() {
//             if i >= j {
//                 let corr = correlations.iter().find(|&c| 
//                     (c.column_x == *header_x && c.column_y == *header_y) || 
//                     (c.column_x == *header_y && c.column_y == *header_x)
//                 );
//                 match corr {
//                     Some(c) => write!(file, "{:.2},", c.correlation)?,
//                     None => write!(file, "{},", "-")?,
//                 }
//             } else {
//                 write!(file, "{},", "-")?;
//             }
//         }
//         writeln!(file)?;
//     }

//     Ok(())
// }



pub fn print_correlation_results(results: &Vec<CorrelationResult>) {
    let mut unique_results = HashSet::new();
    let mut formatted_results: Vec<(String, String, f64)> = Vec::new();

    // Фильтруем повторяющиеся пары
    for result in results {
        let (col_x, col_y) = if result.column_x < result.column_y {
            (result.column_x.clone(), result.column_y.clone())
        } else {
            (result.column_y.clone(), result.column_x.clone())
        };
        
        if unique_results.insert((col_x.clone(), col_y.clone())) {
            formatted_results.push((col_x, col_y, result.correlation));
        }
    }

    // Определяем ширину столбцов
    let max_col_x_len = formatted_results.iter().map(|(x, _, _)| x.len()).max().unwrap_or(0);
    let max_col_y_len = formatted_results.iter().map(|(_, y, _)| y.len()).max().unwrap_or(0);
    
    // Заголовок таблицы
    println!("{:<width_x$} {:<width_y$} {:<15}", "Column X", "Column Y", "Correlation", width_x = max_col_x_len + 2, width_y = max_col_y_len + 2);
    println!("{}", "-".repeat(max_col_x_len + max_col_y_len + 32)); // Разделительная линия

    // Выводим результаты
    for (col_x, col_y, correlation) in formatted_results {
        println!("{:<width_x$} {:<width_y$} {:<15.4}", col_x, col_y, correlation, width_x = max_col_x_len + 2, width_y = max_col_y_len + 2);
        println!("{}", "-".repeat(max_col_x_len + max_col_y_len + 32)); // Пунктирная линия между строками
    }
}       