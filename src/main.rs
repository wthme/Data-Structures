use std::cmp::Ordering;
type Node_Ref = Option<Box<Node>>;


enum RES {
    DONE,
    ALREADY,
    NO,
    YES,
}

#[derive(Debug)]
struct Node {
    element : i32,
    left : Node_Ref,
    right : Node_Ref,
    depth : u8,
}

#[derive(Debug)]
struct Binary_Tree {
    root : Node_Ref,
    size : u16,
}


impl Node {

fn new(new_elem : i32 , depth : u8) -> Self {

    Node { element: new_elem , left: None, right: None, depth : depth }

}

fn add (&mut self, new_elem: i32) -> RES {


    match &mut self.element.cmp(&new_elem) {

        Ordering::Less => {  if self.right.is_none() { 

                                        self.right = Some(Box::new(Node::new(new_elem, self.depth +1)));

                                        RES :: DONE } 
                                                        else {
                                                               if let Some(r_node) =  &mut self.right

                                                                    { r_node.add(new_elem)

                                                            }  else {RES::DONE}

                                                        }   

                                },

        Ordering::Greater => { if self.left.is_none() {
                            
                            self.left = Some(Box::new(Node::new(new_elem , self.depth + 1)));

                            RES :: DONE

                                        }  else {
                                                    if let Some(left_node) = &mut self.left{
                                                        left_node.add(new_elem)

                                                    } else {
                                                        RES::DONE
                                                    }

                                        }
        },
        Ordering::Equal => RES::ALREADY
    }
} 


}



impl Binary_Tree {

    fn new() -> Self {
        Binary_Tree { 
            root: None, 
            size: 0 }
    }

    fn add(&mut self, new_elem : i32) -> RES{

        if self.size == 0 {

            self.root = Some(Box::new(Node::new(new_elem, 0)));
            self.size +=1;
            RES :: DONE
        }

        else {
            self.size +=1;

            if let Some(node) = &mut self.root {
                node.add(new_elem)

            }  else {

                RES::DONE
            }
        }
    }



}




fn main () {
  
let mut btree = Binary_Tree::new();

    btree.add(34);
    btree.add(345);
    btree.add(67);
    btree.add(3);
    btree.add(88);

    dbg!(btree);
}   



mod bheap;