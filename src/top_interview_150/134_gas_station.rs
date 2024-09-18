impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut record = vec![0; gas.len()];
        let mut cur = 0;
        for i in 0..gas.len() {
            record[i] = cur;
            cur += gas[i];
            cur -= cost[i];
        }
        record[0] = cur;
        let min_pos = (0..record.len()).min_by_key(|&i| record[i]).unwrap();
        let (mut pos, mut cur) = (min_pos, 0);
        for _ in 0..gas.len() {
            cur += gas[pos];
            cur -= cost[pos];
            if cur < 0 {
                return -1;
            }
            pos = (pos + 1) % gas.len();
        }
        min_pos as i32
    }
}
