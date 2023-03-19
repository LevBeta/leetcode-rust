impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut result: Vec<(usize, i32)> = nums.iter()
            .enumerate()
            .map(|(i, &num)| (i, num))
            .collect();

        result.sort_by_key(|&(_, num)| num);

        for (l, (i, num)) in result.iter().enumerate(){
            match result[l+1..].binary_search_by_key(&(target - *num), |&(_, num)| num) {
                Ok(index) => {   
                        return vec![*i as i32, result[index+l+1].0 as i32];
                },
                Err(_) => {}
            }
        }

        return vec![0,0];
    }
}
