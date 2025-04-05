fn main() {
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rerun-if-changed=linker.ld");
    
    // src 폴더의 boot.S 파일 변경 감지
    println!("cargo:rerun-if-changed=src/boot.S");
    
    // src-manual 폴더의 boot.S 파일 변경 감지 (존재하는 경우)
    println!("cargo:rerun-if-changed=src-manual/boot.S");
} 
