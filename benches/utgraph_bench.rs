use algraphs::graphs::utgraph::UTGraph;
use algraphs::models::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const MODEL: &GraphData = &GRAPHS[5];

fn get_deg_bench(c: &mut Criterion) {
    let graph = UTGraph::new(MODEL.verts).with_edges(MODEL.edges);
    c.bench_function("get degree", |b| b.iter(|| graph.degree(black_box(6))));
}

fn max_deg_bench(c: &mut Criterion) {
    let graph = UTGraph::new(MODEL.verts).with_edges(MODEL.edges);
    c.bench_function("max degree", |b| b.iter(|| graph.max_deg()));
}

fn is_star_bench(c: &mut Criterion) {
    let graph = UTGraph::new(MODEL.verts).with_edges(MODEL.edges);
    c.bench_function("is star", |b| b.iter(|| graph.is_star()));
}

fn path_bench(c: &mut Criterion) {
    let graph = UTGraph::new(MODEL.verts).with_edges(MODEL.edges);
    c.bench_function("find path", |b| {
        b.iter(|| graph.path(black_box(0), black_box(6)))
    });
}

criterion_group!(
    benches,
    get_deg_bench,
    max_deg_bench,
    is_star_bench,
    path_bench
);
criterion_main!(benches);
