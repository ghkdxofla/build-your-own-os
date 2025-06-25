use core::ptr;
use core::panic::PanicInfo;

// C의 extern char __bss[], __bss_end[], __stack_top[]를 대체
// Rust에서는 extern "C" 블록으로 링커 심볼을 선언
extern "C" {
    // BSS 섹션의 시작 주소 (초기화되지 않은 전역 변수들이 위치)
    static __bss: u8;
    // BSS 섹션의 끝 주소
    static __bss_end: u8;
    // 스택의 최상위 주소
    static __stack_top: u8;
}

fn memset(buf: *mut u8, c: u8, n: usize) {
    unsafe {
        ptr::write_bytes(buf, c, n);
    }
}

fn kernel_main() {
    // C 코드의 memset(__bss, 0, (size_t) __bss_end - (size_t) __bss); 구현
    unsafe {
        // &__bss as *const u8 as *mut u8: BSS 시작 주소를 가변 포인터로 변환
        // &__bss_end as *const u8 as usize: BSS 끝 주소를 정수로 변환
        // 두 주소의 차이로 BSS 섹션 크기 계산
        let bss_start = &__bss as *const u8 as *mut u8;
        let bss_end = &__bss_end as *const u8 as usize;
        let bss_size = bss_end - (bss_start as usize);
        
        // BSS 섹션을 0으로 초기화 (C의 memset과 동일한 동작)
        memset(bss_start, 0, bss_size);
    }
    
    // C의 for(;;); 와 동일한 무한 루프
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// C의 __attribute__들을 Rust 속성으로 대체
#[link_section = ".text.boot"]  // C의 __attribute__((section(".text.boot"))) 대체
#[no_mangle]                    // 함수 이름 맹글링 방지 (링커에서 찾을 수 있도록)
pub unsafe extern "C" fn boot() -> ! {
    // C의 __asm__ __volatile__ 대체: core::arch::asm! 매크로 사용
    core::arch::asm!(
        "mv sp, {stack_top}",   // 스택 포인터 설정
        "j {kernel_main}",      // kernel_main으로 점프
        stack_top = in(reg) &__stack_top,  // 입력: 스택 탑 주소를 레지스터에
        kernel_main = sym kernel_main,     // 심볼: kernel_main 함수 주소
        options(noreturn)       // 이 함수는 리턴하지 않음을 명시
    )
}

