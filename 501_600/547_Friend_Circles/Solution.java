class Solution {

  public class UF {
    public int[] id;
    public int[] e;
    int size;

    public UF(int N) {
      id = new int[N];
      e = new int[N];
      for (int i = 0; i < N; ++i) {
        id[i] = i;
      }
      Arrays.fill(e, 1);
      size = N;
    }

    public void union(int p, int q) {
      int i = find(p);
      int j = find(q);
      if (i != j) {
        if (e[i] < e[j]) {
          id[i] = j;
          e[j] += e[i];
        } else {
          id[j] = i;
          e[i] += e[j];
        }
        size--;
      }
    }

    int find(int p) {
      while (id[p] != p) {
        id[p] = id[id[p]];
        p = id[p];
      }
      return p;
    }

    public boolean connected(int p, int q) {
      return find(p) == find(q);
    }

    public int count() {
      return size;
    }
  }

  public int findCircleNum(int[][] M) {
    UF u = new UF(M.length);
    int l = M.length;
    for (int i = 0; i < l; ++i) {
      for (int j = i + 1; j < l; ++j) {
        if (M[i][j] == 1) {
          u.union(i, j);
        }
      }
    }
    return u.count();
  }
}