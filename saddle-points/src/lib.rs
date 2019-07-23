use std::collections::HashSet;
    // greatest in the row - same row, all cols
    // smallest in the column - same column, all rows

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut max_points = HashSet::new();
    let mut min_points = HashSet::new();
    let rows = input.len();
    if rows != 0 {
        let cols = input[0].len(); 
        println!("\nPrinting input: \n{:?}\n", input);
        //get all max points
        for row in 0..rows {
            let mut max_col = 0;
            let mut row_max = &input[row][0];
            for hxc in 0..cols {
                if row_max < &input[row][hxc] {
                    row_max = &input[row][hxc];
                    max_col = hxc;
                }
            }
            max_points.insert((row, max_col));//fix to take dups later
        }
        //get all min points
        for col in 0..cols {
            let mut min_row = 0;
            let mut col_min = &input[0][col];
            for vyr in 0..rows {
                if col_min > &input[vyr][col] {
                    col_min = &input[vyr][col];
                    min_row = vyr;
                }
            }
            min_points.insert((min_row, col));
        }
        for max_key in max_points {
            if min_points.contains(&max_key){
                points.push(max_key);
                println!("pushing key = {:?}\n", max_key);
            }
        }
        println!("\n");
    }
    points
}

