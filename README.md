# gabe-portfolio

Personal portfolio — a terminal-themed single page compiled from **Rust → WebAssembly** with [Yew](https://yew.rs), built by [Trunk](https://trunkrs.dev), and deployed to GitHub Pages.

**Live:** https://gavhari.github.io/gabe-portfolio/

## Editing content

All site text and links live in one file: [`src/data.rs`](src/data.rs).
Edit the strings there (name, tagline, projects, experience, skills, socials) — anything marked `// TODO` is placeholder. No need to touch the components for content changes.

- **Résumé:** drop a PDF at `assets/resume.pdf`, then set `resume_url: Some("assets/resume.pdf")` in `src/data.rs`. A download button appears automatically.
- **Favicon:** `assets/favicon.svg`.
- **Theme/colors:** CSS variables at the top of [`styles/main.css`](styles/main.css).

## Local development

```bash
rustup target add wasm32-unknown-unknown   # once
cargo install trunk                        # once
trunk serve                                # http://127.0.0.1:8080 with live reload
```

## Build

```bash
trunk build --release
```

Output lands in `dist/` (git-ignored).

## Deploy

Pushing to `main` triggers `.github/workflows/deploy.yml`, which builds with
`trunk build --release --public-url /gabe-portfolio/` and publishes `dist/` to
GitHub Pages.

> One-time setup: in the repo, **Settings → Pages → Build and deployment →
> Source** must be set to **GitHub Actions**.

## Structure

```
index.html            Trunk entry: meta/SEO, fonts, CSS + asset links
styles/main.css       Terminal dark theme
assets/               Static files copied to the site (favicon, résumé, …)
src/
  main.rs             App root — assembles the sections
  data.rs             ← all content lives here
  components/         One module per UI piece (nav, hero, projects, …)
```
