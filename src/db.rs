use std::vec::Vec;

#[derive(Clone)]
enum QueryType {
    None,
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Clone)]
enum WhereType {
    And,
    Or,
}

#[derive(Clone)]
enum OrderType {
    Desc,
    Asc,
}

#[derive(Clone)]
enum JoinType {
    None,
    Left,
    Right,
    Inner,
    Full,
}

#[derive(Clone)]
pub struct WhereCond{
    where_type: WhereType,
    left: String,
    op: String,
    right: String,
}

#[derive(Clone)]
pub struct OrderItem{
    order_type: OrderType,
    column: String,
}

#[derive(Clone)]
pub struct JoinItem{
    join_type: JoinType,
    table: String,
}

#[derive(Clone)]
pub struct ColumnList{
    columns: Vec<String>,
}

#[derive(Clone)]
pub struct WhereList{
    wheres: Vec<WhereCond>,
}

#[derive(Clone)]
pub struct OrderList{
    orders: Vec<OrderItem>,
}

#[derive(Clone)]
pub struct JoinList{
    joins: Vec<JoinItem>,
}

#[derive(Clone)]
pub struct QueryBuilder {
    query_type :QueryType,
    table_name :String,
    columns :ColumnList,
    joins :JoinList,
    wheres :WhereList,
    orders :OrderList,
    distinct :bool
}

impl WhereCond {
    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        rst += &self.left;
        rst += &self.op;
        rst += "'";
        rst += &self.right;
        rst += "'";
        rst
    }
}

impl OrderItem {
    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        rst += &self.column;
        rst += " ";
        rst += match self.order_type{OrderType::Desc=>"DESC",OrderType::Asc=>"ASC"};
        rst
    }
}

impl JoinItem {
    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        rst += match self.join_type{
            JoinType::None => "JOIN",
            JoinType::Left => "LEFT JOIN",
            JoinType::Right => "RIGHT JOIN",
            JoinType::Inner => "INNER JOIN",
            JoinType::Full => "FULL JOIN",
        };
        rst += " ";
        rst += &self.table;
        rst
    }
}

impl WhereList {
    pub fn new() -> WhereList{
        WhereList{wheres:Vec::new()}
    }

    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        rst.push_str("WHERE ");
        for i in 0..self.wheres.len() {
            if i==0 {
                rst += &self.wheres[i].to_string();
            }else{
                rst += match self.wheres[i].where_type{
                    WhereType::And => "AND ",
                    WhereType::Or => "OR "
                };
                rst += &self.wheres[i].to_string();
            }
            rst += " ";
        }
        rst
    }

    pub fn push_back(& mut self, item:WhereCond){
        self.wheres.push(item);
    }
}


impl ColumnList {
    pub fn new() -> ColumnList{
        ColumnList{columns:Vec::new()}
    }

    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        rst.push('(');
        for i in 0..self.columns.len() {
            if i==0 {
                rst += &self.columns[i];
            }else{
                rst += ",";
                rst += &self.columns[i];
            }
        }
        rst.push(')');
        rst
    }

    pub fn push_back(& mut self, item:String){
        self.columns.push(item);
    }
}


impl OrderList {
    pub fn new() -> OrderList{
        OrderList{orders:Vec::new()}
    }

    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        rst+="ORDER BY ";
        for i in 0..self.orders.len() {
            if i==0 {
                rst += &self.orders[i].to_string();
            }else{
                rst += ", ";
                rst += &self.orders[i].to_string();
            }
        }
        rst
    }

    pub fn push_back(& mut self, item:OrderItem){
        self.orders.push(item);
    }
}

impl JoinList {
    pub fn new() -> JoinList{
        JoinList{joins:Vec::new()}
    }

    pub fn to_string(&self) -> String{
        let mut rst = String::new();
        for i in 0..self.joins.len() {
            if i==0 {
                rst += &self.joins[i].to_string();
            }else{
                rst += " ";
                rst += &self.joins[i].to_string();
            }
        }
        rst
    }

    pub fn push_back(& mut self, item:JoinItem){
        self.joins.push(item);
    }
}

impl QueryBuilder {
    pub fn select(&mut self) -> &mut Self{
        self.query_type = QueryType::Select;
        self
    }

    pub fn update(&mut self) -> &mut Self{
        self.query_type = QueryType::Update;
        self
    }

    pub fn column(&mut self, col : &str) -> &mut Self{
        self.columns.push_back(col.to_string());
        self
    }

    pub fn join(&mut self, table : &str) -> &mut Self{
        self.joins.push_back(JoinItem{join_type:JoinType::None, table:table.to_string()});
        self
    }

    pub fn leftjoin(&mut self, table : &str) -> &mut Self{
        self.joins.push_back(JoinItem{join_type:JoinType::Left, table:table.to_string()});
        self
    }

    pub fn rightjoin(&mut self, table : &str) -> &mut Self{
        self.joins.push_back(JoinItem{join_type:JoinType::Right, table:table.to_string()});
        self
    }

    pub fn innerjoin(&mut self, table : &str) -> &mut Self{
        self.joins.push_back(JoinItem{join_type:JoinType::Inner, table:table.to_string()});
        self
    }

    pub fn fulljoin(&mut self, table : &str) -> &mut Self{
        self.joins.push_back(JoinItem{join_type:JoinType::Full, table:table.to_string()});
        self
    }

    pub fn cond(&mut self, left :&str, op :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: op.to_string(),
            right: right.to_string(),});
        self
    }

    pub fn eq(&mut self, left :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: String::from("="),
            right: right.to_string(),});
        self
    }

    pub fn neq(&mut self, left :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: String::from("!="),
            right: right.to_string(),});
        self
    }

    pub fn ls(&mut self, left :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: String::from("<"),
            right: right.to_string(),});
        self
    }

    pub fn nls(&mut self, left :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: String::from(">="),
            right: right.to_string(),});
        self
    }

    pub fn gt(&mut self, left :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: String::from(">"),
            right: right.to_string(),});
        self
    }

    pub fn ngt(&mut self, left :&str, right :&str) -> &mut Self{
        self.wheres.push_back(WhereCond{where_type: WhereType::And,
            left: left.to_string(),
            op: String::from("<="),
            right: right.to_string(),});
        self
    }

    pub fn orderby(&mut self, col :&str, order :&str) -> &mut Self{
        self.orders.push_back(
            OrderItem{
                order_type:OrderType::Asc,
                column: col.to_string()
            }
        );
        self
    }

    pub fn asc(&mut self, col :&str) -> &mut Self{
        self.orders.push_back(
            OrderItem{
                order_type:OrderType::Asc,
                column: col.to_string()
            }
        );
        self
    }

    pub fn desc(&mut self, col :&str) -> &mut Self{
        self.orders.push_back(
            OrderItem{
                order_type:OrderType::Desc,
                column: col.to_string()
            }
        );
        self
    }

    pub fn get_sql(&self) -> String{
        match self.query_type {
            QueryType::None => String::new(),
            QueryType::Select => format!("SELECT {distinct} {columns} FROM {table} {join} {where} {orderby};",
                                distinct=match self.distinct{true=>"DISTINCT",false=>""},
                                columns=self.columns.to_string(),
                                table=self.table_name,
                                join=self.joins.to_string(),
                                where=self.wheres.to_string(),
                                orderby=self.orders.to_string()),
            QueryType::Update => format!("UPDATE {table} SET {update} {where};",
                                update="",
                                table=self.table_name,
                                where=self.wheres.to_string()),
            QueryType::Insert => format!("INSERT INTO {table} {columns} VALUES {values};",
                                columns=self.columns.to_string(),
                                table=self.table_name,
                                values=""),
            QueryType::Delete => format!("DELETE FROM {table} {where};",
                                where=self.wheres.to_string(),
                                table=self.table_name),
        }
    }
}



pub fn table(name: &str) -> QueryBuilder{
    QueryBuilder{
        query_type :QueryType::None,
        table_name :name.to_string(),
        columns :ColumnList::new(),
        joins :JoinList::new(),
        wheres :WhereList::new(),
        orders :OrderList::new(),
        distinct :false
    }
}
