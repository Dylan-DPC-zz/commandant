#[macro_export]
macro_rules! cmd {
    ($cmd:ident $args: ident) => {
        let args: Vec<&str> = $args.split(' ').collect();
        Command::new($cmd, args)
    }
}