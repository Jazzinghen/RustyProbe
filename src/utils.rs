pub const JUMP_OP_CODES: [&str; 8] = ["jne", "jeq", "jlo", "jhs", "jn ", "jge", "jl ", "jmp"];
pub const ONE_BYTE_CODES: [&str; 7] = ["rrc", "swpb", "rra", "sxt", "push", "call", "reti"];
pub const TWO_BYTES_CODES: [&str; 16] = [
    "!!!", "!!!", "!!!", "!!!", "mov", "add", "addc", "subc", "sub", "cmp", "dadd", "bit", "bic",
    "bis", "xor", "and",
];

pub const REGISTERS: [&str; 16] = [
    "pc", "sp", "sr", "cg", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "r13", "r14",
    "r15",
];
