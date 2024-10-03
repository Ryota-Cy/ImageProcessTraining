use std::process::Command;

fn main() {
    // NVCCでCUDAコードをコンパイルする
    Command::new("nvcc")
        .args(&["src/kernel.cu", "-o", "src/kernel.o", "--cubin"])
        .status()
        .expect("Failed to compile CUDA code");
}
