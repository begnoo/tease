use super::read::read_object;

#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub number: u64
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn diff(a_sha1_blob: String, b_sha1_blob: String) {
    let a_blob = read_object(&a_sha1_blob);
    let b_blob = read_object(&b_sha1_blob);

    let a_lines = get_content_from_blob(a_blob);
    let b_lines = get_content_from_blob(b_blob);
    let mut path_trace = shortest_edit(&a_lines, &b_lines);
    let path = retrace(& mut path_trace, a_lines.len(), b_lines.len());
    println!("{:?}", path);
    // println!("Number of needed moves: {}", path_len);
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
    let mut state_vec = vec![0; (2 * max_size + 1) as usize];


    for depth in 0..max_size {
        let mut x;
        let mut y;

        for k in (-depth..depth+1).step_by(2) {
            let index = get_usize_index(k, max_size);
            let back =  if index == 0 { (2*max_size - 1) as usize } else { index - 1 }; 
            //dole
            // println!("===================");
            if k == -depth || (k != depth && state_vec[back] < state_vec[index + 1]) {
                x = state_vec[index + 1];
            }
            //desno
            else {
                x = state_vec[back] + 1;
            }

            y = x - k;

            // println!("({}, {})", x, y);
            // println!("depth={} k={}", depth, k);

            while x < a_size && y < b_size && a_lines[x as usize].content == b_lines[y as usize].content {
                x = x + 1;
                y = y + 1;
            }
            state_vec[index] = x;
            // println!("vec: {:?}", state_vec);
            
            if x >= a_size && y >= b_size {
                trace.push(state_vec.clone());
                return trace;
            }
        }

        trace.push(state_vec.clone());
        
    }

    trace
}

fn retrace(trace: & mut Vec<Vec<i32>>, a_size: usize, b_size: usize) -> Vec<Point> {

    let mut x = a_size as i32;
    let mut y = b_size as i32;
    let max_size = (a_size + b_size) as i32;
    trace.reverse();
    let mut points: Vec<Point> = vec![];

    for (d, state_vec) in trace.iter().enumerate() {

        if state_vec.is_empty() {
            continue;
        }

        let depth = d as i32;
        let k = x - y;
        let prev_k;
        
        let index = get_usize_index(k, max_size);
        let back =  if index == 0 { (2*max_size - 1) as usize } else { index - 1 }; 

        if k == -depth || (k != depth && state_vec[back] < state_vec[index + 1]) {
            prev_k = k + 1;
        } else {
            prev_k = k - 1;
        }

        let prev_x = state_vec[get_usize_index(prev_k, max_size)];
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            // println!("({}, {}) => ({}, {}) depth={}", x - 1, y - 1, x, y, depth);
            points.push(Point {x, y} );
            x = x - 1;
            y = y - 1;
        }

        if d != trace.len() - 1 {
            // println!("({}, {}) => ({}, {}) depth={}", prev_x, prev_y, x, y, depth);
            points.push(Point {x, y} );

        }
        x = prev_x;
        y = prev_y;
    }

    points
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