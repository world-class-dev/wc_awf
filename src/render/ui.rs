#[allow(unused_imports)]
use crate::event::Event;
use crate::types::{Color, VectorShape};

/// Defines the axis direction for arranging child UI elements.
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutDirection {
    Row,
    Column,
}

/// Represents a 2D bounding rectangle used for UI positioning and hit testing.
#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Checks whether a given (px, py) coordinate lies within this rectangle.
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

/// Core UI node enumeration representing all renderable UI components.
#[derive(Clone, Debug, PartialEq)]
pub enum UiNode {
    Container {
        direction: LayoutDirection,
        padding: f32,
        gap: f32,
        background: Color,
        border_radius: f32,
        children: Vec<UiNode>,
    },
    Button {
        label: String,
        padding: f32,
        background: Color,
        hover_background: Option<Color>,
        on_click_id: u32,
        border_radius: f32,
    },
    Text {
        content: String,
        size: f32,
        color: Color,
    },
    Image {
        asset_id: String,
        width: f32,
        height: f32,
    },
    CandleStick {
        open: f32,
        high: f32,
        low: f32,
        close: f32,
    },
}

/// Responsible for calculating layout positions and emitting vector shapes for rendering.
pub struct LayoutEngine;

impl LayoutEngine {
    /// Computes spatial bounds for each UI node recursively and appends vector shapes to render queue.
    pub fn compute_and_render(node: &UiNode, bounds: Rect, shapes: &mut Vec<VectorShape>) {
        match node {
            UiNode::Container {
                direction,
                padding,
                gap,
                background,
                border_radius,
                children,
            } => {
                // Emit container background shape
                shapes.push(VectorShape::Rectangle {
                    x: bounds.x,
                    y: bounds.y,
                    w: bounds.width,
                    h: bounds.height,
                    color: background.clone(),
                    border_radius: *border_radius,
                });

                let mut current_x = bounds.x + padding;
                let mut current_y = bounds.y + padding;

                let num_children = children.len() as f32;
                let total_gap = if num_children > 1.0 { gap * (num_children - 1.0) } else { 0.0 };

                for child in children {
                    let child_width = match direction {
                        LayoutDirection::Row => {
                            (bounds.width - (padding * 2.0) - total_gap) / num_children
                        }
                        LayoutDirection::Column => bounds.width - (padding * 2.0),
                    };

                    let child_height = match direction {
                        LayoutDirection::Row => bounds.height - (padding * 2.0),
                        LayoutDirection::Column => 40.0,
                    };

                    let child_bounds = Rect {
                        x: current_x,
                        y: current_y,
                        width: child_width,
                        height: child_height,
                    };

                    // Recursive render pass for child nodes
                    Self::compute_and_render(child, child_bounds, shapes);

                    match direction {
                        LayoutDirection::Row => current_x += child_width + gap,
                        LayoutDirection::Column => current_y += child_height + gap,
                    }
                }
            }
            UiNode::Button {
                label,
                padding,
                background,
                border_radius,
                ..
            } => {
                // Emit button background
                shapes.push(VectorShape::Rectangle {
                    x: bounds.x,
                    y: bounds.y,
                    w: bounds.width,
                    h: bounds.height,
                    color: background.clone(),
                    border_radius: *border_radius,
                });

                // Calculate centered text position considering button padding
                let text_x = bounds.x + padding.max(10.0);
                let text_y = bounds.y + (bounds.height / 2.0) - 6.0;

                shapes.push(VectorShape::Text {
                    body: label.clone(),
                    x: text_x,
                    y: text_y,
                    size: 14.0,
                    color: Color { r: 255, g: 255, b: 255, a: 1.0 },
                });
            }
            UiNode::Text { content, size, color } => {
                shapes.push(VectorShape::Text {
                    body: content.clone(),
                    x: bounds.x,
                    y: bounds.y,
                    size: *size,
                    color: color.clone(),
                });
            }
            UiNode::Image { asset_id, width, height } => {
                shapes.push(VectorShape::Image {
                    id: asset_id.clone(),
                    x: bounds.x,
                    y: bounds.y,
                    w: *width,
                    h: *height,
                });
            }
            UiNode::CandleStick { open, high, low, close } => {
                shapes.push(VectorShape::CandleStick {
                    x: bounds.x + (bounds.width / 2.0),
                    open: *open,
                    high: *high,
                    low: *low,
                    close: *close,
                    width: bounds.width,
                });
            }
        }
    }
}

/// Maintains the root UI state and handles viewport calculations.
#[derive(Clone, Debug)]
pub struct UiContext {
    pub root: UiNode,
    pub viewport: Rect,
}

impl UiContext {
    /// Creates a new UiContext instance with the specified root node and viewport size.
    pub fn new(root: UiNode, viewport: Rect) -> Self {
        Self { root, viewport }
    }

    /// Triggers layout computation and shape generation for the UI tree.
    pub fn render(&self, shapes: &mut Vec<VectorShape>) {
        LayoutEngine::compute_and_render(&self.root, self.viewport.clone(), shapes);
    }

    /// Dispatches a pointer click event to determine interactive node targets.
    pub fn handle_click(&self, click_x: f32, click_y: f32) -> Option<u32> {
        Self::dispatch_click(&self.root, self.viewport.clone(), click_x, click_y)
    }

    /// Traverses the node tree to find the innermost clickable UI target under coordinates.
    fn dispatch_click(node: &UiNode, bounds: Rect, px: f32, py: f32) -> Option<u32> {
        if !bounds.contains(px, py) {
            return None;
        }

        match node {
            UiNode::Button { on_click_id, .. } => Some(*on_click_id),
            UiNode::Container { direction, padding, gap, children, .. } => {
                let mut current_x = bounds.x + padding;
                let mut current_y = bounds.y + padding;

                let num_children = children.len() as f32;
                let total_gap = if num_children > 1.0 { gap * (num_children - 1.0) } else { 0.0 };

                for child in children {
                    let child_width = match direction {
                        LayoutDirection::Row => (bounds.width - (padding * 2.0) - total_gap) / num_children,
                        LayoutDirection::Column => bounds.width - (padding * 2.0),
                    };

                    let child_height = match direction {
                        LayoutDirection::Row => bounds.height - (padding * 2.0),
                        LayoutDirection::Column => 40.0,
                    };

                    let child_bounds = Rect {
                        x: current_x,
                        y: current_y,
                        width: child_width,
                        height: child_height,
                    };

                    if let Some(id) = Self::dispatch_click(child, child_bounds, px, py) {
                        return Some(id);
                    }

                    match direction {
                        LayoutDirection::Row => current_x += child_width + gap,
                        LayoutDirection::Column => current_y += child_height + gap,
                    }
                }
                None
            }
            _ => None,
        }
    }
}

/// Primary UI interface holding context state and vector shape render queues.
#[derive(Clone, Debug)]
pub struct UiEngine {
    pub context: UiContext,
    pub render_queue: Vec<VectorShape>,
}

impl UiEngine {
    /// Initializes a new UiEngine instance with specified pixel dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        let root = UiNode::Container {
            direction: LayoutDirection::Column,
            padding: 0.0,
            gap: 0.0,
            background: Color { r: 0, g: 0, b: 0, a: 0.0 },
            border_radius: 0.0,
            children: Vec::new(),
        };

        let viewport = Rect {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        };

        Self {
            context: UiContext::new(root, viewport),
            render_queue: Vec::new(),
        }
    }

    /// Sets a new root UI tree for rendering dynamic interfaces.
    pub fn set_root(&mut self, root: UiNode) {
        self.context.root = root;
    }

    /// Clears the temporary render queue buffers.
    pub fn clear(&mut self) {
        self.render_queue.clear();
    }

    /// Executes layout rendering and flushes context shapes into the provided vector buffer.
    pub fn render(&mut self, shapes: &mut Vec<VectorShape>) {
        self.context.render(shapes);
        shapes.extend(self.render_queue.clone());
    }
}