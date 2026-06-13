//! ─────────────────────────────────────────────────────────────────────────
//!  EDIT ME — every piece of text on the site lives in this one file.
//!  Change the strings/links below, then `trunk serve` to preview.
//!  Anything marked TODO is placeholder content you should replace.
//! ─────────────────────────────────────────────────────────────────────────

pub struct Profile {
    pub name: &'static str,
    pub role: &'static str,
    pub tagline: &'static str,
    pub status: &'static str,
    pub email: &'static str,
    /// Set to `Some("assets/resume.pdf")` after dropping the file in `assets/`.
    pub resume_url: Option<&'static str>,
    pub about: &'static [&'static str],
    pub facts: &'static [(&'static str, &'static str)],
}

pub struct Social {
    pub label: &'static str,
    pub href: &'static str,
}

pub struct Project {
    pub name: &'static str,
    pub year: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub repo: Option<&'static str>,
    pub demo: Option<&'static str>,
}

pub struct Job {
    pub role: &'static str,
    pub company: &'static str,
    pub period: &'static str,
    pub location: &'static str,
    pub bullets: &'static [&'static str],
    pub tech: &'static [&'static str],
}

pub struct SkillGroup {
    pub category: &'static str,
    pub items: &'static [&'static str],
}

pub const PROFILE: Profile = Profile {
    name: "Gabe Vahari", // TODO: confirm full name
    role: "Software Engineer",
    tagline: "I build fast, reliable systems for the web — from the browser down to the bytes.",
    status: "Currently open to new opportunities",
    email: "gaberingo12@gmail.com",
    resume_url: None, // TODO: add assets/resume.pdf, then set Some("assets/resume.pdf")
    about: &[
        "I'm a software engineer who likes building things that are fast, correct, and pleasant to use. Most of my time goes to systems and web work — lately a lot of Rust and WebAssembly (this very site is compiled from Rust to Wasm, no JS framework).",
        "I care about the details: tight feedback loops, readable code, and interfaces that respect the people using them. I'm happiest shipping real software and learning the layer below the one I thought I already understood.",
        "Outside of code — TODO: a line about your interests so this reads like you.",
    ],
    facts: &[
        ("location", "Remote"),       // TODO
        ("focus", "Systems · Web"),
        ("languages", "Rust · TS · Python"),
        ("status", "Open to work"),
    ],
};

pub const SOCIALS: &[Social] = &[
    Social { label: "github", href: "https://github.com/gavhari" },
    Social { label: "linkedin", href: "https://www.linkedin.com/in/your-handle" }, // TODO
    Social { label: "email", href: "mailto:gaberingo12@gmail.com" },
];

pub const PROJECTS: &[Project] = &[
    Project {
        name: "gabe-portfolio",
        year: "2026",
        description: "This site — a terminal-themed portfolio compiled to WebAssembly. No JS framework, just Rust + Yew rendering to the DOM.",
        tech: &["Rust", "Yew", "WebAssembly", "Trunk"],
        repo: Some("https://github.com/gavhari/gabe-portfolio"),
        demo: Some("https://gavhari.github.io/gabe-portfolio/"),
    },
    // TODO: replace the three below with your real projects.
    Project {
        name: "project-two",
        year: "2025",
        description: "A short, punchy description of what it does and why it's interesting. Lead with the impact.",
        tech: &["Rust", "Axum", "PostgreSQL"],
        repo: Some("https://github.com/gavhari"),
        demo: None,
    },
    Project {
        name: "project-three",
        year: "2025",
        description: "One or two sentences. What problem it solves, what's notable about how you built it.",
        tech: &["TypeScript", "React", "Vite"],
        repo: Some("https://github.com/gavhari"),
        demo: Some("https://example.com"),
    },
    Project {
        name: "project-four",
        year: "2024",
        description: "Keep these scannable — recruiters skim. The repo/live links do the rest.",
        tech: &["Python", "FastAPI", "Docker"],
        repo: Some("https://github.com/gavhari"),
        demo: None,
    },
];

pub const EXPERIENCE: &[Job] = &[
    // TODO: replace with your real roles.
    Job {
        role: "Software Engineer",
        company: "Company Name",
        period: "2024 — Present",
        location: "Remote",
        bullets: &[
            "Led / built X that achieved Y (use a number — latency, users, revenue, %).",
            "Shipped feature Z end-to-end, from design through deploy and on-call.",
            "Improved A by B through C — show impact, not just activity.",
        ],
        tech: &["Rust", "TypeScript", "AWS", "PostgreSQL"],
    },
    Job {
        role: "Software Engineer Intern",
        company: "Earlier Company",
        period: "2023 — 2024",
        location: "Remote",
        bullets: &[
            "Owned a project that did X.",
            "Collaborated with team on Y.",
        ],
        tech: &["Python", "React", "Docker"],
    },
];

pub const SKILLS: &[SkillGroup] = &[
    // TODO: trim to what's actually true for you.
    SkillGroup { category: "languages", items: &["Rust", "TypeScript", "Python", "Go", "SQL"] },
    SkillGroup { category: "frameworks", items: &["Yew", "React", "Axum", "FastAPI", "Node.js"] },
    SkillGroup { category: "tools", items: &["Git", "Docker", "Linux", "CI/CD", "Nginx"] },
    SkillGroup { category: "practices", items: &["Testing", "Profiling", "REST / gRPC", "WebAssembly"] },
];