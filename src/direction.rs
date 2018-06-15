use super::rand::{thread_rng, Rng};

/// Enum that stores all available directions.
/// This enum also provides some basic functions to allow a game to be using random moves.
#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

lazy_static! {
    static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down
    ];
}

impl Direction {
    /// Returns a random `Direction`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Direction;
    ///
    /// let direction = Direction::sample();
    /// // => Direction::Left
    /// ```
    pub fn sample() -> Direction {
        match thread_rng().gen_range(0, 4) {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Up,
            _ => Direction::Up
        }
    }

    /// Returns a `Vec<Direction>` excluding `dirs: &Vec<Direction>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Direction;
    ///
    /// let available = Direction::without(&vec![Direction::Left, Direction::Right]);
    ///
    /// assert_eq!(available, vec![Direction::Up, Direction::Down]);
    /// ```
    pub fn without(dirs: &Vec<Direction>) -> Vec<Direction> {
        let mut filtered = DIRECTIONS.clone();
        filtered.retain(|dir| dirs.iter().all(|tried| &dir != &tried));
        filtered
    }

    /// Like `tfe::Direction::sample` but combined with `tfe::Direction::without`.
    /// Returns a `Direction` after excluding `dirs: &Vec<Direction>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Direction;
    ///
    /// let direction = Direction::sample_without(&vec![Direction::Left, Direction::Right]);
    /// assert!(vec![Direction::Up, Direction::Down].contains(&direction));
    ///
    /// let up = Direction::sample_without(&vec![Direction::Down, Direction::Left, Direction::Right]);
    /// assert_eq!(Direction::Up, up);
    /// ```
    pub fn sample_without(dirs: &Vec<Direction>) -> Direction {
        let filtered = &Self::without(dirs);

        filtered[thread_rng().gen_range(0, filtered.len())].clone()
    }
}
