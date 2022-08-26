use crate::parser::{Expression, Factor, FactorOperator, FactorVariant, Term, TermOperator};

pub struct Interpreter {
    start_node: Expression,
}

impl Interpreter {
    pub fn new(ast: Expression) -> Self {
        Self { start_node: ast }
    }

    pub fn run(&self) -> f64 {
        self.visit_expression(&self.start_node)
    }

    fn visit_expression(&self, node: &Expression) -> f64 {
        let mut base = self.visit_term(&node.term);

        for (operator, term) in &node.following {
            let other = self.visit_term(&term);

            match operator {
                TermOperator::Plus => base += other,
                TermOperator::Minus => base -= other,
            }
        }
        base
    }

    fn visit_term(&self, node: &Term) -> f64 {
        let mut base = self.visit_factor(&node.factor);

        for (operator, factor) in &node.following {
            let other = self.visit_factor(&factor);
            match operator {
                FactorOperator::Mul => base *= other,
                FactorOperator::Div => base /= other,
            }
        }

        base
    }

    fn visit_factor(&self, node: &Factor) -> f64 {
        let mut value = match &node.variant {
            FactorVariant::Number(n) => *n,
            FactorVariant::Expression(e) => self.visit_expression(e),
        };

        match node.prefix {
            None => (),
            Some(TermOperator::Plus) => value *= 1.0,
            Some(TermOperator::Minus) => value *= -1.0,
        };

        value
    }
}
