mod kore;

fn main() {
    kore::init_logger().expect("failed to init logger");
}
