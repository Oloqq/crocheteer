use super::common::*;
use super::params::Params;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

// pub fn rand_reg(params: &Params, rand: &mut StdRng) -> Token {
//     Token::Reg(rand.gen_range(0, params.memsize))
// }

// pub fn rand_const(params: &Params, rand: &mut StdRng) -> Token {
//     Token::Expr(Expr::Num(
//         rand.gen_range(params.growing.min_const, params.growing.max_const),
//     ))
// }

// pub fn grow_expr(params: &Params, rand: &mut StdRng) -> Vec<Token> {
//     let mut code = vec![];

//     if rand.gen_bool(params.growing.p_expression_plug) {
//         if rand.gen_bool(params.growing.p_prefer_reg_over_num) {
//             code.push(rand_reg(params, rand))
//         } else {
//             code.push(rand_const(params, rand))
//         }
//     } else {
//         let items = &params.growing.d_expr;
//         let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
//         let e: Expr = items[dist2.sample(rand)].0;
//         match e {
//             Expr::Reg(_) => code.push(rand_reg(params, rand)),
//             Expr::Num(_) => code.push(rand_const(params, rand)),
//             _ => {
//                 code.push(Token::Expr(e));
//                 assert!(!matches!(e, Expr::Reg(_)));
//                 for _ in 0..e.argnum() {
//                     code.append(&mut grow_expr(params, rand));
//                 }
//             }
//         }
//     }
//     code
// }

// pub fn grow_stat(size_left: i32, params: &Params, rand: &mut StdRng) -> Vec<Token> {
//     let items = &params.growing.d_stat;
//     let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
//     let stat: Stat = items[dist2.sample(rand)].0;

//     let mut code: Vec<Token> = vec![];
//     code.push(Token::Stat(stat));
//     match stat {
//         Stat::INPUT => {
//             code.push(rand_reg(params, rand));
//         }
//         Stat::OUTPUT => {
//             let mut expr = grow_expr(params, rand);
//             if (code.len() + expr.len()) as i32 > size_left {
//                 return vec![];
//             }
//             code.append(&mut expr);
//         }
//         Stat::LOAD => {
//             code.push(rand_reg(params, rand));
//             let mut expr = grow_expr(params, rand);
//             if (code.len() + expr.len()) as i32 > size_left {
//                 return vec![];
//             }
//             code.append(&mut expr);
//         }
//         Stat::IF => {
//             code.append(&mut grow_expr(params, rand));
//             let mut space: i32 = size_left as i32 - code.len() as i32;

//             let mut inside = grow_stat(space, params, rand);
//             if inside.len() == 0 {
//                 return vec![];
//             }

//             code.append(&mut inside);
//             space = size_left as i32 - code.len() as i32;
//             // code.append(&mut grow_stat(space, params, rand));
//             // space = size_left - code.len();

//             if space < 8 || rand.gen_bool(0.5) {
//                 code.push(Token::END);
//             } else {
//                 code.push(Token::ELSE);
//                 code.append(&mut grow_stat(space, params, rand));
//                 code.push(Token::END);
//             }
//         }
//         Stat::WHILE => {
//             code.append(&mut grow_expr(params, rand));
//             // let mut space = size_left - code.len();

//             code.push(Token::Stat(Stat::LOAD));
//             code.push(Token::Reg(0));
//             code.push(Token::Reg(0));

//             // code.append(&mut grow_stat(space, params, rand));
//             // space = size_left - code.len();
//             // code.append(&mut grow_stat(space, params, rand));

//             code.push(Token::END);
//         }
//     }
//     return if size_left > code.len() as i32 {
//         code
//     } else {
//         vec![]
//     };
// }

pub fn create_random_indiv(params: &Params, rand: &mut StdRng) -> Program {
    todo!()
    // let mut program: Program = Vec::with_capacity(50);
    // program.append(&mut params.prefix.clone());
    // program.append(&mut grow_stat(params.max_size as i32, params, rand));
    // program.append(&mut grow_stat(params.max_size as i32, params, rand));
    // program.append(&mut params.suffix.clone());
    // program
}
