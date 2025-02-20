use plotters::prelude::*;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::process::Command;

fn main() {
    let image_path = "/tmp/area-chart.png";
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(image_path)
        .expect("Error creating image file");

    let root_area = BitMapBackend::new(image_path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Random chart", ("sans-serif", 30))
        .build_cartesian_2d(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let mut rng = rand::rng();
    let data: Vec<_> = (0..11).map(|_| rng.random_range(0..50)).collect();

    ctx.draw_series(AreaSeries::new((0..).zip(data.iter().copied()), 0, RED.mix(0.2)).border_style(RED))
        .unwrap();

    Command::new("open")
        .arg(image_path)
        .status()
        .expect("Error when opening image");
}
