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
 * @return {number}
 */
var sumOfLeftLeaves = function (root) {

    return depthSum(root, 0, false);

};

var depthSum = function (root, depth, isLeft) {
    if (root == null) return 0;

    if (root.left == null && root.right == null) {
        return depth === 0 ? 0 : isLeft ? root.val : 0;
    }
    return depthSum(root.left, depth + 1, true) + depthSum(root.right, depth + 1, false);
}