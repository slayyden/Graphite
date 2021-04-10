mod crop;
mod ellipse;
mod line;
mod navigate;
mod path;
mod pen;
mod rectangle;
mod sample;
mod select;
mod shape;

use crate::events::{Event, ModKeys, MouseState, Response, Trace, TracePoint};
use crate::Color;
use crate::Document;
use crate::EditorError;
use document_core::Operation;
use std::{collections::HashMap, fmt};

pub trait Tool {
	fn handle_input(&mut self, event: &Event, document: &Document) -> (Vec<Response>, Vec<Operation>);
}

pub trait Fsm {
	type ToolData;
	fn transition(self, event: &Event, document: &Document, data: &mut Self::ToolData, responses: &mut Vec<Response>, operations: &mut Vec<Operation>) -> Self;
}

pub struct ToolFsmState {
	pub mouse_state: MouseState,
	pub mod_keys: ModKeys,
	pub trace: Trace,
	pub primary_color: Color,
	pub secondary_color: Color,
	pub active_tool_type: ToolType,
	pub tools: HashMap<ToolType, Box<dyn Tool>>,
	tool_settings: HashMap<ToolType, ToolSettings>,
}

impl Default for ToolFsmState {
	fn default() -> Self {
		ToolFsmState {
			mouse_state: MouseState::default(),
			mod_keys: ModKeys::default(),
			trace: Trace::new(),
			primary_color: Color::BLACK,
			secondary_color: Color::WHITE,
			active_tool_type: ToolType::Select,
			tools: gen_tools_hash_map! {
				Select => select::Select,
				Crop => crop::Crop,
				Navigate => navigate::Navigate,
				Sample => sample::Sample,
				Path => path::Path,
				Pen => pen::Pen,
				Line => line::Line,
				Rectangle => rectangle::Rectangle,
				Ellipse => ellipse::Ellipse,
				Shape => shape::Shape,
			},
			tool_settings: default_tool_settings(),
		}
	}
}

impl ToolFsmState {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn record_trace_point(&mut self) {
		self.trace.push(TracePoint {
			mouse_state: self.mouse_state,
			mod_keys: self.mod_keys,
		})
	}

	pub fn active_tool(&mut self) -> Result<&mut Box<dyn Tool>, EditorError> {
		self.tools.get_mut(&self.active_tool_type).ok_or(EditorError::UnknownTool)
	}

	pub fn swap_colors(&mut self) {
		std::mem::swap(&mut self.primary_color, &mut self.secondary_color);
	}
}

fn default_tool_settings() -> HashMap<ToolType, ToolSettings> {
	let tool_init = |tool: &ToolType| (*tool, tool.default_settings());
	// TODO: when 1.51 is more common, change this to use array::IntoIter
	[
		tool_init(&ToolType::Select),
		tool_init(&ToolType::Ellipse),
		tool_init(&ToolType::Shape), // TODO: Add more tool defaults
	]
	.iter()
	.copied()
	.collect()
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolType {
	Select,
	Crop,
	Navigate,
	Sample,
	Text,
	Fill,
	Gradient,
	Brush,
	Heal,
	Clone,
	Patch,
	BlurSharpen,
	Relight,
	Path,
	Pen,
	Freehand,
	Spline,
	Line,
	Rectangle,
	Ellipse,
	Shape,
}

impl fmt::Display for ToolType {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ToolType::Select => write!(formatter, "Select"),
			ToolType::Crop => write!(formatter, "Crop"),
			ToolType::Navigate => write!(formatter, "Navigate"),
			ToolType::Sample => write!(formatter, "Sample"),
			ToolType::Text => write!(formatter, "Text"),
			ToolType::Fill => write!(formatter, "Fill"),
			ToolType::Gradient => write!(formatter, "Gradient"),
			ToolType::Brush => write!(formatter, "Brush"),
			ToolType::Heal => write!(formatter, "Heal"),
			ToolType::Clone => write!(formatter, "Clone"),
			ToolType::Patch => write!(formatter, "Patch"),
			ToolType::BlurSharpen => write!(formatter, "BlurSharpen"),
			ToolType::Relight => write!(formatter, "Relight"),
			ToolType::Path => write!(formatter, "Path"),
			ToolType::Pen => write!(formatter, "Pen"),
			ToolType::Freehand => write!(formatter, "Freehand"),
			ToolType::Spline => write!(formatter, "Spline"),
			ToolType::Line => write!(formatter, "Line"),
			ToolType::Rectangle => write!(formatter, "Rectangle"),
			ToolType::Ellipse => write!(formatter, "Ellipse"),
			ToolType::Shape => write!(formatter, "Shape"),
		}
	}
}

impl ToolType {
	fn default_settings(&self) -> ToolSettings {
		match self {
			ToolType::Select => ToolSettings::Select { append_mode: SelectAppendMode::New },
			ToolType::Ellipse => ToolSettings::Ellipse,
			ToolType::Shape => ToolSettings::Shape {
				shape: Shape::Polygon { vertices: 3 },
			},
			_ => todo!(),
		}
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ToolSettings {
	Select { append_mode: SelectAppendMode },
	Ellipse,
	Shape { shape: Shape },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SelectAppendMode {
	New,
	Add,
	Subtract,
	Intersect,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Shape {
	Star { vertices: u32 },
	Polygon { vertices: u32 },
}
