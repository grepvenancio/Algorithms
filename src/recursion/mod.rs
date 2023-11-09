use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum MazeTile {
    Empty,
    Wall,
    End,
    Start,
    Incompatible(char)
}

impl Display for MazeTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeTile::Empty => write!(f, "Empty"),
            MazeTile::Wall => write!(f, "Wall"),
            MazeTile::End => write!(f, "End"),
            MazeTile::Start => write!(f, "Start"),
            MazeTile::Incompatible(c) => write!(f, "Incompatible {}", c),
        }
    }
}

#[derive(Error, Debug)]
enum MazeError {
    #[error("cannot convert {0} to a valid maze tile (expected '#', ' ', 'S', 'E')")]
    IncompatibleTile(MazeTile)
}

type VecMaze = Vec<Vec<MazeTile>>;

pub struct Maze(VecMaze);

impl Maze {
    fn new(input_vec_char: Vec<Vec<char>>) -> anyhow::Result<Self> {
        let maze_tile: VecMaze = input_vec_char
            .iter()
            .map(|char_vec| char_vec.iter().map(|c| (*c).into()).collect())
            .collect();
        if let Some(incompatible) = maze_tile.iter().flatten().find(|&e| matches!(e, MazeTile::Incompatible(_))) {
            return Err(MazeError::IncompatibleTile(*incompatible).into());
        }
        Ok(Self(maze_tile))
    }

    fn solve(&self) -> anyhow::Result<()> {
        todo!()
    }
}

impl From<char> for MazeTile {
    fn from(value: char) -> MazeTile {
        match value {
            '#' => MazeTile::Wall,
            ' ' => MazeTile::Empty,
            'E' => MazeTile::End,
            'S' => MazeTile::Start,
            c   => MazeTile::Incompatible(c)
        }
    }
}


#[cfg(test)]
mod tests {
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
        assert!(maze.is_ok());
        assert!(incorrect_maze.is_err());
    }
}
