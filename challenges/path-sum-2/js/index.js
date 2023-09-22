/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} targetSum
 * @return {number[][]}
 */
var pathSum = function (root, targetSum) {
    return pathSumHelper(root, targetSum, [], []);
};

function pathSumHelper(root, targetSum, acc, slate) {
    if (root === null) return slate;

    acc.push(root.val);
    if (root.left == null && root.right == null) {
        //we are at a leaf node
        if (targetSum - root.val == 0) {
            slate.push(Array.from(acc));
        }
        acc.pop();
        return slate;
    }

    pathSumHelper(root.left, targetSum - root.val, acc, slate)
    pathSumHelper(root.right, targetSum - root.val, acc, slate)
    acc.pop()
    return slate
}