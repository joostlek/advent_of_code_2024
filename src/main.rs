const INPUT: &'static str = include_str!("input.txt");

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn is_safe_line(reports: &Vec<&str>) -> bool {
    let mut safe = true;
    let mut increasing = false;
    let mut decreasing = false;
    for report_no in 0..reports.len() - 1 {
        let a: f64 = reports[report_no].parse().unwrap();
        let b:f64 = reports[report_no + 1].parse().unwrap();
        let difference = a - b;
        if abs(difference) > 3.0 {
            safe = false;
        } else if difference == 0.0 {
            safe = false;
        } else if !increasing && !decreasing {
            increasing = difference > 0.0;
            decreasing = difference < 0.0;
        } else if increasing && difference < 0.0 {
            safe = false;
        } else if decreasing && difference > 0.0 {
            safe = false;
        }
    }
    safe
}

fn main() {
    // let lines = INPUT.lines();
    // let mut safe_lines = 0;
    // for line in lines {
    //     let mut reports: Vec<&str> = line.split_whitespace().collect();
    //     let mut safe = true;
    //     let mut increasing = false;
    //     let mut decreasing = false;
    //     for report_no in 0..reports.len() - 1 {
    //         let a: f64 = reports[report_no].parse().unwrap();
    //         let b:f64 = reports[report_no + 1].parse().unwrap();
    //         let difference = a - b;
    //         if abs(difference) > 3.0 {
    //             safe = false;
    //         } else if difference == 0.0 {
    //             safe = false;
    //         } else if !increasing && !decreasing {
    //             increasing = difference > 0.0;
    //             decreasing = difference < 0.0;
    //         } else if increasing && difference < 0.0 {
    //             safe = false;
    //         } else if decreasing && difference > 0.0 {
    //             safe = false;
    //         }
    //     }
    //     if safe {
    //         println!("line is safe: {}", line);
    //         safe_lines += 1;
    //     }
    // }
    // println!("{}", safe_lines);
    let lines = INPUT.lines();
    let mut safe_lines = 0;
    for line in lines {
        let reports: Vec<&str> = line.split_whitespace().collect();
        if is_safe_line(&reports) {
            println!("line is safe: {}", line);
            safe_lines += 1;
        } else {
            for report_index in 0..reports.len() {
                let mut new_report = reports.clone();
                new_report.remove(report_index);
                if is_safe_line(&new_report) {
                    println!("line is safe: {:?}", new_report);
                    safe_lines += 1;
                    break;
                }
            }

        }
    }
    println!("{}", safe_lines);
}
