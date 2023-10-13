
fn override(number: i32) {
     number + 1
}

fn override(text: &str) {
     text.to_uppercase()
}

pub fn basic() {
     // 10 elements, all zeros - array
     let array_with_same_elements = [0; 10];
     // First three elements
     let subset = &[0 .. 3];
     // First four elements
     let subset = &[0 ..=3];

     let ref1 = &subset;
     let ref2 = &subset;
     // Ref1 and ref2 go out of scope here
     println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
     // Afterwards, we can now assign a mutable reference
     // This will not violate the rule of having both
     // mutable and immutable references since ref1 and 2
     // are no longer in scope
     let ref3 = &mut subset;

     // Data should not change when immutable references are in scope
     let mut heap_num = vec![1, 2, 3];
     let ref1 = &heap_num;
     let ref2 = &heap_num;

     // Cannot borrow `heap_num` as mutable because it is also borrowed as immutable
     heap_num.push(68);
     println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
}