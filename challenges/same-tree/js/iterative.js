var isSameTree = function (p, q) {
    if (!p && !q) return true;

    let one = [];
    let two = [];

    one.push(p); two.push(q);
    while (one.length && two.length) {
        let a = one.pop();
        let b = two.pop();

        if (!a && !b) continue;
        if ((!a && b) || (a && !b)) return false;
        if (a.val !== b.val) return false;

        one.push(a.right); two.push(b.right);
        one.push(a.left); two.push(b.left);
    }

    return true;
};