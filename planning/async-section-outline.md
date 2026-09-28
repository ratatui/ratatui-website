# Async documentation section outline

## Conceptual restructure

The current implementation uses a flat async section: overview, event loops, bridging, cooperative
scheduling, redraws, blocking work, tasks, messages, backpressure, overlapping work, cancellation,
terminal I/O, shutdown, and handoffs. Each page explains its mechanism locally and links to Tokio
for depth. Recipes own runnable setup, HTTP adaptation, editor handoff, and troubleshooting. The
earlier outline below records the investigation and previous grouping, not the current sidebar.
Speculative APIs are preserved in [async-terminal-design.md](async-terminal-design.md).

## Purpose

Provide a useful, trustworthy body of information about async and concurrency for people reading
ratatui.rs. It should help newcomers understand the concepts, application developers put them to
work, experienced readers evaluate tradeoffs, and contributors reason about terminal and library
design. Readers should be able to learn, look something up, investigate a problem, or explore a
subject in depth without having to follow a single course.

The durable rationale is that these readers need a coherent place to understand how background work,
application state, input, rendering, and terminal access fit together. Support practical choices
with explanations of their mechanisms, limits, and alternatives. Avoid assuming everyone arrives to
build their first app or to repair an async failure.

A concrete application flow connects much of the material:

**Input → start work → receive a result → update application state → request a frame → draw.**

Use that flow as a teaching aid, not a framework that every page must follow. Some readers need a
worked example; others need a precise API constraint, a comparison, a diagnosis, or design context.
Give those needs clear entry points and connect them with useful links.

The investigation into async input and synchronous terminal operations prompted this work. Its
findings supply important constraints, evidence, and failure lessons. They do not determine the
whole curriculum or require every reader to begin with a terminal-internals study. Organize the
section around readers' questions, drawing on successful application patterns as well as failures.

This is an editorial outline, not a finished guide or a claim that proposed APIs exist.

## Readers, entry points, and outcomes

The landing page should offer several ways into the material. Experience levels overlap: an
experienced Rust developer may be new to terminal protocols, and a new Ratatui user may already know
Tokio well. Route by the reader's question rather than labeling entire pages beginner or expert.

- Understand what async means for a TUI: Overview and vocabulary; An accessible mental model
- Build or extend an application: Event loop and pattern pages; Annotated examples and choices
- Evaluate architecture or performance: Scheduling and coordination; Mechanisms and tradeoffs
- Look up a specific constraint: Topic headings and API links; Precise, scoped explanations
- Diagnose unexpected behavior: Troubleshooting; Symptoms, causes, and evidence
- Improve the libraries: Terminal I/O and design discussion; Contracts and open questions

Explain unfamiliar concepts where they first matter, including tasks versus threads, wakeups,
backpressure, cancellation, and terminal queries. Put prerequisites on the relevant example, not on
the section as a whole. Link to basic Rust, Ratatui, and Tokio material when needed without
requiring readers to complete a general async textbook before understanding a local explanation.

Across the section, readers should be able to:

- Explain what async changes in a TUI and which operations remain synchronous.
- Follow how input, background results, application state, and frames relate to one another.
- Build from an example and understand why its important pieces are present.
- Compare event-loop arrangements and communication patterns, including their costs and assumptions.
- Understand ordering, bounded work, stale results, cancellation, and task lifetime.
- Find accurate guidance on terminal queries, redirected handles, handoffs, and cleanup.
- Diagnose problems and distinguish an application policy from an API or platform constraint.
- Explore design alternatives and open questions without mistaking proposals for available features.

Provide a suggested learning path, while making every substantial page useful when entered directly.
Each page should establish its subject and assumptions, link prerequisite concepts, and offer a
route to deeper explanation. A short overview must not replace the detailed material that an
experienced reader came to find.

## Evidence and earlier decisions

The starting investigation is
[Investigate async ratatui history](codex://threads/019f2d3a-e9a2-7631-a8c0-7c0372ad042e). It began
with recurring confusion about the async counter tutorial: adding `#[tokio::main]` did not explain
coordination, while the older component template was too large to explain a small counter. The
investigation was also motivated by edge cases where synchronous draw/backend operations interact
with async input. Preserve that history as context for the research, while using the broader reader
needs above to decide the section's ongoing scope and organization.

The investigation subsequently collected terminal failure reports, Tokio and Crossterm contracts,
Ratatui call paths, and application patterns. Its explicit editorial requirements remain useful:

- Explain blocking reads and writes, and how an async input adapter wakes its waiting task.
- Separate terminal ownership from the choice of worker coordination mechanism.
- Explain benefits and tradeoffs for every application shape; do not promote one as universally
  best.
- Preserve concrete failures, with their mechanisms and repair boundaries.
- Present useful ideas from applications before explaining what needs adaptation.
- Keep primary API and protocol citations separate from optional background reading.
- Retain short reminders for readers arriving mid-section, using consistent terminology.
- Link API names in prose to their contracts or relevant implementation.
- Keep prototype ideas separate from publicly supported behavior and available APIs.

The later [Rewrite async docs guidance](codex://threads/01a072f4-9a8b-74b0-a13c-45469e81f808)
reduced repetition and corrected source claims. Preserve those improvements rather than restoring
older prose wholesale. Earlier task conclusions are research leads, not a substitute for checking
the pinned code and the versions used by the examples.

## Technical grounding and coordination

Async terminal applications can be difficult to get right. Blocking and non-blocking operations
share state, handles, readers, and lifecycle transitions; an async interface does not make those
interactions independent. The section should acknowledge this complexity and explain the mechanisms
needed to manage it. Accessible prose must not make an incomplete design appear reliable.

Ground recommendations in verifiable behavior of Crossterm, Ratatui, Tokio, and any other library
involved. For each consequential claim or example:

- Identify the operation, its owner, and where it runs. Distinguish an async API from its underlying
  blocking or readiness-based implementation.
- Check the relevant API contract and implementation, including platform, version, backend, and
  feature differences. Mark implementation observations as such rather than promising a contract.
- Trace interactions end to end: a draw may call the backend; a backend operation may issue a query;
  that query may require a reader already occupied by another operation.
- Explain what enforces the ordering: ownership, a channel, a lock, a state transition, or an
  acknowledgement. Being in one module, using an async function, or spawning a task is not evidence
  that the operations are coordinated.
- Check what remains active when a future is dropped, a timeout expires, or shutdown is requested.
  Distinguish requesting cancellation, stopping work, releasing resources, and observing completion.
- Follow failure and interruption paths as well as success: late replies, full queues, closed
  channels, partial I/O, input bursts, resize, failed child startup, and terminal reacquisition.
- State a pattern's assumptions next to its recommendation or example. If a required guarantee is
  unavailable or unverified, explain the limitation and the supported alternatives.

Use evidence appropriate to the claim. API documentation establishes supported contracts; pinned
source explains a particular implementation; a focused test demonstrates the conditions it actually
exercises; an issue report establishes an observed failure. Compilation establishes type
correctness, not responsiveness, freedom from races, or reliable terminal handoff. Passing tests on
one terminal or platform does not establish portability to others.

Each substantial pattern should answer: what does it accomplish, how is it coordinated, under what
conditions does it work, and what can go wrong when those conditions change? Include a concrete
failure sequence where it makes a hidden dependency understandable. Avoid a separate pile of
warnings that readers must mentally combine with an apparently unconditional example.

Where the research cannot establish a safe general recipe, say precisely what is known and what
remains unresolved. Do not invent a reassuring abstraction, a timeout wrapper, or a placeholder
method that hides the missing guarantee. The goal is useful, justified guidance that helps readers
reason about the complexity, not a promise that async makes terminal integration straightforward.

## Proposed navigation and page boundaries

Create **Async applications** as a dedicated group under Concepts, alongside Application Patterns.
Use `src/content/docs/concepts/async/` for the section. Link to it from Application Patterns and
relevant tutorials. Async coordination composes with Elm, components, and other state architectures;
it is not an alternative to them.

Recommended entries, in a suggested learning order rather than a required sequence:

1. **Overview** — `index.md`
1. **Build a responsive event loop** — `event-loops.md`
1. **Schedule work and redraws** — `scheduling.md`
1. **Manage background operations** — `background-work.md`
1. **Coordinate terminal I/O** — `terminal-io.md`
1. **Shut down and hand off the terminal** — `lifecycle.md`
1. **Troubleshoot async applications** — `troubleshooting.md`

Provide **Terminal library design questions** as an optional, clearly labeled discussion. It is
useful to experienced readers and contributors and should be discoverable from the section landing
page and relevant topics. Its final location can be within this section or the Developer Guide;
choose by the site's navigation conventions. Keep proposals distinct from usable patterns without
making the deeper material hard to find.

This expands the earlier four-page practical proposal where the subjects warrant independent depth:
terminal query mechanics and lifecycle procedures have different reader questions, while a symptom
index needs to be reachable without reading either in full. Do not create a page per channel type,
application, or individual pitfall.

The current main page has approximately 3,600 prose words and 269 included code lines. Its companion
survey has about 1,275 words, and the design discussion about 1,000. The problem is uneven
allocation: most teaching is concentrated on one page, while substantial source research sits apart
from it.

Use roughly 400–700 words for the overview and 800–1,400 for a substantial concept page as editorial
checks, not quotas. A diagnostic index can be shorter. Assess expanded code, diagrams, mobile
scrolling, and the number of new concepts as well as prose length. Split when the reader's question
changes; do not pad pages to meet a target or cut a necessary explanation to satisfy one.

## Page outlines

### Overview

**Reader question:** What is async in a Ratatui application, and where can I find what I need?

- Introduce async through familiar application behavior: accepting input while fetching data,
  showing progress, and keeping the current view correct as requests finish. Explain when async
  helps and when a synchronous loop is enough.
- Explain the input/work/state/frame flow with one small diagram.
- Establish essential constraints without a source dive: async does not make all work concurrent,
  drawing remains synchronous, and input/output access needs coordination. Link the detailed
  explanations from the choices they affect.
- Define task, thread, event loop, terminal owner, and worker briefly enough to read the next page.
- Give a compact decision table for a single async UI task, synchronous owner with async workers,
  and a dedicated terminal thread. Separate those choices from channels, shared state, and actors.
- Offer a short set of linked rules: keep handlers short, give results a wakeup path, coordinate
  terminal access, bound work, reject obsolete replies, and plan cleanup.
- Offer routes to the worked event loop, topic explanations, troubleshooting, API references, and
  design discussion. Make both the introductory path and the depth of the reference material
  visible.

**How it helps:** Readers can orient themselves, learn the basic model, and find the explanation or
reference they need. State that a synchronous application remains appropriate when its work finishes
promptly.

### Build a responsive event loop

**Reader question:** How do input and a background result both reach my UI?

- Start with a minimal search/fetch app that remains editable while a request waits.
- Trace the request from input handling to a worker and back as a typed message.
- Explain who mutates state and who draws; a worker does not print into the active terminal.
- State the example's supported conditions and trace how blocking draw and non-blocking event waits
  cooperate. Identify which additions, such as runtime queries or child handoff, need more wiring.
- Introduce `select!` as waiting for a ready source. Its handlers share a task; awaiting a long
  request inside a selected handler prevents the loop from returning to its other sources.
- Show input, result, and draw-deadline branches. Explain input failure, channel closure, initial
  rendering, and why worker completion must wake the UI independently of keyboard input.
- Explain what merely adding `#[tokio::main]` or awaiting `EventStream` does and does not change.
- Compare the same flow with a synchronous terminal owner using `Runtime::spawn` or `Handle::spawn`.
  Explain its polling/wakeup cost and why a current-thread runtime needs active driving.
- Describe the dedicated-thread variant and when its extra lifecycle wiring is justified.

**Example:** One runnable, annotated app with loading, success, failure, and quit behavior. Show the
critical loop inline and link the whole source; keep alternatives focused on the changed boundary.

**How it helps:** Readers can implement the smallest useful async app and explain every moving part.
Templates are supporting examples, not prerequisites or complete reference architectures.

### Schedule work and redraws

**Reader question:** Why does the application lag even though its requests are async?

- Distinguish time waiting for I/O, executing handlers, preparing/rendering widgets, and writing
  output.
- Explain where the top-level `#[tokio::main]` future runs versus spawned tasks and a current-thread
  runtime. Show which other work can still progress during a synchronous call.
- Use finite blocking jobs, CPU pools, or a dedicated thread according to the work's lifetime.
  Explain concurrency limits and the limitations of `spawn_blocking` and `block_in_place`.
- Derive batching from a burst of updates: apply useful state changes before drawing once.
- Bound each event source separately; a count budget does not bound an expensive handler's duration.
- Separate dirty state, a draw deadline, and animation/tick policy. Avoid catch-up redraw bursts.
- Explain which updates can be coalesced and which must retain order: progress versus commands,
  resize requests versus text edits, snapshots versus log records.
- Explain the scale of cooperative scheduling. Alice Ryhl's [blocking article][blocking-guidance]
  suggests 10–100 microseconds between awaits as an application-dependent rule of thumb. Attribute
  it accurately; it is not a Tokio-enforced deadline or a universal limit on a terminal frame.
- Explain that `.await` is a possible yield point, not proof of yielding: an immediately ready
  future can continue in the same poll. Tokio's [cooperative scheduling discussion][cooperative]
  explains why busy async operations can still monopolize execution.
- Apply that guidance to a complete uninterrupted stretch of work, including handlers, widget
  rendering, size checks, and writes/flushes. A 16 ms frame interval is not permission to occupy a
  runtime worker for 16 ms; a yield after drawing cannot undo the time already spent blocking.
- Locate the affected execution context before recommending a remedy: current-thread runtime,
  spawned worker task, top-level `block_on` future, or dedicated synchronous owner. Background tasks
  progressing elsewhere does not mean the UI task can process input during its own draw.
- Measure release builds and separate computation from slow terminal output before choosing a fix.
  Include slow paths and latency variation, not just average frame time. Explain the tradeoff when
  accepting synchronous work in the UI task, and preserve terminal ownership if moving that work. Do
  not prescribe a separate `spawn_blocking` call for every terminal operation.

**Example:** Extend the same app with bursty progress or costly result preparation. Show the event
batch and next-frame calculation, with comments explaining policy constants and fairness limits.

**Evidence:** Yazi batching, Codex redraw/reflow work, Helix redraw coordination, Tokio scheduling.

**How it helps:** Readers can locate latency and choose the appropriate remedy instead of adding
more spawned tasks or raising the frame rate indiscriminately.

### Manage background operations

**Reader question:** How do multiple requests stay correct when completion order and load vary?

- Choose communication by semantics: ordered messages, latest state, one reply, or a shared
  snapshot.
- Explain bounded versus unbounded queues, waiting senders, payload size, and channel-cycle
  deadlocks.
- Distinguish queue capacity, concurrency limits, debouncing, and coalescing; each controls
  something different. A bounded mailbox does not bound the number of waiting producer tasks.
- Walk through two searches finishing in reverse order. Guard failures as well as successes, and
  invalidate results when the user clears the query or leaves the view.
- Explain cancellation as a resource-saving mechanism and identity checks as protection against
  outdated state. Neither replaces the other.
- Explain cancellation safety in `select!`: dropping a receive future is different from dropping a
  partially completed multi-step operation. Identify who retains progress.
- Show when a resource-owning task is helpful and when a short lock is sufficient. Release locks
  before rendering or slow work.
- Introduce retained task handles and ownership of task lifetime; link shutdown details to
  Lifecycle.

**Example:** Add request generations and cancellation to the app. Include a small ordering timeline
and focused tests for superseded success/failure and invalidation without a replacement request.

**Evidence:** Yazi completion tickets, gitui replacement jobs, Tokio channel/cancellation contracts.
Use tokio-console to discuss watching durable selection state versus transient input actions.

**How it helps:** Readers can reason about correctness and resource use separately, including when
copied example code stops being sufficient as an app grows.

### Coordinate terminal I/O

**Reader question:** How do drawing, input, and terminal queries share access to the terminal?

- Follow the draw lifecycle to explain synchronous size checks, rendering, output, and any cursor
  queries. Show which operations occupy the UI task and which can involve the input reader.
- Distinguish API shape, implementation mechanism, and observed cost. A synchronous syscall is not
  automatically a protocol round trip; an awaitable event source may use blocking machinery.
- Explain that the cited Unix Crossterm size path uses an ioctl, while cursor-position queries can
  consume input replies. Do not infer contention or significant latency from synchrony alone.
- Establish terminal output, input, protocol replies, modes, and the role of an event parser.
- Diagram a query written to the terminal and its reply returning through the same input stream as
  keys and paste. Distinguish Unix resize signals from reply bytes.
- Explain the difference between an API contract and a recommendation: Crossterm's event-reader
  restrictions versus the application's choice of one coordinating terminal owner.
- Explain `EventStream`'s helper and why an async-shaped interface does not imply cancellable OS
  I/O. Contrast its lifecycle with Tokio stdin without calling all helper-thread approaches
  defective.
- Trace version-specific query paths: inline viewport setup/resize, explicit clear, and historical
  fullscreen resize behavior. Name the dependency versions being discussed.
- Explain stdin/stdout/stderr, redirected handles, and `/dev/tty` selection where relevant. A
  Ratatui writer does not automatically control where another library writes a query.
- Explain query timeouts and fallback. An async timeout does not interrupt synchronous code, and
  abandoning a blocking query may leave its reader active.
- Describe coordinated parsing and preservation of unrelated input; defer the handoff procedure to
  Lifecycle.

**Example:** A protocol diagram and a short, annotated call-path excerpt. Avoid inventing a generic
query broker API that readers might mistake for an available Crossterm facility.

**Evidence:** Crossterm contracts/source, Ratatui source, XTerm and Windows VT specifications, and
specific query/redirection failures.

**How it helps:** Readers understand why one task alone may not mean one active reader, and can
trace which handle, parser, and query actually participate in a failure.

### Shut down and hand off the terminal

**Reader question:** How do I exit or temporarily give another program the terminal correctly?

- Separate worker cancellation, completion acknowledgement, terminal restoration, and runtime
  shutdown.
- Explain detached handles, child-process lifetime, blocking work, and shutdown timeout limits.
- Trace the exit path from quit/error through stopping work and restoring modes. State what panic
  hooks cover and what they cannot promise; do not imply cleanup runs after every termination.
- Give editor/pager handoff its own sequence: pause and acknowledge reading, release terminal modes,
  run the child, reacquire even after startup failure, perform probes, redraw, and resume input.
- Distinguish requesting a reader to stop from observing it has stopped. State what the chosen
  library exposes; do not infer a join guarantee from `EventStream::drop`.
- Explain buffered-input policy and the risk of indiscriminate flushing.
- Cover suspend/resume separately from child invocation, including platform-specific job control,
  cursor changes, and the distinction between raw mode and alternate-screen state.

**Example:** An explicit lifecycle/state diagram plus one real, narrowly scoped implementation.
Acknowledge library limitations rather than hiding them behind placeholder `pause()` methods.

**Evidence:** Codex handoff/suspend fixes, gitui pause acknowledgements, Tokio shutdown guidance.

**How it helps:** Readers can design the failure path as carefully as the successful interaction.
Protocol mechanics stay on the terminal-boundary page rather than being repeated here.

### Troubleshoot async applications

**Reader question:** Which part of my design should I investigate for this symptom?

Use a compact symptom table linking to explanations, then a few worked diagnoses. For each case,
state the observed symptom, relevant mechanism, evidence/version, repair, and the repair's limits.

Cover:

- Input freezes during a request or CPU-heavy operation.
- A completed result appears only after another keypress.
- Progress creates memory growth or input lag.
- Old results or errors overwrite the current view.
- Drawing times out during resize or when stdout is redirected.
- Queries lose replies or consume ordinary input.
- A child loses keystrokes, or resume produces stray bytes/misplaced output.
- Exit hangs or leaves unwanted work running.

End with an audit path for apps based on older examples: trace wakeups, reader ownership, blocking
calls, queue policy, result identity, and cleanup. Identify mechanisms rather than blaming
templates.

**How it helps:** Readers can enter from a problem report without reading the section in order. This
page indexes and applies the lessons; it should not duplicate all their explanations.

### Optional: Terminal library design questions

Keep query routing, render/present separation, shared redraw scheduling, session ownership, and
platform integration tests as a separate discussion for library authors.

For each proposal, state the problem, existing mechanism, potential improvement, unresolved
contract, and evidence needed. Explain the motivation for moving frame preparation independently of
output. Do not imply an accepted roadmap, a universally non-blocking terminal, or publicly available
support based on private prototypes.

## Examples and source comments

Keep compile-tested source under `code/concepts/async-applications/`. Evolve the current skeletons
into a small runnable reference app and focused companion snippets; a full component framework is
not required. A deterministic delayed local operation can make the first example reproducible, with
real network usage linked separately and the simulation clearly labelled.

- Keep source comments rich enough to explain ownership, policy, ordering, and failure paths.
- Include focused regions in pages; let the complete source provide the deeper reading path.
- Give each snippet its context: owning task/thread, runtime requirements, caller cleanup, and
  omitted application logic. Label conceptual pseudocode distinctly from runnable Rust.
- Use the same message vocabulary and request model across pages. Explain additions before showing
  them, rather than presenting the final large loop all at once.
- Ensure a reader can follow each included region without reconstructing hidden, consequential
  state.
- Prefer one meaningful diagram or timeline to another paragraph where concurrency is the
  difficulty.

## Research map

Reuse the pinned links in the current three documents. This map assigns source material to lessons;
it does not endorse every implementation as a complete architecture to copy.

- Templates, `async-github`, crates-tui: Work/result flow, wakeups, task placement, limitations
- Yazi: Batching, render flags, cancellation, completion identity
- Codex: Redraw requests, resize cost, probes, handoff, suspend/resume
- gitui: Acknowledged reader pause and replacement of pending jobs
- bottom: Blocking producers and input/collection thread tradeoffs
- bacon and dua-cli: Producer pressure, bounded queues, blocking selectors
- tokio-console: Subscription lifetime and state-versus-notification semantics
- Helix and Termina: Redraw coordination, diff work, filtered protocol input

The original investigation's failure inventory should survive the reorganization:

- [Ratatui resize report][resize-report] and [fix][resize-fix]: hidden cursor queries and versions.
- [Crossterm redirected-output report][redirect-report]: query writer versus application writer.
- [Crossterm competing-reader report][reader-report]: partial replies and independent libraries.
- [Codex color-query repair][color-fix]: query routing and ordinary input.
- [Codex event-stream change][handoff-fix]: input ownership during child handoff.
- [Codex suspend repair][suspend-fix]: resume ordering and terminal state.
- [Codex resize guardrails][resize-guardrails]: rendering and reflow under load.

Cite primary API contracts beside requirements, pinned implementation beside version-specific
observations, and issue/fix pairs beside historical failures. Explain both what to borrow from an
application and the boundary that requires adaptation. Classify dua-cli as Ratatui-based if stack
identity matters, and qualify the tokio-console subscription example before recommending it.

Retain a short distinction between **API/protocol references** and **background reading**. The
original task explicitly requested it. Tokio/Crossterm contracts and XTerm/Windows specifications
support technical claims; Alice Ryhl's explanations and TTY/termios introductions provide additional
context. Public pages should link public evidence, not require access to Codex task history.

## Migration and editorial order

1. Agree on the section's reader entry points, suggested learning path, and worked-example role.
1. Trace the reader's application from input through work, state updates, and drawing, including
   error and exit paths. Introduce vocabulary and constraints where they explain the choices.
1. Move existing material to its owning pages using the mapping below; retain the evidence links.
1. Fill explanation gaps and correct the specific findings from this review.
1. Replace project-by-project survey paragraphs with focused cases near the lessons they support.
1. Wire the Concepts sidebar and cross-links. Inspect existing async tutorial routes before deciding
   whether to update, redirect, or mark them as older material; their current navigation entry is
   commented out, but that does not prove their URLs have never been used.
1. Check the complete reading path, individual deep-link entry points, and rendered page sizes.

- Introduction and ownership/design table: Overview and Event loops
- Async and synchronous loop examples: Event loops
- Blocking, batching, frame scheduling: Scheduling
- Channels, stale results, cancellation: Background operations
- Query replies, backend paths, redirection: Terminal I/O
- Shutdown, editor handoff, suspend/resume: Lifecycle
- Failure-mode table and older-material audit: Troubleshooting
- Application survey: Relevant lesson pages, with a compact source index if needed
- Async gaps/design proposals: Optional developer discussion

The new PR pages were described as unpublished in the original task. Do not assume they need a large
redirect framework, but check their current publication status and known links before moving them.
Use stable source links that survive deletion of the PR branch.

Keep implementation changes, unrelated formatting, Elm example maintenance, and dependency updates
out of this editorial work. The rebased async patch already isolates the async package registration.
This outline is a separate planning change and should not accidentally become a public content page.

## Completion criteria

- Newcomers can understand the basic ideas without already knowing Tokio or terminal terminology.
- The landing page exposes learning, implementation, reference, troubleshooting, and design paths.
- Experienced readers can find precise constraints and deeper reasoning without rereading a
  tutorial.
- Application developers can justify choices using their needs and the relevant constraints.
- Contributors can distinguish current behavior, evidence, and unresolved design questions.
- Readers distinguish blocking work from query/reply contention and know which their fix addresses.
- The event-loop example produces visible background progress without another keypress.
- Readers can explain every task/thread boundary and redraw trigger from the annotated source.
- Each architecture includes its benefits, costs, assumptions, and relevant version constraints.
- Consequential library claims have traceable evidence and accurately scoped guarantees.
- Examples make blocking/non-blocking coordination explicit, including failure and interruption.
- Unsupported or unverified guarantees are identified rather than hidden behind simplified code.
- Every original failure lesson has an assigned home, supported explanation, and source.
- Timeouts, cancellation, reader acknowledgement, and shutdown do not promise unavailable
  guarantees.
- The docs distinguish contracts, recommendations, observed implementations, and proposed designs.
- Deep links are understandable locally without duplicating whole lessons across pages.
- Examples compile and have focused behavioral checks for the claims they demonstrate.
- Markdown, links, code includes, and the site build pass; desktop/mobile rendered pages are
  inspected.
- Reader/child handoff and platform behavior are reported as tested only when actually exercised.
- Page length is judged after includes expand, retaining depth that helps the reader make decisions.

[resize-report]: https://github.com/ratatui/ratatui/issues/2483
[resize-fix]: https://github.com/ratatui/ratatui/pull/2485
[redirect-report]: https://github.com/crossterm-rs/crossterm/issues/919
[reader-report]: https://github.com/crossterm-rs/crossterm/issues/1039
[color-fix]: https://github.com/openai/codex/commit/07b8bdfbf1497cf7c478872bd082a13c5bd82c63
[handoff-fix]: https://github.com/openai/codex/commit/cf44511e7780bc30286ec356849970ff7aeabebb
[suspend-fix]: https://github.com/openai/codex/commit/76135cbe7ec8dbcc165aa1f2bd21358f9f1c6571
[resize-guardrails]: https://github.com/openai/codex/commit/3aa637c4750715cf23589ee3f4b1d0b6563c7d3e
[blocking-guidance]: https://ryhl.io/blog/async-what-is-blocking/
[cooperative]: https://tokio.rs/blog/2020-04-preemption
