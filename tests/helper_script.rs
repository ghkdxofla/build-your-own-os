//! 이 파일은 src와 src-manual 구현을 빌드하고 실행하는 도구입니다.
//! 
//! `cargo test --test helper_script -- --nocapture` 명령으로 실행할 수 있습니다.

use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // 자동 구현 빌드
    build_implementation("auto")?;
    
    // 수동 구현 빌드 (파일이 있는 경우에만)
    if check_manual_implementation_files() {
        build_implementation("manual")?;
    } else {
        println!("수동 구현 파일이 없습니다. 빌드를 건너뜁니다.");
    }
    
    Ok(())
}

// 구현 빌드 함수
fn build_implementation(features: &str) -> io::Result<()> {
    println!("{}을(를) 빌드 중...", if features == "auto" { "자동 구현" } else { "수동 구현" });
    
    let status = Command::new("cargo")
        .args(&["build", "--release", &format!("--features={}", features)])
        .status()?;
    
    if status.success() {
        println!("{}을(를) 성공적으로 빌드했습니다.", if features == "auto" { "자동 구현" } else { "수동 구현" });
        
        println!("QEMU로 실행하려면 다음 명령을 사용하세요:");
        println!("qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines -nographic");
    } else {
        println!("{}을(를) 빌드하는 데 실패했습니다.", if features == "auto" { "자동 구현" } else { "수동 구현" });
    }
    
    println!();
    Ok(())
}

// src-manual 폴더에 필요한 파일이 모두 있는지 확인
fn check_manual_implementation_files() -> bool {
    let required_files = [
        "boot.S", "main.rs", "uart.rs", "memory.rs", "process.rs", "fs.rs"
    ];
    
    for file in &required_files {
        let path = std::path::Path::new("src-manual").join(file);
        if !path.exists() {
            return false;
        }
    }
    
    true
}

#[test]
fn run_helper_script() {
    main().unwrap();
} 
