// EDOT engine

use std::{collections::HashMap, fmt::Display};

pub struct Instance {
	label: String,
}

impl Display for Instance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[ label = \"{}\" ];", self.label)
	}
}

pub struct Edge(Option<String>, String, String); // label, instance -> instance

impl Display for Edge {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} -> {}", self.1, self.2)?;
		if let Some(label) = &self.0 {
			write!(f, " [ label = \"{}\" ];", label)
		} else {
			write!(f, ";")
		}
	}
}

pub struct Digraph {
	instances: HashMap<String, Instance>,
	edges: Vec<Edge>,
}

impl Digraph {
	pub fn new() -> Self {
		Self {
			instances: HashMap::new(),
			edges: vec![],
		}
	}

	pub fn add_instance(&mut self, name: &str, label: &str) {
		self.instances.insert(
			name.to_string(),
			Instance {
				label: label.to_string(),
			},
		);
	}

	pub fn add_edge(&mut self, inst1: &str, inst2: &str, label: Option<&str>) {
		self.edges.push(Edge(
			if label.is_some() {
				Some(label.unwrap().to_owned())
			} else {
				None
			},
			inst1.to_string(),
			inst2.to_string(),
		));
	}

	pub fn indegrees_of(&self, n: usize) -> Vec<String> {
		let mut indegrees = self
			.instances
			.iter()
			.map(|(str, _)| (str, 0))
			.collect::<HashMap<&String, usize>>();

		for edge in &self.edges {
			indegrees.insert(&edge.2, indegrees.get(&edge.2).unwrap_or(&0) + 1);
		}

		indegrees.retain(|_, v| *v == n);

		indegrees
			.into_iter()
			.map(|(instance, _)| instance.to_owned())
			.collect()
	}
}

impl Display for Digraph {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "digraph G {{\n")?;
		self.instances.iter().for_each(|(name, inst)| {
			let _ = write!(f, "\t{} {}\n", name, inst);
		});
		self.edges.iter().for_each(|edge| {
			let _ = write!(f, "\t{}\n", edge);
		});
		write!(f, "}}")?;

		write!(f, "\n")
	}
}

#[cfg(test)]
mod tests {
	use super::Digraph;

	#[test]
	fn test() {
		let mut graph = Digraph::new();
		graph.add_instance("option", "Option 1 or 2");
		graph.add_instance("option1", "Option 1");
		graph.add_instance("option2", "Option 2");
		graph.add_edge("option", "option1", Some("Route 1"));
		graph.add_edge("option", "option2", None);

		println!("{}", graph);
	}
}
