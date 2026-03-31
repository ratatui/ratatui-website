---
title: Submitting to the Showcase
sidebar:
  order: 11
  label: Submitting to the Showcase
---

The Ratatui showcase is curated. It is not meant to be a complete directory of every Ratatui app
or widget. The goal is to highlight entries that make someone stop, immediately understand what is
interesting, and want to click through to learn more.

That means showcase reviews are partly subjective. We still want the reasoning to be clear and
consistent, so this page explains what we tend to optimize for and why.

This guidance is not coming out of nowhere. It consolidates previous discussions in
[#986](https://github.com/ratatui/ratatui-website/issues/986),
[#905](https://github.com/ratatui/ratatui-website/issues/905), and
[#1032](https://github.com/ratatui/ratatui-website/pull/1032), along with feedback repeated across
showcase submissions and review threads.

If your project is not a good fit for the showcase today, that does not mean it is not useful or
well made. It often just means it is a better fit for [Awesome Ratatui], the forum, or Discord
until the presentation is stronger.

## What the showcase is for

The showcase is optimized for:

- first impressions
- visual clarity at a glance
- entries that still read well when resized on the website, in a GitHub PR, or on a phone
- a small number of strong examples rather than a complete catalog

The mental model is simple:

> Someone should be able to see the image or GIF, understand what is compelling within a second or
> two, and feel like they want to try the project.

## Principles behind the feedback

Most review comments are trying to improve one of these principles:

- Legibility. Text, borders, labels, and structure should still be readable when the media is
  scaled down.
- Contrast. Dim text, low-contrast separators, and thin visual details often disappear on the
  website.
- Visual hierarchy. The important part of the UI should stand out immediately.
- Signal over coverage. A single strong view is usually better than showing every possible mode,
  configuration, or feature.
- Calm motion. Animation should help people understand the UI, not make it harder to parse.
- Intentional composition. Empty space, crowded layouts, and too many competing colors make entries
  feel less polished.

These are the reasons behind feedback such as "use a smaller width", "increase dwell time", or
"simplify the layout". Those are not arbitrary rules. They are proxies for clarity.

## The bar for apps

The apps page is intentionally selective. In practice, new submissions should look better than a
large share of the apps already on the page, and the safest submissions usually land closer to the
top tier than the middle.

Strong app submissions usually have most of these qualities:

- The UI looks polished rather than prototype-like.
- The purpose of the app is obvious from a single screenshot or short GIF.
- The visual design feels intentional, with clear hierarchy and spacing.
- The app uses color and contrast well instead of relying on a flat or muddy palette.
- The recording focuses on the app, not the shell command used to launch it.
- The entry feels stable and production-ready rather than experimental or broken.

Common reasons for pushback:

- the UI looks too plain, cluttered, or unfinished
- there is too much whitespace or too many boxes competing for attention
- the media is visually busy and does not make the app's purpose obvious
- the color choices reduce readability or fail to create hierarchy
- the app may be useful, but it is not a strong fit for the curated showcase

## The bar for widgets and widget libraries

For built-in widgets and third-party widget libraries, the goal is slightly different. We are not
trying to show every option the widget supports. We are trying to show why someone would want to use
it.

Strong widget submissions usually:

- choose one or two compelling states instead of every permutation
- crop or zoom the scene so the widget occupies more of the frame
- use text sizes that match the rest of the showcase
- avoid dense grids of tiny examples
- keep labels readable without pausing frame-by-frame

If a widget is highly configurable, the right place to show every variation is usually your own
repository or docs. The showcase should usually present a single strong hero view that makes the
widget's value obvious.

## Media guidelines

These are the practical rules we keep repeating because they work well across the website and
GitHub:

- Prefer a width in the 800-1200px range. `1200` is a good default ceiling.
- Prefer the default VHS font size or something visually similar.
- Use a dark theme unless you have a compelling reason not to.
- Hide the shell command and unrelated setup noise.
- Keep the subject large in frame. Remove wasted margins and padding where possible.
- Give viewers enough dwell time to read the screen before changing state.
- Use GIFs only when motion helps explain the UI. A screenshot is often better.
- Avoid fast cuts, simultaneous changes everywhere, or long sequences of tiny states.
- Check the result on a phone-sized viewport before submitting.

For storage:

- Prefer GitHub attachments, `vhs publish`, your own hosting, or Git LFS.
- Avoid committing large binary screenshots directly into normal Git history when possible.

## Where to submit instead

Not every project belongs on the curated showcase, but there are still good places to share it:

- [Awesome Ratatui]
- [Ratatui Forum]
- [Ratatui Discord]

Those are great places for early projects, niche tools, libraries with less visual focus, or apps
that are useful but not yet showcase-ready.

## Before opening a PR or issue

Please review your submission against this checklist:

- Does the media make the core idea obvious immediately?
- Is the text still readable when resized?
- Does the UI have clear hierarchy and good contrast?
- Have you removed extra modes, whitespace, and low-value variation?
- Does the animation linger long enough to understand each important state?
- Would this stand out positively next to the current showcase entries?
- Is the project already documented and shared in a place like Awesome Ratatui?

If the answer to several of those is "not yet", you will probably have a better experience by
iterating on the presentation first.

## Related reading

- [Releasing Your App](./release-your-app/)
- [App Showcase](../../showcase/apps/)
- [Third Party Widgets](../../showcase/third-party-widgets/)
- [Issue #986: Ratatui app showcase criteria](https://github.com/ratatui/ratatui-website/issues/986)
- [Issue #905: Improve Showcase Apps](https://github.com/ratatui/ratatui-website/issues/905)
- [PR #1032 discussion](https://github.com/ratatui/ratatui-website/pull/1032)

[Awesome Ratatui]: https://github.com/ratatui/awesome-ratatui
[Ratatui Discord]: https://discord.gg/pMCEU9hNEj
[Ratatui Forum]: https://forum.ratatui.rs
