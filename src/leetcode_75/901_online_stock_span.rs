struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1_i32;
        while let Some(&(p, num)) = self.stack.last() {
            if p > price {
                break;
            }
            self.stack.pop();
            res += num;
        }
        self.stack.push((price, res));
        res
    }
}
