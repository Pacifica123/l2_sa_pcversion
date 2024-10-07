

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
    // Заголовок таблицы
    println!("{:<20} {:<20} {:<15}", "Column X", "Column Y", "Correlation");
    println!("{}", "-".repeat(55)); // Разделительная линия

    // Выводим результаты
    for result in results {
        println!("{:<20} {:<20} {:<15.4}", result.column_x, result.column_y, result.correlation);
    }
}
