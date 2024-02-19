use crate::Width;

use std::collections::{HashMap, HashSet};

use xmltree::*;

use super::regs::CSR_BLOCK_SIZE;

#[derive(Debug)]
pub struct Register {
	pub name: String,
	pub offset: u32,
	pub width: Width,
	pub aliases: Vec<String>,
}

#[derive(Debug)]
pub struct Block {
	pub name: String,
	pub base: u32,
	pub count: u32,
	pub registers: Vec<Register>,
}

#[derive(Debug)]
pub struct Registry {
	pub blocks: HashMap<String, Block>,
}

impl Registry {
	pub fn new() -> Registry {
		Registry {
			blocks: HashMap::new(),
		}
	}
}

#[derive(Debug)]
pub struct RegistryParser {
	registry: Registry,
	relative_blocks: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Error {
	InvalidNode,
	MissingAttribute,
	InvalidBaseAddress,
	UnresolvedBlock(String),
}

impl RegistryParser {
	pub fn new() -> RegistryParser {
		RegistryParser {
			registry: Registry::new(),
			relative_blocks: HashMap::new(),
		}
	}

	pub fn add_block(&mut self, node: &Element) -> Result<(), Error> {
		if node.name != "block" {
			return Err(Error::InvalidNode);
		}

		if !node.attributes.contains_key("base") {
			return Err(Error::MissingAttribute);
		}

		if !node.attributes.contains_key("name") {
			return Err(Error::MissingAttribute);
		}

		if !node.attributes.contains_key("count") {
			return Err(Error::MissingAttribute);
		}

		let block_name = node.attributes.get("name").unwrap().clone();
		let count = node.attributes.get("count")
			.map(|s| s.parse::<u32>().ok())
			.flatten()
			.ok_or(Error::MissingAttribute)?;
			
		let base = node.attributes.get("base")
			.map(|s| s.strip_prefix("0x"))
			.flatten()
			.and_then(
				|x| u32::from_str_radix(&x, 16).ok()
			);
		let base = if let Some(addr) = base {
			addr
		} else {
			// This is a relative block
			let base_name = node.attributes.get("base").unwrap().clone();
			if let Some(block) = self.registry.blocks.get(&base_name) {
				if !self.relative_blocks.contains_key(&base_name) {
					block.base + block.count * CSR_BLOCK_SIZE
				} else {
					self.relative_blocks.insert(block_name.clone(), base_name);
					0
				}
			} else {
				self.relative_blocks.insert(block_name.clone(), base_name);
				0
			}
		};

		let mut block = Block {
			name: block_name.clone(),
			base,
			count,
			registers: Vec::new(),
		};

		for node in &node.children {
			if let Some(register) = node.as_element() {
				if register.name != "reg" {
					return Err(Error::InvalidNode);
				}

				let reg_name = register.attributes.get("name")
					.ok_or(Error::MissingAttribute)?.clone();
				let offset_string = register.attributes.get("offset")
					.map(|s| s.strip_prefix("0x"))
					.flatten()
					.ok_or(Error::MissingAttribute)?;
				let offset = u32::from_str_radix(offset_string, 16)
					.map_err(|_| Error::InvalidBaseAddress)?;

				let mut qualified_name = block_name.clone() + "_";
				qualified_name.push_str(&reg_name);

				let width = register.attributes.get("size").and_then(|w| match w.as_str() {
					"byte" => Some(Width::Byte),
					"short" => Some(Width::Short),
					"word" => Some(Width::Word),
					_ => None,
				}).ok_or(Error::MissingAttribute)?;

				// Handle alias
				let mut aliases = Vec::new();
				for alias in register.children.iter() {
					if let Some(alias) = alias.as_element() {
						if alias.name != "alias" {
							Err(Error::InvalidNode)?;
						}

						let name = alias.attributes.get("name")
							.ok_or(Error::MissingAttribute)?;
						aliases.push(name.clone());
					}
				}

				let reg = Register {
					aliases,
					name: reg_name.clone(),
					offset,
					width,
				};

				block.registers.push(reg);
			}
		}

		self.registry.blocks.insert(block_name, block);
		Ok(())
	}

	fn resolve_blocks(&mut self) -> Result<(), Error> {
		let mut visited = HashSet::new();
		let mut order = vec![];

		// Flatten the DAG represented by the block references
		for block_name in self.registry.blocks.keys() {
			if !visited.contains(block_name) {
				self.visit_block(block_name, &mut visited, &mut order)?;
			}
		}

		for block_name in order {
			if !self.relative_blocks.contains_key(&block_name) {
				continue;
			}

			let referenced_name = self.relative_blocks.get(&block_name).unwrap();
			let referenced = self.registry.blocks.get(referenced_name)
				.ok_or(Error::UnresolvedBlock(block_name.clone()))?;
			let base = referenced.base + referenced.count * CSR_BLOCK_SIZE;

			if let Some(block) = self.registry.blocks.get_mut(&block_name) {
				block.base = base;
			} else {
				return Err(Error::UnresolvedBlock(block_name));
			}
		}

		Ok(())
	}

	fn visit_block(&self, block_name: &String, visited: &mut HashSet<String>, order: &mut Vec<String>) -> Result<(), Error> {
		visited.insert(block_name.clone());
		if let Some(referenced_block) = self.relative_blocks.get(block_name) {
			if !visited.contains(referenced_block) {
				self.visit_block(referenced_block, visited, order)?;
			}
		}

		order.push(block_name.clone());
		Ok(())
	}

	pub fn finish(mut self) -> Result<Registry, Error> {
		self.resolve_blocks()?;
		Ok(self.registry)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		let psr_block =
		r##"<block name="psr" base="0x0" count="1">
			<reg name="psr0" offset="0x0" size="word" alias="psr" />
		</block>"##;
		let isr_block = r##"<block name="isr" base="psr" count="3">
			<reg name="base" offset="0x0" size="word" />
			<reg name="err1" offset="0x4" size="word" />
		</block>"##;

		let psr_root = Element::parse(psr_block.as_bytes()).unwrap();
		let isr_root = Element::parse(isr_block.as_bytes()).unwrap();
		let mut parser = RegistryParser::new();

		parser.add_block(&isr_root).unwrap();
		println!("Parser: {parser:?}");

		parser.add_block(&psr_root).unwrap();
		println!("Parser: {parser:?}");

		let registry = parser.finish();
		println!("Registry: {registry:?}");
	}
}
