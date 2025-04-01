#![no_std] // 표준 라이브러리를 사용하지 않음
#![no_main] // 표준 main 함수를 사용하지 않음

use core::panic::PanicInfo;

mod memory;
mod process;
mod uart;
mod fs;

// 부트 시퀀스를 위한 기본 진입점
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 모든 하트(코어)가 여기로 이동하지만, 하나만 계속 진행합니다.
    // 나머지는 어셈블리 코드의 spin 루프에서 대기합니다.
    loop {}
}

// 실제 Rust 코드의 진입점 (어셈블리에서 호출됨)
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // UART 초기화
    uart::init();
    
    println!("Rust OS in 1,000 Lines 부팅 중...");
    
    // 메모리 관리자 초기화
    memory::init_memory(0x8020_0000, 4 * 1024 * 1024); // 4MB 힙 크기
    
    // 프로세스 테이블 초기화
    process::init_process_table();
    
    // 파일 시스템 초기화
    fs::init_filesystem();
    
    println!("OS 초기화 완료!");
    
    // 셸 실행 (TODO)
    shell_loop();
    
    // 여기에 도달하지 않음
    loop {}
}

// 간단한 셸 루프
fn shell_loop() -> ! {
    const MAX_CMD_LEN: usize = 64;
    let mut cmd_buf = [0u8; MAX_CMD_LEN];
    let mut pos = 0;
    
    println!("간단한 셸에 오신 것을 환영합니다!");
    print_prompt();
    
    loop {
        // 키 입력 대기
        if let Some(c) = uart::getchar() {
            match c {
                b'\r' | b'\n' => {
                    println!();
                    
                    // 명령어 처리
                    if pos > 0 {
                        cmd_buf[pos] = 0; // 널 종료자
                        execute_command(&cmd_buf[..pos]);
                        pos = 0;
                    }
                    
                    print_prompt();
                },
                b'\x08' | b'\x7F' => { // 백스페이스 또는 Delete
                    if pos > 0 {
                        pos -= 1;
                        uart::puts("\x08 \x08"); // 백스페이스, 공백, 백스페이스
                    }
                },
                _ if pos < MAX_CMD_LEN - 1 => {
                    cmd_buf[pos] = c;
                    pos += 1;
                    uart::putchar(c); // 에코
                },
                _ => {} // 버퍼가 가득 차면 무시
            }
        }
    }
}

// 프롬프트 출력
fn print_prompt() {
    uart::puts("rust-os> ");
}

// 명령어 실행
fn execute_command(cmd: &[u8]) {
    let cmd_str = core::str::from_utf8(cmd).unwrap_or("");
    
    match cmd_str {
        "help" => {
            println!("사용 가능한 명령어:");
            println!("  help - 이 도움말 표시");
            println!("  echo [텍스트] - 텍스트 출력");
            println!("  reboot - 시스템 재시작");
        },
        cmd if cmd.starts_with("echo ") => {
            let text = &cmd[5..]; // "echo " 다음 부분
            println!("{}", text);
        },
        "reboot" => {
            println!("시스템을 재시작합니다...");
            // TODO: 실제 재시작 구현
        },
        "" => {}, // 빈 명령어
        _ => {
            println!("알 수 없는 명령어: {}", cmd_str);
        }
    }
}

// 패닉 핸들러
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("커널 패닉 발생: {}", info);
    loop {}
}

// println 매크로는 uart 모듈을 사용합니다
#[macro_export]
macro_rules! println {
    () => ($crate::uart::_print(format_args!("\n")));
    ($($arg:tt)*) => ({
        $crate::uart::_print(format_args!($($arg)*));
        $crate::uart::_print(format_args!("\n"));
    });
}

// print 매크로 추가
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::uart::_print(format_args!($($arg)*));
    });
} 
