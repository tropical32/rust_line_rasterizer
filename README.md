# LineRasterizer

![line rasterization](rasterized_line.png "Line rasterization")

## Usage

### Get all points

    use rust_line_rasterizer::LineRasterizer;

    fn main(){
        let points: Vec<_> = LineRasterizer::new((0, 0), (2, 4)).collect();
        println!("points = {:?}", points);
        // points = [(0, 1), (1, 1), (1, 2), (1, 3), (2, 3)]
    }


### Iterate points

    use rust_line_rasterizer::LineRasterizer;

    fn main() {
        let line_rasterizer_iter = LineRasterizer::new((2, 4), (0, 0));

        for point in line_rasterizer_iter {
            println!("{:?}", point);
            // (2, 3)
            // (1, 3)
            // (1, 2)
            // (1, 1)
            // (0, 1)
        }
    }
