class Solution {
    public int surfaceArea(int[][] grid) {
        int size = grid.length; 
        int places = 0;
        for (int i = 0; i < size; i++) {
            for (int j = 0;j < size; j++) {
                if (grid[i][j] > 0) {
                    places += 2;
                }
                places += i >= 1 ? Math.max(0, grid[i][j] - grid[i- 1][j]) : grid[i][j];
                places += j >= 1 ? Math.max(0, grid[i][j] - grid[i][j - 1]) : grid[i][j];
                places += i < size - 1 ? Math.max(grid[i][j] - grid[i + 1][j], 0): grid[i][j];                
                places += j < size - 1 ? Math.max(grid[i][j] - grid[i][ j + 1], 0): grid[i][j];  
            }
        }
        
        return places;
    }
}
