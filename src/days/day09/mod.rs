use std::str::FromStr;

use crate::{solution::Solution, utils::vec2d::Vec2D};
use anyhow::Result;

pub struct Day09;

impl Solution for Day09 {
    type Answer = u64;

    fn day(&self) -> u8 {
        9
    }

    /// Part 1: Find the largest rectangle constructable by any two opposing red tiles.
    /// No restriction on whether the rectangle crosses polygon boundaries.
    fn part1(input: &str) -> Result<Self::Answer> {
        let tile_floor = input.parse::<TileFloor>()?;
        Ok(tile_floor.find_largest_rectangle_area().unwrap())
    }

    /// Part 2: Find the largest rectangle constructable by two opposing red tiles,
    /// but the entire rectangle area must be either red or green tiles
    /// (i.e., entirely enclosed within the polygon, not crossing any edges).
    fn part2(input: &str) -> Result<Self::Answer> {
        let tile_floor = input.parse::<TileFloor>()?;
        let (p1, p2) = tile_floor
            .find_non_intersecting_rectangle()
            .ok_or_else(|| anyhow::anyhow!("No non-intersecting rectangle found"))?;
        Ok(square_area(&p1, &p2))
    }
}

/// Represents a tile floor with red tiles at vertices forming a polygon.
/// - Red tiles: The Vec2D points in the list (vertices of the polygon)
/// - Green tiles: All tiles between consecutive red tiles (edges) and inside the polygon
struct TileFloor(Vec<Vec2D>);

impl TileFloor {
    /// Part 1 solution: Find the largest rectangle area formed by any two red tiles.
    /// Simply computes all possible rectangles and returns the maximum area.
    fn find_largest_rectangle_area(&self) -> Option<u64> {
        let all_pairs = self.rectangles();
        all_pairs
            .into_iter()
            .map(|(a, b)| square_area(&a, &b))
            .max()
    }

    /// Generate all possible pairs of red tiles and sort them by rectangle area (largest first).
    /// Each pair represents opposing corners of a potential rectangle.
    fn rectangles(&self) -> Vec<(Vec2D, Vec2D)> {
        let mut all_pairs = vec![];
        // Generate all unique pairs of red tiles
        for i in 0..self.0.len() {
            for j in i + 1..self.0.len() {
                all_pairs.push((self.0[i], self.0[j]));
            }
        }

        // Sort by area in descending order (largest rectangles first)
        all_pairs.sort_by(|p1, p2| square_area(&p1.0, &p1.1).cmp(&square_area(&p2.0, &p2.1)));
        all_pairs.reverse();

        all_pairs
    }

    /// Part 2 solution: Find the largest rectangle that doesn't cross any polygon edges.
    /// Checks each rectangle (in descending area order) to see if it intersects with
    /// any of the polygon's edges (formed by consecutive red tiles).
    fn find_non_intersecting_rectangle(&self) -> Option<(Vec2D, Vec2D)> {
        let length = self.0.len();
        let rectangles = self.rectangles();

        rectangles
            .iter()
            .find(|(p1, p2)| {
                // Get the bounds of the rectangle formed by these two red tiles
                let (xmin, xmax, ymin, ymax) = edges(p1, p2);

                // Check if this rectangle crosses any edge of the polygon
                for (i, red_tile) in self.0.iter().enumerate() {
                    let next_red_tile = &self.0[(i + 1) % length];

                    // Check if edge is vertical (same x-coordinate)
                    if red_tile.x == next_red_tile.x {
                        let (ylmin, ylmax) = (
                            red_tile.y.min(next_red_tile.y),
                            red_tile.y.max(next_red_tile.y),
                        );
                        // Check if rectangle crosses this vertical edge
                        // Rectangle crosses if: edge's x is strictly between rectangle's x bounds
                        // AND there's overlap in y coordinates
                        if xmin < red_tile.x
                            && xmax > red_tile.x
                            && !(ymin >= ylmax || ymax <= ylmin)
                        {
                            return false; // Rectangle crosses an edge, invalid
                        }
                    }
                    // Check if edge is horizontal (same y-coordinate)
                    else if red_tile.y == next_red_tile.y {
                        let (xlmin, xlmax) = (
                            red_tile.x.min(next_red_tile.x),
                            red_tile.x.max(next_red_tile.x),
                        );
                        // Check if rectangle crosses this horizontal edge
                        // Rectangle crosses if: edge's y is strictly between rectangle's y bounds
                        // AND there's overlap in x coordinates
                        if ymin < red_tile.y
                            && ymax > red_tile.y
                            && !(xmin >= xlmax || xmax <= xlmin)
                        {
                            return false; // Rectangle crosses an edge, invalid
                        }
                    } else {
                        // All edges should be either horizontal or vertical
                        unreachable!()
                    }
                }

                // Rectangle doesn't cross any edges, it's valid
                true
            })
            .copied()
    }
}

/// Calculate the area of a rectangle formed by two opposing corner points.
/// The area includes both corner tiles, hence the +1 for width and height.
fn square_area(top_left: &Vec2D, bottom_right: &Vec2D) -> u64 {
    let width = i64::abs(bottom_right.x - top_left.x) + 1;
    let height = i64::abs(bottom_right.y - top_left.y) + 1;
    width as u64 * height as u64
}

/// Extract the bounding box edges from two points.
/// Returns (xmin, xmax, ymin, ymax) representing the rectangle boundaries.
fn edges(p1: &Vec2D, p2: &Vec2D) -> (i64, i64, i64, i64) {
    let xmin = p1.x.min(p2.x);
    let xmax = p1.x.max(p2.x);
    let ymin = p1.y.min(p2.y);
    let ymax = p1.y.max(p2.y);
    (xmin, xmax, ymin, ymax)
}

/// Parse the input into a TileFloor.
/// Each line represents a red tile position (vertex of the polygon).
impl FromStr for TileFloor {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(tiles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day09.run_test1();
        assert_eq!(answer, 50);
    }

    #[test]
    fn test_part2() {
        let answer = Day09.run_test2();
        assert_eq!(answer, 24);
    }
}
