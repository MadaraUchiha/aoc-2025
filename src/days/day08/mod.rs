use std::str::FromStr;

use crate::{
    solution::Solution,
    utils::vec3d::{Vec3D, ZERO},
};
use anyhow::Result;

pub struct Day08;

impl Solution for Day08 {
    type Answer = u64;

    fn day(&self) -> u8 {
        8
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        // Part 1: Find the sizes of the 3 largest junction boxes after 1000 connections.
        // Strategy: Connect vectors based on their proximity (closest pairs first),
        // then multiply the sizes of the 3 largest resulting boxes.

        // Parse the input into a junction room where each vector starts in its own box
        let mut junction_room = input.parse::<JunctionRoom>()?;
        let all_vectors = junction_room.all_vectors();

        // Find all possible vector pairs sorted by distance (closest first)
        let pairs = find_closest_vector_mapping(&all_vectors);

        // Limit connections to 10 for tests, 1000 for the actual puzzle
        let max_pairs = if cfg!(test) { 10 } else { 1000 };

        // Process the first max_pairs connections
        for (i, (v1, v2)) in pairs.iter().enumerate() {
            log::debug!("Processing pair {}: {} -> {}", i, v1, v2);
            if i >= max_pairs {
                break;
            }

            // Find which boxes contain these vectors
            let from_index = junction_room.find_box_index(v1);
            let to_index = junction_room.find_box_index(v2);

            match (from_index, to_index) {
                // If vectors are in different boxes, merge them
                (Some(from_index), Some(to_index)) if from_index != to_index => {
                    log::debug!("Merging box {} into box {}", from_index, to_index);
                    junction_room.merge_junction_box(from_index, to_index);
                }
                // If they're already in the same box, skip
                (Some(from_index), Some(to_index)) => {
                    log::debug!("Boxes {} and {} are already merged", from_index, to_index);
                }
                _ => {
                    return Err(anyhow::anyhow!("Failed to find box index"));
                }
            }
        }

        // Sort boxes by size (largest first) and calculate score
        junction_room.sort_by_box_size();

        // Score is the product of the 3 largest box sizes
        Ok(junction_room.score())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        // Part 2: Find the last junction box connection needed to connect all vectors
        // into one large box. The answer is the product of the x-coordinates of the
        // two vectors involved in the final connection.

        // Parse input and find all pairs sorted by distance
        let mut junction_room = input.parse::<JunctionRoom>()?;
        let pairs = find_closest_vector_mapping(&junction_room.all_vectors());

        // Track the last successful merge
        let mut last_from = ZERO;
        let mut last_to = ZERO;

        // Keep connecting boxes until only one remains
        for (from_vector, to_vector) in pairs {
            // Stop when all vectors are in a single box
            if junction_room.0.len() == 1 {
                break;
            }

            // Find which boxes contain these vectors
            let from_index = junction_room.find_box_index(&from_vector);
            let to_index = junction_room.find_box_index(&to_vector);

            match (from_index, to_index) {
                // If vectors are in different boxes, merge them
                (Some(from_index), Some(to_index)) if from_index != to_index => {
                    log::debug!(
                        "Merging box {} ({}) into box {} ({})",
                        from_index,
                        from_vector,
                        to_index,
                        to_vector
                    );
                    junction_room.merge_junction_box(from_index, to_index);

                    // Remember this connection as it might be the last one
                    last_to = to_vector;
                    last_from = from_vector;
                }
                // If they're already in the same box, skip
                (Some(from_index), Some(to_index)) => {
                    log::debug!("Boxes {} and {} are already merged", from_index, to_index);
                }
                _ => {
                    return Err(anyhow::anyhow!("Failed to find box index"));
                }
            }
        }

        // Answer is the product of the x-coordinates of the last connection
        log::debug!(
            "{} * {} = {}",
            last_from.x,
            last_to.x,
            last_from.x * last_to.x
        );
        Ok((last_from.x * last_to.x) as u64)
    }
}

/// Represents a junction room containing multiple junction boxes.
/// Each junction box is a collection of 3D vectors that are connected together.
/// Initially, each vector starts in its own separate box.
#[derive(Clone, Debug)]
struct JunctionRoom(Vec<Vec<Vec3D>>);

impl JunctionRoom {
    /// Creates a new junction room where each vector starts in its own box
    fn new(vectors: Vec<Vec3D>) -> Self {
        Self(vectors.into_iter().map(|v| vec![v]).collect())
    }

    /// Merge two junction boxes into one, consuming the first box.
    /// Combines all vectors from the 'from' box into the 'to' box,
    /// then removes the 'from' box from the room.
    fn merge_junction_box(&mut self, from: usize, to: usize) {
        let from_box = &self.0[from];
        let to_box = &self.0[to];
        let merged_box = from_box.iter().chain(to_box.iter()).collect::<Vec<_>>();
        self.0[to] = merged_box.into_iter().cloned().collect();
        self.0.remove(from);
    }

    /// Sort junction boxes by size in descending order (largest first)
    fn sort_by_box_size(&mut self) {
        self.0.sort_by_key(|junction_box| junction_box.len());
        self.0.reverse();
    }

    /// Find which box contains a given vector
    fn find_box_index(&self, vector: &Vec3D) -> Option<usize> {
        self.0
            .iter()
            .position(|junction_box| junction_box.contains(vector))
    }

    /// Calculate the score as the product of the sizes of the 3 largest boxes
    fn score(&self) -> u64 {
        self.0
            .iter()
            .take(3)
            .map(|junction_box| junction_box.len() as u64)
            .product()
    }

    /// Get a flat list of all vectors in all boxes
    fn all_vectors(&self) -> Vec<Vec3D> {
        self.0.iter().flatten().cloned().collect()
    }
}

impl FromStr for JunctionRoom {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vectors = s
            .lines()
            .map(|line| line.parse::<Vec3D>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(vectors))
    }
}

/// Given a list of vectors, return all possible pairs sorted by their distance.
/// This creates a "connection plan" where closest vectors are connected first.
/// Uses squared distance for efficiency (avoids square root calculations).
fn find_closest_vector_mapping(vectors: &[Vec3D]) -> Vec<(Vec3D, Vec3D)> {
    log::debug!("Finding all vector pairs for: {:?}", vectors);
    let mut pairs = Vec::new();

    // Generate all unique pairs (combinations, not permutations)
    // For n vectors, this produces n*(n-1)/2 pairs
    for i in 0..vectors.len() {
        for j in i + 1..vectors.len() {
            let v1 = vectors[i];
            let v2 = vectors[j];
            pairs.push((v1, v2));
        }
    }

    // Sort by squared distance (smallest first)
    // This ensures we connect closest vectors first
    pairs.sort_by_key(|(v1, v2)| v1.square_distance_to(v2));

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_part1() {
        init_logger();
        let answer = Day08.run_test1();
        assert_eq!(answer, 40);
    }

    #[test]
    fn test_part2() {
        init_logger();
        let answer = Day08.run_test2();
        assert_eq!(answer, 25272);
    }

    #[test]
    fn test_find_closest_vector_mapping_two_vectors() {
        let vectors = vec![Vec3D::new(0, 0, 0), Vec3D::new(1, 0, 0)];
        let pairs = find_closest_vector_mapping(&vectors);

        // With 2 vectors, we should have 1 unique pair
        assert_eq!(pairs.len(), 1);
        // The pair should be the two vectors
        assert!(
            (pairs[0].0 == Vec3D::new(0, 0, 0) && pairs[0].1 == Vec3D::new(1, 0, 0))
                || (pairs[0].0 == Vec3D::new(1, 0, 0) && pairs[0].1 == Vec3D::new(0, 0, 0))
        );
    }

    #[test]
    fn test_find_closest_vector_mapping_three_collinear() {
        // Three points in a line: (0,0,0), (1,0,0), (5,0,0)
        // Pairs and their squared distances:
        // (0,0,0) <-> (1,0,0): distance² = 1
        // (0,0,0) <-> (5,0,0): distance² = 25
        // (1,0,0) <-> (5,0,0): distance² = 16
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 0, 0),
            Vec3D::new(5, 0, 0),
        ];
        let pairs = find_closest_vector_mapping(&vectors);

        // With 3 vectors, we should have 3 unique pairs
        assert_eq!(pairs.len(), 3);

        // First pair should be the closest: (0,0,0) <-> (1,0,0) with distance² = 1
        let first_pair = pairs[0];
        assert!(
            (first_pair.0 == Vec3D::new(0, 0, 0) && first_pair.1 == Vec3D::new(1, 0, 0))
                || (first_pair.0 == Vec3D::new(1, 0, 0) && first_pair.1 == Vec3D::new(0, 0, 0))
        );
        assert_eq!(first_pair.0.square_distance_to(&first_pair.1), 1);
    }

    #[test]
    fn test_find_closest_vector_mapping_3d_points() {
        // Test with actual 3D points
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 1, 1),    // sqrt(3) away from origin
            Vec3D::new(2, 0, 0),    // 4 away from origin (squared distance)
            Vec3D::new(10, 10, 10), // far away
        ];
        let pairs = find_closest_vector_mapping(&vectors);

        // With 4 vectors, we should have 6 unique pairs (4 choose 2)
        assert_eq!(pairs.len(), 6);

        // The first pair should be one of the closest pairs with distance² = 3
        // Either (0,0,0) <-> (1,1,1) or (2,0,0) <-> (1,1,1)
        let first_pair = pairs[0];
        assert_eq!(first_pair.0.square_distance_to(&first_pair.1), 3);

        // Verify pairs are sorted by distance
        for i in 0..pairs.len() - 1 {
            let dist1 = pairs[i].0.square_distance_to(&pairs[i].1);
            let dist2 = pairs[i + 1].0.square_distance_to(&pairs[i + 1].1);
            assert!(dist1 <= dist2, "Pairs not sorted by distance");
        }
    }

    #[test]
    fn test_find_closest_vector_mapping_negative_coords() {
        let vectors = vec![
            Vec3D::new(-5, -5, -5),
            Vec3D::new(0, 0, 0),
            Vec3D::new(5, 5, 5),
        ];
        let pairs = find_closest_vector_mapping(&vectors);

        // With 3 vectors, we should have 3 unique pairs
        assert_eq!(pairs.len(), 3);

        // All pairs have the same distances:
        // (-5,-5,-5) <-> (0,0,0): distance² = 75
        // (0,0,0) <-> (5,5,5): distance² = 75
        // (-5,-5,-5) <-> (5,5,5): distance² = 300

        // The first two pairs should have distance² = 75
        assert_eq!(pairs[0].0.square_distance_to(&pairs[0].1), 75);
        assert_eq!(pairs[1].0.square_distance_to(&pairs[1].1), 75);
        assert_eq!(pairs[2].0.square_distance_to(&pairs[2].1), 300);
    }

    #[test]
    fn test_find_closest_vector_mapping_square_formation() {
        // Four corners of a square on xy plane
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 0, 0),
            Vec3D::new(0, 1, 0),
            Vec3D::new(1, 1, 0),
        ];
        let pairs = find_closest_vector_mapping(&vectors);

        // With 4 vectors, we should have 6 unique pairs (4 choose 2)
        assert_eq!(pairs.len(), 6);

        // The first 4 pairs should be the edges (distance² = 1)
        // The last 2 pairs should be the diagonals (distance² = 2)
        for i in 0..4 {
            assert_eq!(pairs[i].0.square_distance_to(&pairs[i].1), 1);
        }
        for i in 4..6 {
            assert_eq!(pairs[i].0.square_distance_to(&pairs[i].1), 2);
        }
    }

    #[test]
    fn test_merge_junction_box_basic() {
        // Create a JunctionRoom with 3 separate boxes
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 1, 1),
            Vec3D::new(2, 2, 2),
        ];
        let mut room = JunctionRoom::new(vectors);

        // Initially should have 3 boxes, each with 1 vector
        assert_eq!(room.0.len(), 3);
        assert_eq!(room.0[0].len(), 1);
        assert_eq!(room.0[1].len(), 1);
        assert_eq!(room.0[2].len(), 1);

        // Merge box 0 into box 1
        room.merge_junction_box(0, 1);

        // Should now have 2 boxes
        assert_eq!(room.0.len(), 2);
        // The box at index 1 (now index 0 after removal) should have 2 vectors
        assert_eq!(room.0[0].len(), 2);
        assert!(room.0[0].contains(&Vec3D::new(0, 0, 0)));
        assert!(room.0[0].contains(&Vec3D::new(1, 1, 1)));
        // The box at index 2 (now index 1) should still have 1 vector
        assert_eq!(room.0[1].len(), 1);
        assert!(room.0[1].contains(&Vec3D::new(2, 2, 2)));
    }

    #[test]
    fn test_merge_junction_box_reverse_order() {
        // Test merging in reverse order (higher index into lower index)
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 1, 1),
            Vec3D::new(2, 2, 2),
        ];
        let mut room = JunctionRoom::new(vectors);

        // Merge box 2 into box 0
        room.merge_junction_box(2, 0);

        // Should now have 2 boxes
        assert_eq!(room.0.len(), 2);
        // The box at index 0 should have 2 vectors
        assert_eq!(room.0[0].len(), 2);
        assert!(room.0[0].contains(&Vec3D::new(2, 2, 2)));
        assert!(room.0[0].contains(&Vec3D::new(0, 0, 0)));
        // The box at index 1 should still be the original box 1
        assert_eq!(room.0[1].len(), 1);
        assert!(room.0[1].contains(&Vec3D::new(1, 1, 1)));
    }

    #[test]
    fn test_merge_junction_box_multiple_vectors() {
        // Create boxes with multiple vectors each
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 1, 1),
            Vec3D::new(2, 2, 2),
        ];
        let mut room = JunctionRoom::new(vectors);

        // First, merge box 0 into box 1 to create a box with 2 vectors
        room.merge_junction_box(0, 1);
        // Now we have 2 boxes: one with 2 vectors, one with 1 vector

        // Now merge the remaining single-vector box into the multi-vector box
        room.merge_junction_box(1, 0);

        // Should now have 1 box with all 3 vectors
        assert_eq!(room.0.len(), 1);
        assert_eq!(room.0[0].len(), 3);
        assert!(room.0[0].contains(&Vec3D::new(0, 0, 0)));
        assert!(room.0[0].contains(&Vec3D::new(1, 1, 1)));
        assert!(room.0[0].contains(&Vec3D::new(2, 2, 2)));
    }

    #[test]
    fn test_merge_junction_box_preserves_order() {
        // Test that vectors from 'from' box come before vectors from 'to' box
        let vectors = vec![Vec3D::new(0, 0, 0), Vec3D::new(1, 1, 1)];
        let mut room = JunctionRoom::new(vectors);

        // Merge box 0 into box 1
        room.merge_junction_box(0, 1);

        // The merged box should have vectors in order: from_box then to_box
        assert_eq!(room.0.len(), 1);
        assert_eq!(room.0[0].len(), 2);
        assert_eq!(room.0[0][0], Vec3D::new(0, 0, 0)); // from box 0
        assert_eq!(room.0[0][1], Vec3D::new(1, 1, 1)); // from box 1
    }

    #[test]
    fn test_merge_junction_box_consecutive_merges() {
        // Test multiple consecutive merges
        let vectors = vec![
            Vec3D::new(0, 0, 0),
            Vec3D::new(1, 0, 0),
            Vec3D::new(2, 0, 0),
            Vec3D::new(3, 0, 0),
        ];
        let mut room = JunctionRoom::new(vectors);

        assert_eq!(room.0.len(), 4);

        // Merge 0 into 1
        room.merge_junction_box(0, 1);
        assert_eq!(room.0.len(), 3);
        assert_eq!(room.0[0].len(), 2);

        // Merge 0 (which was originally 1) into 1 (which was originally 2)
        room.merge_junction_box(0, 1);
        assert_eq!(room.0.len(), 2);
        assert_eq!(room.0[0].len(), 3);

        // Merge 1 (which was originally 3) into 0
        room.merge_junction_box(1, 0);
        assert_eq!(room.0.len(), 1);
        assert_eq!(room.0[0].len(), 4);

        // All vectors should be in the final box
        assert!(room.0[0].contains(&Vec3D::new(0, 0, 0)));
        assert!(room.0[0].contains(&Vec3D::new(1, 0, 0)));
        assert!(room.0[0].contains(&Vec3D::new(2, 0, 0)));
        assert!(room.0[0].contains(&Vec3D::new(3, 0, 0)));
    }
}
