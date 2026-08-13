# Rust for Bitcoin 2.0 — Week 2, Session 4

Build a small lending library while practising structs, enums, traits,
ownership, borrowing, collections, and `Result`-based error handling. No
Bitcoin and no external crates — just Rust.

The crate is intentionally incomplete. Search for `TODO` and implement each
part; do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Part 2 in `error.rs`, then Part 3 in `library.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits in Part 4 and the two operations in Parts 5–6.
5. Run the ownership experiments and record the errors.
6. Build the demo in `main.rs`.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in
the submission.

## Written answers

Answer in your own words. Add both ownership compiler errors from Part 7 as
fenced text blocks, then explain what caused each.

1. Why is `LoanStatus` an enum rather than a `bool` plus two `Option` fields?

   > An item's loan state is inherently "one of" — `Available`, `OnLoan`, or `Lost` — never more than one at once. A `bool` plus two `Option` fields can't express that exclusivity in the type itself: nothing stops `is_on_loan: false` from coexisting with `member_id: Some(100)`, an invalid state the compiler would happily accept. The enum makes that combination impossible to construct, so every function that touches `LoanStatus`can trust it's always in exactly one valid shape.

2. What does `match` force you to do when a fourth `MediaKind` is added later?

   > `match` is exhaustive — the compiler rejects the code if any variant is unhandled and there's no `_` catch-all. So adding a fourth `MediaKind` variant (say `Magazine`) doesn't fail quietly at runtime; it fails to *compile*, at every single `match` over `MediaKind` anywhere in the crate that doesn't already have a wildcard arm. The compiler finds every call site that needs a decision for the new variant — there's no way to forget one.

3. `Item::new` takes `String` rather than `&str`. Who owns the title afterwards?

   > `Item::new` takes `title: String` by value, so the `String` is moved into the new `Item` — the `Item` now owns its title outright. If it took `&str` instead, the `Item` would be borrowing someone else's data, which means it could only live as long as whatever it borrowed from — a `library` full of `Item`s couldn't outlive the `String`s the caller built them from. Taking ownership means an `Item` is self-contained and can be stored in the `Library`'s `Vec<Item>` indefinitely, with no lifetime tied to the caller

4. Why does `add_item` take `self` by `&mut` but `item` by value?

    > `self: &mut Library` because `add_item` needs to mutate the library's internal state (push into `items`) without taking ownership of the `Library` itself — the caller keeps using their `library` variable after the call returns; a `&mut` borrow is enough to modify it in place.
   > `item: Item` by value, on the other hand, because the library needs to *keep* the item forever, stored in its `Vec<Item>` — you can't store a borrowed reference there without giving `Library` a lifetime tied to wherever the caller's `Item` came from. Taking ownership is how the `Item` is handed off permanently.

5. When `add_item` returns `Err`, what happened to the `Item` the caller passed
   in? Was that a good design choice, and what is the alternative?

   > Because `item: Item` is taken by value, ownership moves into `add_item` at the call site — the caller has already lost direct access to it before the function body even runs. If `add_item` returns `Err`, the `Item` is simply dropped at the end of the function; its data is gone for good. For a *recoverable* error like a duplicate id, that's a questionable tradeoff — the caller may have done real work building that `Item` and now has to rebuild it from scratch just to retry with a different id. The alternative, used by `std::sync::mpsc::Sender::send(`Result<(), SendError<T>>`), is to hand the value back inside the error: something like `Result<(), (LibraryError, Item)>`, so a failed call returns both *why* it failed and the `Item` to retry with.

6. Why does `find_item` return `Option<&Item>` rather than `Option<Item>`?

   > _answer_

7. What is the lifetime `'a` in `items_by_author` actually saying?

   > _answer_

8. Why can't `checkout` hold a `&mut Item` and a `&mut Member` from the same
   `Library` at once, and how did you structure the method around that?

   > _answer_

9. Why are `Library`'s fields private?

   > _answer_

10. What duplication does the provided `late_fee_cents` remove, and what would
    you lose by making it a free function instead?

    > _answer_

11. Why is `Result` preferable to `panic!` for validation failures? Name a
    place in this crate where a panic would be defensible.

    > _answer_

12. Which derive did you deliberately leave off a type, and why?

    > _answer_

### Ownership experiments (Part 7)

**Experiment A** — read `item.title` after `library.add_item(item)?`.

```text
paste the real `cargo check` error here
```

> _explanation_

**Experiment B** — hold the result of `library.find_item(1)`, call
`library.checkout(..)?`, then print what you held.

```text
paste the real `cargo check` error here
```

> _explanation_

## Design notes

Describe any choices you made, including how you kept an item's status and its
borrower's list from drifting apart, and (if attempted) the optional generic
search.

## Example output

Paste the output of `cargo run` here once Part 8 is complete.
