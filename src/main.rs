use rs_2048::process_row;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// fn next_grid(grid: [u16; 4]) -> [u16; 4] {
//     let mut new_grid: [u16; 4] = [0; 4];
//     for i in (0..3).rev() {
//         if grid[i] == 0 {
//             continue;
//         }

//         let mut destination = i;
//         let mut value = grid[i];
//         for j in (i + 1)..4 {
//             if grid[j] == 0 {
//                 if j == 3 {
//                     destination = j;
//                 }
//                 continue;
//             }
//             if grid[j] == grid[i] {
//                 destination = j;
//                 value = grid[i] + grid[j];
//                 break;
//             }
//             break;
//         }

//         new_grid[destination] = value;
//     }

//     new_grid
// }
//

fn foo(row: &Vec<u8>) -> Vec<u8> {
    let mut new_row = row.clone();
    new_row[row.len() - 1] = 1;
    new_row
}

fn format_row(row: &Vec<u8>) -> String {
    let transformed_row_values: Vec<u32> = row
        .into_iter()
        .map(|v| {
            if v == &0 {
                return 0;
            }
            let base: u32 = 2;
            let exp = v.to_owned() as u32;
            return base.pow(exp);
        })
        .collect();
    format!("{:?}", transformed_row_values)
}

fn main() {
    let mut row: Vec<u8> = vec![1, 1, 1, 1];
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    // write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
    // .unwrap();
    stdout.flush().unwrap();
    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        write!(stdout, "{}", format_row(&row)).unwrap();

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Left => {
                row = process_row(row);
                row = foo(&row);
                // write!(stdout, "{}", format_row(&row)).unwrap();
                // row[row.len() - 1] = 1;
            }
            Key::Right => print!("Eight"),
            Key::Ctrl('q') => break,
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
