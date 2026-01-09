use omp_codegen::native;

native!(Test_AddNumbers, a: i32, b: i32, -> i32);

#[doc(hidden)]
pub fn load_functions() {
    crate::load_streamer!(Test_AddNumbers);
}
