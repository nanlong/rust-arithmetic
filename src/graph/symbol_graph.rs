use std::collections::HashMap;
use super::graph::Graph;

// 符号图
pub struct SymbolGraph<'a> {
    st: HashMap<&'a str, usize>,    // 符号名 -> 索引
    keys: Vec<&'a str>,             // 索引 -> 符号名
    g: Graph,                       // 图
}

impl<'a> SymbolGraph<'a> {
    pub fn new(data: Vec<Vec<&'a str>>) -> Self {
        // 初始化 st
        let mut st = HashMap::new();

        for row in &data {
            for v in row {
                if ! st.contains_key(v) {
                    let index = st.keys().len();
                    st.insert(*v, index);
                }
            }
        }

        // 初始化 keys
        let mut keys = Vec::with_capacity(st.keys().len());

        for _ in 0..st.keys().len() {
            keys.push("");
        }

        for key in st.keys() {
            if let Some(index) = st.get(key) {
                keys[*index] = key;
            }
        }

        // 初始化 g
        let mut g = Graph::new(keys.len());

        for row in &data {
            if let (Some(v), Some(w)) = (st.get(row[0]), st.get(row[1])) {
                g.add_edge(*v, *w);
            }
        }

        // 主数据
        SymbolGraph { st, keys, g }
    }
}

#[test]
fn test() {
    let routes = vec![
        vec!["JFK", "MCO"],
        vec!["ORD", "DEN"],
        vec!["ORD", "HOU"],
        vec!["DFW", "PHX"],
        vec!["JFK", "ATL"],
        vec!["ORD", "DFW"],
        vec!["ORD", "PHX"],
        vec!["ATL", "HOU"],
        vec!["DEN", "PHX"],
        vec!["PHX", "LAX"],
        vec!["JFK", "ORD"],
        vec!["DEN", "LAS"],
        vec!["DFW", "HOU"],
        vec!["ORD", "ATL"],
        vec!["LAS", "LAX"],
        vec!["ATL", "MCO"],
        vec!["HOU", "MCO"],
        vec!["LAS", "PHX"],
    ];

    let symbol_graph = SymbolGraph::new(routes);
}