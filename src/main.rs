#[derive(Debug)]
struct Heap {
    root_heap: Tree,
    size_heap : u16,
}

#[derive(Debug)]
struct Tree {
            root : Option<Box<Node>>, 
            size : u8,}


#[derive(Debug)]            
struct Node{
    element : Option<i32>,
    left : Tree,
    right : Tree,
}

enum DIR {
    Left,
    Right,
    Nowhere,
}


impl Tree{


fn new() -> Tree {
    Tree{ root : None,
          size : 0,}
    }



fn new_node(new_node : Node) -> Tree{

      Tree    { root : Some(Box::new(new_node)), 
                size : 0,}
    
}


fn value(&self) -> Option<i32>{
    if let Some(node) = &self.root{
        node.element 
    }
    else  {None}
}




fn balance_dir(&self) -> DIR {

    if let Some(node)   =   &self.root{

        if let None = node.element {

            match &node.left.root {

                None => {

                     if  let Some(_) = &node.right.root{

                        DIR::Right}

                        else { DIR :: Nowhere}
                }

                Some(left_node) => {

                    if let None = node.right.root {DIR::Left}  else {

                        match &node.right.root {

                            Some(right_node) =>  {

                                if  Some(left_node.element) > Some(right_node.element) {DIR::Left} 

                                     else {DIR::Right}
                            },

                            None => DIR::Left,

                        }
                    }
                }
            }
        }
        else {DIR::Nowhere}
    }
    else {DIR::Nowhere}
}



fn direction(&self ) -> DIR {

    match &self.root {

            Some(node) => {  
                    if node.left.size <= node.right.size {
                        println!(" DIRTCTINON LEFT!!!");
                        DIR::Left
                    }
                    else {
                        println!(" DIRTCTINON RIGHT!!!");
                        DIR::Right
                    }
            },
            _  =>    
                    {  println!(" NOWHERE");
                        DIR::Nowhere }
    }

}





fn balance(&mut self) {


    match self.balance_dir() {

        DIR::Left  =>  {

            
            if let Some(node) = self.root.as_mut() {

                   if let Some(node1) = &mut node.left.root{
                        std::mem :: swap (&mut Some(node.element) , &mut Some(node1.element));
                   }

                node.left.balance();
            }
            }

        DIR::Right =>  {

                self.size -= 1;

                if let Some(node) = &mut self.root {

                       if let Some(node1) = &mut node.right.root{
                        std::mem :: swap (&mut Some(node.element) , &mut Some(node1.element));
                       }

                    node.right.balance();
                }
        }

        DIR::Nowhere => {},
    }

}






fn add(&mut self, new_elem : i32) {

    let mut new_elem = new_elem;
    let direction = self.direction();

    match &mut self.root {

        
        None =>  self.root = Some(Box::new(Node::new(new_elem))),


        Some(node) =>    {
                        
                        self.size +=1;
                        
                        if new_elem > node.element.unwrap() { std::mem::swap (&mut new_elem , &mut node.element.unwrap()); }

                        match direction {
                                DIR::Right => node.right.add(new_elem),
                                DIR::Left =>  node.left.add(new_elem) , 
                                DIR::Nowhere => {},
                            }
                },  
        }                                                                           
}       

}





impl Node {

    fn new(new_element:i32 ) -> Node {
        Node {
            element : Some(new_element),
            left : Tree {root : None, size :0},
            right: Tree {root : None, size :0},
        }
    }
}


impl Heap {
    fn new() -> Self {
        Self {  root_heap : Tree::new(),
                size_heap : 0,}
        }


fn insert(&mut self , new_elem : i32){
        self.root_heap.add(new_elem);
        self.size_heap +=1;
    }


fn pop (&mut self) -> Option<i32> {

    let mut pop_item = None; 

        if let Some (node)  = &mut self.root_heap.root {

            pop_item = node.element.take();

            self.root_heap.balance();

            dbg!(&self.root_heap.root);
        }

    pop_item
 }

    
}



fn main () {
  
    let mut  heap = Heap::new();

    heap.insert(95);
    heap.insert(43);
    heap.insert(34);
    heap.insert(777);
    heap.insert(34);



    dbg!(heap);


    
}   