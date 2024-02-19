use rand::{rngs::StdRng, Rng};

use super::common::*;

#[allow(unused)]
#[derive(Debug)]
pub enum EvalError {
    Finished,
    Syntax(usize, String),
    Semantic(String),
    MaxIteration,
}

pub struct Runtime<'a> {
    memory: Vec<Number>,
    pub input: &'a Vec<Number>,
    pub output: Vec<Number>,
    pub input_cursor: usize,
    max_iterations: usize,
    pub overread: bool,
}

impl<'a> Runtime<'a> {
    pub fn new(memsize: usize, input: &'a Vec<Number>, randomize_memory: &mut Option<&mut StdRng>) -> Self {
        let memory = if let Some(rand) = randomize_memory {
            (0..memsize).map(|_| {rand.gen::<i32>()}).collect()
        } else {
            vec![0; memsize]
        };

        Runtime {
            memory,
            input,
            output: Vec::new(),
            input_cursor: 0,
            max_iterations: 100,
            overread: false,
        }
    }

    fn next_input(&mut self) -> Option<Number> {
        if self.input_cursor < self.input.len() {
            let val = self.input[self.input_cursor];
            self.input_cursor += 1;
            Some(val)
        } else {
            self.overread = true;
            None
        }
    }

    fn set_reg(&mut self, num: usize, val: Number) -> Result<(), EvalError> {
        if num > self.memory.len() {
            Err(EvalError::Semantic(format!(
                "Tried to set memory[{num}], when length is {}",
                self.memory.len()
            )))
        } else {
            self.memory[num] = val;
            Ok(())
        }
    }

    fn read_reg(&self, num: usize) -> Result<Number, EvalError> {
        if num > self.memory.len() {
            Err(EvalError::Semantic(format!(
                "Tried to read memory[{num}], when length is {}",
                self.memory.len()
            )))
        } else {
            Ok(self.memory[num].clone())
        }
    }
}

pub fn execute(program: &Program, runtime: &mut Runtime) {
    log::info!("executing {}", serialize(program));
    return match eval_block(program, 0, runtime) {
        Ok(_pos) => {
            log::trace!("program ended with output {:?}", runtime.output);
            // log::debug!("finished at pos {}/{}", pos, program.len() - 1);
        }
        Err(EvalError::Finished) => {
            log::trace!(
                "terminated due to input end with output {:?}",
                runtime.output
            );
        }
        Err(EvalError::MaxIteration) => {
            log::trace!("terminated due reaching max iteration {:?}", runtime.output);
        }
        Err(EvalError::Syntax(pos, reason)) => {
            log::error!("Invalid program: {program:?}");
            log::error!("Invalid syntax at {pos}: {reason}");
            panic!("Invalid syntax at {pos}: {reason}");
        }
        Err(EvalError::Semantic(reason)) => {
            log::error!("Invalid program: {program:?}");
            log::error!("Invalid program reason: {reason}");
        }
    };
}

// eval_block returns position after the last STAT. This means the cursor will point to ELSE or END tokens
fn eval_block(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    // log::debug!("eval block {pos}");
    let mut pos = pos;
    loop {
        if pos >= program.len() || matches!(program[pos], Token::ELSE | Token::END) {
            // log::debug!("returning from block, returning {pos}");
            return Ok(pos);
        }
        pos = eval_stat(program, pos, runtime)?;
    }
}

fn handle_if_true(
    program: &Program,
    pos: usize,
    runtime: &mut Runtime,
) -> Result<usize, EvalError> {
    let end_or_else = eval_block(program, pos, runtime)?;
    match program[end_or_else] {
        Token::ELSE => {
            let else_part_end = get_node_end(program, end_or_else);
            log::trace!("got node end {else_part_end}");
            Ok(else_part_end)
        }
        Token::END => Ok(end_or_else + 1),
        _ => Err(EvalError::Syntax(
            end_or_else,
            "Expected END or ELSE".into(),
        )),
    }
}

fn handle_if_false(
    program: &Program,
    true_block_pos: usize,
    runtime: &mut Runtime,
    if_stat_pos: usize,
) -> Result<usize, EvalError> {
    let true_block_end = skip_block(program, true_block_pos);
    match program[true_block_end] {
        Token::ELSE => {
            let else_block_pos = true_block_end + 1;
            log::trace!("IF condition at {if_stat_pos} entered ELSE branch at {else_block_pos}");
            let endpos = eval_block(program, else_block_pos, runtime)?;
            Ok(endpos + 1)
        }
        Token::END => {
            log::trace!(
                "IF condition at {if_stat_pos} has no ELSE branch (reached END at {true_block_end}"
            );
            Ok(true_block_end + 1)
        }
        _ => unreachable!(),
    }
}

fn skip_block(program: &Program, pos: usize) -> usize {
    let mut level = 1;
    let mut cursor = pos;
    match program[cursor] {
        Token::Stat(Stat::IF | Stat::WHILE) => (),
        Token::END => return cursor,
        Token::ELSE => return cursor,
        _ => (),
    }

    while cursor < program.len() && level > 0 {
        cursor += 1;
        match program[cursor] {
            Token::Stat(Stat::IF | Stat::WHILE) => level += 1,
            Token::END => level -= 1,
            Token::ELSE if level == 1 => level -= 1,
            _ => (),
        }
    }
    cursor
}

fn handle_while(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    let while_pos = pos;
    let expr_pos = pos + 1;
    let block_pos;
    let mut expr_val;
    (block_pos, expr_val) = eval_expr(program, expr_pos, runtime)?;
    let block_end_pos = skip_block(program, block_pos);
    let mut iteration = 0;

    while is_truthy(expr_val) {
        log::trace!("WHILE at {while_pos}: iteration {iteration}");
        eval_block(program, block_pos, runtime)?;
        (_, expr_val) = eval_expr(program, expr_pos, runtime)?;
        iteration += 1;
        if iteration >= runtime.max_iterations {
            return Err(EvalError::MaxIteration);
        }
    }

    match program[block_end_pos] {
        Token::END => Ok(block_end_pos + 1),
        _ => Err(EvalError::Syntax(
            block_end_pos,
            format!("Expected END after a WHILE started at {while_pos}. Got to {block_end_pos}"),
        )),
    }
}

fn eval_stat(program: &Program, pos: usize, runtime: &mut Runtime) -> Result<usize, EvalError> {
    // log::debug!("eval stat {pos}");
    match program[pos] {
        Token::Stat(stat) => match stat {
            Stat::OUTPUT => {
                let (newpos, val) = eval_expr(program, pos + 1, runtime)?;
                runtime.output.push(val);
                Ok(newpos)
            }
            Stat::INPUT => {
                let destination = match program[pos + 1] {
                    Token::Reg(num) => num,
                    _ => return Err(EvalError::Syntax(pos + 1, "Expected REG".into())),
                };
                let val = match runtime.next_input() {
                    Some(val) => val,
                    None => return Err(EvalError::Finished),
                };
                runtime.set_reg(destination, val)?;
                Ok(pos + 2)
            }
            Stat::LOAD => {
                let destination = match program[pos + 1] {
                    Token::Reg(num) => num,
                    _ => return Err(EvalError::Syntax(pos + 1, "Expected REG".into())),
                };
                let (newpos, val) = eval_expr(program, pos + 2, runtime)?;
                runtime.set_reg(destination, val)?;
                Ok(newpos)
            }
            Stat::IF => {
                let (true_block_pos, condition_val) = eval_expr(program, pos + 1, runtime)?;
                if is_truthy(condition_val) {
                    log::trace!("IF condition at {pos} evaluated to TRUE");
                    handle_if_true(program, true_block_pos, runtime)
                } else {
                    log::trace!("IF condition at {pos} evaluated to FALSE");
                    handle_if_false(program, true_block_pos, runtime, pos)
                }
            }
            Stat::WHILE => handle_while(program, pos, runtime),
        },
        _ => panic!("called eval_stat on non-stat at {pos}"),
    }
}

fn eval_expr(
    program: &Program,
    pos: usize,
    runtime: &mut Runtime,
) -> Result<(usize, Number), EvalError> {
    let opcode = program[pos];

    let one_arg = |func: fn(Number) -> Number, runtime: &mut Runtime| {
        let (pos, arg) = eval_expr(program, pos + 1, runtime)?;
        Ok((pos, func(arg)))
    };

    let two_arg = |func: fn(Number, Number) -> Number, runtime: &mut Runtime| {
        let (pos, lhs) = eval_expr(program, pos + 1, runtime)?;
        let (pos, rhs) = eval_expr(program, pos, runtime)?;
        Ok((pos, func(lhs, rhs)))
    };

    return match opcode {
        Token::Expr(func) => match func {
            Expr::Num(val) => Ok((pos + 1, val)),
            Expr::Reg(regnum) => Ok((pos + 1, runtime.read_reg(regnum).unwrap_or(0))),
            Expr::ADD => two_arg(add, runtime),
            Expr::SUB => two_arg(sub, runtime),
            Expr::MUL => two_arg(mul, runtime),
            Expr::DIV => two_arg(protected_div, runtime),
            Expr::EQ => two_arg(equal, runtime),
            Expr::LT => two_arg(less_than, runtime),
            Expr::GT => two_arg(greater_than, runtime),
            Expr::OR => two_arg(or, runtime),
            Expr::AND => two_arg(and, runtime),
            Expr::NOT => one_arg(negation, runtime),
        },
        Token::Reg(num) => Ok((pos + 1, runtime.read_reg(num)?)),
        _ => unreachable!("called eval_expr on non-expr: {opcode:?}"),
    };

    fn add(lhs: Number, rhs: Number) -> Number {
        lhs.checked_add(rhs)
            .unwrap_or_else(|| rhs.signum() * Number::MAX)
    }
    fn sub(lhs: Number, rhs: Number) -> Number {
        let sign = if rhs > lhs { -1 } else { 1 };
        lhs.checked_sub(rhs).unwrap_or_else(|| sign * Number::MAX)
    }
    fn mul(lhs: Number, rhs: Number) -> Number {
        let sign = lhs.signum() * rhs.signum();
        lhs.checked_mul(rhs).unwrap_or_else(|| sign * Number::MAX)
    }
    fn protected_div(lhs: Number, rhs: Number) -> Number {
        if rhs == 0 {
            lhs
        } else {
            lhs / rhs
        }
    }
    fn equal(lhs: Number, rhs: Number) -> Number {
        if lhs == rhs {
            NUMBER_TRUE
        } else {
            NUMBER_FALSE
        }
    }
    fn less_than(lhs: Number, rhs: Number) -> Number {
        if lhs < rhs {
            NUMBER_TRUE
        } else {
            NUMBER_FALSE
        }
    }
    fn greater_than(lhs: Number, rhs: Number) -> Number {
        if lhs > rhs {
            NUMBER_TRUE
        } else {
            NUMBER_FALSE
        }
    }
    fn or(lhs: Number, rhs: Number) -> Number {
        if is_truthy(lhs) || is_truthy(rhs) {
            NUMBER_TRUE
        } else {
            NUMBER_FALSE
        }
    }
    fn and(lhs: Number, rhs: Number) -> Number {
        if is_truthy(lhs) && is_truthy(rhs) {
            NUMBER_TRUE
        } else {
            NUMBER_FALSE
        }
    }
    fn negation(arg: Number) -> Number {
        if is_truthy(arg) {
            NUMBER_FALSE
        } else {
            NUMBER_TRUE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rand::{SeedableRng, RngCore};

    #[test]
    fn test_runtime_input() {
        let inputs = &vec![2, 3, 4];
        let mut runtime = Runtime::new(2, &inputs, &mut None);
        assert_eq!(runtime.next_input(), Some(2));
        assert_eq!(runtime.next_input(), Some(3));
        assert_eq!(runtime.next_input(), Some(4));
        assert_eq!(runtime.next_input(), None);
    }

    #[test]
    fn test_stat_input() {
        let program: Vec<Token> = vec![Token::Stat(Stat::INPUT), Token::Reg(0)];
        let inputs = vec![2];
        let mut runtime = Runtime::new(2, &inputs, &mut None);
        assert_eq!(runtime.memory, vec![0, 0]);
        assert_eq!(runtime.input.len(), 1);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![2, 0]);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(matches!(res, Err(EvalError::Finished)));
    }

    #[test]
    fn test_stat_input_multiple() {
        let program: Vec<Token> = vec![Token::Stat(Stat::INPUT), Token::Reg(0)];
        let inputs = vec![2, 3];
        let mut runtime = Runtime::new(2, &inputs, &mut None);
        assert_eq!(runtime.memory, vec![0, 0]);
        assert_eq!(runtime.input.len(), 2);

        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![2, 0]);

        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![3, 0]);

        let res = eval_stat(&program, 0, &mut runtime);
        assert!(matches!(res, Err(EvalError::Finished)));
    }

    #[test]
    fn test_stat_input_second_register() {
        let program: Vec<Token> = vec![Token::Stat(Stat::INPUT), Token::Reg(1)];
        let inputs = vec![4];
        let mut runtime = Runtime::new(2, &inputs, &mut None);
        assert_eq!(runtime.memory, vec![0, 0]);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![0, 4]);
    }

    #[test]
    fn test_stat_output() {
        let program: Vec<Token> = vec![Token::Stat(Stat::OUTPUT), Token::Reg(0)];
        let inputs = vec![];
        let mut runtime = Runtime {
            memory: vec![2, 0],
            input: &inputs,
            output: vec![],
            input_cursor: 0,
            max_iterations: 100,
            overread: false
        };
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.memory, vec![2, 0]);
        assert_eq!(runtime.output, vec![2]);
    }

    #[test]
    fn test_expression() {
        let program: Vec<Token> = vec![
            Token::Expr(Expr::ADD),
            Token::Reg(0),
            Token::Expr(Expr::DIV),
            Token::Reg(1),
            Token::Reg(1),
        ];
        let data = vec![1, -2];
        let inputs = &vec![];
        let mut runtime = Runtime::new(3, &inputs, &mut None);
        runtime.memory = data;
        let (pos, val) = eval_expr(&program, 0, &mut runtime).unwrap();
        assert_eq!(5, pos);
        assert_eq!(2, val);
    }

    #[test]
    fn test_exec_identity() {
        let program: Vec<Token> = vec![
            Token::Stat(Stat::INPUT),
            Token::Reg(0),
            Token::Stat(Stat::OUTPUT),
            Token::Reg(0),
        ];
        let inputs = &vec![2];
        let mut runtime = Runtime::new(2, &inputs, &mut None);
        let res = eval_stat(&program, 0, &mut runtime);
        assert!(res.is_ok());
        let res = eval_stat(&program, 2, &mut runtime);
        assert!(res.is_ok());
        assert_eq!(runtime.output, vec![2]);
    }

    #[test]
    fn test_random_memory() {
        let seed = StdRng::from_entropy().next_u64();
        println!("Seed: {seed}");
        let mut rand = StdRng::seed_from_u64(seed);
        let inputs = &vec![2];
        let runtime = Runtime::new(5, &inputs, &mut Some(&mut rand));
        assert_eq!(runtime.memory.len(), 5);
        assert!(runtime.memory[0] != runtime.memory[1] || runtime.memory[0] != runtime.memory[2] || runtime.memory[0] != runtime.memory[3] || runtime.memory[0] != runtime.memory[4]);
    }
}
