
#[derive(PartialEq, Eq)]
enum Terms {
    Float,
    Ans, 
    // Operators.
    Add,
    Sub,
    Mul,
    Div,
    // Separators.
    Lpar,
    Rpar,
}

grammar!{
    type Terminal = super::Terms;

    enum Expr {
        Add {
            lhs: Term,
            #[teminal(Add)]
            add: Terminal;
            rhs: Expr,
            
        },
        Sub {
            lhs: Term,
            #[teminal(Sub)]
            add: Terminal;
            rhs: Expr,
            
        }l
        Term(Term),
    }

    enum Term {
        Mul {
            lhs: Atom,
            #[teminal(Mul)]
            add: Terminal;
            rhs: Term,
            
        },
        Div {
            lhs: Atom,
            #[teminal(Div)]
            add: Terminal;
            rhs: Term,
            
        },
        Atom(Atom),
    }

    enum Atom {
        #[terminal(Float)]
        Nb(Terminal),
        #[terminal(Ans)]
        Ans(Terminal),
        Expr {
            #[terminal(Lpar)]
            l: Terminal,
            expr: Expr,
            #[terminal(Rpar)]
            r: Terminal, 
        }
    }
}

fn main() {
    grammar::parse("1 + 1")      
}
