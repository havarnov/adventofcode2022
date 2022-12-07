
#[derive(Debug, Clone)]
enum Tree {
    Dir { name: String, children: Vec<Box<Tree>>, size: usize },
    File { name: String, size: usize },
}

fn create_tree<'a>(lines: &'a [String], mut position: usize) -> (Vec<Box<Tree>>, usize) {
    let org = position;
    let mut children = Vec::new();
    loop {
        if position >= lines.len() {
            return (children, position - org + 1);
        } else if lines[position].starts_with("$ ls") {
            position += 1;
        } else if lines[position].starts_with("$ cd ..") {
            return (children, position - org + 1);
        } else if lines[position].starts_with("dir") {
            position += 1;
        } else if lines[position].starts_with("$ cd ") {
            let mut split = lines[position].split(" cd ");
            _ = split.next().expect("$");
            let name = split.next().expect("name").to_string();
            let (sub_children, taken) = create_tree(&lines, position + 1);
            let total = sub_children
                .iter()
                .fold(
                    0, 
                    |s, n| {
                        match **n {
                            Tree::Dir { size, .. } => s + size,
                            Tree::File { size, .. } => s + size,
                        }
                    });
            children.push(Box::new(Tree::Dir { name, children: sub_children, size: total, }));
            position += taken + 1;
        } else {
            let mut split = lines[position].split(" ");
            let size = split.next().expect("size").parse::<usize>().expect("parse size");
            let name = split.next().expect("name").to_string();
            children.push(Box::new(Tree::File { name, size }));
            position += 1;
        }
    }
}

fn find_at_most(tree: &Tree, at_most: usize) -> Vec<Tree> {
    let mut res = Vec::new();
    match *tree {
        Tree::Dir { size, ref children, .. } => {
            if size <= at_most {
                res.push(tree.clone());
            }

            for child in children.iter() {
                res.append(&mut find_at_most(child, at_most));
            }
        },
        _ => {},
    }

    res
}

fn find_at_least(tree: &Tree, at_least: usize) -> Vec<Tree> {
    let mut res = Vec::new();
    match *tree {
        Tree::Dir { size, ref children, .. } => {
            if size >= at_least {
                res.push(tree.clone());
            }

            for child in children.iter() {
                res.append(&mut find_at_least(child, at_least));
            }
        },
        _ => {},
    }

    res
}

fn main() {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>().expect("couldn't read lines");

    let (root, _) = create_tree(lines.as_slice(), 0);
    let res1 = 
        find_at_most(&root[0], 100000)
        .iter()
        .fold(
            0,
            |s, n| {
                match n {
                    Tree::Dir { size, .. } => s + size,
                    _ => s,
                }
            });

    println!("{}", res1);

    let used = match *root[0] { Tree::Dir { size, .. } => size, _ => panic!("root must be dir"), };
    let free = 70000000 - used;
    let required = 30000000 - free;

    let mut candidates = find_at_least(&root[0], required);

    candidates.sort_by(
        |a, b| {
            match (a, b) {
                (Tree::Dir { size: size_a, .. }, Tree:: Dir { size: size_b, .. }) => size_a.cmp(&size_b),
                _ => panic!("Only Dir"),
            }
        });

    match candidates.get(0) {
        Some(Tree::Dir { size, .. }) => println!("{:?}", size),
        _ => {},
    }
}
