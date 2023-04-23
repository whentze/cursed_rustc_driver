struct NoCallbacks;
impl rustc_driver::Callbacks for NoCallbacks {}

fn main() {
    rustc_driver::install_ice_hook();
    rustc_driver::init_rustc_env_logger();

    rustc_driver::RunCompiler::new(&[String::from("rustc"), String::from("./hello.rs")], &mut NoCallbacks).run().unwrap();
}