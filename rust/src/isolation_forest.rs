//	MIT License
//
//  Copyright © 2017 Michael J Simms. All rights reserved.
//
//	Permission is hereby granted, free of charge, to any person obtaining a copy
//	of this software and associated documentation files (the "Software"), to deal
//	in the Software without restriction, including without limitation the rights
//	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//	copies of the Software, and to permit persons to whom the Software is
//	furnished to do so, subject to the following conditions:
//
//	The above copyright notice and this permission notice shall be included in all
//	copies or substantial portions of the Software.
//
//	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//	SOFTWARE.

use std::collections::HashMap;

/// Each feature has a name and value.
pub struct Feature {
    name: String,
    value: u64,
}

impl Feature {
    pub fn new (name: &str, value: u64) -> Feature {
        Feature { name: name.to_string(), value: value }
    }
}

pub type FeatureList = Vec<Feature>;
pub type Uint64Vec = Vec<u64>;
pub type FeatureNameToValuesMap<'a> = HashMap<&'a String, Uint64Vec>;

/// This class represents a sample.
/// Each sample has a name and list of features.
pub struct Sample {
    name: String,
    features: FeatureList,
}

impl Sample {
    pub fn new (name: &str) -> Sample {
        Sample { name: name.to_string(), features: Sample::create_feature_list() }
    }

    fn create_feature_list() -> FeatureList {
        let v: FeatureList = vec![];
        v
    }

    pub fn add_features(&mut self, features: &mut FeatureList) {
        self.features.append(features);
    }

    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }
}

pub type SampleList = Vec<Sample>;

/// Tree node, used internally.
struct Node {
    feature_name: String,
    split_value: u64,
    left: NodeLink,
    right: NodeLink,
}

impl Node {
    pub fn new (feature_name: &str, split_value: u64) -> Node {
        Node { feature_name: feature_name.to_string(), split_value: split_value, left: None, right: None }
    }

	fn set_left_subtree(&mut self, subtree: NodeBox) {
		self.left = Some(subtree);
	}

	fn set_right_subtree(&mut self, subtree: NodeBox) {
		self.right = Some(subtree);
	}
}

type NodeBox = Box<Node>;
type NodeLink = Option<Box<Node>>;
type NodeList = Vec<Box<Node>>;

/// Isolation Forest implementation.
pub struct Forest<'a> {
    feature_values: FeatureNameToValuesMap<'a>, // Lists each feature and maps it to all unique values in the training set
    trees: NodeList, // The decision trees that comprise the forest
    num_trees_to_create: u32, // The maximum number of trees to create
    sub_sampling_size: u32, // The maximum depth of a tree
}

impl<'a> Forest<'a> {
    pub fn new (num_trees_to_create: u32, sub_sampling_size: u32) -> Forest<'a> {
        Forest { num_trees_to_create: num_trees_to_create, sub_sampling_size: sub_sampling_size, trees: Forest::initialize_trees(), feature_values: Forest::create_feature_name_to_values_map() }
    }

    fn initialize_trees() -> NodeList {
        let v: NodeList = vec![];
        v
    }

    fn create_feature_name_to_values_map() -> FeatureNameToValuesMap<'a> {
        let m = HashMap::new();
        m
    }

    pub fn add_sample(&mut self, sample: Sample) {
		// Add each of this sample's features to the list of known features
		// with the corresponding set of unique values.

        for feature in &sample.features {
            if self.feature_values.contains_key(&feature.name) {
                let mut feature_value_set = &mut self.feature_values[&feature.name];
                feature_value_set.push(feature.value);
            }
            else {
                let mut feature_value_set = Vec::new();
                feature_value_set.push(feature.value);
                self.feature_values.insert(&feature.name, feature_value_set);
            }
        }
    }

    fn create_tree(&mut self, feature_values: FeatureNameToValuesMap<'a>, depth: u32) -> NodeLink {
        // Creates and returns a single tree. As this is a recursive function, depth indicates the current depth of the recursion.

		// Sanity check.
		if feature_values.len() <= 1 {
			return None;
		}

		// If we've exceeded the maximum desired depth, then stop.
		if (self.sub_sampling_size > 0) && (depth >= self.sub_sampling_size) {
			return None;
		}

		// Randomly select a feature.
		let mut selected_feature_index = 0;
		if feature_values.len() > 1 {
			return None;
		}

		// Get the value list to split on.

		// Randomly select a split value.
		let mut split_value_index = 0;

		// Create a tree node to hold the split value.
        let mut tree = Some(Box::new(Node::new("", split_value_index)));

        // Create two versions of the feature value set that we just used,
        // one for the left side of the tree and one for the right.

        // Create the left subtree.

        // Create the right subtree.

        tree
    }

    pub fn create(&mut self) {
        // Creates a forest containing the number of trees specified to the constructor.

    	for _i in 0..self.num_trees_to_create {
            let tree = self.create_tree(self.feature_values, 0);
            match tree {
                None => {
                }
                Some(ref new_tree) => {
                    self.trees.push(*new_tree);
                }
            }
        }
    }

    fn score_tree(&self, sample: &Sample, tree: &NodeBox) -> f64 {
        // Scores the sample against the specified tree.

        let mut depth = 0.0;
        let mut current_node = tree;
        let mut done = false;

        while !done {
            let mut found_feature = false;

            for feature in &sample.features {
                if feature.name == current_node.feature_name {
					if feature.value < current_node.split_value {
                        match current_node.left {
                            None => {
                                done = true;
                            }
                            Some(ref next_node) => {
                                current_node = next_node;
                            }
                        }
                    } else {
                        match current_node.right {
                            None => {
                                done = true;
                            }
                            Some(ref next_node) => {
                                current_node = next_node;
                            }
                        }
                    }

                    depth = depth + 1.0;
                    found_feature = true;
                }
            }

			// If the tree contained a feature not in the sample then take
			// both sides of the tree and average the scores together.
			if found_feature == false {
                let left_tree = &current_node.left;
                let right_tree = &current_node.right;
                let mut left_depth = depth;
                let mut right_depth = depth;

                match left_tree {
                    &None => {
                    }
                    &Some(ref left_tree) => {
        			    left_depth = left_depth + self.score_tree(sample, left_tree);
                    }
                }
                match right_tree {
                    &None => {
                    }
                    &Some(ref right_tree) => {
        			    right_depth = right_depth + self.score_tree(sample, right_tree);
                    }
                }
				return (left_depth + right_depth) / 2.0;
            }
        }
        depth
    }

    pub fn score(&self, sample: &Sample) -> f64 {
        // Scores the sample against the entire forest of trees.

        let mut score = 0.0;

        if self.trees.len() > 0 {
            for tree in &self.trees {
                score += self.score_tree(sample, tree);
            }
            score /= self.trees.len() as f64;
        }
        score
    }
}
