mod single_host;
mod multiple_hosts;

fn main() {
    single_host::main_host();
    multiple_hosts::main_range();
}