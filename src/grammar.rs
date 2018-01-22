#[cfg(debug_assertions)]
// A hack to enable gramatic re-evaluation on each build
const _GRAMMAR: &'static str = include_str!("n-quads.pest");

#[derive(Parser)]
#[grammar = "n-quads.pest"]
pub struct NQuadsParser;
