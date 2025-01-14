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
    pub columns: Vec<SelectColumn>,
    pub from: TableReference,
    pub where_clause: Option<WhereClause>,
    pub group_by: Option<Vec<Expression>>,
    pub having: Option<Expression>,
    pub order_by: Option<Vec<OrderByItem>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectColumn {
    pub span: Span,
    pub expr: Expression,
    pub alias: Option<String>,
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
    BinaryOp {
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
