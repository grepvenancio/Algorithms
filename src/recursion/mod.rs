use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MazeTile {
    Empty,
    Wall,
    End,
    Start,
    /* Rope, */
    Incompatible(char),
}

impl Display for MazeTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeTile::Empty => write!(f, "Empty"),
            MazeTile::Wall => write!(f, "Wall"),
            MazeTile::End => write!(f, "End"),
            MazeTile::Start => write!(f, "Start"),
            /* MazeTile::Rope => write!(f, "Rope"), */
            MazeTile::Incompatible(c) => write!(f, "Incompatible {}", c),
        }
    }
}

#[derive(Error, Debug)]
enum MazeError {
    #[error("cannot convert {0} to a valid maze tile (expected '#', ' ', 'S', 'E')")]
    IncompatibleTile(MazeTile),
    #[error("cannot solve the maze as there is no clear path from the start to end")]
    MazeWithoutSolution,
}

type VecMaze = Vec<Vec<MazeTile>>;

type Point = (usize, usize);

#[derive(PartialEq, Eq, Clone)]
pub struct Maze {
    maze: VecMaze,
    end: Point,
    start: Point,
    limit: Point,
}

impl Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.maze {
            write!(f, "{:?}\n", row)?
        }
        Ok(())
    }
}

impl Maze {
    pub fn new(input_vec_char: Vec<Vec<char>>) -> anyhow::Result<Self> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let maze_tile: VecMaze = input_vec_char
            .iter()
            .map(|char_vec| char_vec.iter().map(|c| (*c).into()).collect())
            .collect();
        if let Some(incompatible) = maze_tile
            .iter()
            .flatten()
            .find(|&e| matches!(e, MazeTile::Incompatible(_)))
        {
            return Err(MazeError::IncompatibleTile(*incompatible).into());
        }
        for (i, row) in maze_tile.iter().enumerate() {
            for j in 0..row.len() {
                match row[j] {
                    MazeTile::End => end = (i, j),
                    MazeTile::Start => start = (i, j),
                    _ => {}
                }
            }
        }
        let limit = (maze_tile.len(), maze_tile[0].len());
        Ok(Self {
            maze: maze_tile,
            end,
            start,
            limit,
        })
    }

    pub fn solve(&mut self) -> anyhow::Result<()> {
        let mut path = vec![self.start];
        let mut curr = self.start;
        while self.walk(&mut path, &mut curr) {}
        Ok(())
    }

    fn walk(&self, path: &mut Vec<Point>, curr: &mut Point) -> bool {
        if *curr < (0, 0) || *curr > self.limit {
            return false
        }

        true
    }
} 

impl From<char> for MazeTile {
    fn from(value: char) -> MazeTile {
        match value {
            '#' => MazeTile::Wall,
            ' ' => MazeTile::Empty,
            'E' => MazeTile::End,
            'S' => MazeTile::Start,
            c => MazeTile::Incompatible(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::recursion::Point;

    use super::Maze;

    #[test]
    fn test_maze_solver() {
        let maze = Maze::new(vec![
            vec!['#', '#', '#', '#', 'E', '#'],
            vec!['#', ' ', ' ', ' ', ' ', '#'],
            vec!['#', 'S', '#', '#', '#', '#'],
        ]);
        let incorrect_maze = Maze::new(vec![
            vec!['#', '#', '#', '#', 'X', '#'],
            vec!['#', ' ', ' ', ' ', ' ', '#'],
            vec!['#', 'A', '#', '#', '#', '#'],
        ]);
        assert_eq!(maze.as_ref().unwrap().start, (2,1));
        assert_eq!(maze.as_ref().unwrap().end, (0,4));
        assert_eq!(maze.as_ref().unwrap().limit, (3,6));
        assert!((2,5) < maze.as_ref().unwrap().limit);
        assert!(maze.is_ok());
        assert!(incorrect_maze.is_err());
    }
}
