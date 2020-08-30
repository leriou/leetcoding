impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut x, mut y) = (0,0);
        for i in moves.bytes() {
            let c = i as char;
            match c {
                'U' => x+=1,
                'D' => x-=1,
                'R' => y+=1,
                'L' => y-=1,
                _ => (),
            }
        }
        x == 0 && y == 0
    }
}
