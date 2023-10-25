#[cfg(test)]
mod board_test {
    use snake::core::{
        board::Board,
        cell::CellType,
        constants::{LEFT, UP},
    };

    #[test]
    fn test_update_board() {
        let mut board = Board::new(10, 10);
        assert_eq!((board.snake.head.x, board.snake.head.y), (5, 5));
        assert_eq!(board.snake.head.direction, LEFT);

        board.update();
        assert_eq!((board.snake.head.x, board.snake.head.y), (4, 5));
    }

    #[test]
    fn test_grow_snake() {
        let mut board = Board::new(10, 10);
        assert_eq!((board.snake.head.x, board.snake.head.y), (5, 5));
        assert_eq!(board.snake.head.direction, LEFT);

        board.update();
        assert_eq!((board.snake.head.x, board.snake.head.y), (4, 5));

        board.snake.grow();
        board.update();

        assert_eq!(board.snake.head.position(), (3, 5));
        assert_eq!(board.snake.body.len(), 1);
        assert_eq!(board.snake.body.first().unwrap().position(), (4, 5))
    }

    #[test]
    fn test_change_snake_direction() {
        let mut board = Board::new(10, 10);
        assert_eq!(board.snake.head.position(), (5, 5));

        board.snake.grow();
        board.update(); // (4, 5), (5, 5)

        board.snake.grow();
        board.update(); // (3, 5), (4, 5), (5, 5)

        board.snake.change_direction(UP);
        board.update(); // (3, 4), (3, 5), (4, 5)
        board.update(); // (3, 3), (3, 4), (3, 5)

        assert_eq!(board.snake.head.position(), (3, 3));
        assert_eq!(board.snake.body.get(0).unwrap().position(), (3, 4));
        assert_eq!(board.snake.body.get(1).unwrap().position(), (3, 5));
    }

    #[test]
    fn test_food_generation() {
        let mut board = Board::new(10, 10);

        assert!(board.get_cell(0, 0).unwrap().cell_type == CellType::Empty);

        let _ = board.place_food((0, 0));

        assert!(board.get_cell(0, 0).unwrap().cell_type == CellType::Food)
    }

    #[test]
    fn test_get_board_cell() {
        let board = Board::new(4, 3);

        let cell = board.get_cell(0, 1).unwrap();
        assert_eq!((cell.x, cell.y), (0, 1));

        let cell = board.get_cell(2, 0).unwrap();
        assert_eq!((cell.x, cell.y), (2, 0));
    }
}
