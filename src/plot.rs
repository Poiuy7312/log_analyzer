use plotters::prelude::*;

fn get_y_axis_range(data: &Vec<(f64, f64)>) -> f64 {
    let mut max = 0.0;
    for value in data {
        if value.1 > max {
            max = value.1
        }
    }
    return max;
}

pub(crate) fn plot_graph(
    x_range: f64,
    data_type: String,
    time: String,
    data_point: Vec<(f64, f64)>,
) {
    let y_range = get_y_axis_range(&data_point);
    let root_area = BitMapBackend::new("images/2.7.png", (1400, 1000)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 100)
        .caption("Log Data", ("sans-serif", 40))
        .build_cartesian_2d(0.0..x_range, 0.0..y_range * 1.25)
        .unwrap();

    ctx.configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc(&data_type)
        .x_desc(&time)
        .axis_desc_style(("sans-serif", 30))
        .draw()
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data_point.iter().map(|point| Circle::new(*point, 1, &BLUE)))
        .unwrap();

    ctx.draw_series(LineSeries::new(
        data_point.iter().map(|point| (point.0, point.1)),
        &BLUE,
    ))
    .unwrap();
}

//let (stats, log_count) = count_status_code(log_data);
//println!("{:?}\n{}", stats, log_count)
