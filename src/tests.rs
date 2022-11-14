#[cfg(test)]
mod test {
    use crate::LineRasterizer;
    use crate::Point;

    #[test]
    fn test_symmetry() {
        let points: Vec<Point> = LineRasterizer::new((0, 0), (2, 5)).collect();
        let points2: Vec<Point> = LineRasterizer::new((2, 5), (0, 0)).collect();

        assert_eq!(points.len(), points2.len());

        assert!(points.contains(&(0, 1)));
        assert!(points2.contains(&(0, 1)));

        assert!(points.contains(&(1, 1)));
        assert!(points2.contains(&(1, 1)));

        assert!(points.contains(&(1, 2)));
        assert!(points2.contains(&(1, 2)));

        assert!(points.contains(&(1, 3)));
        assert!(points2.contains(&(1, 3)));

        assert!(points.contains(&(1, 4)));
        assert!(points2.contains(&(1, 4)));

        assert!(points.contains(&(2, 4)));
        assert!(points2.contains(&(2, 4)));
    }

    #[test]
    fn test_symmetry_2() {
        let points: Vec<Point> = LineRasterizer::new((0, 0), (2, 4)).collect();
        let points2: Vec<Point> = LineRasterizer::new((2, 4), (0, 0)).collect();

        assert_eq!(points.len(), points2.len());

        assert!(points.contains(&(0, 1)));
        assert!(points2.contains(&(0, 1)));

        assert!(points.contains(&(1, 1)));
        assert!(points2.contains(&(1, 1)));

        assert!(points.contains(&(1, 2)));
        assert!(points2.contains(&(1, 2)));

        assert!(points.contains(&(1, 3)));
        assert!(points2.contains(&(1, 3)));

        assert!(points.contains(&(2, 3)));
        assert!(points2.contains(&(2, 3)));
    }

    #[test]
    fn test_iterator() {
        let line_rasterizer_iter = LineRasterizer::new((0, 0), (2, 4));
        let mut points = Vec::new();

        for point in line_rasterizer_iter {
            points.push(point);
        }

        assert_eq!(points.len(), 5);

        assert!(points.contains(&(0, 1)));
        assert!(points.contains(&(1, 1)));
        assert!(points.contains(&(1, 2)));
        assert!(points.contains(&(1, 3)));
        assert!(points.contains(&(2, 3)));
    }

    #[test]
    fn test_perfect_diagonal() {
        let line_rasterizer_iter = LineRasterizer::new((0, 0), (4, 4));
        let mut points = Vec::new();

        for point in line_rasterizer_iter {
            points.push(point);
        }

        assert_eq!(points.len(), 3);

        assert!(points.contains(&(1, 1)));
        assert!(points.contains(&(2, 2)));
        assert!(points.contains(&(3, 3)));
    }

    #[test]
    fn test_consistency_both_paths_omit_perfect_diagonal_south_east() {
        let points: Vec<Point> = LineRasterizer::new((1, 1), (4, 2)).collect();
        let points2: Vec<Point> = LineRasterizer::new((1, 1), (2, 4)).collect();

        assert_eq!(points.len(), points2.len());
        assert_eq!(points.len(), 2);
        assert!(!points.contains(&(2, 2)));
        assert!(!points2.contains(&(2, 2)));
        assert!(!points.contains(&(3, 1)));
        assert!(!points2.contains(&(3, 1)));
    }

    #[test]
    fn test_consistency_both_paths_omit_perfect_diagonal_south_west() {
        let points: Vec<Point> = LineRasterizer::new((1, 1), (0, 4)).collect();
        let points2: Vec<Point> = LineRasterizer::new((1, 1), (-2, 2)).collect();

        assert_eq!(points.len(), points2.len());
        assert_eq!(points.len(), 2);
        assert!(!points.contains(&(0, 2)));
        assert!(!points2.contains(&(0, 2)));
        assert!(!points.contains(&(1, 3)));
        assert!(!points2.contains(&(1, 3)));
    }

    #[test]
    fn test_consistency_both_paths_omit_perfect_diagonal_north_east() {
        let points: Vec<Point> = LineRasterizer::new((1, 1), (4, 0)).collect();
        let points2: Vec<Point> = LineRasterizer::new((1, 1), (2, -2)).collect();

        assert_eq!(points.len(), points2.len());
        assert_eq!(points.len(), 2);
        assert!(!points.contains(&(2, 0)));
        assert!(!points2.contains(&(2, 0)));
        assert!(!points.contains(&(3, 1)));
        assert!(!points2.contains(&(3, 1)));
    }

    #[test]
    fn test_consistency_both_paths_omit_perfect_diagonal_north_west() {
        let points: Vec<Point> = LineRasterizer::new((1, 1), (-2, 0)).collect();
        let points2: Vec<Point> = LineRasterizer::new((1, 1), (0, -2)).collect();

        assert_eq!(points.len(), points2.len());
        assert_eq!(points.len(), 2);
        assert!(!points.contains(&(0, 0)));
        assert!(!points2.contains(&(0, 0)));
        assert!(!points.contains(&(-1, 1)));
        assert!(!points2.contains(&(-1, 1)));
    }

    #[test]
    fn test_long_line_one_diagonal_intersection() {
        let points: Vec<Point> = LineRasterizer::new((0, 0), (11, 5)).collect();

        assert!(points.contains(&(1, 0)));
        assert!(points.contains(&(1, 1)));
        assert!(points.contains(&(2, 1)));
        assert!(points.contains(&(3, 1)));
        assert!(points.contains(&(3, 2)));
        assert!(points.contains(&(4, 2)));
        assert!(points.contains(&(5, 2)));
        assert!(points.contains(&(6, 3)));
        assert!(points.contains(&(7, 3)));
        assert!(points.contains(&(8, 3)));
        assert!(points.contains(&(8, 4)));
        assert!(points.contains(&(9, 4)));
        assert!(points.contains(&(10, 4)));
        assert!(points.contains(&(10, 5)));
        assert_eq!(points.len(), 14);
    }

    #[test]
    fn test_long_line_one_diagonal_intersection_flipped() {
        let points: Vec<Point> = LineRasterizer::new((0, 0), (11, -5)).collect();

        assert!(points.contains(&(1, 0)));
        assert!(points.contains(&(1, -1)));
        assert!(points.contains(&(2, -1)));
        assert!(points.contains(&(3, -1)));
        assert!(points.contains(&(3, -2)));
        assert!(points.contains(&(4, -2)));
        assert!(points.contains(&(5, -2)));
        assert!(points.contains(&(6, -3)));
        assert!(points.contains(&(7, -3)));
        assert!(points.contains(&(8, -3)));
        assert!(points.contains(&(8, -4)));
        assert!(points.contains(&(9, -4)));
        assert!(points.contains(&(10, -4)));
        assert!(points.contains(&(10, -5)));
        assert_eq!(points.len(), 14);
    }
}
