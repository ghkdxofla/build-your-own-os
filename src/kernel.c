typedef unsigned char uint8_t;
typedef unsigned int uint32_t;
typedef uint32_t size_t;

extern char __bss[], __bss_end[], __stack_top[];

void *memset(void *buf, char c, size_t n) {
    uint8_t *p = (uint8_t *) buf;
    while (n--)
        *p++ = c;
    return buf;
}

void kernel_main(void) {
    memset(__bss, 0, (size_t) __bss_end - (size_t) __bss);

    for (;;);
}

// __attribute__((section(".text.boot"))): 함수를 특정 링커 섹션에 배치
// - 이 함수가 .text.boot 섹션에 위치하도록 지시
// - 부트 코드는 메모리의 특정 위치(0x80200000)에 있어야 함

// __attribute__((naked)): 함수 프롤로그/에필로그 제거
// - 컴파일러가 자동으로 생성하는 스택 프레임 설정 코드를 제거
// - 순수한 어셈블리 코드만 실행되도록 함

// __asm__ __volatile__(): 인라인 어셈블리
// - __asm__: 어셈블리 코드를 C 함수 내에 삽입
// - __volatile__: 컴파일러 최적화 방지 (어셈블리 코드 순서 보장)

__attribute__((section(".text.boot")))
__attribute__((naked))
void boot(void) {
    __asm__ __volatile__(
        "mv sp, %[stack_top]\n" // Set the stack pointer
        "j kernel_main\n"       // Jump to the kernel main function
        :                       // 출력 오퍼랜드 (없음)
        : [stack_top] "r" (__stack_top) // 입력 오퍼랜드: 레지스터에 스택 탑 주소 전달
    );
}
