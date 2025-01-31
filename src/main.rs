use anyhow::{Context, Result};
use euler::*;

fn main() -> Result<()> {
    let input_args = read_args()?;
    if input_args.problem_num == 60 {
        let result = problem60(input_args.input_parameter).context("Error in problem 60")?;
        println!("{}", result);
    } else if input_args.problem_num == 201 {
        if input_args.input_parameter == 1 {
            // This is the example case
            let sorted_numset: Vec<usize> = [1, 3, 6, 8, 10, 11].into_iter().rev().collect();
            let subset_size = 3;
            let result = problem201(&sorted_numset, subset_size).context("Error in problem 201")?;
            println!("{}", result);
        } else if input_args.input_parameter == 2 {
            // This is the actual case
            let sorted_numset: Vec<usize> = (1..81).map(|x| x * x).rev().collect();
            let subset_size = 30;
            let result = problem201(&sorted_numset, subset_size).context("Error in problem 201")?;
            println!("{}", result);
        } else {
            return Err(ProgramError::NotImplemented.into());
        }
    } else if input_args.problem_num == 202 {
        if input_args.input_parameter == 1 {
            let res = problem202(11);
            println!("{}", res);
        } else if input_args.input_parameter == 2 {
            let res = problem202(1000001);
            println!("{}", res);
        } else if input_args.input_parameter == 3 {
            let res = problem202(12017639147);
            println!("{}", res);
        }
    } else if input_args.problem_num == 208 {
        let res = problem208(input_args.input_parameter * 5);
        println!("{}", res);
    } else if input_args.problem_num == 209 {
        let res = problem209();
        println!("{}", res);
    } else {
        return Err(ProgramError::NotImplemented.into());
    }
    Ok(())
}
