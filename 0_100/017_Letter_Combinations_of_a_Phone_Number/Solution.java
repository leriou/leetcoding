class Solution {
    public List<String> letterCombinations(String digits) {
        if (digits.length() == 0) return new LinkedList<>();
        String[] dics = {"","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"};
        return Arrays.asList(digits.split(""))
                 .parallelStream()
                 .map((x) -> Arrays.asList(dics[Integer.parseInt(x)].split("")))
                .reduce((a, b) -> {
                    List<String> t = new LinkedList<>();
                    for (String x : a) {
                        for (String y : b) {
                            t.add(x.concat(y));
                        }
                    }
                    return t;
                }).get();
    }
}
