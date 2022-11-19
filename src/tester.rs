use std::time::Instant;

pub fn run_test<Solver>(path: &str, solver: Solver)
where
    Solver: Fn(Vec<&str>) -> String,
{
    let mut i = 0;
    let mut ok = 0;
    let mut failed = 0;
    while let Ok(str_n) = std::fs::read_to_string(dbg!(format!("{path}/test.{i}.in"))) {
        let start = Instant::now();
        let input_data: Vec<&str> = str_n.lines().collect();
        let solved = solver(input_data);
        let expected_result =
            std::fs::read_to_string(dbg!(format!("{path}/test.{i}.out"))).unwrap();
        let expected_result = expected_result.trim();
        let elapsed = dbg!(Instant::now().duration_since(start));
        if expected_result.trim() == solved {
            ok += 1;
            println!("Test {i} passed in {:?}", elapsed);
        } else {
            failed += 1;
            println!(
                "FAILED in {:?} Expected = {expected_result:?} Solved = {solved:?}",
                elapsed
            );
        }
        i += 1;
    }
    println!("Have run {i} tests.");
    println!("Success --- {ok}");
    if failed > 0 {
        println!("FAILED --- {failed}");
    }
}

pub fn run_silently<Solver>(path: &str, solver: Solver)
where
    Solver: Fn(Vec<&str>) -> String,
{
    let mut i = 0;
    while let Ok(str_n) = std::fs::read_to_string(dbg!(format!("{path}/test.{i}.in"))) {
        let start = Instant::now();
        let input_data: Vec<&str> = str_n.lines().collect();
        let solved = solver(input_data);
        let expected_result =
            std::fs::read_to_string(dbg!(format!("{path}/test.{i}.out"))).unwrap();
        let elapsed = dbg!(Instant::now().duration_since(start));
        if expected_result.trim() == solved {
            println!("Test {i} passed in {:?}", elapsed);
        } else {
            println!("{i} FAILED in {:?}", elapsed);
        }
        i += 1;
    }
}

pub fn run_test_lim<Solver>(path: &str, solver: Solver, max_test: u8)
where
    Solver: Fn(Vec<&str>) -> String,
{
    let mut i = 0;
    while let Ok(str_n) = std::fs::read_to_string(dbg!(format!("{path}/test.{i}.in"))) {
        let start = Instant::now();
        let input_data: Vec<&str> = str_n.lines().collect();
        let solved = solver(input_data);
        let expected_result =
            std::fs::read_to_string(dbg!(format!("{path}/test.{i}.out"))).unwrap();
        let elapsed = dbg!(Instant::now().duration_since(start));
        if expected_result.trim() == solved {
            println!("Test {i} passed in {:?}", elapsed);
        } else {
            println!(
                "FAILED in {:?} Expected = {expected_result} Solved = {solved}",
                elapsed
            );
        }
        i += 1;
        if i > max_test {
            break;
        }
    }
    println!("Have run {i} tests.");
}
