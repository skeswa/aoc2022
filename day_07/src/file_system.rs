use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{command_invocation::CommandInvocation, directory_entry};

/// Tree of files within directories.
#[derive(Debug)]
pub(crate) struct FileSystem {
    /// Id of the current directory node - starts at the root's id.
    current_node_id: usize,
    /// All nodes in the file system.
    nodes: Vec<FileSystemNode>,
    /// Id of the root directory node.
    root_node_id: usize,
}

impl FileSystem {
    /// Returns new [FileSystem] imperatively constructed from the given
    /// sequence of `command_invocations` describing a tree of files and
    /// directories, or [Err] if `command_invocations` is not well-formed.
    pub(crate) fn build_imperatively(
        command_invocations: Vec<CommandInvocation>,
    ) -> Result<FileSystem> {
        let mut file_system =
            FileSystem::new(ESTIMATED_ENTRIES_PER_DIRECTORY * command_invocations.len());

        for command_invocation in command_invocations {
            match command_invocation {
                CommandInvocation::ChangeDirectory { directory_path } => {
                    file_system.relocate(directory_path)?;
                }
                CommandInvocation::ListDirectoryContents { directory_entries } => {
                    for directory_entry in directory_entries {
                        match directory_entry {
                            directory_entry::DirectoryEntry::File { name, size } => {
                                file_system.upsert_file(name, size)?;
                            }
                            directory_entry::DirectoryEntry::Subdirectory { name } => {
                                file_system.upsert_subdirectory(name)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(file_system)
    }

    /// Returns a fresh instance of [FileSystem].
    fn new(approximate_size: usize) -> Self {
        let current_node_id: usize = 0;
        let mut nodes: Vec<FileSystemNode> = Vec::with_capacity(approximate_size);

        nodes.push(FileSystemNode::new_root(current_node_id));

        FileSystem {
            current_node_id: current_node_id,
            nodes: nodes,
            root_node_id: current_node_id,
        }
    }

    /// Returns an [Iterator] that iterates over all the directory nodes in
    /// this [FileSystem].
    pub(crate) fn sizes(&self) -> Vec<(&FileSystemNode, u32)> {
        let mut node_size_tuples: Vec<(&FileSystemNode, u32)> =
            Vec::with_capacity(self.nodes.len());

        self.furnish_node_size_tuples(self.root_node_id, &mut node_size_tuples);

        node_size_tuples
    }

    /// Traverses the entire file system, calculating the size of each node and
    /// adding it to `node_size_tuples`.
    ///
    ///  * `node_id` is the id of the currently visited node
    pub(crate) fn furnish_node_size_tuples<'a>(
        &'a self,
        node_id: usize,
        node_size_tuples: &mut Vec<(&'a FileSystemNode, u32)>,
    ) -> u32 {
        let node = &self.nodes[node_id];

        let size = if let Some(size) = node.size {
            size
        } else {
            let mut size: u32 = 0;
            for child_id in node.child_ids.iter() {
                size = size + self.furnish_node_size_tuples(*child_id, node_size_tuples)
            }

            size
        };

        if node_id != self.root_node_id {
            node_size_tuples.push((node, size));
        }

        size
    }

    /// Creates a new file within the current directory named `name`, returning
    /// its node id.
    ///
    /// * `size` is the number of bytes that the file contains
    fn create_file(&mut self, name: String, size: u32) -> usize {
        let next_node_id = self.nodes.len();

        self.nodes.push(FileSystemNode::new_file(
            next_node_id,
            name,
            self.current_node_id,
            size,
        ));
        self.nodes[self.current_node_id]
            .child_ids
            .push(next_node_id);

        return next_node_id;
    }

    /// Creates a new directory within the current directory named `name`,
    /// returning its node id.
    fn create_subdirectory(&mut self, name: String) -> usize {
        let next_node_id = self.nodes.len();

        self.nodes.push(FileSystemNode::new_directory(
            next_node_id,
            name,
            Some(self.current_node_id),
        ));
        self.nodes[self.current_node_id]
            .child_ids
            .push(next_node_id);

        return next_node_id;
    }

    /// Navigates to the directory named "name", creating it if it does not
    /// already exist.
    ///
    /// Returns [Some] `true` if a diretoy was created, or [Err] if relocation
    /// was impossible.
    fn relocate(&mut self, name: String) -> Result<bool> {
        let mut did_create_new_directory = false;

        lazy_static! {
            /// Regular expression designed to match strings that are all dots.
            static ref DOTS_PATTERN: Regex =
                Regex::new(r"^\s*(\.+)\s*$").unwrap();
        }

        // Handle if `name` is dots (e.g. "cd .", "cd ..").
        if let Some(captures) = DOTS_PATTERN.captures(&name) {
            let number_of_dots = captures.get(1).unwrap().as_str().len();

            let mut i = 1;
            let mut maybe_node = Some(&self.nodes[self.current_node_id]);
            while let Some(node) = maybe_node {
                if i >= number_of_dots {
                    break;
                }

                maybe_node = node.parent_id.map(|parent_id| &self.nodes[parent_id]);
                i = i + 1;
            }

            if let Some(node) = maybe_node {
                self.current_node_id = node.id;

                return Ok(did_create_new_directory);
            } else {
                return Err(anyhow!(
                    "Failed to relocate to \"{}\": no such directory",
                    name
                ));
            }
        }

        let maybe_existing_node = self.find_child_by_name(&name);

        let next_node_id = match maybe_existing_node {
            Some(existing_node) => {
                if existing_node.is_directory() {
                    Ok(existing_node.id)
                } else {
                    Err(anyhow!(
                        "Failed to relocate to \"{}\": it is a file not a directory",
                        name
                    ))
                }
            }
            None => {
                did_create_new_directory = true;

                Ok(self.create_subdirectory(name))
            }
        }?;

        self.current_node_id = next_node_id;

        return Ok(did_create_new_directory);
    }

    /// Creates a new file within the current directory named `name` if it does
    /// not already exist, returning whether a new node was created, or [Err] if
    /// creation fails.
    ///
    /// * `size` is the number of bytes that the file contains
    fn upsert_file(&mut self, name: String, size: u32) -> Result<bool> {
        let maybe_existing_node = self.find_child_by_name(&name);

        if let Some(existing_node) = maybe_existing_node {
            if existing_node.is_directory() {
                return Err(anyhow!(
                    "Cannot create file \"{}\" - it is already a directory",
                    name
                ));
            } else if existing_node.size.unwrap_or(0) != size {
                return Err(anyhow!(
                    "Cannot create file \"{}\" - it is already exists with a different size",
                    name
                ));
            } else {
                // File already exists.
                return Ok(false);
            }
        }

        self.create_file(name, size);

        Ok(true)
    }
    /// Creates a new directory of the current directory named `name` if it
    /// does not already exist, returning whether a new node was created, or
    /// [Err] if creation fails.
    fn upsert_subdirectory(&mut self, name: String) -> Result<bool> {
        let maybe_existing_node = self.find_child_by_name(&name);

        if let Some(existing_node) = maybe_existing_node {
            if !existing_node.is_directory() {
                return Err(anyhow!(
                    "Cannot create directory \"{}\" - it is already a file",
                    name
                ));
            } else {
                // Directory already exists.
                return Ok(false);
            }
        }

        self.create_subdirectory(name);

        Ok(true)
    }

    /// Returns some [FileSystemNode] that has the specified `name` and
    /// `parent_id`, returning [None] if no such node exists.
    pub(crate) fn find_child_by_name(&self, name: &str) -> Option<&FileSystemNode> {
        self.nodes[self.current_node_id]
            .child_ids
            .iter()
            .map(|child_id| &self.nodes[*child_id])
            .filter(|child| child.name == name)
            .next()
    }
}

/// Represents a single file or directory in the file system.
#[derive(Debug)]
pub(crate) struct FileSystemNode {
    /// Ids of [FileSystemNode] instances belonging to this [FileSystemNode].
    pub(crate) child_ids: Vec<usize>,

    /// Uniquely identifies this [FileSystemNode].
    pub(crate) id: usize,

    /// Textual label of this [FileSystemNode].
    pub(crate) name: String,

    /// Id of the [FileSystemNode] to which this [FileSystemNode] belongs.
    ///
    /// [None] if this [FileSystemNode] is the root directory.
    pub(crate) parent_id: Option<usize>,

    /// Size (in bytes) of this [FileSystemNode].
    ///
    /// [None] if this [FileSystemNode] is a directory.
    pub(crate) size: Option<u32>,
}

impl FileSystemNode {
    /// Returns a new [FileSystemNode] representing a directory.
    ///
    /// * `id` uniquely identifies the resulting [FileSystemNode]
    /// * `name` is the textual label of the directory
    /// * `parent_id` uniquely identifies the parent [FileSystemNode] of the
    ///   resulting [FileSystemNode]
    fn new_directory(id: usize, name: String, parent_id: Option<usize>) -> FileSystemNode {
        FileSystemNode {
            child_ids: Vec::with_capacity(ESTIMATED_ENTRIES_PER_DIRECTORY),
            id: id,
            name: name,
            parent_id: parent_id,
            size: None,
        }
    }

    /// Returns a new [FileSystemNode] representing a file.
    ///
    /// * `id` uniquely identifies the resulting [FileSystemNode]
    /// * `name` is the textual label of the file
    /// * `parent_id` uniquely identifies the parent [FileSystemNode] of the
    ///   resulting [FileSystemNode]
    /// * `size` is the number of bytes that the file contains
    fn new_file(id: usize, name: String, parent_id: usize, size: u32) -> FileSystemNode {
        FileSystemNode {
            child_ids: Vec::with_capacity(ESTIMATED_ENTRIES_PER_DIRECTORY),
            id: id,
            name: name,
            parent_id: Some(parent_id),
            size: Some(size),
        }
    }

    /// Returns a new [FileSystemNode] representing a root directory.
    ///
    /// * `id` is the name of the resulting root node
    fn new_root(id: usize) -> FileSystemNode {
        FileSystemNode::new_directory(id, "".to_string(), None)
    }

    /// Returns `true` if this [FileSystemNode] represents a directory.
    pub(crate) fn is_directory(&self) -> bool {
        self.size.is_none()
    }
}

/// How many entries are assumed to belong to a directory.
///
/// This value is used to calculate initial vector sizes.
const ESTIMATED_ENTRIES_PER_DIRECTORY: usize = 3;
