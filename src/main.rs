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

                                if  left_node.element > right_node.element {DIR::Left} 

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



fn direction( &self ) -> DIR {

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

            self.size -= 1;

            if let Some(node) = self.root.as_mut() {

                   if let Some(node_left) = &mut node.left.root {

                        std::mem :: swap (&mut node.element , &mut node_left.element);
                        node.left.balance();
                   }
                   
            }
        },

        DIR::Right =>  {

            self.size -= 1;

            if let Some(node) = &mut self.root {

                    if let Some(node_right) = &mut node.right.root{

                        std::mem :: swap (&mut node.element , &mut node_right.element);
                        node.right.balance();

                       }
                }
        }

        DIR::Nowhere => {},
    }

}






fn add(&mut self, new_elem : i32) {

    self.size +=1;
    let mut new_elem = Some(new_elem);
    let direction = self.direction();

    match &mut self.root {

        
        None =>  self.root = Some(Box::new(Node::new(new_elem.unwrap()))) ,


        Some(node) =>   {

                if node.element.is_none() { node.element = new_elem;} else {

                    if new_elem > node.element { std::mem::swap (&mut new_elem , &mut node.element  ); }

                    match direction {
                            DIR::Right => node.right.add(new_elem.unwrap()),
                            DIR::Left =>  node.left.add(new_elem.unwrap()) , 
                            DIR::Nowhere => {},
                        }

                }
                                                                 
                },  
        }                                                                           
}  




fn remove (&mut self) -> Option<i32>{

    let removing_element = match &mut self.root {

        Some(node) => node.element.take() ,

        None => None,
    };

    self.balance();

    removing_element
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

let mut result = None;    

if self.size_heap > 0  {

    self.size_heap -=1;

    result =  match &mut self.root_heap.root {

        Some (_) => self.root_heap.remove(),   
        None => None,
    };
}

result

}

    
}



fn main () {
  
    let mut  heap = Heap::new();

    heap.insert(34);
    heap.insert(43);
    heap.insert(95);
    heap.insert(777);
    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());
    
    heap.insert(228);
    heap.insert(17);
    heap.insert(88);


    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());
    dbg!(heap.pop());


    heap.insert(1);
    heap.insert(546);
    heap.insert(17);
    heap.insert(88);

    dbg!(heap.pop());
    dbg!(heap.pop());

    dbg!(heap);


    
}   