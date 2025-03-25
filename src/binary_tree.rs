use std::cmp::Ordering;
type NodeRef = Option<Box<Node>>;


pub enum RES {
    DONE,
    ALREADY,
    NO,
    YES,
}

#[derive(Debug)]
struct Node {
    element : i32,
    left : NodeRef,
    right : NodeRef,
    depth : u8,
}

#[derive(Debug)]
pub struct BinaryTree {

    root : NodeRef,
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



impl BinaryTree {

    pub fn new() -> Self {
        BinaryTree { 
            root: None, 
            size: 0 }
    }

    pub fn add(&mut self, new_elem : i32) -> RES{

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
