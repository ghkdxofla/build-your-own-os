use core::fmt;

// QEMU RISC-V UART 레지스터 주소
pub const UART_BASE: usize = 0x1000_0000;
const UART_RBR: usize = UART_BASE + 0x00; // 수신 버퍼 레지스터 (읽기 모드)
const UART_THR: usize = UART_BASE + 0x00; // 송신 홀딩 레지스터 (쓰기 모드)
const UART_IER: usize = UART_BASE + 0x01; // 인터럽트 활성화 레지스터
const UART_FCR: usize = UART_BASE + 0x02; // FIFO 제어 레지스터
const UART_LCR: usize = UART_BASE + 0x03; // 라인 제어 레지스터
const UART_LSR: usize = UART_BASE + 0x05; // 라인 상태 레지스터

// LSR 비트
const LSR_RX_READY: u8 = 1 << 0; // 수신 데이터 준비됨
const LSR_TX_IDLE: u8 = 1 << 5;  // 송신기 유휴 상태

// UART 초기화
pub fn init() {
    unsafe {
        // 인터럽트 비활성화
        write_reg(UART_IER, 0x00);
        
        // FIFO 활성화, FIFO 리셋
        write_reg(UART_FCR, 0x07);
        
        // 8비트 문자, 1 정지 비트, 패리티 없음
        write_reg(UART_LCR, 0x03);
    }
}

// UART 레지스터에서 바이트 읽기
fn read_reg(reg: usize) -> u8 {
    unsafe {
        core::ptr::read_volatile(reg as *const u8)
    }
}

// UART 레지스터에 바이트 쓰기
fn write_reg(reg: usize, val: u8) {
    unsafe {
        core::ptr::write_volatile(reg as *mut u8, val);
    }
}

// 단일 문자 전송
pub fn putchar(c: u8) {
    // 송신기가 준비될 때까지 대기
    while (read_reg(UART_LSR) & LSR_TX_IDLE) == 0 {}
    
    // 문자 전송
    write_reg(UART_THR, c);
}

// 단일 문자 수신
pub fn getchar() -> Option<u8> {
    // 수신 데이터가 있는지 확인
    if (read_reg(UART_LSR) & LSR_RX_READY) != 0 {
        Some(read_reg(UART_RBR))
    } else {
        None
    }
}

// 문자열 출력
pub fn puts(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}

// 포맷된 문자열 출력을 위한 Writer 구현
struct UartWriter;

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

// println! 매크로가 사용하는 함수
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = UartWriter {};
    writer.write_fmt(args).unwrap();
} 
