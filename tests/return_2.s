	.file	"return_2.c"
	.def	___main;	.scl	2;	.type	32;	.endef
	.section	.text.unlikely,"x"
LCOLDB0:
	.section	.text.startup,"x"
LHOTB0:
	.p2align 4,,15
	.globl	_main
	.def	_main;	.scl	2;	.type	32;	.endef
_main:
	pushl	%ebp
	movl	%esp, %ebp
	andl	$-16, %esp
	call	___main
	movl	$2, %eax
	leave
	ret
	.section	.text.unlikely,"x"
LCOLDE0:
	.section	.text.startup,"x"
LHOTE0:
	.ident	"GCC: (GNU) 4.9.3"
