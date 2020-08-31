class Solution {
    public boolean canVisitAllRooms(List<List<Integer>> rooms) {
        boolean[] s = new boolean[rooms.size()];        
        searchRoom(s, rooms, 0);
        for (boolean f: s) {
            if (!f) return false;
        }
        return true;
    }
    
    void searchRoom(boolean[] s, List<List<Integer>> r,int n) {
        if (s[n] != true) {
            s[n] = true;
            for (Integer i: r.get(n))
                searchRoom(s, r,i);
        } 
    }
}
