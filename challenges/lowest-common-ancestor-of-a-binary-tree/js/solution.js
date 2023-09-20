/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {
    if (root === null) return root;
    if (root === p || root === q) return root;

    let leftRoot = lowestCommonAncestor(root.left, p, q);
    let rightRoot = lowestCommonAncestor(root.right, p, q);

    if (!leftRoot) return rightRoot;
    if (!rightRoot) return leftRoot;
    return root;
};

