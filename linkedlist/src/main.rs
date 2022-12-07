#[derive(Debug, Clone)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32, next: Box<Node>) -> Self {
        Self {
            data,
            next: Some(next),
        }
    }

    fn get_next(&self) -> &Box<Node> {
        self.next.as_ref().unwrap()
    }

    fn get_data(&self) -> i32 {
        self.data
    }

    fn display(&self) {
        println!("{}", self.data);
    }
}
fn main() {
    let node1 = Node::new(
        1,
        Box::new(Node {
            data: 2,
            next: Some(Box::new(Node {
                data: 2,
                next: None,
            })),
        }),
    );

    println!("{}", node1.get_data());
    println!("{}", node1.get_next().get_data());
    println!("{:?}", node1.display());
}

// Array
fn array() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    arr[0] = 10;
    println!("{:?}", arr);


    todo!()
}
