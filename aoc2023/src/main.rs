#[cfg(feature = "day01")]
pub mod day01;
#[cfg(feature = "day02")]
pub mod day02;

fn main() {
    #[cfg(feature = "day01p1")]
    day01::sol1::solve();
    #[cfg(feature = "day01p2")]
    day01::sol2::solve();

    #[cfg(feature = "day02p1")]
    day02::sol1::solve();
    #[cfg(feature = "day02p2")]
    day02::sol2::solve();
}