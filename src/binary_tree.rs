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
    element: i32,
    left: NodeRef,
    right: NodeRef,
    depth: u8,
}

#[derive(Debug)]
pub struct BinaryTree {
    root: NodeRef,
    size: u16,
}

impl Node {
    fn new(new_elem: i32, depth: u8) -> Self {
        Node {
            element: new_elem,
            left: None,
            right: None,
            depth: depth,
        }
    }

    fn found(&self, elem: i32) -> RES {
        match &elem.cmp(&self.element) {
            Ordering::Less => {
                if let Some(left_node) = &self.left {
                    left_node.found(elem)
                } else {
                    RES::NO
                }
            }

            Ordering::Greater => {
                if let Some(right_node) = &self.right {
                    right_node.found(elem)
                } else {
                    RES::NO
                }
            }

            Ordering::Equal => RES::YES,
        }
    }

    fn add(&mut self, new_elem: i32) -> RES {
        match &mut self.element.cmp(&new_elem) {
            Ordering::Less => {
                if self.right.is_none() {
                    self.right = Some(Box::new(Node::new(new_elem, self.depth + 1)));

                    RES::DONE
                } else {
                    if let Some(r_node) = &mut self.right {
                        r_node.add(new_elem)
                    } else {
                        RES::DONE
                    }
                }
            }

            Ordering::Greater => {
                if self.left.is_none() {
                    self.left = Some(Box::new(Node::new(new_elem, self.depth + 1)));

                    RES::DONE
                } else {
                    if let Some(left_node) = &mut self.left {
                        left_node.add(new_elem)
                    } else {
                        RES::DONE
                    }
                }
            }
            Ordering::Equal => RES::ALREADY,
        }
    }

    fn print(&self) {
        if self.left.is_some() {
            if let Some(left_node) = &self.left {
                left_node.print();
            }
        }

        let mut coma = self.depth;

        while coma > 0 {
            print!(".");
            coma -= 1;
        }

        println!("{}", self.element);

        if self.right.is_some() {
            if let Some(right_node) = &self.right {
                right_node.print();
            }
        }
    }
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree {
            root: None,
            size: 0,
        }
    }

    pub fn add(&mut self, new_elem: i32) -> RES {
        if self.size == 0 {
            self.root = Some(Box::new(Node::new(new_elem, 0)));
            self.size += 1;
            RES::DONE
        } else {
            self.size += 1;

            if let Some(node) = &mut self.root {
                node.add(new_elem)
            } else {
                RES::DONE
            }
        }
    }

    fn print(&self) {
        if self.size > 0 {
            if let Some(sas) = &self.root {
                sas.print();
            }
        }
    }

    fn search(&self, element: i32) {
        if self.size == 0 {
            println!("NO");
        } else {
            if let Some(root) = &self.root {
                match root.found(element) {
                    RES::YES => println!("YES"),
                    RES::NO => println!("NO"),
                    _ => (),
                }
            }
        }
    }
}
