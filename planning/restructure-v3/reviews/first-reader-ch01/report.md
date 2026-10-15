# Independent first-reader report: released chapter 1

I reached the feed's explicit STOP after reading and saving packets 000–048 in order. Each packet's record was saved before requesting the next. This report concerns my understanding as a basic-Python reader, not technical verification, an author review, or a full-book verdict.

## Exact read scope and isolation

| Released section | Packets |
| --- | --- |
| 1.1 Rust environment/tool roles | 000–004 |
| 1.2 First executable | 005–009 |
| 1.3 Package/crate/workspace/example selection | 010–015 |
| 1.4 Types/numbers/arithmetic | 016–021 |
| 1.5 Immutable/mutable/shadowing | 022–027 |
| 1.6 Functions/expressions/blocks/scope | 028–034 |
| 1.7 Computer foundations/runtime memory | 035–041 |
| 1.8 Tuple/array | 042–048 |

I used only the prescribed feed for prose/code/configuration. I did not inspect chapter files, source files, queue/units, author/planning ledgers, prior reviews, or external references. I did not follow outbound links or execute/install/test tutorial commands. Source shown inside a released packet was read as part of that packet.

I directly viewed the five current-unit required PNGs, each before proceeding: 036 program-process; 037 i32-value-size-location; 038 subtotal-frames; 039 heap-location-content; 040 heap-lifecycle. The records identify actual objects, labels, arrows, time/order meaning, and what the picture cannot establish. I did not inspect other images.

Boundary disclosure: after 009 I expected STOP under the original two-section scope, but the feed returned 010. I recorded the surprise and paused. Root then confirmed 1.3–1.6 had legitimately been released in order and authorized continuation; that clarification is appended in 010 without rewriting the initial concern. Root later explicitly announced 1.7–1.8. There was no confirmed exposure to unreleased content.

## Local understanding I recovered

- The setup/tool sections let me distinguish editable source, executable file, and a running instance, and assign rustup/rustc/Cargo separate jobs. Check/build/run have distinct purposes and expected visible behavior. I cannot report actual installation success because I did not execute anything.
- Workspace/package/example options have a recoverable selection hierarchy. The lock option preserves selected package versions while permitting source changes. I did not read future members merely because their names occur in the workspace list.
- The arithmetic examples support my own small predictions, including 125×3, toward-zero negative division, precedence, unchanged i32 representation size, and index versus element value. I did not infer overflow results or universal decimal exactness.
- Mutable reassignment, new-binding shadowing, and type changes are distinct. I predicted that later ordinary let shadows do not inherit an earlier mut permission. I did not interpret a new binding as proof of a new physical address.
- The function/block sequence supports positional calls, return values, unit, meaningful semicolons, scoped intermediate names, and inner shadowing that leaves the outer binding unchanged. I can trace with_shipping→subtotal without importing reference or ownership rules.
- The memory section's questions are recoverable at its stated conceptual depth: files versus one run, RAM/CPU roles, bit/byte size versus numeric value/location, logical invocation frames, caller result after callee completion, separate heap location/content, and allocation/use/release timing. I kept the logical-frame caveat, did not draw a new frame for each brace pair, and did not infer fixed-size=stack.
- Tuple and array have different positional/type arrangements. I can use the taught destructuring/index/len/update forms and distinguish fixed length from immutable elements. Invalid-index outcome was left unknown until the panic packet actually explained it.

These observations apply to the released units and their shown fixed examples. They do not establish implementation correctness or understanding of unread material.

## First-use hesitation and later clarification, preserved chronologically

1. In 006, compile-time macro processing made display timing briefly uncertain. I recorded runtime display as an inference grounded in main/check/run context. 007 explicitly resolved it: check/build show no Hello; run does. I did not retroactively label macro internals taught.
2. In 025, I knew the last count was f64 but did not know whether four prints as4 or4.0. I preserved that uncertainty. 026 explicitly supplied4 and warned that display appearance does not determine type.
3. In 029, no-tail block behavior was unknown beyond the instruction to omit the tail semicolon. 030 explained unit; 032 demonstrated the extra-semicolon mismatch. The records append that knowledge as it arrives.
4. In 044, I knew index3 was outside a length-three array but did not know the failure mechanism. 047 supplied runtime bounds checking/panic and possible compiler early rejection. The initial unknown remains recorded.
5. In 038, I reread A to avoid imagining first starts at0; prose and the actual diagram explicitly show the initialization has not completed. In 040, I reread T2→T3 to distinguish receiving location information from the inner call already ending.

## Conditional missing steps and bounded inferences

- Setup navigation: opening a terminal is requested without OS-specific navigation. After reopen fails, checking/changing command-search locations is requested without instructions. Other Linux distributions and exact Windows installer choices are delegated outward. These are practical gaps for a reader needing those branches, not observed failures in my environment. The common given command recipe remains understandable at the copy/use level.
- Current-directory inspection is requested without a directory-listing command. A reader may rely on visible folder context, but a general terminal-only diagnosis is not taught. This matters if they become lost; the explicit full command paths themselves are recoverable.
- Some local source paths abbreviate the repo-root base established in 010. In 013 I initially hesitated at hello/examples/two_lines.rs, then inferred the full path from the previously shown member location. The full runnable command avoids blocking target selection. Later shorthand uses the same inferred convention.
- curl's individual flags, exact Unicode-scalar validity, binary integer encoding, resolver algorithms, actual memory addresses/instructions, allocator internals, and concrete heap/ownership syntax are not known from this reading. Current packets either do not require them for the supplied operation or explicitly defer them. I have not silently counted these as explained.

I have no separate optional style requests. The items above distinguish practical missing setup help from local inference and explicitly deferred internals.

## Section-end promise tracking

Each section-end record states its recovered question and open promises. By 048, the opening promises of a numeric program, reusable calculation functions, and a conceptual text-to-executable/runtime-memory account have concrete released examples. Runtime memory includes five inspected diagrams, not only alt prose. Future tests/conditional handling in chapter2, formal data fields/ownership in chapter3, later text/multi-item heap operations, and growable lists in chapter5 remain explicitly future. I have not read those chapters or used them to repair earlier understanding.

All initial records remain intact, with later clarifications recorded separately. Current state: STOP reached; waiting for root follow-up.
