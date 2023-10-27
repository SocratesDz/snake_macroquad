#[cfg(test)]
mod game_test {
    use snake::core::{game::Game, constants::{LEFT, UP}, cell::CellType};

    #[test]
    fn test_update_board() {
        let mut game = Game::new(10, 10);
        assert_eq!((game.snake.head.x, game.snake.head.y), (5, 5));
        assert_eq!(game.snake.head.direction, LEFT);

        game.update();
        assert_eq!((game.snake.head.x, game.snake.head.y), (4, 5));
    }

    #[test]
    fn test_grow_snake() {
        let mut game = Game::new(10, 10);

        assert_eq!((game.snake.head.x, game.snake.head.y), (5, 5));
        assert_eq!(game.snake.head.direction, LEFT);

        game.update();
        assert_eq!((game.snake.head.x, game.snake.head.y), (4, 5));

        game.snake.grow();
        game.update();

        assert_eq!(game.snake.head.position(), (3, 5));
        assert_eq!(game.snake.body.len(), 1);
        assert_eq!(game.snake.body.first().unwrap().position(), (4, 5))
    }

    #[test]
    fn test_change_snake_direction() {
        let mut game = Game::new(10, 10);
        assert_eq!(game.snake.head.position(), (5, 5));

        game.snake.grow();
        game.update(); // (4, 5), (5, 5)

        game.snake.grow();
        game.update(); // (3, 5), (4, 5), (5, 5)

        game.snake.change_direction(UP);
        game.update(); // (3, 4), (3, 5), (4, 5)
        game.update(); // (3, 3), (3, 4), (3, 5)

        assert_eq!(game.snake.head.position(), (3, 3));
        assert_eq!(game.snake.body.get(0).unwrap().position(), (3, 4));
        assert_eq!(game.snake.body.get(1).unwrap().position(), (3, 5));
    }

    #[test]
    fn test_food_generation() {
        let mut game = Game::new(10, 10);

        assert!(game.board.cells.is_empty());

        let _ = game.board.place_food((0, 0), &game.snake);

        assert!(game.board.cells.get(0).unwrap().cell_type == CellType::Food)
    }
}