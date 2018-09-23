#define	ENTRY(sym)						\
	.text; .globl sym; .align 2; .type sym,#function; sym:
#define	EENTRY(sym)						\
	.globl	sym; sym:
#define	END(sym) .size sym, . - sym
#define	EEND(sym)