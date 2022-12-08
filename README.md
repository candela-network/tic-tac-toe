# tic-tac-toe

Yet another Tic Tac Toe implementation

# Contract

```rust
impl TicTacToeContract {
    /*
        Launch a new game.
        If no one awaits in the lobby, the invoker of launch() will be placed in the lobby (the method return GameState::PENDING)
        If a GameState::PENDING exists, a game is launched and the play() function can be called the with game_id returned in GameState::RUNNING(game_id).
        Else, the invoker of launch() will be placed in the lobby (the function returns GameState::PENDING)
    */
    pub fn launch(env: Env) -> GameState {
        ...
    }

    /*
        Play a move for a given game id.
    */
    pub fn play(env: Env, game_id: u32, square: BytesN<2>) -> PlayResult {
        ...
    }
```

The first user that issue the `launch()` function will wait in the lobby, the second player will actually start the game, pairing with the player waiting in the lobby.

# Implementation

The goal of this implementation is to reduce the size of the code, for that this implementation keeps the board game in a `u32` field. I suppose that the wasm file and the memory footprint to be quiet small, the optimized wasm file is 4245 bytes.

For now, I'm not sure to be able to mesure the memory footprint and execution cost.

# How to play

The board coordinate are [x,y] where `x` is from left to right, from 0 to 2, and `y`from top to bottom, from 0 to 2.

From the command line with soroban tool.


```
player1$ soroban invoke --wasm ... -fn launch
player2$ soroban invoke --wasm ... -fn launch
player1$ soroban invoke --wasm ... -fn play  --arg 1 --arg [0,0]  
player2$ soroban invoke --wasm ... -fn play  --arg 1 --arg [0,1] 
...
```
