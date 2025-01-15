use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Select(SelectStatement),
    // Insert(InsertStatement),
    // Update(UpdateStatement),
    // Delete(DeleteStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStatement {
    pub span: Span,
    pub columns: Vec<SelectItem>,
    pub from: TableReference,
    pub where_clause: Option<WhereClause>,
    pub group_by: Option<Vec<Expression>>,
    pub having: Option<Expression>,
    // pub order_by: Option<Vec<OrderByItem>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectItem {
    Wildcard {
        span: Span,
    },
    QualifiedWildcard {
        span: Span,
        qualifier: String,
    },
    Expression {
        span: Span,
        expr: Expression,
        alias: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Column {
        span: Span,
        name: String,
        table: Option<String>,
    },
    Literal {
        span: Span,
        value: LiteralValue,
    },
    Asterisk {
        span: Span,
    },
    BinaryOperation {
        span: Span,
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
    Function {
        span: Span,
        name: String,
        args: Vec<Expression>,
    },
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Column { span, .. } => span.clone(),
            Expression::Literal { span, .. } => span.clone(),
            Expression::BinaryOperation { span, .. } => span.clone(),
            Expression::Function { span, .. } => span.clone(),
            Expression::Asterisk { span } => span.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableReference {
    pub span: Span,
    pub name: String,
    pub alias: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    pub span: Span,
    pub condition: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessEquals,
    GreaterEquals,
    And,
    Or,
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equals => write!(f, "="),
            Operator::NotEquals => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessEquals => write!(f, "<="),
            Operator::GreaterEquals => write!(f, ">="),
            Operator::And => write!(f, "AND"),
            Operator::Or => write!(f, "OR"),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
        }
    }
}
