class Solution:
    def judgeCircle(self, moves: str) -> bool:
        x, y= 0,0
        for i in moves:
            if i == 'U':
                x += 1
            if i == 'D':
                x -= 1
            if i == 'L':
                y += 1
            if i == 'R':
                y -= 1
        return x == 0 and y == 0
