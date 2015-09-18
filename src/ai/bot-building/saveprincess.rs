use std::{io, fmt, iter, ops, str};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y }
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Coord {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let cells: Vec<Vec<Cell>> =
            iter::repeat(
                iter::repeat(Cell::Empty)
                .take(width)
                .collect())
            .take(height)
            .collect();

        Board {
            width: width,
            height: height,
            cells: cells,
        }
    }

    fn iter(&self) -> BoardIterator {
        BoardIterator::new(self)
    }

    fn position<P>(&self, predicate: P) -> Option<Coord>
        where P: Fn(Cell) -> bool
    {
        for (idx, cell) in self {
            if predicate(cell) {
                return Some(idx);
            }
        }
        None
    }

    fn path(&self, pos1: Coord, pos2: Coord) -> Vec<Direction> {
        let delta = pos1 - pos2;
        let horiz = if delta.x > 0 { Direction::Left } else { Direction::Right };
        let vert = if delta.y > 0 { Direction::Up } else { Direction::Down };

        let mut path = vec![];
        for _ in 0..delta.x.abs() {
            path.push(horiz);
        }
        for _ in 0..delta.y.abs() {
            path.push(vert);
        }

        path
    }
}

impl ops::Index<Coord> for Board {
    type Output = Cell;

    fn index(&self, c: Coord) -> &Cell {
        self.cells.index(c.y as usize).index(c.x as usize)
    }
}

impl ops::IndexMut<Coord> for Board {
    fn index_mut(&mut self, c: Coord) -> &mut Cell {
        self.cells.index_mut(c.y as usize).index_mut(c.x as usize)
    }
}

impl str::FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Board, ()> {
        let lines: Vec<&str> = s.split('\n').filter(|l| l.trim().len() > 0).collect();
        let width = lines[0].len();
        let height = lines.len();

        let mut board = Board::new(width, height);

        for (y, l) in lines.iter().enumerate() {
            assert_eq!(l.len(), width);
            for (x, c) in l.chars().enumerate() {
                board[Coord::new(x as i32, y as i32)] = match c {
                    '-' => Cell::Empty,
                    'p' => Cell::Princess,
                    'm' => Cell::Bot,
                    _ => panic!(format!("Unknown cell: '{:?}'.", c)),
                }
            }
        }

        Ok(board)
    }
}

impl<'a> iter::IntoIterator for &'a Board {
    type Item = (Coord, Cell);
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct BoardIterator<'a> {
    board: &'a Board,
    x: i32,
    y: i32,
    done: bool,
}

impl<'a> BoardIterator<'a> {
    fn new(board: &'a Board) -> BoardIterator<'a> {
        BoardIterator { board: board, x: 0, y: 0, done: false }
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Coord, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let idx = Coord::new(self.x, self.y);
            let res = (idx, self.board[idx]);
            self.x += 1;
            if self.x as usize >= self.board.width {
                self.x = 0;
                self.y += 1;
            }
            if self.y as usize >= self.board.height {
                self.done = true;
            }
            Some(res)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Princess,
    Bot,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt.write_str(match *self {
            Direction::Up => "UP",
            Direction::Right => "RIGHT",
            Direction::Down => "DOWN",
            Direction::Left => "LEFT",
        }));
        Ok(())
    }
}

fn read_line<T>() -> T
    where T: str::FromStr + fmt::Debug,
          T::Err: fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_lines<T>() -> T
    where T: str::FromStr + fmt::Debug,
          T::Err: fmt::Debug,
{
    let size: usize = read_line();
    let mut acc = String::new();
    for _ in 0..size {
        let line: String = read_line();
        acc = acc + &line[..] + "\n";
    }
    acc.parse().unwrap()
}

fn find_path(board: &Board) -> Vec<Direction> {
    let princess_position = board.position(|c| c == Cell::Princess).unwrap();
    let bot_position = board.position(|c| c == Cell::Bot).unwrap();
    board.path(princess_position, bot_position)
}

fn main() {
    let board: Board = read_lines();
    let path = find_path(&board);
    for dir in path {
        println!("{}", dir);
    }
}


#[test]
fn test_sample() {
    let board = Board { width: 3, height: 3, cells: vec![
        vec![Cell::Princess, Cell::Empty, Cell::Empty],
        vec![Cell::Empty, Cell::Bot, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Empty],
    ]};
    assert_eq!(find_path(&board), vec![Direction::Right, Direction::Down]);
}

#[test]
fn test_indexing() {
    let mut board = Board::new(3, 3);
    assert_eq!(board[Coord::new(1, 2)], Cell::Empty);
    board[Coord::new(1, 2)] = Cell::Princess;
    assert_eq!(board[Coord::new(1, 2)], Cell::Princess);
}

#[test]
fn test_from_str() {
    let input = "\
        p--\n\
        -m-\n\
        ---\n\
    ";
    let actual: Board = input.parse().unwrap();
    let mut expected = Board::new(3, 3);
    expected[Coord::new(0, 0)] = Cell::Princess;
    expected[Coord::new(1, 1)] = Cell::Bot;

    assert_eq!(actual, expected);
}

#[test]
fn test_position() {
    let input = vec![
        "---",
        "--m",
        "p--",
    ].join("\n");
    let board: Board = input.parse().unwrap();

    assert_eq!(board[Coord::new(2, 1)], Cell::Bot);
    assert_eq!(board.position(|c| c == Cell::Princess), Some(Coord::new(0, 2)));
}
