use serde::Deserialize;
use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum NodeType {
    Execution,
    Blocked,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum Port {
    Up,
    Down,
    Left,
    Right,
    Any,
    Last,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum PortStatus {
    Read,
    Write,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct PortInfo {
    pub port: Port,
    pub status: PortStatus,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub ports: Vec<PortInfo>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct PuzzleDataNode {
    pub id: usize,
    pub node_type: NodeType,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct NodeElementProps {
    pub node: Node,
}

#[function_component]
pub fn NodeElement(props: &NodeElementProps) -> Html {
    html! {
        <div class="node">
            
        </div>
    }
}