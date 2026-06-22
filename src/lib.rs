pub trait Parser: Sized {
    fn parse() -> Self;
}

pub trait Subcommand {
    fn run(&self);
}
