use std::collections::HashMap;
impl Solution {
    pub fn is_number(s: String) -> bool {
        if s.is_empty() {return false;}
        let trans_table = [
            [1, 2, 7, -1, -1, 0],
            [-1, 2, 7, -1, -1, -1],
            [-1, 2, 3, 4, -1, 9],
            [-1, 3, -1, 4, -1, 9],
            [6, 5, -1, -1, -1, -1],
            [-1, 5, -1, -1, -1, 9],
            [-1, 5, -1, -1, -1, -1],
            [-1, 8, -1, -1, -1, -1],
            [-1, 8, -1, 4, -1, 9],
            [-1, -1, -1, -1, -1, 9],
        ];
        let cols = vec![("sign",0),("number",1),(".",2),("exp",3),("other",4),("blank",5)]
            .iter()
            .map(|x| *x)
            .collect::<HashMap<&str,usize>>();
        let legal_state = [2,3,5,8,9];
        let mut state = 0_i32;
        for i in s.chars() {
            let str = match i {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'=> "number",
                '+' | '-' => "sign",
                '.' => ".",
                ' ' => "blank",
                'E' | 'e' => "exp",
                _ => "other",
            };
            state = trans_table[state as usize][cols[str]];
            if state == -1 {
                return false;
            }
        }
        legal_state.contains(&state)
    }
}
