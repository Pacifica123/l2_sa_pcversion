use calculate::{calculate_correlation, calculate_filter_by_target, calculate_filter_independent_factors, calculate_student_t_for_correlations};
use graphix::{print_correlation_results, print_student_coeffs};

mod clinning;
pub mod calculate;
pub mod utils;
pub mod graphix;

fn main() {
    let filepath = "fda_approved_food_items_w_nutrient_info.csv";
    let category_column = "branded_food_category";
    let category_value = "Tomatoes";

    match clinning::clean_csv(filepath, category_column, category_value) {
        Ok(cleaned_data) => {
            println!("Заголовки: {:?}", cleaned_data.headers);
            // for row in cleaned_data.rows {
            //     println!("{:?}", row);
            // }
            println!("Объем выборки - {}", cleaned_data.rows.len());

            // let _ = graphix::write_corr_table_to_csv(&calculate::calculate_correlation(&cleaned_data), "correlation_table.csv");
            let corr_data = calculate_correlation(&cleaned_data);
            print_correlation_results(&corr_data);

            let target = "Energy-KCAL";
            // let target = "Vitamin C, total ascorbic acid-MG";
            let filter_corr = calculate_filter_independent_factors(&calculate_filter_by_target(&corr_data, &target), &corr_data, &target, 0.8);

            println!("\nОтфильтрованное по целевому признаку:");
            print_correlation_results(&filter_corr);

            let coeff_student_by_original = calculate_student_t_for_correlations(&corr_data);
            let coeff_student = calculate_student_t_for_correlations(&filter_corr);
            println!("Коэффициент Стьюдента:");
            print_student_coeffs(&coeff_student, &filter_corr);

        }
        Err(e) => println!("Error: {}", e),
    }
}
