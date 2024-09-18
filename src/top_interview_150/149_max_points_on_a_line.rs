impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..points.len() {
            let (x0, y0) = (points[i][0], points[i][1]);
            for j in (i + 1)..points.len() {
                let (x1, y1) = (points[j][0], points[j][1]);
                let mut count = 0;
                for k in (j + 1)..points.len() {
                    let (x2, y2) = (points[k][0], points[k][1]);
                    if (y0 - y1) * (x2 - x0) == (y2 - y0) * (x0 - x1) {
                        count += 1;
                    }
                }
                res = res.max(2 + count);
            }
        }
        match points.len() {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => res,
        }
    }
}
