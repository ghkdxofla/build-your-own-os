use core::sync::atomic::{AtomicUsize, Ordering};

// 프로세스 ID 카운터
static NEXT_PID: AtomicUsize = AtomicUsize::new(1);

// 간단한 프로세스 상태 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Zombie,
}

// 프로세스 컨텍스트 구조체
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    // RISC-V 레지스터 상태
    pub ra: usize,   // 반환 주소
    pub sp: usize,   // 스택 포인터
    pub gp: usize,   // 전역 포인터
    pub tp: usize,   // 스레드 포인터
    pub t0: usize,   // 임시 레지스터
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,   // 저장 레지스터
    pub s1: usize,
    pub a0: usize,   // 인자 레지스터
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,   // 저장 레지스터
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,   // 임시 레지스터
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

impl Context {
    pub fn new() -> Self {
        Context {
            ra: 0, sp: 0, gp: 0, tp: 0,
            t0: 0, t1: 0, t2: 0,
            s0: 0, s1: 0,
            a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0,
            s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0, s9: 0, s10: 0, s11: 0,
            t3: 0, t4: 0, t5: 0, t6: 0,
        }
    }
}

// 간단한 프로세스 번들
#[derive(Clone, Copy)]
pub struct ProcessBundle {
    pub process: Process,
    pub active: bool,
}

// 프로세스 구조체
#[derive(Clone, Copy)]
pub struct Process {
    pub pid: usize,
    pub state: ProcessState,
    pub context: Context,
    pub stack_ptr: usize,  // 스택 포인터
    pub stack_size: usize, // 스택 크기
    pub page_table: usize, // 페이지 테이블 주소 (나중에 개선)
}

impl Process {
    pub fn new() -> Self {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
        Process {
            pid,
            state: ProcessState::Ready,
            context: Context::new(),
            stack_ptr: 0,
            stack_size: 0,
            page_table: 0,
        }
    }
}

// 간단한 프로세스 테이블
pub const MAX_PROCESSES: usize = 64;

// 프로세스 테이블 초기값 생성을 위한 매크로
macro_rules! create_empty_process_bundle {
    () => {
        ProcessBundle {
            process: Process {
                pid: 0,
                state: ProcessState::Ready,
                context: Context {
                    ra: 0, sp: 0, gp: 0, tp: 0,
                    t0: 0, t1: 0, t2: 0,
                    s0: 0, s1: 0,
                    a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0,
                    s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0, s9: 0, s10: 0, s11: 0,
                    t3: 0, t4: 0, t5: 0, t6: 0,
                },
                stack_ptr: 0,
                stack_size: 0,
                page_table: 0,
            },
            active: false,
        }
    };
}

// 고정 크기 배열 사용
static mut PROCESSES: [ProcessBundle; MAX_PROCESSES] = [create_empty_process_bundle!(); MAX_PROCESSES];

// 현재 실행 중인 프로세스의 인덱스
static mut CURRENT_PROCESS: usize = 0;

// 프로세스 테이블 초기화
pub fn init_process_table() {
    unsafe {
        // 커널 프로세스 (PID 0) 생성
        PROCESSES[0].process = Process::new();
        PROCESSES[0].active = true;
        CURRENT_PROCESS = 0;
    }
}

// 다음 프로세스로 컨텍스트 스위칭 (나중에 구현)
pub fn schedule() -> *mut Context {
    // 간단한 라운드 로빈 스케줄러
    unsafe {
        let current = CURRENT_PROCESS;
        let mut next = (current + 1) % MAX_PROCESSES;
        
        // 다음 Ready 상태의 프로세스 찾기
        while next != current {
            if PROCESSES[next].active && PROCESSES[next].process.state == ProcessState::Ready {
                break;
            }
            next = (next + 1) % MAX_PROCESSES;
        }
        
        if next != current && PROCESSES[next].active {
            CURRENT_PROCESS = next;
        }
        
        // 현재 프로세스의 컨텍스트 반환
        if PROCESSES[CURRENT_PROCESS].active {
            PROCESSES[CURRENT_PROCESS].process.state = ProcessState::Running;
            &mut PROCESSES[CURRENT_PROCESS].process.context as *mut Context
        } else {
            // 실행 가능한 프로세스가 없을 경우
            // 첫 번째 프로세스를 생성하고 반환
            PROCESSES[0].process = Process::new();
            PROCESSES[0].active = true;
            CURRENT_PROCESS = 0;
            PROCESSES[0].process.state = ProcessState::Running;
            &mut PROCESSES[0].process.context as *mut Context
        }
    }
} 
