pub struct RecentCounter {
    call_time: Vec<i32>,
    left: i32,
    right: i32,
}

impl RecentCounter {
    pub fn new() -> Self {
        Self {
            call_time: vec![],
            left: 0,
            right: 0,
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.call_time.push(t);
        loop {
            if self.call_time[self.left as usize] < self.call_time[self.right as usize] - 3000 {
                self.left += 1;
            } else {
                break;
            }
        }
        self.right += 1;
        self.right - self.left
    }
}
