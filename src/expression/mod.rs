use query_source::QuerySource;
use types::NativeSqlType;

pub trait Expression {
    type SqlType: NativeSqlType;

    fn to_sql(&self) -> String;
}

pub trait SelectableExpression<
    QS: QuerySource,
    Type: NativeSqlType = <Self as Expression>::SqlType,
>: Expression {
}