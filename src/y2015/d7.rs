use crate::*;

type Instructions<'a> = &'a HashMap<&'a str, Expression>;
type Values<'a> = HashMap<&'a str, u16>;

fn get<'a>(s: &'a str, insns: Instructions<'a>, values: &mut Values<'a>) -> u16 {
    if let Some(v) = values.get(s) {
        *v
    } else {
        let val = insns[s].eval(insns, values);
        values.insert(s, val);
        val
    }
}

#[derive(Clone)]
enum Operand {
    Int(u16),
    Use(String),
}

impl Operand {
    fn parse(s: String) -> Self {
        if let Ok(i) = s.parse() {
            Self::Int(i)
        } else {
            Self::Use(s)
        }
    }

    fn eval<'a>(&'a self, insns: Instructions<'a>, values: &mut Values<'a>) -> u16 {
        match self {
            Self::Int(i) => *i,
            Self::Use(s) => get(s, insns, values),
        }
    }
}

#[derive(Clone)]
enum Expression {
    Constant(Operand),
    BinOp {
        ty: String,
        op1: Operand,
        op2: Operand,
    },
    Not(String),
}

impl Expression {
    fn parse(s: &str) -> Result<Self> {
        Ok(if s.contains(' ') {
            if s.starts_with("NOT") {
                let not;
                scanfmt!(s, "NOT {}", not);
                Self::Not(not)
            } else {
                let ty;
                let op1: String;
                let op2: String;
                scanfmt!(s, "{} {} {}", op1, ty, op2);
                Self::BinOp {
                    ty,
                    op1: Operand::parse(op1),
                    op2: Operand::parse(op2),
                }
            }
        } else {
            Self::Constant(Operand::parse(s.into()))
        })
    }

    fn eval<'a>(&'a self, insns: Instructions<'a>, values: &mut Values<'a>) -> u16 {
        match self {
            Self::Constant(op) => op.eval(insns, values),
            Self::Not(op) => !get(op, insns, values),
            Self::BinOp { ty, op1, op2 } => {
                let x = op1.eval(insns, values);
                let y = op2.eval(insns, values);
                match &**ty {
                    "AND" => x & y,
                    "OR" => x | y,
                    "LSHIFT" => x << y,
                    "RSHIFT" => x >> y,
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn part1(s: String) -> Result<()> {
    let mut registers = HashMap::default();
    for l in s.lines() {
        let (lhs, rhs) = l.split_once(" -> ").unwrap();
        registers.insert(rhs.trim(), Expression::parse(lhs)?);
    }
    let mut values = HashMap::default();

    dbg!(registers["a"].eval(&registers, &mut values));

    Ok(())
}

pub fn part2(s: String) -> Result<()> {
    let mut registers = HashMap::default();
    for l in s.lines() {
        let (lhs, rhs) = l.split_once(" -> ").unwrap();
        registers.insert(rhs.trim(), Expression::parse(lhs)?);
    }

    let mut values = HashMap::default();
    let a = registers["a"].eval(&registers, &mut values);

    values.clear();
    let mut new_reg = registers.clone();

    new_reg.insert("b", Expression::Constant(Operand::Int(a)));

    dbg!(new_reg["a"].eval(&new_reg, &mut values));

    Ok(())
}
