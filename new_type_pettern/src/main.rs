#[derive(Debug)]
struct Meter(u32);
#[derive(Debug)]
struct CentiMeter(u32);

impl From<CentiMeter> for Meter {
    fn from(cm: CentiMeter) -> Meter {
        assert!(cm.0 % 100 == 0);
        Meter(cm.0 / 100)
    }
}

fn main() {
    let ten_meter = Meter(10); // m
    let twenty_centimeter = CentiMeter(200); // cm

    println!(
        "Area of rectangle is {:?} meter",
        rectangle_perimeter_length(ten_meter, twenty_centimeter.into())
    );
}

fn rectangle_perimeter_length(width: Meter, height: Meter) -> Meter {
    Meter((width.0 + height.0) * 2)
}
