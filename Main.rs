use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


/*
    A few constants for tuning the solver.
*/
static ColOrder: [usize;9] = [4,3,5,2,6,1,7,0,8];
static EvalDepth: usize = 9;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let my_id = parse_input!(inputs[0], i32); // 0 or 1 (Player 0 plays first)
    let opp_id = parse_input!(inputs[1], i32); // if your index is 0, this will be 1, and vice versa
    let mut stolen: bool = false;

    let mut solver = Solver::new();
    // game loop

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let turn_index = parse_input!(input_line, usize); // starts from 0; As the game progresses, first player gets [0,2,4,...] and second player gets [1,3,5,...]
        
        let mut _board = BitBoard::from_input(turn_index);
        
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_valid_actions = parse_input!(input_line, i32); // number of unfilled columns in the board

        
        for i in 0..num_valid_actions as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let action = parse_input!(input_line, i32); // a valid column index into which a chip can be dropped
            
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let opp_previous_action = parse_input!(input_line, i32); // opponent's previous chosen column index (will be -1 for first player in the first turn)
        

        if turn_index == 0{
            println!("3");
        }
        else if turn_index == 1 && opp_previous_action == 4 {
            println!("STEAL");
        } else{
            let mut best_score = -127;
            let mut best_move = 0;
            for i in &ColOrder{
                if _board.is_legal_move(*i){
                    let mut b2 = _board.clone();
                    &b2.make_move(*i);
                    let t_score = -solver.negamax(&b2, best_score, 127, 0);
                    if t_score > best_score{
                        best_score = t_score;
                        best_move = *i;
                    }
                }
            }
            println!("{}", best_move);
        }
    }
}


const N_ROWS: usize = 7;
const N_COLS: usize = 9;

// Methods provided by the board
trait Board{

    /* Can this column be played? */
    fn is_legal_move(&self, col: usize) -> bool;

    /* Play the given move on the given column */
    fn make_move(&mut self, col: usize);

    /* Would playing this move cause a win? */
    fn move_causes_win(&self, col: usize) -> bool;

    /* How many moves were played since the start of the game */
    fn moves_played(&self) -> usize;
}

#[derive(Copy,Clone)]
/* A bitboard allows bitwise operations to be used instead of loops, improving performance
   Bit format:
   let n[b] = n&(1<<b)
   The bit mask for row r, column c is (1 << (c * 8 + r))
   Row 0 is the bottom row, row 6 is the top row
*/

struct BitBoard {

    
    pos: u128, // '1' wherever current player's pieces are
    mask: u128, // '1' wherever anyone's pieces are
    moves: usize
}
impl BitBoard{
    const BOTTOM_MASK: u128 = 0x010101010101010101; // Mask for bottom 9 cells
    
    // The current position, and a '1' above the top of each column.
    // Every position has a unique key
    // Every token belonging to the current player is marked with a 1
    // The cell above the topmost token in any column is also marked with a 1,
    // Even if it is out of bounds
    fn key(&self) -> u128 {
        self.pos ^ (self.mask + Self::BOTTOM_MASK)
    }
    // mask corresponding to the top cell of a column
    fn top_mask(col:usize) -> u128{
        (1 as u128) << (col*8 + 6)
    }
    
    // mask corresponding to the bottom cell of a column
    fn bottom_mask(col:usize) -> u128{
        (1 as u128) << (col*8)
    }
    fn column_mask(col:usize) -> u128{
        (127 as u128) << (col*8)
    }
    fn has_4_in_row(pos:u128) -> bool {
        // Horizontal
        let hb = pos & (pos >> 8);
        if hb & (hb >> 16) != 0 {return true}

        // Diagonal
        let d1 = pos & (pos >> 7);
        if d1 & (d1 >> 14) != 0 {return true}
        
        // Other diagonal
        let d2 = pos & (pos >> 9);
        if d2 & (d2 >> 18) != 0 {return true}

        // Vertical
        let v = pos & (pos >> 1);
        if v & (v >> 2) != 0 {return true}

        return false;
    }
    fn from_input(turn_index: usize) -> BitBoard{
        let mut bb = BitBoard{pos: 0, mask: 0, moves: 0};

        bb.moves = turn_index; // starts from 0; As the game progresses, first player gets [0,2,4,...] and second player gets [1,3,5,...]
        let plr_id = (bb.moves %2) + 1;

        for i in (0..7 as usize).rev() {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let board_row: Vec<char> = input_line.trim().to_string().chars().collect(); // one row of the board (from top to bottom)
            for j in 0..9 as usize {
                let c = board_row[j];
                let cell_code = if c == '.'{0}else{if c== '0'{1}else{2}};
                if cell_code > 0 {
                    let bindex = j * 8 + i;
                    bb.mask |= (1 as u128) << bindex;
                    if cell_code == plr_id{
                        bb.pos |= (1 as u128) << bindex;
                    }
                }
            }
        }
        bb
    }
    fn print_board(&self){
        eprintln!("{:b}",self.pos);
        eprintln!("{:b}", self.mask);

        for r in (0..7).rev(){
            for c in 0..9{
                let brc = (1<<(c*8 + r));
                let mrc = brc & self.mask;
                let prc = brc & self.pos;
                eprint!("{}", if mrc ==0{'_'} else {if prc==0 {'2'} else {'1'}});
            }
            eprintln!("");
        }
    }
}
impl Board for BitBoard {
    fn is_legal_move(&self, col:usize)-> bool{
        self.mask & BitBoard::top_mask(col) == 0
    }

    fn make_move(&mut self, col: usize){
        self.pos ^= self.mask; // flip position
        self.mask |= self.mask + BitBoard::bottom_mask(col); // add a tile
        self.moves += 1;
    }
    
    fn move_causes_win(&self, col: usize) -> bool{
        let newpos = self.pos | ((self.mask + BitBoard::bottom_mask(col)) & BitBoard::column_mask(col));
        BitBoard::has_4_in_row(newpos)
    }

    fn moves_played(&self) -> usize{ 
        self.moves
    }
}

/*
    A simple hash table optimized for connect 4.
    The hash algorithm could need to be changed.
*/
struct TranspositionTable {
    keys:Vec<u128>,
    values:Vec<i8>
}

impl TranspositionTable {
    const SIZE:usize = (1<<24);
    fn new()->Self{
        Self{
            keys:vec![0;Self::SIZE],
            values:vec![0;Self::SIZE]
        }
        
    }
    fn hash(key: u128) -> usize{
        (key % (Self::SIZE as u128)) as usize
    }

    fn put(&mut self, key:u128, val:i8){
        let loc = Self::hash(key);
        self.keys[loc] = key;
        self.values[loc] = val;
    }

    fn get(&self, key:u128) -> Option<i8>{
        let loc = Self::hash(key);
        if self.keys[loc] == key{
            return Option::Some(self.values[loc]);
        } else {
            return Option::None;
        }
    }
}

struct Solver{
    tt:TranspositionTable
}

impl Solver{
    fn new() -> Solver{
        Solver{
            tt:TranspositionTable::new()
        }
    }
    /*
        The Negamax algorithm is a case of the minmax algorithm for games 
        where (your score + opponent's score) = k, where k is constant
        This is true for games such as chess, battleship and connect 4
        
        Alpha beta pruning is used to reduce the time spent evaluating bad moves.
        If, partway through evaluation, a move can be determined to be worse than
        some other move, then there is no need to evaluate it further.
        
        Alpha represents the lower bound on the current player's possible score,
        while Beta represents the upper bound (assuming optimal play by the opponent)
        
        Alpha-beta pruning works especially well when the order in which moves
        are evaluated is based on a heuristic which generally puts better
        moves first. Connect 4 has a simple one; start from the middle and move outward.
        
        A transposition table is used to cache the beta values of 
        previously-evaluated positions. In connect 4, a position may often 
        be reachable through many different sequence of moves.
        
    */
    fn negamax (&mut self, b: &BitBoard, mut alpha: i8, mut beta: i8, depth: usize) -> i8
    {
        // Cut off evaluation to avoid timeout
        if depth > 9{
            return 0;
        }

        // Check if win is possible on the next move
        for x in 0..9{
            if b.is_legal_move(x) && b.move_causes_win(x){
                return 127-(b.moves_played() as i8);
            }
        }

        let ttv = self.tt.get(b.key());
        
        let max = ttv.unwrap_or(127);

        if beta > max{
            beta = max;
            
            if alpha >= beta{
                return beta;
            }
        }
        
        for x in &ColOrder{
            if b.is_legal_move(*x){
                let mut b2 = b.clone();
                b2.make_move(*x);
                let b2_score = -self.negamax(&b2, -beta, -alpha, depth+1);
                if b2_score >= beta {return b2_score};
                if b2_score > alpha {alpha = b2_score}
            }
        }
        self.tt.put(b.key(), alpha);

        alpha
    }
}
