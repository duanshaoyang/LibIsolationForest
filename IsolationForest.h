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

#pragma once

#include <map>
#include <random>
#include <stdint.h>
#include <string>
#include <time.h>
#include <vector>

namespace IsolationForest
{
	/// Each feature has a name and value.
	class Feature
	{
	public:
		Feature(const std::string& name, uint64_t value) { m_name = name; m_value = value; };
		virtual ~Feature() {};

		virtual std::string Name() const { return m_name; };
		virtual uint64_t Value() const { return m_value; };

	private:
		std::string m_name;
		uint64_t m_value;

		Feature() {};
	};

	typedef Feature* FeaturePtr;
	typedef std::vector<FeaturePtr> FeaturePtrList;

	/// This class represents a sample.
	/// Each sample has a name and list of features.
	class Sample
	{
	public:
		Sample(const std::string& name) { m_name = name; };
		virtual ~Sample() {};

		virtual void AddFeatures(const FeaturePtrList& features) { m_features.insert(std::end(m_features), std::begin(features), std::end(features)); };
		virtual void AddFeature(const FeaturePtr feature) { m_features.push_back(feature); };
		virtual FeaturePtrList Features() const { return m_features; };

	private:
		std::string m_name;
		FeaturePtrList m_features;

		Sample() {};
	};

	typedef Sample* SamplePtr;
	typedef std::vector<SamplePtr> SamplePtrList;

	/// Tree node, used internally.
	class Node
	{
	public:
		Node();
		Node(uint64_t splitValue);
		virtual ~Node();

		virtual std::string Name() const { return m_featureName; };

	private:
		std::string m_featureName;
		uint64_t m_splitValue;

		Node* m_left;
		Node* m_right;
	};

	typedef Node* NodePtr;
	typedef std::vector<NodePtr> NodePtrList;

	/// This class abstracts the random number generation.
	/// Inherit from this class if you wish to provide your own randomizer.
	/// Use Forest::SetRandomizer to override the default randomizer with one of your choosing.
	class Randomizer
	{
	public:
		Randomizer() : m_gen(m_rand()) {} ;
		virtual ~Randomizer() { srand((unsigned int)time(NULL)); };

		virtual uint64_t Rand() { return m_dist(m_gen); };
		virtual uint64_t RandUInt64(uint64_t min, uint64_t max) { return min + (Rand() % (max - min + 1)); }

	private:
		std::random_device m_rand;
		std::mt19937_64 m_gen;
		std::uniform_int_distribution<uint64_t> m_dist;
	};

	typedef std::pair<uint64_t, uint64_t> Uint64Pair;

	/// Isolation Forest implementation.
	class Forest
	{
	public:
		Forest();
		Forest(uint32_t numTrees, uint32_t subSamplingSize);
		virtual ~Forest();

		void SetRandomizer(Randomizer* newRandomizer);
		void AddSample(const Sample& sample);
		void Create();
		void Predict(const Sample& sample);

	private:
		Randomizer* m_randomizer; // Performs random number generation
		std::map<std::string, Uint64Pair> m_featureMinMax; // Lists each feature and maps it to it's corresponding min and max values
		std::vector<NodePtr> m_trees; // The decision trees that comprise the forest
		uint32_t m_numTreesToCreate; // The maximum number of trees to create
		uint32_t m_subSamplingSize;

		NodePtr CreateTree();
		void DestroyTree(NodePtr tree);
		void Destroy();
		void DestroyRandomizer();
	};
};
