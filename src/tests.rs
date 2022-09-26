#[cfg(test)]
mod tests {
    use crate::LineRasterizer;
    use crate::Point;

    #[test]
    fn test_symmetry() {
        let points: Vec<Point> = LineRasterizer::new((0, 0), (2, 5)).collect();
        let points2: Vec<Point> = LineRasterizer::new((2, 5), (0, 0)).collect();

        assert_eq!(points.len(), points2.len());

        assert_eq!(points.contains(&(0, 1)), true);
        assert_eq!(points2.contains(&(0, 1)), true);

        assert_eq!(points.contains(&(1, 1)), true);
        assert_eq!(points2.contains(&(1, 1)), true);

        assert_eq!(points.contains(&(1, 2)), true);
        assert_eq!(points2.contains(&(1, 2)), true);

        assert_eq!(points.contains(&(1, 3)), true);
        assert_eq!(points2.contains(&(1, 3)), true);

        assert_eq!(points.contains(&(1, 4)), true);
        assert_eq!(points2.contains(&(1, 4)), true);

        assert_eq!(points.contains(&(2, 4)), true);
        assert_eq!(points2.contains(&(2, 4)), true);

        assert_eq!(points.contains(&(2, 5)), true);
        assert_eq!(points2.contains(&(2, 4)), true);
    }

    #[test]
    fn test_symmetry_2() {
        let points: Vec<Point> = LineRasterizer::new((0, 0), (2, 4)).collect();
        let points2: Vec<Point> = LineRasterizer::new((2, 4), (0, 0)).collect();

        assert_eq!(points.len(), points2.len());

        assert_eq!(points.contains(&(0, 1)), true);
        assert_eq!(points2.contains(&(0, 1)), true);

        assert_eq!(points.contains(&(1, 1)), true);
        assert_eq!(points2.contains(&(1, 1)), true);

        assert_eq!(points.contains(&(1, 2)), true);
        assert_eq!(points2.contains(&(1, 2)), true);

        assert_eq!(points.contains(&(1, 3)), true);
        assert_eq!(points2.contains(&(1, 3)), true);

        assert_eq!(points.contains(&(2, 3)), true);
        assert_eq!(points2.contains(&(2, 3)), true);

        assert_eq!(points.contains(&(2, 4)), true);
        assert_eq!(points2.contains(&(0, 0)), true);
    }

    #[test]
    fn test_iterator() {
        let line_rasterizer_iter = LineRasterizer::new((0, 0), (2, 4));
        let mut points = Vec::new();

        for point in line_rasterizer_iter {
            points.push(point);
        }

        assert_eq!(points.len(), 6);

        assert_eq!(points.contains(&(0, 1)), true);
        assert_eq!(points.contains(&(1, 1)), true);
        assert_eq!(points.contains(&(1, 2)), true);
        assert_eq!(points.contains(&(1, 3)), true);
        assert_eq!(points.contains(&(2, 3)), true);
        assert_eq!(points.contains(&(2, 4)), true);
    }
}
