# MELTDOWN EXPLOIT POC

Otimizações especulativas executam código de maneira não segura deixando dados
traços em microarquitetura, como cache.

Só pode despejar `linux_proc_banner` no momento, já que requer memória acessada
estar em cache e `linux_proc_banner` é armazenado em cache em cada leitura de
`/proc/versão`. Pode funcionar com `prefetch`. Funciona com `sched_yield`.

Compile o código com `rustc`