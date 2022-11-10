# rustc ICE for unused todo var

To reproduce simply run `cargo c`. Tried on stable 1.64.0, 1.65.0 and nightly 1.66.0.

```
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:810:36: cannot convert `ReErased` to a region vid

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/compiler/rustc_errors/src/lib.rs:1392:9
stack backtrace:
   0:     0x7f01cc56d19d - std::backtrace_rs::backtrace::libunwind::trace::h9135f25bc195152c
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f01cc56d19d - std::backtrace_rs::backtrace::trace_unsynchronized::h015ee85be510df51
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f01cc56d19d - std::sys_common::backtrace::_print_fmt::h5fad03caa9652a2c
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f01cc56d19d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2b42ca28d244e5c7
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f01cc5c86ac - core::fmt::write::h401e827d053130ed
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f01cc55e4e1 - std::io::Write::write_fmt::hffec93268f5cde32
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/io/mod.rs:1672:15
   6:     0x7f01cc56feb5 - std::sys_common::backtrace::_print::h180c4c706ee1d3fb
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f01cc56feb5 - std::sys_common::backtrace::print::hd0c35d18765761c9
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f01cc56feb5 - std::panicking::default_hook::{{closure}}::h1f023310983bc730
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/panicking.rs:295:22
   9:     0x7f01cc56fbd1 - std::panicking::default_hook::h188fec3334afd5be
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/panicking.rs:314:9
  10:     0x7f01cee45484 - rustc_driver[4568cc0a685fd94d]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f01cc5705ea - std::panicking::rust_panic_with_hook::hf26e9d4f97b40096
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/panicking.rs:702:17
  12:     0x7f01cfded641 - std[306a94a967d8f5ee]::panicking::begin_panic::<rustc_errors[5fcb1a1f56a762e6]::ExplicitBug>::{closure#0}
  13:     0x7f01cfded516 - std[306a94a967d8f5ee]::sys_common::backtrace::__rust_end_short_backtrace::<std[306a94a967d8f5ee]::panicking::begin_panic<rustc_errors[5fcb1a1f56a762e6]::ExplicitBug>::{closure#0}, !>
  14:     0x7f01cfe3fee6 - std[306a94a967d8f5ee]::panicking::begin_panic::<rustc_errors[5fcb1a1f56a762e6]::ExplicitBug>
  15:     0x7f01cfdd3e66 - std[306a94a967d8f5ee]::panic::panic_any::<rustc_errors[5fcb1a1f56a762e6]::ExplicitBug>
  16:     0x7f01cfdd2c95 - <rustc_errors[5fcb1a1f56a762e6]::HandlerInner>::bug::<&alloc[6d6f83537459af52]::string::String>
  17:     0x7f01cfdd29f0 - <rustc_errors[5fcb1a1f56a762e6]::Handler>::bug::<&alloc[6d6f83537459af52]::string::String>
  18:     0x7f01cfecfc6d - rustc_middle[b6cf56a787d1e2a1]::ty::context::tls::with_opt::<rustc_middle[b6cf56a787d1e2a1]::util::bug::opt_span_bug_fmt<rustc_span[721a57c036170ce]::span_encoding::Span>::{closure#0}, ()>
  19:     0x7f01cfecfce6 - rustc_middle[b6cf56a787d1e2a1]::util::bug::opt_span_bug_fmt::<rustc_span[721a57c036170ce]::span_encoding::Span>
  20:     0x7f01cd89aea3 - rustc_middle[b6cf56a787d1e2a1]::util::bug::bug_fmt
  21:     0x7f01cdc58c49 - <rustc_borrowck[9bf94ea6123d60bb]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  22:     0x7f01cdc29a8a - <rustc_borrowck[9bf94ea6123d60bb]::type_check::TypeChecker>::normalize_and_prove_instantiated_predicates
  23:     0x7f01cdc26156 - <rustc_borrowck[9bf94ea6123d60bb]::type_check::TypeVerifier as rustc_middle[b6cf56a787d1e2a1]::mir::visit::Visitor>::visit_constant
  24:     0x7f01cdc10769 - <rustc_borrowck[9bf94ea6123d60bb]::type_check::TypeVerifier as rustc_middle[b6cf56a787d1e2a1]::mir::visit::Visitor>::visit_body
  25:     0x7f01cdbb5c32 - rustc_borrowck[9bf94ea6123d60bb]::type_check::type_check
  26:     0x7f01cdb922ce - rustc_borrowck[9bf94ea6123d60bb]::nll::compute_regions
  27:     0x7f01cdb7cdaf - rustc_borrowck[9bf94ea6123d60bb]::do_mir_borrowck
  28:     0x7f01ce82794d - rustc_borrowck[9bf94ea6123d60bb]::mir_borrowck
  29:     0x7f01ce826e60 - <rustc_borrowck[9bf94ea6123d60bb]::provide::{closure#0} as core[6fcc70bcc91a5bf5]::ops::function::FnOnce<(rustc_middle[b6cf56a787d1e2a1]::ty::context::TyCtxt, rustc_span[721a57c036170ce]::def_id::LocalDefId)>>::call_once
  30:     0x7f01ce13a39f - <rustc_query_system[8caf4755e287670e]::dep_graph::graph::DepGraph<rustc_middle[b6cf56a787d1e2a1]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b6cf56a787d1e2a1]::ty::context::TyCtxt, rustc_span[721a57c036170ce]::def_id::LocalDefId, &rustc_middle[b6cf56a787d1e2a1]::mir::query::BorrowCheckResult>
  31:     0x7f01ce13967d - rustc_query_system[8caf4755e287670e]::query::plumbing::try_execute_query::<rustc_query_impl[232efbc2900411f9]::plumbing::QueryCtxt, rustc_query_system[8caf4755e287670e]::query::caches::DefaultCache<rustc_span[721a57c036170ce]::def_id::LocalDefId, &rustc_middle[b6cf56a787d1e2a1]::mir::query::BorrowCheckResult>>
  32:     0x7f01ced4356a - <rustc_query_impl[232efbc2900411f9]::Queries as rustc_middle[b6cf56a787d1e2a1]::ty::query::QueryEngine>::mir_borrowck
  33:     0x7f01ce8304d3 - rustc_data_structures[49e7163a34b889d1]::sync::par_for_each_in::<&[rustc_span[721a57c036170ce]::def_id::LocalDefId], <rustc_middle[b6cf56a787d1e2a1]::hir::map::Map>::par_body_owners<rustc_interface[f934826ba4c12dd9]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  34:     0x7f01ce82fc8b - <rustc_middle[b6cf56a787d1e2a1]::hir::map::Map>::par_body_owners::<rustc_interface[f934826ba4c12dd9]::passes::analysis::{closure#2}::{closure#0}>
  35:     0x7f01ce82fb63 - <rustc_session[98f7faf1d37bb5fb]::session::Session>::time::<(), rustc_interface[f934826ba4c12dd9]::passes::analysis::{closure#2}>
  36:     0x7f01ce82f955 - rustc_interface[f934826ba4c12dd9]::passes::analysis
  37:     0x7f01cebea2bc - <rustc_query_system[8caf4755e287670e]::dep_graph::graph::DepGraph<rustc_middle[b6cf56a787d1e2a1]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b6cf56a787d1e2a1]::ty::context::TyCtxt, (), core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>
  38:     0x7f01cebe9aab - rustc_query_system[8caf4755e287670e]::query::plumbing::try_execute_query::<rustc_query_impl[232efbc2900411f9]::plumbing::QueryCtxt, rustc_query_system[8caf4755e287670e]::query::caches::DefaultCache<(), core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>>
  39:     0x7f01cebe95fe - rustc_query_system[8caf4755e287670e]::query::plumbing::get_query::<rustc_query_impl[232efbc2900411f9]::queries::analysis, rustc_query_impl[232efbc2900411f9]::plumbing::QueryCtxt>
  40:     0x7f01ce5c3d87 - <rustc_interface[f934826ba4c12dd9]::passes::QueryContext>::enter::<rustc_driver[4568cc0a685fd94d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>
  41:     0x7f01ce5bc97f - <rustc_interface[f934826ba4c12dd9]::interface::Compiler>::enter::<rustc_driver[4568cc0a685fd94d]::run_compiler::{closure#1}::{closure#2}, core[6fcc70bcc91a5bf5]::result::Result<core[6fcc70bcc91a5bf5]::option::Option<rustc_interface[f934826ba4c12dd9]::queries::Linker>, rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>
  42:     0x7f01ce5b186a - rustc_span[721a57c036170ce]::with_source_map::<core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>, rustc_interface[f934826ba4c12dd9]::interface::create_compiler_and_run<core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>, rustc_driver[4568cc0a685fd94d]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7f01ce5b11a2 - <scoped_tls[80743de900a7f844]::ScopedKey<rustc_span[721a57c036170ce]::SessionGlobals>>::set::<rustc_interface[f934826ba4c12dd9]::interface::run_compiler<core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>, rustc_driver[4568cc0a685fd94d]::run_compiler::{closure#1}>::{closure#0}, core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>
  44:     0x7f01ce5af5cf - std[306a94a967d8f5ee]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f934826ba4c12dd9]::util::run_in_thread_pool_with_globals<rustc_interface[f934826ba4c12dd9]::interface::run_compiler<core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>, rustc_driver[4568cc0a685fd94d]::run_compiler::{closure#1}>::{closure#0}, core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>::{closure#0}, core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>
  45:     0x7f01cec690b9 - <<std[306a94a967d8f5ee]::thread::Builder>::spawn_unchecked_<rustc_interface[f934826ba4c12dd9]::util::run_in_thread_pool_with_globals<rustc_interface[f934826ba4c12dd9]::interface::run_compiler<core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>, rustc_driver[4568cc0a685fd94d]::run_compiler::{closure#1}>::{closure#0}, core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>::{closure#0}, core[6fcc70bcc91a5bf5]::result::Result<(), rustc_errors[5fcb1a1f56a762e6]::ErrorGuaranteed>>::{closure#1} as core[6fcc70bcc91a5bf5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7f01cc57a723 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h56d5fc072706762b
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/alloc/src/boxed.rs:1935:9
  47:     0x7f01cc57a723 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h41deef8e33b824bb
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/alloc/src/boxed.rs:1935:9
  48:     0x7f01cc57a723 - std::sys::unix::thread::Thread::new::thread_start::ha6436304a1170bba
                               at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7f01cc31c8fd - <unknown>
  50:     0x7f01cc39ea60 - <unknown>
  51:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: rustc 1.64.0 (a55dd71d5 2022-09-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `new`
#1 [analysis] running analysis passes on this crate
end of query stack
```
