use std::fs;
use tracing::{error, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mzn_path = "./minizinc/guest.mzn";
    let mzn_code =
        fs::read_to_string(mzn_path).expect(format!("Failed to read {mzn_path}").as_str());
    info!("Minizinc code read from {mzn_path}");

    let dzn_path = "./minizinc/guest.dzn";
    let dzn_code =
        fs::read_to_string(dzn_path).expect(format!("Failed to read {dzn_path}").as_str());
    info!("Minizinc code read from {dzn_path}");

    let combined_code = match zelen::load_dzn_data(&dzn_code, &mzn_code) {
        Ok(c) => {
            info!("✓ combining successful!");
            c
        }
        Err(e) => {
            error!("✗ combining failed: {}", e);
            std::process::exit(1);
        }
    };

    println!("Combined Minizinc Code:\n{}", combined_code);

    let ast = match zelen::parse(&combined_code) {
        Ok(ast) => {
            info!("✓ Parsing successful!");
            info!("AST contains {} items", ast.items.len());
            ast
        }
        Err(e) => {
            error!("✗ Parsing failed: {}", e);
            std::process::exit(1);
        }
    };

    println!("AST: {:#?}", ast);

    let translated = zelen::Translator::translate_with_vars(&ast)?;
    for (name, var_id) in &translated.int_vars {
        println!("Variable: {} -> {:?}", name, var_id);
    }
    for (name, var_id) in &translated.int_var_arrays {
        println!("Variable: {} -> {:?}", name, var_id);
    }

    let solution = translated.model.solve()?;
    println!("Solution: {:#?}", solution);

    Ok(())
}
