/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var moveZeroes = function (nums) {
    let end = nums.length - 1;
    let current_index = 0;
    let starting_index = 0;
    while (starting_index < end) {
        if (nums[current_index] === 0) {
            // find the next value that is non-zero
            while (current_index < end && nums[current_index] === 0) {
                current_index++;
            }
            // Current index points to the next non-zero number, swap
            nums[starting_index] = nums[current_index]
            nums[current_index] = 0;
        }
        starting_index++;
        current_index = starting_index
    }
};


/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var moveZeroes = function (nums) {
    const swap = (a, b) => {
        let tmp = nums[a];
        nums[a] = nums[b];
        nums[b] = tmp;
    }
    for (let lastNonZeroIdx = 0, curr = 0; curr < nums.length; curr++) {
        if (nums[curr] != 0) {
            swap(lastNonZeroIdx++, curr)
        }
    }
};

