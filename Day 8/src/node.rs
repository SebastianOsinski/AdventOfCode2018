pub struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>
}

impl Node {
    pub fn new(numbers: &Vec<i32>) -> Node {
        Node::new_with_index(&mut 0, numbers)
    }

    fn new_with_index(index: &mut usize, numbers: &Vec<i32>) -> Node {
        let children_count = *numbers.get(*index).unwrap() as usize;
        *index += 1;
        let metadata_count = *numbers.get(*index).unwrap() as usize;
        *index += 1;

        let mut children: Vec<Node> = Vec::with_capacity(children_count);
        for _ in 0..children_count {
            children.push(Node::new_with_index(index, numbers));
        }

        let mut metadata: Vec<i32> = Vec::with_capacity(metadata_count);
        for _ in 0..metadata_count {
            metadata.push(*numbers.get(*index).unwrap());
            *index += 1;
        }

        Node {
            children,
            metadata,
        }
    }

    pub fn all_metadata_sum(&self) -> i32 {
        self.children.iter().fold(self.metadata_sum(), |sum, child| sum + child.all_metadata_sum())
    }

    pub fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            self.metadata.iter().fold(0, { |sum, index| 
                sum + self.children.get(*index as usize - 1).map_or(0, Node::value)
            })
        }
    }

    fn metadata_sum(&self) -> i32 {
        self.metadata.iter().fold(0, |sum, m| sum + m)
    }
}