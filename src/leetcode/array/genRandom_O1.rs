use std::collections::HashMap;
use rand::Rng; 


/**
 * I never got this one finished
 * I think it requiers double linkage
 * when you delete from the middle of a list but need to maintain order
 *  you have to get the current node and then make its parent point to prev
 * Its a list with a head and a tail, doubly linked
 */
struct RandomizedSet {
    xs: HashMap<i32, usize>,
    arr: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet {
            xs: HashMap::new(),
            arr: vec![],
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.xs.contains_key(&val) {
            return false;
        }
        self.arr.push(val);
        let n = self.arr.len();
        self.xs.insert(val, n-1);
        // println!("arr {:?} xs {:?}", self.arr, self.xs);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if !self.xs.contains_key(&val) {
            return false;
        }
        let i = self.xs[&val];
        let n = self.arr.len();
        // (self.arr[i], self.arr[n-1]) = (self.arr[n-1], self.arr[i]);
        let tail = self.arr[n-1];
        // self.arr[n-1] = self.arr[i];
        // self.arr[i] = tmp;
        self.arr.swap(i, n-1);
        self.arr.pop();
        self.xs.remove(&val);
        if val != tail {
            self.xs.insert(tail, i);
        }
        // println!("arr {:?} xs {:?}", self.arr, self.xs);
        true
    }
    
    fn get_random(&self) -> i32 {
        if self.arr.len() <= 1 {
            return self.arr[0];
        }
        let mut rng = rand::thread_rng();

        let n = self.arr.len();
        let i = rng.gen_range(1..=n - 1) as usize;
        self.arr[i]
    }
}