use rand::Rng;
use rand::seq::SliceRandom;

pub type Answer = Vec<usize>;
type Guess = Vec<Cell>;

#[derive(Clone, Debug)]
pub struct Board {
    xsize: usize,
    ysize: usize,
    board: Vec<Option<usize>>,
}

#[derive(Clone, Copy)]
struct Cell {
    pos: usize,
    num: usize,
}

#[derive(Clone, Copy)]
struct PartialCell {
    pos: usize,
    num: Option<usize>,
}

struct GuessResult {
    guess: Guess,
    count: usize
}

struct Figurebits {
    allowed: Vec<usize>,
    needed: Vec<usize>
}

struct PuzzleState {
    guesses: Vec<Cell>,
    count: usize,
    board: Board
}

struct PuzzleAnswer {
    state: Vec<PuzzleState>,
    answer: Vec<usize>,
}

fn random_int (max: usize) -> usize {
    if max == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

fn shuffle_array <T>(mut list: Vec<T>) -> Vec<T> {
    if list.len() == 0 {
        return list;
    }
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);
    list
}

fn pickbetter(guess: &Option<Guess>, count: usize, t: Guess) -> GuessResult {
    match (guess, count, t) {
        (None, _, t) => GuessResult {
            guess: t,
            count: 1
        },
        (Some(g), _, t) if t.len() < g.len() => GuessResult {
            guess: t,
            count: 1
        },
        (Some(g), count, t) if t.len() > g.len() => GuessResult {
            guess: g.clone(),
            count: count
        },
        (_, count, t) if random_int(count) == 0 => GuessResult {
            guess: t,
            count: count + 1
        },
        (Some(g), count, _) => GuessResult {
            guess: g.clone(),
            count: count + 1
        }
    }
}

fn listbits(bits: usize) -> Vec<usize> {
    let mut list: Vec<usize> = vec![];

    for i in 0..9 {
        let tester = bits & 1 << i;
        if tester != 0 {
            list.push(i)
        }
    }
    list
}

fn posfor(x: usize, y: usize, axis: usize) -> usize {
    match (x, y, axis) {
        (x, y, 0) => x * 9 + y,
        (x, y, 1) => y * 9 + x,
        _ => [0, 3, 6, 27, 30, 33, 54, 57, 60][x] + [0, 1, 2, 9, 10, 11, 18, 19, 20][y]
    }
}

fn axismissing(board: &Board, x: usize, axis: usize) -> usize {
    let mut bits = 0;
    for y in 0..9 {
        let pos = posfor(x, y, axis);
        if let Some(e) = board.board[pos] {
            bits = bits | 1 << e;
        }
    }
    511 ^ bits
}

fn figurebits(board: &Board) -> Figurebits {
    let mut needed:Vec<usize> = vec![];
    let mut allowed:Vec<usize> = board.board.iter().map(|x| match x {
        Some(_) => 0,
        None => 511,
    }).collect();

    for axis in 0..3 {
        for x in 0..9 {
            let bits = axismissing(&board, x, axis);
            needed.push(bits);
            for y in 0..9 {
                let pos = posfor(x, y, axis);
                allowed[pos] = allowed[pos] & bits;
            }
        }
    }

    Figurebits {
        allowed: allowed, 
        needed: needed
    }
}

fn deduce(board: &mut Board) -> Option<Guess> {
    loop {
        let mut stuck = true;
        let mut guess:Option<Guess> = None;
        let mut count = 0;

        let tuple1 = figurebits(&board);
        let mut allowed = tuple1.allowed;
        let mut needed = tuple1.needed;

        for pos in 0..81 {
            if board.board[pos] == None {
                let numbers = listbits(allowed[pos]);
                if numbers.len() == 0 {
                   return Some(vec![]);
                } else if numbers.len() == 1 {
                    board.board[pos] = Some(numbers[0]);
                    stuck = false;
                } else if stuck {
                    let t = numbers.iter().map(|val| Cell {
                        pos: pos,
                        num: *val,
                    }).collect();
                    let tuple2 = pickbetter(&guess, count, t);
                    guess = Some(tuple2.guess);
                    count = tuple2.count;
                }
            }
        }

        if stuck == false {
            let tuple3 = figurebits(&board);
            allowed = tuple3.allowed;
            needed = tuple3.needed;
        }

        for axis in 0..3 {
            for x in 0..9 {
                let numbers = listbits(needed[axis * 9 + x]);
                for n in numbers {
                    let bit = 1 << n;
                    let mut spots = vec![];

                    for y in 0..9 {
                        let pos = posfor(x, y, axis);
                        let tester = allowed[pos] & bit;
                        if tester != 0 {
                            spots.push(pos);
                        }
                    }

                    if spots.len() == 0 {
                        return Some(vec![]);
                    } else if spots.len() == 1 {
                        board.board[spots[0]] = Some(n);
                        stuck = false
                    } else if stuck {
                        let t: Vec<Cell> = spots.iter().map(|val| Cell{
                            pos: *val,
                            num: n,
                        }).collect();
                        let tuple4 = pickbetter(&guess, count, t);
                        guess = Some(tuple4.guess);
                        count = tuple4.count;
                    }
                }
            }
        }

        if stuck {
            if let Some(g) = guess {
                return Some(shuffle_array(g));
            }
            return guess;
        }
    }
}

fn solvenext(mut remembered: Vec<PuzzleState>) -> PuzzleAnswer {
    loop {
        if let Some(tuple1) = remembered.pop() {
            if tuple1.count >= tuple1.guesses.len() {
                continue;
            }
    
            remembered.push(PuzzleState {
                guesses: tuple1.guesses.clone(),
                count: tuple1.count + 1,
                board: tuple1.board.clone(),
            });
            let mut workspace = tuple1.board.clone();
            let tuple2 = tuple1.guesses[tuple1.count];
            workspace.board[tuple2.pos] = Some(tuple2.num);
            let guesses = deduce(&mut workspace);

            if let Some(g) = guesses {
                remembered.push(PuzzleState {
                    guesses: g,
                    count: 0,
                    board: workspace,
                })
            } else {
                return PuzzleAnswer {
                    state: remembered,
                    answer: unwarp_board(&workspace),
                }
            }
        } else {
            break;
        }
    }

    PuzzleAnswer {
        state: vec![],
        answer: vec![],
    }
}

fn solveboard(mut board: Board) -> PuzzleAnswer {
    let guesses = deduce(&mut board);

    if let Some(g) = guesses {
        let track = vec![
            PuzzleState {
                guesses: g,
                count: 0,
                board: board,
            }
        ];
        solvenext(track)
    } else {
        PuzzleAnswer {
            state: vec![],
            answer: unwarp_board(&board),
        }
    }
}

pub fn solvepuzzle(board: Board) -> Answer {
    solveboard(board).answer
}

fn checkpuzzle(puzzle: &Board, board: Option<Board>) -> isize {
    let tuple1 = solveboard(puzzle.clone());

    match (board, tuple1.answer) {
        (_, answer) if answer.len() == 0 => -1,
        (Some(b), answer) if unwarp_board(&b) != answer => -1,
        _ => {
            let difficulty = tuple1.state.len();
            let tuple2 = solvenext(tuple1.state);
            if tuple2.answer.len() > 0 {
                -1
            } else {
                difficulty as isize
            }
        }
    }
}

pub fn ratepuzzle(puzzle: &Board, samples: usize) -> f64 {
    let mut total = 0.0;
    let fsample = samples as f64;
    for _ in 0..samples {
        let tuple = solveboard(puzzle.clone());

        if tuple.answer.len() > 0 {
            total = total + tuple.state.len() as f64;
        } else {
            return -1.0;
        }
    }
    total / fsample
}

fn generate_puzzle(board: Board) -> Board {
    let mut puzzle: Vec<PartialCell> = vec![];
    let mut deduced:Board = empty_board(board.xsize, board.ysize);

    let order: Vec<usize> = shuffle_array((0..board.board.len()).map(|x| x).collect());

    for pos in order {
        if None == deduced.board[pos] {
            puzzle.push(PartialCell {
                pos: pos,
                num: board.board[pos],
            });
            deduced.board[pos] = board.board[pos];
            deduce(&mut deduced);
        }
    }

    let mut puzzle = shuffle_array(puzzle);

    let mut i = puzzle.len() - 1;
    loop {
        let e = puzzle[i];
        puzzle.remove(i);
        let mut check_puzzle: Board = empty_board(board.xsize, board.ysize);
        apply_to_board(&puzzle, &mut check_puzzle);
        let rating = checkpuzzle(&check_puzzle, Some(board.clone()));

        if rating == -1 {
            puzzle.push(e);
        }

        if i == 0 {
            break;
        }
        i = i - 1
    }

    let mut board: Board = empty_board(board.xsize, board.ysize);
    apply_to_board(&puzzle, &mut board);
    board
}

fn empty_board(xsize: usize, ysize: usize) -> Board {
    Board {
        xsize: xsize,
        ysize: ysize, 
        board: (0..xsize * ysize).map(|_| None).collect()
    }
}

fn apply_to_board(entries: &Vec<PartialCell>, board: &mut Board) {
    for item in entries.iter() {
        board.board[item.pos] = item.num
    }
}

pub fn make_puzzle() -> Board {
    let mut board = empty_board(9, 9);
    let answer = solveboard(board.clone()).answer;
    board.board = warp_answer(&answer);
    generate_puzzle(board)
}

fn unwarp_board(board: &Board) -> Answer {
    board.board.iter().map(|x| x.unwrap()).collect()
}

fn warp_answer(answer: &Answer) -> Vec<Option<usize>> {
    answer.iter().map(|x| Some(*x)).collect()
}