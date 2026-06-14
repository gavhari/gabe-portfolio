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
    name: "Gabe Siringoringo",
    role: "ERP Developer & Systems Engineer",
    tagline: "ERP developer by day, building fast systems in Rust by night.",
    status: "Currently open to new opportunities",
    email: "gaberingo12@gmail.com",
    resume_url: Some("assets/Resume_Gabe_Siringoringo.pdf"),
    about: &[
        "I work as an ERP developer (Odoo) — building custom modules, fixing bugs, and migrating clients across versions. Odoo's performance characteristics got me curious about what an ERP built in Rust could look like — a language where correctness is enforced by the compiler and speed is the default.",
        "Outside ERP work, I enjoy building things in Rust — automation tooling, web systems, and a marketplace for local Indonesian products with a Python/LangChain AI layer planned on top.",
        "Lately I've been deep into AI-augmented systems — building agents, automation pipelines, and LLM-powered tooling. Currently learning Claude (Anthropic's API and agentic SDK) and Hermes, and applying them directly to real projects rather than toy demos.",
    ],
    facts: &[
        ("location", "Medan, Indonesia"),
        ("focus", "ERP · Systems · Automation"),
        ("languages", "Rust · Python · JS"),
        ("status", "Open to work"),
    ],
};

pub const SOCIALS: &[Social] = &[
    Social {
        label: "github",
        href: "https://github.com/gavhari",
    },
    Social {
        label: "linkedin",
        href: "https://linkedin.com/in/gabe-siringoringo-a827aa16b",
    },
    Social {
        label: "email",
        href: "mailto:gaberingo12@gmail.com",
    },
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
    Project {
        name: "DairiMaronan",
        year: "2026",
        description: "Multi-seller marketplace for Indonesian UMKM — subscription-based, not commission. Full-stack Rust (Actix + Yew/WASM), event-driven over Redis Streams, blue-green deploys.",
        tech: &["Rust", "Actix", "Yew", "WebAssembly", "PostgreSQL", "Redis"],
        repo: Some("https://github.com/gavhari/DairiMaronan"),
        demo: None,
    },
    Project {
        name: "WebAutomationMITM",
        year: "2026",
        description: "HTTPS MITM proxy + browser automation with ±5ms scheduled fire precision. Records, templates, and replays HTTP flows; syncs to target server clock via NTP.",
        tech: &["Rust", "Axum", "React", "CDP"],
        repo: Some("https://github.com/gavhari/WebAutomationMITM"),
        demo: None,
    },
    Project {
        name: "hyprlan_nvim_wezterm_conf",
        year: "2026",
        description: "Personal dotfiles — Hyprland compositor, Neovim, and WezTerm configured as a unified keyboard-driven dev environment.",
        tech: &["Shell", "Lua", "Nix"],
        repo: Some("https://github.com/gavhari/hyprlan_nvim_wezterm_conf"),
        demo: None,
    },
    Project {
        name: "odoo-hr-attendance-module",
        year: "2025",
        description: "Custom Odoo HR attendance module with geofence validation and overtime calculation rules. Built for a manufacturing client at Hashmicro.",
        tech: &["Python", "PostgreSQL", "QWeb", "XML"],
        repo: None,
        demo: None,
    },
    Project {
        name: "odoo-v18-migration",
        year: "2024",
        description: "Full migration of a retail client's Odoo instance from v15 to v18 — resolved deprecated API calls, rewrote OWL components, and re-validated all custom reports.",
        tech: &["Python", "JavaScript", "QWeb", "PostgreSQL"],
        repo: None,
        demo: None,
    },
    Project {
        name: "palm-oil-mapping-model",
        year: "2024",
        description: "Trained a model to detect and map palm oil plantation plots from satellite/aerial imagery. Built during tenure at Witech for agricultural land analysis.",
        tech: &["Python", "Machine Learning"],
        repo: None,
        demo: None,
    },
];

pub const EXPERIENCE: &[Job] = &[
    Job {
        role: "ERP Developer",
        company: "Hashmicro",
        period: "2024 — Present",
        location: "Remote",
        bullets: &[
            "Diagnosed and resolved bugs across multiple ERP modules, reducing recurring client-reported issues.",
            "Delivered custom features and business-specific adjustments based on client requirements.",
            "Collaborated with stakeholders to translate business needs into working Odoo customizations.",
        ],
        tech: &["Python", "JavaScript", "XML", "PostgreSQL", "QWeb", "Linux"],
    },
    Job {
        role: "ERP Developer",
        company: "Witech",
        period: "2023 — 2024",
        location: "On Site",
        bullets: &[
            "Built custom reports for business operations using QWeb and PostgreSQL queries.",
            "Updated and extended existing ERP features to meet evolving client workflows.",
            "Led migration of Odoo modules from v15 to v18, resolving breaking API changes and deprecations.",
        ],
        tech: &["Python", "JavaScript", "XML", "PostgreSQL", "QWeb", "Linux"],
    },
];

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        category: "languages",
        items: &["Rust", "Python", "JavaScript", "SQL", "XML"],
    },
    SkillGroup {
        category: "frameworks",
        items: &["Actix", "Yew", "Axum", "Odoo", "React", "QWeb"],
    },
    SkillGroup {
        category: "tools",
        items: &["Git", "Docker", "Linux", "PostgreSQL", "Redis", "Nginx"],
    },
    SkillGroup {
        category: "practices",
        items: &[
            "ERP Customization",
            "REST",
            "WebAssembly",
            "Machine Learning",
            "Blue-Green Deploy",
        ],
    },
];
