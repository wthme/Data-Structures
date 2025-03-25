use binary_tree::*;



fn main () {
  
let mut btree = BinaryTree::new();

    btree.add(34);
    btree.add(345);
    btree.add(67);
    btree.add(3);
    btree.add(88);

    dbg!(btree);
}   



mod bheap;
mod binary_tree;