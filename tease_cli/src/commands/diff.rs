use colored::Colorize;

use super::read::read_object;

#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub number: u64
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub is_diag: bool
}

#[derive(Debug)]
pub struct Snake {
    pub points: Vec<Point>,
}

pub fn diff(a_sha1_blob: String, b_sha1_blob: String) {
    // let a_blob = "blob 164\0use crate::index_structs::index::remove_index_row;\n\npub fn reset_index_row(filename: String) {\nremove_index_row(filename);\n}".to_string();
    // let b_blob = "blob 278\0use crate::index_structs::index::remove_index_row;\ndwdfad\ngsghz\nhzht\npub fn reset_index_row(filename: String) {\nMajku mu jebem jel radi vise\nauuuuuuuu samo da radi\nremove_index_row(filename);\nboze moj\nmolim te radi\n}".to_string();
    let a_blob = read_object(&a_sha1_blob);
    let b_blob = read_object(&b_sha1_blob);
    let a_lines = get_content_from_blob(a_blob);
    let b_lines = get_content_from_blob(b_blob);
    let mut path_trace = shortest_edit(&a_lines, &b_lines);
    let mut path = retrace(& mut path_trace, a_lines.len(), b_lines.len());
    path.reverse();
    println!("{:?}", path);
    format_lines(&path, &a_lines, &b_lines);
    // println!("Number of needed moves: {}", path_len);
}

fn format_lines(path: &Vec<Snake>, a_lines: &Vec<Line>, b_lines: &Vec<Line>) {
    println!("len {}", path.len());
    for snake in path.iter() {
        for points in snake.points.windows(2) {
            let start = &points[0];
            let end = &points[1];
            
            let a_line = get_line_by_index(&a_lines, start.x);
            let b_line = get_line_by_index(&b_lines, start.y);

            if start.x == end.x {
                println!("{}", format!("+ {}", b_line).green());
            } else if start.y == end.y {
                println!("{}", format!("- {}", a_line).red());
            } else {
                println!("{}", format!("= {}", if a_line == "" { b_line } else { a_line }));
            }
        }
    }
}


fn get_line_by_index(lines: &Vec<Line>, index: i32) -> &str {
    if (index as usize) < lines.len() {
         return &lines[index as usize].content; 
        } 
     ""
}

    //       A     B     C     A     B     B     A
    //     o-----o-----o-----o-----o-----o-----o-----o   0
    //     |     |     | \   |     |     |     |     |
    // C   |     |     |  \  |     |     |     |     |
    //     |     |     |   \ |     |     |     |     |
    //     o-----o-----o-----o-----o-----o-----o-----o   1
    //     |     | \   |     |     | \   | \   |     |
    // B   |     |  \  |     |     |  \  |  \  |     |
    //     |     |   \ |     |     |   \ |   \ |     |
    //     o-----o-----o-----o-----o-----o-----o-----o   2
    //     | \   |     |     | \   |     |     | \   |
    // A   |  \  |     |     |  \  |     |     |  \  |
    //     |   \ |     |     |   \ |     |     |   \ |
    //     o-----o-----o-----o-----o-----o-----o-----o   3
    //     |     | \   |     |     | \   | \   |     |
    // B   |     |  \  |     |     |  \  |  \  |     |
    //     |     |   \ |     |     |   \ |   \ |     |
    //     o-----o-----o-----o-----o-----o-----o-----o   4
    //     | \   |     |     | \   |     |     | \   |
    // A   |  \  |     |     |  \  |     |     |  \  |
    //     |   \ |     |     |   \ |     |     |   \ |
    //     o-----o-----o-----o-----o-----o-----o-----o   5
    //     |     |     | \   |     |     |     |     |
    // C   |     |     |  \  |     |     |     |     |
    //     |     |     |   \ |     |     |     |     |
    //     o-----o-----o-----o-----o-----o-----o-----o   6
    //     0     1     2     3     4     5     6     7
    // po uzoru na https://blog.jcoglan.com/2017/02/15/the-myers-diff-algorithm-part-2/

fn shortest_edit(a_lines: &Vec<Line>, b_lines: &Vec<Line>) -> Vec<Vec<i32>> {
    let a_size = a_lines.len() as i32;
    let b_size = b_lines.len() as i32;
    let max_size = a_size + b_size;
    println!("max size {}", max_size);
    let mut trace: Vec<Vec<i32>> = vec![];
    let mut state_vec = vec![0; (2 * max_size) as usize];

    for depth in 0..max_size {
        let mut x;
        let mut y;

        for k in (-depth..depth+1).step_by(2) {

            let down = k == -depth || (k != depth && state_vec[get_usize_index(k-1, max_size)] < state_vec[get_usize_index(k+1, max_size)]);
            // dole ili desno
            let prev_k = if down { get_usize_index(k+1, max_size) } else { get_usize_index(k-1, max_size) };

            x = if down { state_vec[prev_k] } else { state_vec[prev_k] + 1 };
            y = x - k;

            while x < a_size && y < b_size && a_lines[x as usize].content == b_lines[y as usize].content {
                x = x + 1;
                y = y + 1;
            }

            state_vec[get_usize_index(k, max_size)] = x;
            
            if x >= a_size && y >= b_size {
                trace.push(state_vec.clone());
                return trace;
            }
        }

        trace.push(state_vec.clone());
        
    }

    trace
}

fn retrace(trace: & mut Vec<Vec<i32>>, a_size: usize, b_size: usize) -> Vec<Snake> {

    let mut x = a_size as i32;
    let mut y = b_size as i32;
    let max_size = (a_size + b_size) as i32;
    let mut snakes: Vec<Snake> = vec![];

    for (d, state_vec) in trace.iter().enumerate().rev() {
        let mut snake = Snake { points: vec![] };

        let depth = d as i32;
        let k = x - y;
        let prev_k;

        let down = k == -depth || (k != depth && state_vec[get_usize_index(k-1, max_size)] < state_vec[get_usize_index(k+1, max_size)]);
        prev_k = if down { k + 1 } else { k - 1 };

        let prev_x = state_vec[get_usize_index(prev_k, max_size)];
        let prev_y = prev_x - prev_k;

        snake.points.push( Point { x, y, is_diag: false });

        while x > prev_x && y > prev_y {
            snake.points.push( Point {x: x - 1, y: y - 1, is_diag: true});
            x = x - 1;
            y = y - 1;
        }

        if prev_x >= 0 && prev_y >= 0 {
            snake.points.push(Point {x: prev_x, y: prev_y, is_diag: false});
        }
        snake.points.reverse();
        snakes.push(snake);

        x = prev_x;
        y = prev_y;

        if x == 0 && y == 0 {
            break;
        }
    }

    snakes
}

fn get_usize_index(k: i32, max_size: i32) -> usize {
    if k < 0 { 
        return (k + 2*max_size) as usize;
    } 

    k as usize
}

fn get_content_from_blob(blob: String) -> Vec<Line> {
    let parts: Vec<&str> = blob.split("\0").collect();
    let content: Vec<String> = parts[1].to_string()
                            .split("\n")
                            .map(|line| line.to_string())
                            .collect();
    
    content.iter()
            .enumerate()
            .map(|(index, line)| Line {content: line.trim().to_string(), number: (index + 1) as u64})
            .collect()
}