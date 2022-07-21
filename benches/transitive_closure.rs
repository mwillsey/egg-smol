use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg_smol::{parse_program, EGraph};

fn line_graph_edges(n: usize) -> impl Iterator<Item = (i64, i64)> {
    assert!(n > 0);
    (0..(n - 1)).map(|x| (x as i64, x as i64 + 1))
}

fn bench_program(n: usize) -> String {
    const HEADER: &str = r#"
(relation path (i64 i64))
(relation edge (i64 i64))

(rule ((edge x y))
      ((path x y)))

(rule ((path x y) (edge y z))
      ((path x z)))
"#;
    let facts = Vec::from_iter(
        line_graph_edges(n).map(|(src, dst)| format!("(assert (edge {} {}))", src, dst)),
    );
    format!(
        "{}\n{}\n(run {})\n(check (path 0 {}))",
        HEADER,
        facts.join("\n"),
        n * n,
        n - 1
    )
}

fn line_graph(c: &mut Criterion, n: usize) {
    let program = bench_program(n);
    let bench_name = format!("egg-smol reachability ({} nodes)", n);
    let program = parse_program(&program).expect("parsing  should succeed");
    c.bench_function(&bench_name, |b| {
        b.iter(|| black_box(EGraph::default().run_program(program.clone())))
    });
}

mod ascent_defs {
    #![allow(unused_attributes, clippy::let_unit_value, clippy::clone_on_copy)]
    use ascent::ascent;
    ascent! {
        relation edge(i64, i64);
        relation path(i64, i64);

        path(x, y) <-- edge(x, y);
        path(x, z) <-- edge(x, y), path(y, z);
    }
}

fn line_graph_ascent(c: &mut Criterion, n: usize) {
    let name = format!("ascent reachability ({} nodes)", n);
    let edges = Vec::from_iter(line_graph_edges(n));
    c.bench_function(&name, |b| {
        b.iter(|| {
            let mut prog = ascent_defs::AscentProgram {
                edge: edges.clone(),
                ..Default::default()
            };
            prog.run();
            black_box(prog.path.len());
        })
    });
}

fn line_graph_16(c: &mut Criterion) {
    line_graph(c, 16);
}

fn line_graph_32(c: &mut Criterion) {
    line_graph(c, 32)
}

criterion_group!(egg_smol_benches, line_graph_16, line_graph_32);

fn line_graph_ascent_16(c: &mut Criterion) {
    line_graph_ascent(c, 16);
}

fn line_graph_ascent_32(c: &mut Criterion) {
    line_graph_ascent(c, 32)
}

fn line_graph_ascent_1k(c: &mut Criterion) {
    line_graph_ascent(c, 1 << 10)
}

criterion_group!(
    ascent_benches,
    line_graph_ascent_16,
    line_graph_ascent_32,
    line_graph_ascent_1k
);

criterion_main!(egg_smol_benches, ascent_benches);
