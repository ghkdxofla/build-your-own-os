//! 이 파일은 src와 src-manual 구현이 동일한 결과를 내는지 테스트합니다.
//! 
//! 메모: 이 테스트는 일반적인 단위 테스트와 달리 QEMU에서 실행되어야 하며,
//! 출력을 캡처하여 결과를 비교하는 통합 테스트입니다.

use std::process::Command;
use std::str;
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;

// QEMU에서 OS 실행 후 출력을 캡처하는 함수
fn run_os_and_capture_output(features: &str, input_commands: &[&str]) -> io::Result<String> {
    // 먼저 해당 features로 OS 빌드
    let build_status = Command::new("cargo")
        .args(&["build", "--release", &format!("--features={}", features)])
        .status()?;
    
    if !build_status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, 
            format!("{} 기능으로 OS 빌드 실패", features)));
    }
    
    // 입력 명령을 파일로 저장
    let input_file = format!("qemu_input_{}.txt", features);
    let mut file = File::create(&input_file)?;
    for cmd in input_commands {
        writeln!(file, "{}", cmd)?;
    }
    
    // QEMU 실행 및 출력 캡처
    let output = Command::new("qemu-system-riscv64")
        .args(&[
            "-machine", "virt", 
            "-bios", "none", 
            "-kernel", "target/riscv64gc-unknown-none-elf/release/rust-os-1000-lines",
            "-nographic",
            "-serial", "mon:stdio",
            "-initrd", &input_file,
            "-no-reboot",
            "-display", "none"
        ])
        .output()?;
    
    // 임시 파일 삭제
    std::fs::remove_file(input_file)?;
    
    // 출력 반환
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// 출력에서 프롬프트 차이만 무시하고 비교하는 함수
fn compare_outputs(auto_output: &str, manual_output: &str) -> bool {
    let auto_lines: Vec<&str> = auto_output.lines()
        .map(|line| line.replace("rust-os>", "prompt>").trim())
        .filter(|line| !line.is_empty())
        .collect();
    
    let manual_lines: Vec<&str> = manual_output.lines()
        .map(|line| line.replace("rust-os-manual>", "prompt>").trim())
        .filter(|line| !line.is_empty())
        .collect();
    
    // 디버깅을 위해 차이 출력
    if auto_lines.len() != manual_lines.len() {
        println!("출력 라인 수가 다릅니다:");
        println!("자동 구현: {} 라인", auto_lines.len());
        println!("수동 구현: {} 라인", manual_lines.len());
        return false;
    }
    
    for (i, (auto_line, manual_line)) in auto_lines.iter().zip(manual_lines.iter()).enumerate() {
        if auto_line != manual_line {
            println!("라인 {}에서 차이 발견:", i + 1);
            println!("자동 구현: {}", auto_line);
            println!("수동 구현: {}", manual_line);
            return false;
        }
    }
    
    true
}

// src-manual 폴더에 필요한 파일이 모두 있는지 확인
fn check_manual_implementation_files() -> bool {
    let required_files = [
        "boot.S", "main.rs", "uart.rs", "memory.rs", "process.rs", "fs.rs"
    ];
    
    for file in &required_files {
        let path = Path::new("src-manual").join(file);
        if !path.exists() {
            println!("파일 {}가 src-manual 폴더에 없습니다", file);
            return false;
        }
    }
    
    true
}

// 테스트 함수
#[test]
fn test_implementation_comparison() {
    // 수동 구현 파일이 있는지 먼저 확인
    if !check_manual_implementation_files() {
        println!("수동 구현 파일이 없습니다. 테스트를 건너뜁니다.");
        return;
    }
    
    // 테스트할 명령어 목록
    let test_commands = [
        "help",
        "echo 안녕하세요",
        "version",
        "unknown_command",
        "reboot"
    ];
    
    // 각 구현 실행 및 출력 캡처
    let auto_output = match run_os_and_capture_output("auto", &test_commands) {
        Ok(output) => output,
        Err(e) => {
            panic!("자동 구현 실행 중 오류 발생: {}", e);
        }
    };
    
    let manual_output = match run_os_and_capture_output("manual", &test_commands) {
        Ok(output) => output,
        Err(e) => {
            panic!("수동 구현 실행 중 오류 발생: {}", e);
        }
    };
    
    // 출력 저장 (디버깅용)
    let _ = std::fs::write("auto_output.log", &auto_output);
    let _ = std::fs::write("manual_output.log", &manual_output);
    
    // 출력 비교
    assert!(compare_outputs(&auto_output, &manual_output), 
        "자동 구현과 수동 구현의 출력이 다릅니다");
}

// QEMU가 설치되어 있는지 확인하는 테스트
#[test]
fn test_qemu_installed() {
    let output = Command::new("qemu-system-riscv64")
        .arg("--version")
        .output();
    
    match output {
        Ok(_) => assert!(true, "QEMU가 설치되어 있습니다"),
        Err(_) => panic!("QEMU가 설치되어 있지 않습니다. 테스트를 실행하려면 QEMU를 설치하세요.")
    }
}

// src와 src-manual 폴더 구조가 동일한지 확인하는 테스트
#[test]
fn test_folder_structure() {
    let src_files = std::fs::read_dir("src").unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    
    for file in &src_files {
        if file != "boot.S" && file.ends_with(".rs") {
            let manual_path = Path::new("src-manual").join(file);
            assert!(manual_path.exists(), 
                "src-manual 폴더에 {}가 없습니다", file);
        }
    }
    
    assert!(Path::new("src-manual/boot.S").exists(), 
        "src-manual 폴더에 boot.S가 없습니다");
} 
