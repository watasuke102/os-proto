int add(int lhs, int rhs) { return lhs + rhs; }

void _start(void) {
  int ret = add(3, 7);
  asm volatile(
      ".intel_syntax noprefix\n"
      "pop  r10\n"
      "mov  rdi, rax\n"
      "ret"
      : "=a"(ret));
}
