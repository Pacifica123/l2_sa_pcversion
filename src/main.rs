use calculate::calculate_correlation;
use graphix::print_correlation_results;

mod clinning;
pub mod calculate;
pub mod utils;
pub mod graphix;

fn main() {
    let filepath = "fda_approved_food_items_w_nutrient_info.csv";
    let category_column = "branded_food_category";
    let category_value = "Cheese";

    match clinning::clean_csv(filepath, category_column, category_value) {
        Ok(cleaned_data) => {
            println!("Заголовки: {:?}", cleaned_data.headers);
            // for row in cleaned_data.rows {
            //     println!("{:?}", row);
            // }
            println!("Объем выборки - {}", cleaned_data.rows.len());

            // let _ = graphix::write_corr_table_to_csv(&calculate::calculate_correlation(&cleaned_data), "correlation_table.csv");
            print_correlation_results(&calculate_correlation(&cleaned_data))
        }
        Err(e) => println!("Error: {}", e),
    }
}
