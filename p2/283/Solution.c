void moveZeroes(int* nums, int numsSize){
    int pos = 0;
    for (int i = 0; i < numsSize; i++) {
         if (nums[i] != 0) {
             int t = nums[i];
             nums[i] = nums[pos];
             nums[pos++] = t;     
         }
    }        
}
