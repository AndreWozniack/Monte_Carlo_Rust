use plotters::prelude::*;
use rand::Rng;

fn main() {
    let iterations = [100, 10_000, 1_000_000, 10_000_000, 100_000_000];
    for &n in &iterations {
        let (inside_points, outside_points, pi) = monte_carlo_pi(n);
        plot_monte_carlo(&inside_points, &outside_points, pi, n);
    }
}

fn monte_carlo_pi(n: usize) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, f64) {
    let mut rng = rand::thread_rng();
    let mut inside_points = Vec::new();
    let mut outside_points = Vec::new();

    for _ in 0..n {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if (x - 0.5).powi(2) + (y - 0.5).powi(2) <= 0.25 {
            inside_points.push((x, y));
        } else {
            outside_points.push((x, y));
        }
    }
    let pi = 4.0 * (inside_points.len() as f64) / (n as f64);
    (inside_points, outside_points, pi)
}

fn plot_monte_carlo(
    inside_points: &[(f64, f64)],
    outside_points: &[(f64, f64)],
    pi: f64,
    n: usize,
) {
    let base_size = 3.0;
    let size_points = (base_size * 100.0 / n as f64).max(0.2);

    let name = format!("monte_carlo_{}.png", n);
    let root = BitMapBackend::new(&name, (600, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let text = format!("Monte Carlo Simulation for Pi = {:.5}", pi);
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(text, ("Arial", 20))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Plotando pontos dentro do círculo
    chart
        .draw_series(inside_points.iter().map(|&(x, y)| {
            Circle::new(
                (x, y),
                size_points,
                Into::<ShapeStyle>::into(&GREEN).filled(),
            )
        }))
        .unwrap();

    // Plotando pontos fora do círculo
    chart
        .draw_series(outside_points.iter().map(|&(x, y)| {
            Circle::new(
                (x, y),
                size_points,
                Into::<ShapeStyle>::into(&BLUE).filled(),
            )
        }))
        .unwrap();

    // Desenhando o contorno do círculo
    chart
        .draw_series(std::iter::once(PathElement::new(
            (0..360)
                .map(|d| {
                    let rad = (d as f64).to_radians();
                    (0.5 + 0.5 * rad.cos(), 0.5 + 0.5 * rad.sin())
                })
                .collect::<Vec<_>>(),
            &RED,
        )))
        .unwrap()
        .label("Circle")
        .legend(|(x, y)| Circle::new((x, y), 3, &RED));

    // Salvando a imagem
    root.present().unwrap();
}
