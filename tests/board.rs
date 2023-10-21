use snake::core::{board::Board, constants::LEFT};

#[test]
fn test_board_generation() {
    let board = Board::new(10, 10);
    assert!(!board.cells.is_empty())
}

#[test]
fn test_update_board() {
    let mut board = Board::new(10, 10);
    assert_eq!((board.snake.head.x, board.snake.head.y), (5, 5));
    assert_eq!(board.snake.head.direction, LEFT);

    board.update();
    assert_eq!((board.snake.head.x, board.snake.head.y), (4, 5));
}