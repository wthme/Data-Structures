

fn main () {
  
    let mut  heap = binary_heap::Heap::new();
    let mut input = std::io::stdin().lines();
    let mut n : i32 = input.next().unwrap().unwrap().trim().parse().unwrap();



    while n > 0{
    
        	if let Some(line) = input.next(){

                if let  Ok(str)  = line {
            
                    let mut iter      = str.split_whitespace();
        
                    if let Some("0")  =  iter.next() {
                        let num : i32 = iter.next().unwrap().parse().unwrap();
                        heap.insert(num);
                    }
        
                    else {
                        println!("{}" , heap.pop().unwrap_or_default());
                    }
            }
        }

        n -= 1; 

    }


}



mod binary_heap;