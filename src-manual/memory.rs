// 메모리 관련 상수
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_ORDER: usize = 12; // 2^12 = 4096

// 물리적 메모리 관리를 위한 구조체
pub struct MemoryManager {
    // 메모리 관리 상태를 저장하는 필드들
    heap_start: usize,
    heap_end: usize,
    next_free: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            heap_start: 0,
            heap_end: 0,
            next_free: 0,
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        self.next_free = start;
    }

    // 간단한 메모리 할당자 (나중에 버디 할당자로 확장 가능)
    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        // 간단한 범프 할당자로 시작
        let aligned_size = align_up(size, 8); // 8바이트 정렬
        
        if self.next_free + aligned_size > self.heap_end {
            return None; // 메모리 부족
        }
        
        let addr = self.next_free;
        self.next_free += aligned_size;
        
        Some(addr)
    }

    pub fn free(&mut self, _addr: usize) {
        // 간단한 범프 할당자에서는 아무것도 하지 않음
        // 실제 구현은 나중에
    }
}

// 값을 정렬에 맞게 반올림합니다
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

// 전역 메모리 관리자 인스턴스
static mut MEMORY_MANAGER: Option<MemoryManager> = None;

// 전역 메모리 관리자 초기화
pub fn init_memory(heap_start: usize, heap_size: usize) {
    unsafe {
        MEMORY_MANAGER = Some(MemoryManager::new());
        MEMORY_MANAGER.as_mut().unwrap().init(heap_start, heap_size);
    }
}

// kmalloc 함수 (C의 malloc과 유사)
pub fn kmalloc(size: usize) -> Option<*mut u8> {
    unsafe {
        MEMORY_MANAGER
            .as_mut()
            .and_then(|mm| mm.alloc(size).map(|addr| addr as *mut u8))
    }
}

// kfree 함수 (C의 free와 유사)
pub fn kfree(ptr: *mut u8) {
    unsafe {
        if let Some(mm) = MEMORY_MANAGER.as_mut() {
            mm.free(ptr as usize);
        }
    }
} 
