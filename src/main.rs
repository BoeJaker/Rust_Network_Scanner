mod single_host;
mod network_range_scan_optimised;

fn main() {
    single_host::main_host();
    network_range_scan_optimised::main_range();
}