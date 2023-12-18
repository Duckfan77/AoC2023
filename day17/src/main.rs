use petgraph::{algo::dijkstra, graph::NodeIndex, Graph};

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct NodeLayer(Vec<Vec<NodeIndex>>);

impl NodeLayer {
    fn make_node_layer(map: &mut Graph<i32, i32>, input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| map.add_node(c.to_digit(10).unwrap() as i32))
                        .collect()
                })
                .collect(),
        )
    }

    fn lookup_node(&self, row: usize, col: usize) -> Option<&NodeIndex> {
        self.0.get(row).and_then(|r| r.get(col))
    }

    fn lookup_adjacent(&self, row: usize, col: usize, dir: Direction) -> Option<&NodeIndex> {
        match dir {
            Direction::Up => {
                if let Some(row) = row.checked_sub(1) {
                    self.lookup_node(row, col)
                } else {
                    None
                }
            }
            Direction::Down => self.lookup_node(row + 1, col),
            Direction::Left => {
                if let Some(col) = col.checked_sub(1) {
                    self.lookup_node(row, col)
                } else {
                    None
                }
            }
            Direction::Right => self.lookup_node(row, col + 1),
        }
    }
}

struct Map {
    map: Graph<i32, i32>,
    up_nodes: Vec<NodeLayer>,
    down_nodes: Vec<NodeLayer>,
    left_nodes: Vec<NodeLayer>,
    right_nodes: Vec<NodeLayer>,
    start: NodeIndex,
    end: NodeIndex,
    nlayers: usize,
    nrows: usize,
    ncols: usize,
}

impl Map {
    fn with_layers(layers: usize, input: &str) -> Self {
        let mut map = Graph::new();
        let up_nodes = (0..layers)
            .map(|_| NodeLayer::make_node_layer(&mut map, input))
            .collect();
        let down_nodes = (0..layers)
            .map(|_| NodeLayer::make_node_layer(&mut map, input))
            .collect();
        let left_nodes = (0..layers)
            .map(|_| NodeLayer::make_node_layer(&mut map, input))
            .collect();
        let right_nodes = (0..layers)
            .map(|_| NodeLayer::make_node_layer(&mut map, input))
            .collect();
        let start = map.add_node(
            input
                .lines()
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as i32,
        );
        let end = map.add_node(0);
        Self {
            map,
            up_nodes,
            down_nodes,
            left_nodes,
            right_nodes,
            start,
            end,
            nlayers: layers,
            nrows: input.lines().count(),
            ncols: input.lines().next().unwrap().chars().count(),
        }
    }

    fn add_edge(
        &mut self,
        start_idx: NodeIndex,
        end_idx: NodeIndex,
    ) -> petgraph::prelude::EdgeIndex {
        self.map.add_edge(start_idx, end_idx, self.map[end_idx])
    }

    fn construct_part1_edges(&mut self) {
        use Direction as Dir;
        assert_eq!(self.nlayers, PART1_LAYERS);

        // special case, start node isn't in any layers
        {
            let down_node = *self.down_nodes[0].lookup_adjacent(0, 0, Dir::Down).unwrap();
            let right_node = *self.right_nodes[0]
                .lookup_adjacent(0, 0, Dir::Right)
                .unwrap();
            self.add_edge(self.start, down_node);
            self.add_edge(self.start, right_node);
        }

        // special case, end node isn't in any layers
        for layer in 0..self.nlayers {
            self.add_edge(
                *self.up_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
            self.add_edge(
                *self.down_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
            self.add_edge(
                *self.left_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
            self.add_edge(
                *self.right_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
        }

        // handle nodes going up
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.up_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(lnode) = self.left_nodes[0].lookup_adjacent(row, col, Dir::Left) {
                        self.add_edge(start, *lnode);
                    }
                    if let Some(rnode) = self.right_nodes[0].lookup_adjacent(row, col, Dir::Right) {
                        self.add_edge(start, *rnode);
                    }
                    if let Some(unode) = self
                        .up_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Up))
                    {
                        self.add_edge(start, *unode);
                    }
                }
            }
        }

        // handle nodes going down
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.down_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(lnode) = self.left_nodes[0].lookup_adjacent(row, col, Dir::Left) {
                        self.add_edge(start, *lnode);
                    }
                    if let Some(rnode) = self.right_nodes[0].lookup_adjacent(row, col, Dir::Right) {
                        self.add_edge(start, *rnode);
                    }
                    if let Some(dnode) = self
                        .down_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Down))
                    {
                        self.add_edge(start, *dnode);
                    }
                }
            }
        }

        // handle nodes going left
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.left_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(unode) = self.up_nodes[0].lookup_adjacent(row, col, Dir::Up) {
                        self.add_edge(start, *unode);
                    }
                    if let Some(dnode) = self.down_nodes[0].lookup_adjacent(row, col, Dir::Down) {
                        self.add_edge(start, *dnode);
                    }
                    if let Some(lnode) = self
                        .left_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Left))
                    {
                        self.add_edge(start, *lnode);
                    }
                }
            }
        }

        // handle nodes going right
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.right_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(unode) = self.up_nodes[0].lookup_adjacent(row, col, Dir::Up) {
                        self.add_edge(start, *unode);
                    }
                    if let Some(dnode) = self.down_nodes[0].lookup_adjacent(row, col, Dir::Down) {
                        self.add_edge(start, *dnode);
                    }
                    if let Some(rnode) = self
                        .right_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Right))
                    {
                        self.add_edge(start, *rnode);
                    }
                }
            }
        }
    }

    fn construct_part2_edges(&mut self) {
        use Direction as Dir;
        assert_eq!(self.nlayers, PART2_LAYERS);

        // special case, start node isn't in any layers
        {
            let down_node = *self.down_nodes[0].lookup_adjacent(0, 0, Dir::Down).unwrap();
            let right_node = *self.right_nodes[0]
                .lookup_adjacent(0, 0, Dir::Right)
                .unwrap();
            self.add_edge(self.start, down_node);
            self.add_edge(self.start, right_node);
        }

        // special case, end node isn't in any layers
        for layer in PART2_CANT_TURN..self.nlayers {
            self.add_edge(
                *self.up_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
            self.add_edge(
                *self.down_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
            self.add_edge(
                *self.left_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
            self.add_edge(
                *self.right_nodes[layer]
                    .lookup_node(self.nrows - 1, self.ncols - 1)
                    .unwrap(),
                self.end,
            );
        }

        // handle nodes going up
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.up_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(unode) = self
                        .up_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Up))
                    {
                        self.add_edge(start, *unode);
                    }
                    if layer >= PART2_CANT_TURN {
                        if let Some(lnode) = self.left_nodes[0].lookup_adjacent(row, col, Dir::Left)
                        {
                            self.add_edge(start, *lnode);
                        }
                        if let Some(rnode) =
                            self.right_nodes[0].lookup_adjacent(row, col, Dir::Right)
                        {
                            self.add_edge(start, *rnode);
                        }
                    }
                }
            }
        }

        // handle nodes going down
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.down_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(dnode) = self
                        .down_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Down))
                    {
                        self.add_edge(start, *dnode);
                    }
                    if layer >= PART2_CANT_TURN {
                        if let Some(lnode) = self.left_nodes[0].lookup_adjacent(row, col, Dir::Left)
                        {
                            self.add_edge(start, *lnode);
                        }
                        if let Some(rnode) =
                            self.right_nodes[0].lookup_adjacent(row, col, Dir::Right)
                        {
                            self.add_edge(start, *rnode);
                        }
                    }
                }
            }
        }

        // handle nodes going left
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.left_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(lnode) = self
                        .left_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Left))
                    {
                        self.add_edge(start, *lnode);
                    }
                    if layer >= PART2_CANT_TURN {
                        if let Some(unode) = self.up_nodes[0].lookup_adjacent(row, col, Dir::Up) {
                            self.add_edge(start, *unode);
                        }
                        if let Some(dnode) = self.down_nodes[0].lookup_adjacent(row, col, Dir::Down)
                        {
                            self.add_edge(start, *dnode);
                        }
                    }
                }
            }
        }

        // handle nodes going right
        for layer in 0..self.nlayers {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    let start = *self.right_nodes[layer].lookup_node(row, col).unwrap();
                    if let Some(rnode) = self
                        .right_nodes
                        .get(layer + 1)
                        .and_then(|layer| layer.lookup_adjacent(row, col, Dir::Right))
                    {
                        self.add_edge(start, *rnode);
                    }
                    if layer >= PART2_CANT_TURN {
                        if let Some(unode) = self.up_nodes[0].lookup_adjacent(row, col, Dir::Up) {
                            self.add_edge(start, *unode);
                        }
                        if let Some(dnode) = self.down_nodes[0].lookup_adjacent(row, col, Dir::Down)
                        {
                            self.add_edge(start, *dnode);
                        }
                    }
                }
            }
        }
    }

    fn find_cost(&self) -> i32 {
        *dijkstra(&self.map, self.start, Some(self.end), |edge| *edge.weight())
            .get(&self.end)
            .unwrap()
    }
}

static PART1_LAYERS: usize = 3;

fn part1(text: &str) {
    let mut map = Map::with_layers(PART1_LAYERS, text);
    map.construct_part1_edges();
    println!("{}", map.find_cost());
}

static PART2_LAYERS: usize = 10;
static PART2_CANT_TURN: usize = 3;

fn part2(text: &str) {
    let mut map = Map::with_layers(PART2_LAYERS, text);
    map.construct_part2_edges();
    println!("{}", map.find_cost());
}
