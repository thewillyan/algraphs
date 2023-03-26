use algraphs::benchmark;

fn main() {
    let joke = "My software never has bugs. It just develops random features.";
    let perf_msg = benchmark::med_exec_time(&|| joke).msg("Joke");
    println!("{perf_msg}");
}
