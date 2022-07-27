use super::read::read_object;

#[derive(Debug)]
pub struct Line {
    pub content: String,
    pub number: u64
}

pub fn diff(a_sha1_blob: String, b_sha1_blob: String) {
    let a_blob = read_object(&a_sha1_blob);
    let b_blob = read_object(&b_sha1_blob);

    let a_lines = get_content_from_blob(a_blob);
    let b_lines = get_content_from_blob(b_blob);
    let path_len = shortest_edit(&a_lines, &b_lines);
    println!("Number of needed moves: {}", path_len);
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

fn shortest_edit(a_lines: &Vec<Line>, b_lines: &Vec<Line>) -> i32 {
    let a_size = a_lines.len() as i32;
    let b_size = b_lines.len() as i32;
    let max_size = a_size + b_size;
    println!("max size {}", max_size);
    let mut trace:Vec<Vec<i32>> = vec![vec![]; (2 * max_size + 1) as usize];


    for depth in 0..max_size {
        let mut x;
        let mut y;
        let mut state_vec = vec![0; (2 * max_size + 1) as usize];

        for k in (-depth..depth+1).step_by(2) {
            let index = if k < 0 { (k + 2*max_size) as usize } else {k as usize};
            let back = if index == 0 { (2*max_size - 1) as usize } else { index - 1 };
            //dole
            if k == -depth || (k != depth && state_vec[back] < state_vec[index + 1]) {
                x = state_vec[index + 1];
            }
            //desno
            else {
                x = state_vec[back] + 1;
            }

            y = x - k;

            while x < a_size && y < b_size && a_lines[x as usize].content == b_lines[y as usize].content {
                x = x + 1;
                y = y + 1;
            }
            state_vec[index] = x;

            
            if x >= a_size && y >= b_size {
                return depth;
            }
        }
        // trace.push(state_vec.clone());
    }
    
    0
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