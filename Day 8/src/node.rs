pub struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>
}

impl Node {
    pub fn new(index: &mut usize, numbers: &Vec<i32>) -> Node {
        // println!("Children count index {}", index);
        let children_count = *numbers.get(*index).unwrap() as usize;
        // println!("Children count {}", children_count);
        *index += 1;
        // println!("Metadata count index {}", index);
        let metadata_count = *numbers.get(*index).unwrap() as usize;
        *index += 1;

        let mut children: Vec<Node> = Vec::with_capacity(children_count);
        for _ in 0..children_count {
            children.push(Node::new(index, numbers));
        }

        let mut metadata: Vec<i32> = Vec::with_capacity(metadata_count);
        for _ in 0..metadata_count {
            metadata.push(*numbers.get(*index).unwrap());
            *index += 1;
        }

        Node {
            children: children,
            metadata: metadata,
        }
    }

    pub fn all_metadata_sum(&self) -> i32 {
        let mut sum = 0;

        for number in &self.metadata {
            sum += number;
        }

        for child in &self.children {
            sum += child.all_metadata_sum();
        }

        sum
    }
}