// A schedual has just been released for an upcoming tech conference.
// The schedule provides the start and end time of each of the presentations.
// Determine the maximum

pub fn max(start: &[usize], end: &[usize]) -> usize {
    let n = start.len();
    let mut visited = vec![false; n];

    let mut m = 1usize;

    for i in 0..n {
        visit(&start, &end, &mut visited, i, &mut m);
    }
    m
}

fn visit(start: &[usize], end: &[usize], visited: &mut [bool], i: usize, m: &mut usize) {
    println!("{}", i);
    println!("{:?}", visited);
    if visited[i] == true {
        return;
    }

    visited[i] = true;
    let s = start.iter().position(|&r| r >= end[i]);
    
    if s.is_some() {
        *m += 1;
    println!("m {}", m);
        visit(start, end, visited, i+1, m);
    } else {
        return;
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let start: [usize; 3] = [1, 1, 2];
        let end: [usize; 3] = [3, 2, 4];

        let res = max(&start, &end);

        assert_eq!(res, 2);
    }
    #[test]
    fn complicate() {
        let start: [usize; 8] = [1, 1, 1, 2, 5, 6, 6, 8];
        let end: [usize; 8] = [10, 3, 2, 4, 8, 7, 8, 9];

        let res = max(&start, &end);

        assert_eq!(res, 7);
    }
}
