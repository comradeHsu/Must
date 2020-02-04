pub mod file_output_stream;
pub mod file_input_stream;
mod file_descriptor;

pub fn init() {
    file_output_stream::init();
    file_input_stream::init();
    file_descriptor::init();
}