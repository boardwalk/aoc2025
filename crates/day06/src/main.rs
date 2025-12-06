use anyhow::Error;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

pub struct Worksheet {
    operands: Vec<u64>,
    operand_lines: usize,
    operators: Vec<Operator>,
}

impl Worksheet {
    fn parse() -> Result<Self, Error> {
        let mut operands = Vec::new();
        let mut operand_lines = 0;
        let mut operator_lines = 0;

        let mut operators = Vec::new();

        for line in std::io::stdin().lines() {
            let line = line?;

            let mut operator_line = false;

            for token in line.split_whitespace() {
                if token == "+" {
                    operators.push(Operator::Add);
                    operator_line = true;
                } else if token == "*" {
                    operators.push(Operator::Multiply);
                    operator_line = true;
                } else {
                    let num = u64::from_str_radix(token, 10)?;
                    operands.push(num);
                }
            }

            if operator_line {
                operator_lines += 1;
            } else {
                operand_lines += 1;
            }
        }

        assert_eq!(operator_lines, 1);

        let sheet = Self {
            operands,
            operand_lines,
            operators,
        };

        assert_eq!(
            sheet.num_operands_per_equation() * sheet.num_equations(),
            sheet.num_operands()
        );

        Ok(sheet)
    }

    fn num_operands_per_equation(&self) -> usize {
        self.operand_lines
    }

    fn num_operands(&self) -> usize {
        self.operands.len()
    }

    fn num_equations(&self) -> usize {
        self.operators.len()
    }

    fn get_operator(&self, equation_num: usize) -> Operator {
        self.operators[equation_num]
    }

    fn get_operand(&self, equation_num: usize, operand_num: usize) -> u64 {
        let index = operand_num * self.num_equations() + equation_num;

        self.operands[index]
    }
}

fn main() -> Result<(), Error> {
    let sheet = Worksheet::parse()?;
    let mut total = 0;

    for equation_num in 0..sheet.num_equations() {
        let operator = sheet.get_operator(equation_num);

        println!("working on {equation_num}, {operator:?}");
        let mut accum = match operator {
            Operator::Add => 0,
            Operator::Multiply => 1,
        };

        for operand_num in 0..sheet.num_operands_per_equation() {
            let operand = sheet.get_operand(equation_num, operand_num);
            println!("operand = {operand}");

            match operator {
                Operator::Add => {
                    accum += operand;
                }
                Operator::Multiply => {
                    accum *= operand;
                }
            }
        }

        println!("accum = {accum}");

        total += accum;
    }

    println!("total = {total}");

    Ok(())
}
