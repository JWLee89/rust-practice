
// pub struct Node<T> {
//     pub data: T,
//     pub next: Option<Box<Node<T>>>,
// }

// pub struct LinkedList<T> {
//     pub head: Option<Box<Node<T>>>,
// }

// impl<T> LinkedList<T> {
//     fn new() -> Self {
//         LinkedList { head: None }
//     }
//     fn add(&mut self, val: &T) {
//         let node = Box::new(Node {
//             data: val,
//             next: None,
//         });
//         if self.head.is_none() {
//             println!("Head is none. Adding new node");
//             self.head = node;
//         } else {
//             // Traverse to last node
//             while self.head.next.is_some() {
//                 self.head = self.head.next;
//             }
//             self.head.next = Some(Box::new(node));
//         }
//     }
//     /**
//      * Delete a node from the linked list
//      * @param val: value to delete
//      * @return void
//      */
//     fn delete(&mut self, val: &T) {
//         if self.head.is_none() {
//             println!("Head is none. Nothing to delete");
//             return;
//         } 
//         let dummy = None;
//         let current = self.head;
//         while current.is_some() {
//             if current.data == val {
//                 println!("Found node to delete");
//                 dummy.next = current.next;
//                 return;
//             }
//             dummy = current;
//             current = current.next;
//         }
//     }
//     fn print(&self) {
//         let current = self.head;
//         while current.is_some() {
//             println!("Node: {:?}", current.data);
//             current = current.next;
//         }
//     }
// }

