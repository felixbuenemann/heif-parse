# What this fork is, and how to keep it

`heif-parse` is a fork of [kornelski/avif-parse](https://github.com/kornelski/avif-parse)
by way of [imazen/zenavif](https://github.com/imazen/zenavif), carried on `main`
of `felixbuenemann/heif-parse` and consumed by SIS as a git submodule at
`image_engine/crates/heif-parse`.

## Where this is going

Three futures, and the commit rule below serves all of them:

1. **A submodule for SIS.** What it is today.
2. **A standalone crate on crates.io.** `heif-parse` and `heif-serialize` are
   both unclaimed, checked 2026-08-11. This is a live option, and it is the one
   that most rewards the discipline: a published crate is judged on its
   history, its docs and whether it can be used without knowing who wrote it.
3. **Upstream.** Unlikely — see below.

## Why it diverged, and why upstreaming is unlikely

SIS is replacing libheif with this crate plus direct codec bindings. libheif's
container layer was doing two jobs SIS wanted separated: reading the
ISO-BMFF/HEIF item model, and dispatching to codec plugins. The item model is
codec-agnostic by design, so the parser needed only to be told that HEVC exists
— everything below the codec configuration property was already general.

That is the divergence, and it is a wide one. This crate now admits
`avif`/`avis`, `heic`/`heix`/`heim`/`heis`, `hevc`/`hevx`/`hevm`/`hevs` and
`mif1`/`mif2`/`mif3`/`msf1`; it reads `hvcC`, HEVC sample entries, `iden`
chains, thumbnails and the low-overhead `mini` box. AVIF is one codec in the
HEIF item model and HEVC is another. Upstream is an AVIF decoder project, and
its monorepo is organised around that. **A HEIF-shaped parser is probably not
what it wants**, and the crate rename makes the divergence explicit rather than
pretending otherwise.

So treat upstreaming as unlikely. Individual fixes might still be worth
offering — keeping every `ipma` rather than only the last, and reporting frame
durations in ticks, are plain bugs that affect AVIF callers too — but the fork
as a whole is not heading home.

It is also not named for MIAF, which is a constrained *profile* of HEIF whose
conformance requirements this crate does not check. `mif1` is one brand of the
fourteen it accepts.

## The commit rule

**Every commit here should stand alone as a pull request to a project that has
never heard of SIS.**

- No SIS imports, SIS types, or SIS-specific naming. A reader who has never
  heard of SIS should not be able to tell what the change is for.
- Tests included, in the crate's own idiom, exercising the change on its own
  terms rather than through the SIS path that motivated it.
- A message that says what changed and why, with any measurement or file that
  prompted it named explicitly.
- `cargo fmt` scoped per package (`cargo fmt -p heif-parse`) — never `--all`,
  which is banned across the consuming repo.

**Keep following it even if no PR is ever sent.** The rule was written to keep
the fork mergeable, but mergeability was never the whole value. What it
actually enforces is a commit that is about one thing, explains itself, and
carries its own test — which is what makes a change reviewable, a regression
bisectable, and a decision recoverable a year later.

It also keeps the crate honestly general, which is what a standalone release
depends on: a change that cannot be explained without mentioning SIS is usually
a change that belongs in the SIS crates that wrap this one, and the rule
catches that before the coupling is written. A crate published with SIS-shaped
seams in it is one nobody else can use.

And rebasing onto upstream stays possible for as long as the rule holds, which
is worth something even if it is never done.

## Things a reader will trip over

- **Admitting HEVC item types widens an error path.** Files that previously
  failed with "ftyp must be 'avif' or 'avis'" may now parse. That is the point,
  but it is a behaviour change for anyone relying on the parser to reject HEIF.
- **Reading `hvcC` from sample entries swaps a derive for a trait impl.**
  `TrackCodecConfig` gives up `Clone` for `TryClone`, because it can now hold an
  `HEVCConfig`, which owns its parameter sets. This follows the crate's
  fallible-allocation policy rather than departing from it.
- **Thumbnails carry their own geometry.** The coded size is not the size to
  show — libheif codes a 32x20 thumbnail at 64x64, since HEVC pads to its
  alignment — so `ThumbnailInfo` carries the thumbnail's own `clap` and offers
  `display_size()`. Choosing among thumbnails by size is the entire reason to
  list them.
- **The `hvcC` reader is cross-pinned with `heif-serialize`** by a test holding
  the exact bytes that serializer emits, so the two cannot drift apart without
  one going red.
- **The alpha auxiliary's own `hvcC` fixes a silent wrong answer.** A caller
  decoding an alpha auxiliary had no way to reach its decoder configuration,
  and using the primary's does not fail: the auxiliary is monochrome, so its
  SPS says `chroma_format_idc = 0` where the picture's says 4:2:0, and the
  decoder returns a three-plane frame whose luma is not the coverage.
- **Derivation-chain transform ORDER is the whole risk.** Properties along an
  `iden` chain apply CODED ITEM FIRST, then each item deriving from it — what
  the recursion produces, not what the chain reads like. Its test is
  deliberately asymmetric because a symmetric file cannot detect the reverse:
  two centred square apertures and two equal rotations compose the same either
  way round.

## Divergence from upstream

**The crate is renamed.** Upstream it is `zenavif-parse`; the rename is its own
commit. Read it as this crate's identity rather than as damage: `heif-parse`
is what it would publish as, and the name says what it does. It is also the one
commit that could never be upstreamed, so it is kept isolated — which happens
to be exactly what makes it cheap to lift off if anyone ever does rebase onto
upstream instead.

Nothing else diverges. If something SIS-specific ever has to live here, record
it in this section with the reason, so it is visible later — and treat its
appearance as a sign the change belongs in the SIS crates that wrap this one.

## Building and testing

This repo is the crate, so the usual commands work directly:

```sh
cargo test -p heif-parse
cargo fmt -p heif-parse -- --check
```

The corpus tests fail loud on the `av1-avif` and `link-u-samples` submodules;
run `git submodule update --init` once per checkout.
