// Kadenes Algorithm is an algorithm which finds the maximum or minimum sum
// of a contiguous subarray within a given array

pub fn kadenes_algorithm(v: &[i64], n: usize) -> (usize, usize, i64) {
    let mut maxsum: i64 = v[0];
    let mut cursum: i64 = v[0];
    let mut l: usize = 0;
    let mut r: usize = 0;
    for i in 1usize..n {
        cursum += v[i];
        if cursum > maxsum {
            maxsum = cursum;
            r = i;
        }
        if cursum < 0 {
            cursum = 0;
            l = i + 1;
            r = i + 1;
        }
    }
    (l, r, maxsum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut token = crate::Scanner::new(std::io::stdin().lock());
        let n: usize = token.next();
        let mut v: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(token.next());
        }
        let (l, r, sum) = kadenes_algorithm(&v, n);
        println!("[{}..={}] = {}", l, r, sum);
    }
}

/*
8
1 3 -4 8 2 -5 0 10
*/