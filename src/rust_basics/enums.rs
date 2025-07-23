#[derive(Debug)]
pub enum IppAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub enum Opinion {
    BAD(String),
    DECENT(String),
    GOOD(String),
} 
