use std::fs;
use tracing::{error, info};

/*
This doesn't yet work as Zelen doesn't yet support sets or enums.

We get this error:
```
2025-11-22T11:30:23.824633Z  INFO zelen_team_assignment: Minizinc code read from ./team-assignment.mzn
2025-11-22T11:30:23.824746Z ERROR zelen_team_assignment: ✗ Parsing failed: Error at line 6, column 4: Expected Colon, found Of
  set of int: PRIORITY = 0..card(PROJECT);
```
*/

fn main() {
    tracing_subscriber::fmt::init();

    let mzn_path = "./team-assignment.mzn";
    let mzn_code =
        fs::read_to_string(mzn_path).expect(format!("Failed to read {mzn_path}").as_str());
    info!("Minizinc code read from {mzn_path}");

    let ast = match zelen::parse(&mzn_code) {
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
}
