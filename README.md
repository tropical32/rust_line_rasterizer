# LineRasterizer

![line rasterization](rasterized_line.png "Line rasterization")

## Usage

### Get all points

    let points: Vec<Point> = LineRasterizer::new((0, 0), (2, 4)).collect();
    println!("points = {:?}", points);
    // points = [(0, 1), (1, 1), (1, 2), (1, 3), (2, 3), (2, 4)]

### Iterate points

    let line_rasterizer_iter = LineRasterizer::new((0, 0), (2, 4));

    for point in points {
        // (0, 1)
        // (1, 1)
        // (1, 2)
        // (1, 3)
        // (2, 3)
        // (2, 4)
    }
