use std::hash::Hash;

use crate::{
    function::FunctionDefinition,
    log,
    math::{AsLine, Ellipse, Line, Matrix, Point, Rect, RoundedRect, Size, Vec2},
    widget::{Component, Stack, Widget},
    Shape, WidgetStyleExt,
};

pub type NodeId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SocketId {
    node: NodeId,
    id: usize,
    kind: SocketKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputSocketId {
    node: NodeId,
    id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OutputSocketId {
    node: NodeId,
    id: usize,
}

pub trait AsSocketId: Sized + Copy {
    fn node(&self) -> NodeId;

    fn id(&self) -> usize;

    fn kind(&self) -> SocketKind;

    #[inline]
    fn into_generic(self) -> SocketId {
        SocketId {
            id: self.id(),
            kind: self.kind(),
            node: self.node(),
        }
    }

    #[inline]
    fn into_input(self) -> InputSocketId {
        assert!(self.kind() == SocketKind::Input);
        InputSocketId {
            id: self.id(),
            node: self.node(),
        }
    }

    #[inline]
    fn into_output(self) -> OutputSocketId {
        assert!(self.kind() == SocketKind::Output);
        OutputSocketId {
            id: self.id(),
            node: self.node(),
        }
    }

    #[inline]
    fn is_same_node(&self, other: impl AsSocketId) -> bool {
        self.node() == other.node()
    }

    #[inline]
    fn is_same_kind(&self, other: impl AsSocketId) -> bool {
        self.kind() == other.kind()
    }
}

impl AsSocketId for SocketId {
    #[inline]
    fn node(&self) -> NodeId {
        self.node
    }

    #[inline]
    fn id(&self) -> usize {
        self.id
    }

    #[inline]
    fn kind(&self) -> SocketKind {
        self.kind
    }
}

impl AsSocketId for InputSocketId {
    #[inline]
    fn node(&self) -> NodeId {
        self.node
    }

    #[inline]
    fn id(&self) -> usize {
        self.id
    }

    #[inline]
    fn kind(&self) -> SocketKind {
        SocketKind::Input
    }
}

impl AsSocketId for OutputSocketId {
    #[inline]
    fn node(&self) -> NodeId {
        self.node
    }

    #[inline]
    fn id(&self) -> usize {
        self.id
    }

    #[inline]
    fn kind(&self) -> SocketKind {
        SocketKind::Output
    }
}

impl AsSocketId for (usize, usize, SocketKind) {
    fn node(&self) -> NodeId {
        self.0
    }

    fn id(&self) -> usize {
        self.1
    }

    fn kind(&self) -> SocketKind {
        self.2
    }
}

const NODE_POINT_RADIUS: f64 = 4.0;
const NODE_POINT_COLLISION_RADIUS: f64 = NODE_POINT_RADIUS * 1.5;
const NODE_CONNECTION_WIDTH: f64 = 4.0;

struct NodeData {
    function: FunctionDefinition,

    sockets: Vec<Socket>,

    position: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SocketKind {
    Input,
    Output,
}

#[derive(Clone, Copy, Debug)]
struct Socket {
    enabled: bool,
    position: Point,
    kind: SocketKind,
}

#[derive(Debug, Clone)]
struct Connection {
    line: Line,
    input: InputSocketId,
    output: OutputSocketId,
}

pub struct Tree {
    connections: Vec<Connection>,
    nodes: Vec<NodeData>,
    transform: Matrix,
}

impl Default for Tree {
    fn default() -> Self {
        Tree {
            connections: Default::default(),
            nodes: Default::default(),
            transform: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum CastResult {
    Node(NodeId),
    Socket(SocketId, Point),
    Connection(InputSocketId),
    None,
}

impl Tree {
    pub fn new() -> Tree {
        Tree::default()
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.transform.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.transform.y
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.transform.a
    }

    pub fn screen_to_canvas(&self, point: impl Into<Point>) -> Point {
        let mouse_point = point.into();
        let canvas_point = (
            (mouse_point.x - self.x()) / self.z(),
            (mouse_point.y - self.y()) / self.z(),
        );
        canvas_point.into()
    }

    pub fn canvas_to_screen(&self, point: impl Into<Point>) -> Point {
        let canvas_point = point.into();
        (
            canvas_point.x * self.z() + self.x(),
            canvas_point.y * self.z() + self.y(),
        )
            .into()
    }

    pub fn drag(&mut self, mouse_delta: Vec2) {
        self.transform = self.transform * Matrix::translation(mouse_delta);
    }

    pub fn zoom(&mut self, delta: f64, pivot: Point) {
        const ZOOM_MIN: f64 = 0.3;
        const ZOOM_MAX: f64 = 2.0;

        let new_transform = self.transform * Matrix::scaling(1.0 + delta, pivot);
        let new_zoom = new_transform.a;
        if new_zoom >= ZOOM_MIN && new_zoom <= ZOOM_MAX {
            self.transform = new_transform;
        }
    }

    pub fn create_node(&mut self, function: FunctionDefinition, position: Point) {
        let mut node = NodeData::new(function);
        node.position = self.screen_to_canvas(position);
        self.nodes.push(node);
    }

    fn set_socket_state(&mut self, socket: impl AsSocketId, new_state: bool) {
        let node = &mut self.nodes[socket.node()];
        let mut socket = &mut node.sockets[socket.id()];
        socket.enabled = new_state;
    }

    fn socket_position(&self, socket: impl AsSocketId) -> Point {
        self.nodes[socket.node()].socket_position(socket.id())
    }

    pub fn delete_connection(&mut self, input_id: InputSocketId) {
        self.remove_connection(input_id);
    }

    fn remove_connection(&mut self, input_id: InputSocketId) -> Option<Connection> {
        if let Some(connection_pos) = self.connections.iter().position(|x| x.input == input_id) {
            let connection = self.connections.swap_remove(connection_pos);

            self.set_socket_state(connection.input, false);
            if !self
                .connections
                .iter()
                .any(|x| x.output == connection.output)
            {
                self.set_socket_state(connection.output, false);
            }
            Some(connection)
        } else {
            None
        }
    }

    pub fn create_connection(&mut self, from: impl AsSocketId, to: impl AsSocketId) {
        if from.is_same_node(to) || from.is_same_kind(to) {
            return;
        }

        let (input, output) = match from.kind() {
            SocketKind::Input => (from.into_input(), to.into_output()),
            SocketKind::Output => (to.into_input(), from.into_output()),
        };

        // if there are already connection with this input id and this output id just return.
        if self
            .connections
            .iter()
            .find(|x| x.input == input)
            .map(|connection| connection.output == output)
            .unwrap_or(false)
        {
            // connection already exists
            return;
        }

        // Remove connection for this input if exist.
        // If existed check for existence of other connections with
        // the same output_id. If there are none - disable output.
        self.remove_connection(input);

        self.set_socket_state(input, true);
        self.set_socket_state(output, true);
        self.connections.push(Connection {
            line: Line {
                start: self.socket_position(input),
                end: self.socket_position(output),
            },
            input,
            output,
        });
    }

    pub fn line_cast(&self, line: impl AsLine) -> Vec<CastResult> {
        let line = Line {
            start: self.screen_to_canvas(line.start()),
            end: self.screen_to_canvas(line.end()),
        };
        self.connections
            .iter()
            .filter(|x| x.line.is_intersect(line))
            .map(|x| CastResult::Connection(x.input))
            .collect()
    }

    pub fn point_cast(&self, point: Point) -> CastResult {
        let point = self.screen_to_canvas(point);
        for (node_id, node) in self.nodes.iter().enumerate().rev() {
            if node.bound_rect().contains_point(point) {
                for socket in 0..node.sockets.len() {
                    let world_position = node.socket_position(socket);
                    if NODE_POINT_COLLISION_RADIUS >= (point - world_position).len() {
                        return CastResult::Socket(
                            SocketId {
                                node: node_id,
                                id: socket,
                                kind: node.get_socket_kind(socket),
                            },
                            self.canvas_to_screen(world_position),
                        );
                    }
                }

                return CastResult::Node(node_id);
            }
        }

        for connection in self.connections.iter() {
            if connection.line.bound_rect().contains_point(point)
                && connection
                    .line
                    .are_collinear(point, NODE_CONNECTION_WIDTH * 1.5)
            {
                return CastResult::Connection(connection.input);
            }
        }

        CastResult::None
    }

    pub fn drag_node(&mut self, node: NodeId, delta: Vec2) {
        self.nodes[node].position += self.transform.transform_vector(delta);
    }
}

impl NodeData {
    const SIZE: Size = Size {
        height: 100.0,
        width: 200.0,
    };
    const CORNER_RADIUS: f64 = 10.0;

    const HOR_PADDING: f64 = Self::SIZE.width / 10.0;
    const VER_PADDING: f64 = Self::SIZE.height / 6.0;

    const LEFT_SIDE: f64 = -Self::SIZE.width / 2.0;
    const RIGHT_SIDE: f64 = Self::SIZE.width / 2.0;
    const TOP_SIDE: f64 = -Self::SIZE.height / 2.0;
    const BOTTOM_SIDE: f64 = Self::SIZE.height / 2.0;

    const LEFT_DOT_X: f64 = Self::LEFT_SIDE + Self::HOR_PADDING;
    const RIGHT_DOT_X: f64 = Self::RIGHT_SIDE - Self::HOR_PADDING;

    const OUTPUT_DOT_Y: f64 = Self::BOTTOM_SIDE - Self::VER_PADDING;
    const INPUT_DOT_Y: f64 = Self::TOP_SIDE + Self::VER_PADDING;

    pub fn new(function: FunctionDefinition) -> NodeData {
        let input_count = function.inputs.len();
        let output_count = function.outputs.len();

        let input_spacing = (Self::RIGHT_DOT_X - Self::LEFT_DOT_X) / (input_count + 1) as f64;
        let output_spacing = (Self::RIGHT_DOT_X - Self::LEFT_DOT_X) / (output_count + 1) as f64;

        let sockets = function
            .inputs
            .iter()
            .enumerate()
            .map(|(i, _)| Socket {
                enabled: false,
                position: (
                    input_spacing * (i as f64 + 1.0) + Self::LEFT_DOT_X,
                    Self::INPUT_DOT_Y,
                )
                    .into(),
                kind: SocketKind::Input,
            })
            .chain(function.outputs.iter().enumerate().map(|(i, _)| {
                Socket {
                    enabled: false,
                    position: (
                        output_spacing * (i as f64 + 1.0) + Self::LEFT_DOT_X,
                        Self::OUTPUT_DOT_Y,
                    )
                        .into(),
                    kind: SocketKind::Output,
                }
            }))
            .collect::<Vec<_>>();

        NodeData {
            sockets,
            function,
            position: Default::default(),
        }
    }

    #[inline]
    fn socket_position(&self, socket_id: usize) -> Point {
        self.sockets[socket_id].position + self.position.to_vector()
    }

    #[inline]
    fn get_socket_kind(&self, socket_id: usize) -> SocketKind {
        self.sockets[socket_id].kind
    }

    fn bound_rect(&self) -> Rect {
        Rect::from_center_size(self.position, Self::SIZE)
    }
}

impl Line {
    fn are_collinear(&self, point: Point, tolerance: f64) -> bool {
        let a = self.start;
        let b = self.end;
        let c = point;
        let slope_delta = (a.y - b.y) * (a.x - c.x) - (a.y - c.y) * (a.x - b.x);
        tolerance > (slope_delta.abs()) / 100.0
    }
}

impl Component for Socket {
    fn build(&self) -> Box<dyn Widget> {
        const RADIUS: f64 = 4.0;
        if !self.enabled {
            Ellipse::round(self.position, RADIUS).stroked().boxed()
        } else {
            Stack::from(vec![
                Ellipse::round(self.position, RADIUS).stroked().boxed(),
                Ellipse::round(self.position, RADIUS * 0.4).filled().boxed(),
            ])
            .boxed()
        }
        .with_fill_style("#DAD2BC")
        .with_shadow_color("#1B264F")
        .with_shadow_blur(4.0)
        .with_shadow_offset(0.0, 0.0)
        .with_line_width(1.0)
        .inspect(|| log!("drawing socket -----------------"))
        .boxed()
    }
}

impl Component for NodeData {
    fn build(&self) -> Box<dyn Widget> {
        const RREC: RoundedRect = RoundedRect {
            rect: Rect {
                left: NodeData::LEFT_SIDE,
                top: NodeData::TOP_SIDE,
                right: NodeData::RIGHT_SIDE,
                bottom: NodeData::BOTTOM_SIDE,
            },
            radius_x: NodeData::CORNER_RADIUS,
            radius_y: NodeData::CORNER_RADIUS,
        };

        let node_rect = RREC
            .with_shadow_blur(10.0)
            .with_fill_style("#25232388")
            .with_stroke_style("#F5F1ED")
            .with_line_width(2.5)
            .with_shadow_offset(0.0, 5.0)
            .stroked()
            .filled()
            .inspect(|| log!("drawing node body -----------------"))
            .boxed();

        let iter = std::iter::once(node_rect).chain(self.sockets.iter().map(|x| x.build()));

        Stack::of(iter)
            .translated(self.position.to_vector())
            .inspect(|| log!("drawing node -----------------"))
            .boxed()
    }
}

impl Component for Connection {
    fn build(&self) -> Box<dyn Widget> {
        self.line
            .with_shadow_blur(3.0)
            .with_stroke_style("#A99985")
            .with_line_width(4.0)
            .stroked()
            .inspect(|| log!("drawing connection -----------------"))
            .boxed()
    }
}

impl Component for Tree {
    fn build(&self) -> Box<dyn Widget> {
        Stack::from(vec![
            Stack::of(self.connections.iter().map(|x| x.build()))
                .inspect(|| log!("start drawing connections ---------------"))
                .boxed(),
            Stack::of(self.nodes.iter().map(|x| x.build()))
                .inspect(|| log!("start drawing all nodes ---------------"))
                .boxed(),
        ])
        .transformed(self.transform)
        .inspect(|| log!("drawing tree -----------------"))
        .boxed()
    }
}
