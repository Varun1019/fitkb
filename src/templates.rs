use maud::{html, Markup, DOCTYPE};

pub struct SeoMetadata {
    pub title: &'static str,
    pub description: &'static str,
    pub canonical_url: &'static str,
    pub h1: &'static str,
    pub preselected_target_kb: Option<usize>,
}

pub fn base_layout(seo: &SeoMetadata, content: Markup) -> Markup {
    let target_preset = seo.preselected_target_kb.unwrap_or(0);

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5.0";
                title { (seo.title) }
                meta name="description" content=(seo.description);
                link rel="canonical" href=(seo.canonical_url);

                // Open Graph & Social
                meta property="og:title" content=(seo.title);
                meta property="og:description" content=(seo.description);
                meta property="og:url" content=(seo.canonical_url);
                meta property="og:type" content="website";
                meta name="twitter:card" content="summary_large_image";
                meta name="theme-color" content="#2563EB";

                // Fonts: Plus Jakarta Sans & Inter
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Plus+Jakarta+Sans:wght@600;700;800&display=swap" rel="stylesheet";

                style {
                    (maud::PreEscaped(r#"
                        :root {
                            --primary: #2563EB;
                            --primary-hover: #1D4ED8;
                            --primary-light: #EFF6FF;
                            --primary-ring: rgba(37, 99, 235, 0.2);
                            --success: #10B981;
                            --success-hover: #059669;
                            --success-light: #ECFDF5;
                            --slate-900: #0F172A;
                            --slate-800: #1E293B;
                            --slate-700: #334155;
                            --slate-600: #475569;
                            --slate-500: #64748B;
                            --slate-400: #94A3B8;
                            --slate-300: #CBD5E1;
                            --slate-200: #E2E8F0;
                            --slate-100: #F1F5F9;
                            --slate-50: #F8FAFC;
                            --font-heading: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, sans-serif;
                            --font-body: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
                            --radius-sm: 8px;
                            --radius-md: 12px;
                            --radius-lg: 16px;
                            --radius-xl: 20px;
                            --radius-full: 9999px;
                            --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
                            --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.07), 0 2px 4px -2px rgba(0, 0, 0, 0.05);
                            --shadow-lg: 0 10px 25px -5px rgba(15, 23, 42, 0.08), 0 8px 10px -6px rgba(15, 23, 42, 0.03);
                            --shadow-xl: 0 20px 35px -10px rgba(15, 23, 42, 0.1);
                        }

                        *, *::before, *::after {
                            box-sizing: border-box;
                            margin: 0;
                            padding: 0;
                        }

                        html {
                            font-size: 16px;
                            background: var(--slate-50);
                            color: var(--slate-900);
                            font-family: var(--font-body);
                            scroll-behavior: smooth;
                            -webkit-font-smoothing: antialiased;
                            -moz-osx-font-smoothing: grayscale;
                        }

                        body {
                            min-height: 100vh;
                            display: flex;
                            flex-direction: column;
                            line-height: 1.6;
                        }

                        ::selection {
                            background: #BFDBFE;
                            color: #1E3A8A;
                        }

                        h1, h2, h3, h4, h5, h6 {
                            font-family: var(--font-heading);
                            color: var(--slate-900);
                            line-height: 1.25;
                            letter-spacing: -0.02em;
                        }

                        p {
                            color: var(--slate-600);
                        }

                        a {
                            color: inherit;
                            text-decoration: none;
                        }

                        /* Header */
                        header.site-header {
                            background: rgba(255, 255, 255, 0.95);
                            backdrop-filter: blur(12px);
                            -webkit-backdrop-filter: blur(12px);
                            border-bottom: 1px solid var(--slate-200);
                            position: sticky;
                            top: 0;
                            z-index: 100;
                        }
                        .header-inner {
                            max-width: 1140px;
                            margin: 0 auto;
                            padding: 0.85rem 1.25rem;
                            display: flex;
                            justify-content: space-between;
                            align-items: center;
                        }
                        .brand-logo {
                            display: flex;
                            align-items: center;
                            gap: 0.65rem;
                            font-family: var(--font-heading);
                            font-weight: 800;
                            font-size: 1.35rem;
                            color: var(--slate-900);
                            text-decoration: none;
                        }
                        .logo-badge {
                            width: 34px;
                            height: 34px;
                            background: linear-gradient(135deg, #EF4444 0%, #DC2626 100%);
                            border-radius: var(--radius-sm);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: #ffffff;
                            font-size: 0.75rem;
                            font-weight: 800;
                            letter-spacing: -0.05em;
                            box-shadow: 0 2px 6px rgba(220, 38, 38, 0.25);
                        }
                        .logo-text span {
                            color: var(--primary);
                        }
                        nav.site-nav {
                            display: flex;
                            align-items: center;
                            gap: 1.5rem;
                        }
                        nav.site-nav ul {
                            display: flex;
                            list-style: none;
                            gap: 0.5rem;
                            align-items: center;
                        }
                        nav.site-nav a {
                            font-size: 0.9rem;
                            font-weight: 600;
                            color: var(--slate-600);
                            padding: 0.4rem 0.8rem;
                            border-radius: var(--radius-full);
                            transition: all 0.15s ease;
                        }
                        nav.site-nav a:hover {
                            color: var(--primary);
                            background: var(--primary-light);
                        }
                        .nav-badge-pill {
                            display: inline-flex;
                            align-items: center;
                            gap: 0.35rem;
                            font-size: 0.8rem;
                            font-weight: 600;
                            color: var(--success);
                            background: var(--success-light);
                            padding: 0.35rem 0.8rem;
                            border-radius: var(--radius-full);
                            border: 1px solid rgba(16, 185, 129, 0.25);
                            white-space: nowrap;
                        }

                        /* Buttons */
                        .btn {
                            display: inline-flex;
                            align-items: center;
                            justify-content: center;
                            gap: 0.5rem;
                            font-family: var(--font-heading);
                            font-weight: 700;
                            font-size: 1rem;
                            padding: 0.75rem 1.5rem;
                            border-radius: var(--radius-md);
                            border: none;
                            cursor: pointer;
                            transition: all 0.18s cubic-bezier(0.16, 1, 0.3, 1);
                            text-align: center;
                            text-decoration: none;
                            -webkit-tap-highlight-color: transparent;
                        }
                        .btn:active {
                            transform: scale(0.98);
                        }
                        .btn-primary {
                            background: linear-gradient(135deg, var(--primary) 0%, var(--primary-hover) 100%);
                            color: #ffffff;
                            box-shadow: 0 4px 12px rgba(37, 99, 235, 0.25);
                        }
                        .btn-primary:hover {
                            background: var(--primary-hover);
                            box-shadow: 0 6px 18px rgba(37, 99, 235, 0.35);
                            transform: translateY(-1px);
                        }
                        .btn-success {
                            background: linear-gradient(135deg, var(--success) 0%, var(--success-hover) 100%);
                            color: #ffffff;
                            box-shadow: 0 4px 14px rgba(16, 185, 129, 0.25);
                        }
                        .btn-success:hover {
                            background: var(--success-hover);
                            box-shadow: 0 6px 20px rgba(16, 185, 129, 0.35);
                            transform: translateY(-1px);
                        }
                        .btn-secondary {
                            background: var(--slate-100);
                            color: var(--slate-700);
                            border: 1px solid var(--slate-200);
                        }
                        .btn-secondary:hover {
                            background: var(--slate-200);
                            color: var(--slate-900);
                        }
                        .btn-sm {
                            padding: 0.45rem 0.85rem;
                            font-size: 0.85rem;
                            border-radius: var(--radius-sm);
                        }
                        .btn-lg {
                            padding: 0.95rem 1.85rem;
                            font-size: 1.1rem;
                            border-radius: var(--radius-md);
                        }

                        /* Main Content Area */
                        main {
                            flex: 1;
                        }

                        /* Tool Container */
                        .tool-card {
                            background: #ffffff;
                            border: 1px solid var(--slate-200);
                            border-radius: var(--radius-xl);
                            padding: 2.25rem;
                            box-shadow: var(--shadow-xl);
                            margin-bottom: 2.5rem;
                        }

                        /* Dropzone / Upload Area */
                        .dropzone-container {
                            border: 2px dashed var(--slate-300);
                            background: #ffffff;
                            border-radius: var(--radius-lg);
                            padding: 3.25rem 1.5rem;
                            text-align: center;
                            cursor: pointer;
                            transition: all 0.2s ease;
                            -webkit-tap-highlight-color: transparent;
                        }
                        .dropzone-container:hover, .dropzone-container.drag-over {
                            border-color: var(--primary);
                            background: var(--primary-light);
                            box-shadow: 0 0 0 4px var(--primary-ring);
                            transform: translateY(-2px);
                        }
                        .upload-icon-circle {
                            width: 68px;
                            height: 68px;
                            background: linear-gradient(135deg, #EFF6FF 0%, #DBEAFE 100%);
                            border-radius: var(--radius-full);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            margin: 0 auto 1.25rem auto;
                            color: var(--primary);
                            transition: transform 0.2s ease;
                        }
                        .dropzone-container:hover .upload-icon-circle {
                            transform: scale(1.08);
                        }

                        /* Mode Cards Grid */
                        .mode-cards-grid {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
                            gap: 0.85rem;
                        }
                        .mode-card {
                            border: 2px solid var(--slate-200);
                            background: #ffffff;
                            border-radius: var(--radius-md);
                            padding: 1.1rem;
                            cursor: pointer;
                            transition: all 0.2s ease;
                            display: flex;
                            flex-direction: column;
                            gap: 0.3rem;
                            -webkit-tap-highlight-color: transparent;
                        }
                        .mode-card:hover {
                            border-color: var(--slate-400);
                            transform: translateY(-1px);
                            box-shadow: var(--shadow-sm);
                        }
                        .mode-card.selected {
                            border-color: var(--primary);
                            background: #F0F7FF;
                            box-shadow: 0 0 0 3px var(--primary-ring);
                        }
                        .mode-card-header {
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                        }
                        .mode-title {
                            font-family: var(--font-heading);
                            font-weight: 700;
                            font-size: 1rem;
                            color: var(--slate-900);
                        }
                        .mode-card.selected .mode-title {
                            color: var(--primary);
                        }
                        .mode-desc {
                            font-size: 0.82rem;
                            color: var(--slate-500);
                            line-height: 1.4;
                        }
                        .badge-popular {
                            font-size: 0.68rem;
                            font-weight: 700;
                            text-transform: uppercase;
                            letter-spacing: 0.05em;
                            padding: 0.2rem 0.5rem;
                            border-radius: var(--radius-full);
                            background: #DBEAFE;
                            color: var(--primary);
                        }

                        /* Target Size Pills */
                        .target-pill {
                            padding: 0.45rem 1rem;
                            border: 1px solid var(--slate-300);
                            background: #ffffff;
                            border-radius: var(--radius-full);
                            font-family: var(--font-body);
                            font-weight: 600;
                            font-size: 0.85rem;
                            color: var(--slate-700);
                            cursor: pointer;
                            transition: all 0.15s ease;
                        }
                        .target-pill:hover {
                            border-color: var(--primary);
                            color: var(--primary);
                            background: var(--primary-light);
                        }
                        .target-pill.active {
                            background: var(--primary);
                            border-color: var(--primary);
                            color: #ffffff;
                            box-shadow: 0 2px 6px rgba(37, 99, 235, 0.25);
                        }

                        /* File Queue Item */
                        .file-queue-item {
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                            padding: 0.85rem 1.1rem;
                            background: var(--slate-50);
                            border: 1px solid var(--slate-200);
                            border-radius: var(--radius-md);
                            margin-bottom: 0.65rem;
                            transition: all 0.15s ease;
                        }
                        .file-queue-item:hover {
                            background: #ffffff;
                            border-color: var(--slate-300);
                        }

                        /* Realistic Progress System */
                        .progress-card {
                            padding: 3rem 1.5rem;
                            text-align: center;
                        }
                        .progress-spinner {
                            width: 52px;
                            height: 52px;
                            border: 4px solid var(--slate-200);
                            border-top-color: var(--primary);
                            border-radius: 50%;
                            margin: 0 auto 1.5rem auto;
                            animation: spin 0.9s cubic-bezier(0.55, 0.15, 0.45, 0.85) infinite;
                        }
                        @keyframes spin {
                            to { transform: rotate(360deg); }
                        }
                        .progress-headline {
                            font-size: 2.25rem;
                            font-weight: 800;
                            font-family: var(--font-heading);
                            color: var(--slate-900);
                            margin-bottom: 0.25rem;
                            letter-spacing: -0.02em;
                        }
                        .progress-subtext {
                            font-size: 0.95rem;
                            font-weight: 500;
                            color: var(--slate-500);
                            margin-bottom: 1.5rem;
                            min-height: 24px;
                            transition: color 0.2s ease;
                        }
                        .progress-bar-outer {
                            width: 100%;
                            max-width: 460px;
                            height: 12px;
                            background: var(--slate-100);
                            border: 1px solid var(--slate-200);
                            border-radius: var(--radius-full);
                            overflow: hidden;
                            margin: 0 auto;
                            box-shadow: inset 0 1px 2px rgba(0,0,0,0.06);
                        }
                        .progress-bar-fill {
                            height: 100%;
                            width: 0%;
                            background: linear-gradient(90deg, #3B82F6 0%, #2563EB 60%, #10B981 100%);
                            border-radius: var(--radius-full);
                            transition: width 0.12s linear;
                            position: relative;
                            overflow: hidden;
                        }
                        .progress-bar-fill::after {
                            content: '';
                            position: absolute;
                            top: 0; left: 0; bottom: 0; right: 0;
                            background-image: linear-gradient(
                                -45deg,
                                rgba(255, 255, 255, 0.2) 25%,
                                transparent 25%,
                                transparent 50%,
                                rgba(255, 255, 255, 0.2) 50%,
                                rgba(255, 255, 255, 0.2) 75%,
                                transparent 75%,
                                transparent
                            );
                            background-size: 24px 24px;
                            animation: move-stripes 1.2s linear infinite;
                        }
                        @keyframes move-stripes {
                            0% { background-position: 0 0; }
                            100% { background-position: 24px 0; }
                        }
                        .progress-meta {
                            display: flex;
                            justify-content: space-between;
                            max-width: 460px;
                            margin: 0.75rem auto 0 auto;
                            font-size: 0.8rem;
                            font-weight: 600;
                            color: var(--slate-400);
                        }

                        /* Results Section */
                        .savings-stat-badge {
                            display: inline-flex;
                            align-items: center;
                            gap: 0.4rem;
                            background: var(--success-light);
                            color: var(--success);
                            border: 1px solid rgba(16, 185, 129, 0.3);
                            padding: 0.4rem 1rem;
                            border-radius: var(--radius-full);
                            font-weight: 700;
                            font-size: 0.95rem;
                        }

                        /* Footer */
                        footer.site-footer {
                            background: #ffffff;
                            border-top: 1px solid var(--slate-200);
                            padding: 3rem 1.5rem 2rem 1.5rem;
                            margin-top: 5rem;
                        }
                        .footer-inner {
                            max-width: 1140px;
                            margin: 0 auto;
                        }

                        /* Mobile Responsiveness (Phone-First Polish) */
                        @media (max-width: 640px) {
                            .header-inner {
                                padding: 0.75rem 1rem;
                            }
                            nav.site-nav ul {
                                display: none; /* Keep header completely clean and uncrowded on phones */
                            }
                            .nav-badge-pill {
                                font-size: 0.75rem;
                                padding: 0.3rem 0.65rem;
                            }
                            .tool-card {
                                padding: 1.25rem 1rem;
                                border-radius: var(--radius-lg);
                                margin-bottom: 1.75rem;
                            }
                            .dropzone-container {
                                padding: 2rem 0.85rem;
                            }
                            .upload-icon-circle {
                                width: 56px;
                                height: 56px;
                                margin-bottom: 1rem;
                            }
                            .dropzone-container h2 {
                                font-size: 1.35rem !important;
                            }
                            .btn-lg {
                                width: 100%;
                                min-height: 52px;
                                font-size: 1.05rem;
                                padding: 0.85rem 1.25rem;
                            }
                            .mode-cards-grid {
                                grid-template-columns: 1fr; /* Stacks cleanly on phones for effortless thumb tapping */
                                gap: 0.65rem;
                            }
                            .mode-card {
                                padding: 0.95rem 1rem;
                            }
                            .file-queue-item {
                                padding: 0.75rem 0.85rem;
                            }
                            .progress-card {
                                padding: 2rem 0.5rem;
                            }
                            .progress-headline {
                                font-size: 1.85rem;
                            }
                            .progress-subtext {
                                font-size: 0.85rem;
                            }
                            #single-download-btn, #zip-download-btn, #restart-btn {
                                width: 100% !important;
                                min-height: 50px;
                            }
                        }
                    "#))
                }
            }
            body {
                // Header Navigation
                header class="site-header" role="banner" {
                    div class="header-inner" {
                        a href="/" class="brand-logo" {
                            div class="logo-badge" { "PDF" }
                            div class="logo-text" {
                                "Fit" span { "KB" }
                            }
                        }
                        nav class="site-nav" role="navigation" {
                            ul {
                                li { a href="/#how-it-works" { "How It Works" } }
                                li { a href="/#features" { "Features" } }
                                li { a href="/#faq" { "FAQ" } }
                            }
                            span class="nav-badge-pill" {
                                "🔒 100% Free & Private"
                            }
                        }
                    }
                }

                // Main Content
                main role="main" {
                    (content)
                }

                // Friendly Clean Footer
                footer class="site-footer" role="contentinfo" {
                    div class="footer-inner" {
                        div style="display:flex; flex-wrap:wrap; justify-content:space-between; align-items:center; gap:1.5rem; margin-bottom:2rem;" {
                            div {
                                div class="brand-logo" style="margin-bottom:0.4rem;" {
                                    div class="logo-badge" { "PDF" }
                                    div class="logo-text" { "Fit" span { "KB" } }
                                }
                                p style="font-size:0.88rem; color:var(--slate-500); max-width:320px;" {
                                    "Fast, private in-memory PDF compression engine built with Rust. No tracking, no storage, no account required."
                                }
                            }
                            div style="display:flex; flex-wrap:wrap; gap:1.5rem; font-size:0.9rem; font-weight:600; color:var(--slate-600);" {
                                a href="/#how-it-works" { "How It Works" }
                                a href="/#features" { "Features" }
                                a href="/#faq" { "FAQ" }
                                a href="/sitemap.xml" { "Sitemap" }
                                a href="/robots.txt" { "Robots.txt" }
                            }
                        }
                        div style="border-top:1px solid var(--slate-200); padding-top:1.5rem; display:flex; flex-wrap:wrap; justify-content:space-between; align-items:center; gap:0.75rem; font-size:0.85rem; color:var(--slate-400);" {
                            div { "© 2026 FitKB. Completely free and secure PDF utility." }
                            div { "🔒 Zero file retention • Purged immediately upon download" }
                        }
                    }
                }

                // Interactive JavaScript Controller with REALISTIC PROGRESS
                script {
                    (maud::PreEscaped(format!(r#"
                        const preselectedTarget = {};
                        let selectedFiles = [];
                        let selectedMode = preselectedTarget > 0 ? 'target' : 'recommended';
                        let targetSizeKb = preselectedTarget > 0 ? preselectedTarget : 200;

                        const dropzone = document.getElementById('dropzone');
                        const fileInput = document.getElementById('file-input');
                        const fileQueueSection = document.getElementById('file-queue-section');
                        const fileListContainer = document.getElementById('file-list-container');
                        const uploadArea = document.getElementById('upload-area');
                        const compressBtn = document.getElementById('compress-btn');
                        const addMoreBtn = document.getElementById('add-more-btn');

                        const progressStage = document.getElementById('progress-stage');
                        const stageLabel = document.getElementById('stage-label');
                        const progressBar = document.getElementById('progress-bar');
                        const progressPercent = document.getElementById('progress-percent');
                        const progressByteMeta = document.getElementById('progress-byte-meta');
                        const progressSpeedMeta = document.getElementById('progress-speed-meta');

                        const resultStage = document.getElementById('result-stage');
                        const resultHeadline = document.getElementById('result-headline');
                        const singleDownloadBtn = document.getElementById('single-download-btn');
                        const zipDownloadBtn = document.getElementById('zip-download-btn');
                        const restartBtn = document.getElementById('restart-btn');
                        const resultList = document.getElementById('result-list');
                        const errorNotice = document.getElementById('error-notice');
                        const errorMessage = document.getElementById('error-message');

                        const cardExtreme = document.getElementById('card-extreme');
                        const cardRecommended = document.getElementById('card-recommended');
                        const cardLess = document.getElementById('card-less');
                        const cardTarget = document.getElementById('card-target');
                        const targetControls = document.getElementById('target-controls');
                        const customTargetInput = document.getElementById('custom-target-input');

                        let activeTickerInterval = null;
                        let currentProgress = 0;

                        function updateModeCards() {{
                            [cardExtreme, cardRecommended, cardLess, cardTarget].forEach(c => {{
                                if (c) c.classList.remove('selected');
                            }});

                            if (selectedMode === 'extreme' && cardExtreme) cardExtreme.classList.add('selected');
                            if (selectedMode === 'recommended' && cardRecommended) cardRecommended.classList.add('selected');
                            if (selectedMode === 'less' && cardLess) cardLess.classList.add('selected');
                            if (selectedMode === 'target' && cardTarget) {{
                                cardTarget.classList.add('selected');
                                if (targetControls) targetControls.style.display = 'block';
                            }} else if (targetControls) {{
                                targetControls.style.display = 'none';
                            }}
                        }}

                        if (cardExtreme) cardExtreme.addEventListener('click', () => {{ selectedMode = 'extreme'; updateModeCards(); }});
                        if (cardRecommended) cardRecommended.addEventListener('click', () => {{ selectedMode = 'recommended'; updateModeCards(); }});
                        if (cardLess) cardLess.addEventListener('click', () => {{ selectedMode = 'less'; updateModeCards(); }});
                        if (cardTarget) cardTarget.addEventListener('click', () => {{ selectedMode = 'target'; updateModeCards(); }});

                        document.querySelectorAll('.target-pill').forEach(pill => {{
                            pill.addEventListener('click', () => {{
                                document.querySelectorAll('.target-pill').forEach(p => p.classList.remove('active'));
                                pill.classList.add('active');
                                targetSizeKb = parseInt(pill.dataset.target);
                                if (customTargetInput) customTargetInput.value = '';
                            }});
                        }});

                        if (customTargetInput) {{
                            customTargetInput.addEventListener('input', (e) => {{
                                const val = parseInt(e.target.value);
                                if (val > 0) {{
                                    targetSizeKb = val;
                                    document.querySelectorAll('.target-pill').forEach(p => p.classList.remove('active'));
                                }}
                            }});
                        }}

                        if (dropzone && fileInput) {{
                            dropzone.addEventListener('click', () => fileInput.click());
                            if (addMoreBtn) addMoreBtn.addEventListener('click', () => fileInput.click());

                            fileInput.addEventListener('change', (e) => {{
                                if (e.target.files) addFiles(Array.from(e.target.files));
                            }});

                            dropzone.addEventListener('dragover', (e) => {{
                                e.preventDefault();
                                dropzone.classList.add('drag-over');
                            }});
                            dropzone.addEventListener('dragleave', () => {{
                                dropzone.classList.remove('drag-over');
                            }});
                            dropzone.addEventListener('drop', (e) => {{
                                e.preventDefault();
                                dropzone.classList.remove('drag-over');
                                if (e.dataTransfer.files) addFiles(Array.from(e.dataTransfer.files));
                            }});

                            window.addEventListener('paste', (e) => {{
                                if (e.clipboardData && e.clipboardData.files.length > 0) {{
                                    addFiles(Array.from(e.clipboardData.files));
                                }}
                            }});
                        }}

                        function formatBytes(bytes) {{
                            if (!bytes || bytes === 0) return '0 Bytes';
                            const k = 1024;
                            if (bytes < k * 1024) {{
                                return (bytes / k).toFixed(1) + ' KB';
                            }}
                            return (bytes / (k * k)).toFixed(2) + ' MB';
                        }}

                        function addFiles(files) {{
                            if (errorNotice) errorNotice.style.display = 'none';
                            for (const f of files) {{
                                if (!f.name.toLowerCase().endsWith('.pdf')) {{
                                    showError(`"${{f.name}}" is not a PDF file. Please select PDF documents.`);
                                    return;
                                }}
                                if (f.size > 50 * 1024 * 1024) {{
                                    showError(`"${{f.name}}" exceeds the 50 MB limit.`);
                                    return;
                                }}
                                selectedFiles.push(f);
                            }}
                            renderFileQueue();
                        }}

                        function showError(msg) {{
                            if (errorNotice && errorMessage) {{
                                errorNotice.style.display = 'block';
                                errorMessage.textContent = msg;
                            }}
                        }}

                        function renderFileQueue() {{
                            if (selectedFiles.length === 0) {{
                                if (fileQueueSection) fileQueueSection.style.display = 'none';
                                if (uploadArea) uploadArea.style.display = 'block';
                                return;
                            }}

                            if (uploadArea) uploadArea.style.display = 'none';
                            if (fileQueueSection) fileQueueSection.style.display = 'block';
                            if (fileListContainer) {{
                                fileListContainer.innerHTML = '';
                                selectedFiles.forEach((file, idx) => {{
                                    const item = document.createElement('div');
                                    item.className = 'file-queue-item';
                                    item.innerHTML = `
                                        <div style="display:flex; align-items:center; gap:0.75rem; min-width:0; flex:1;">
                                            <div style="width:34px; height:34px; background:#FEE2E2; border-radius:6px; display:flex; align-items:center; justify-content:center; color:#DC2626; font-weight:700; font-size:0.7rem; flex-shrink:0;">
                                                PDF
                                            </div>
                                            <div style="min-width:0; flex:1;">
                                                <div style="font-weight:600; font-size:0.92rem; color:var(--slate-800); white-space:nowrap; overflow:hidden; text-overflow:ellipsis;">${{file.name}}</div>
                                                <div style="font-size:0.78rem; color:var(--slate-500);">${{formatBytes(file.size)}}</div>
                                            </div>
                                        </div>
                                        <button type="button" class="btn btn-secondary btn-sm" onclick="removeFile(${{idx}})" title="Remove file" style="color:var(--slate-500); width:32px; height:32px; padding:0; flex-shrink:0;">
                                            ✕
                                        </button>
                                    `;
                                    fileListContainer.appendChild(item);
                                }});
                            }}

                            if (compressBtn) {{
                                compressBtn.textContent = selectedFiles.length === 1 
                                    ? 'Compress PDF →' 
                                    : `Compress ${{selectedFiles.length}} PDFs →`;
                            }}
                        }}

                        window.removeFile = (idx) => {{
                            selectedFiles.splice(idx, 1);
                            renderFileQueue();
                        }};

                        if (restartBtn) {{
                            restartBtn.addEventListener('click', () => {{
                                selectedFiles = [];
                                if (resultStage) resultStage.style.display = 'none';
                                if (uploadArea) uploadArea.style.display = 'block';
                                if (fileInput) fileInput.value = '';
                                if (activeTickerInterval) clearInterval(activeTickerInterval);
                            }});
                        }}

                        function setProgress(pct, label, metaLeft, metaRight) {{
                            currentProgress = Math.max(currentProgress, pct);
                            if (progressBar) progressBar.style.width = `${{currentProgress}}%`;
                            if (progressPercent) progressPercent.textContent = `${{Math.round(currentProgress)}}%`;
                            if (label && stageLabel) stageLabel.textContent = label;
                            if (metaLeft && progressByteMeta) progressByteMeta.textContent = metaLeft;
                            if (metaRight && progressSpeedMeta) progressSpeedMeta.textContent = metaRight;
                        }}

                        if (compressBtn) {{
                            compressBtn.addEventListener('click', () => {{
                                if (selectedFiles.length === 0) return;

                                if (fileQueueSection) fileQueueSection.style.display = 'none';
                                if (progressStage) progressStage.style.display = 'block';

                                currentProgress = 0;
                                setProgress(1, 'Establishing secure in-memory buffer...', '0 Bytes uploaded', 'Stage: Upload');

                                const formData = new FormData();
                                let totalUploadSize = 0;
                                selectedFiles.forEach(file => {{
                                    formData.append('pdf', file);
                                    totalUploadSize += file.size;
                                }});
                                formData.append('mode', selectedMode);
                                if (selectedMode === 'target') {{
                                    formData.append('target_kb', targetSizeKb);
                                }}

                                const xhr = new XMLHttpRequest();
                                xhr.open('POST', '/api/compress-v2', true);

                                // 1. Real Upload Phase: 0% to 45% based on actual bytes sent
                                const uploadStartTime = Date.now();
                                xhr.upload.addEventListener('progress', (e) => {{
                                    if (e.lengthComputable && e.total > 0) {{
                                        const uploadPct = (e.loaded / e.total) * 45;
                                        const elapsedSec = (Date.now() - uploadStartTime) / 1000;
                                        const speedBytesSec = elapsedSec > 0 ? (e.loaded / elapsedSec) : 0;
                                        const speedStr = speedBytesSec > 0 ? `${{formatBytes(speedBytesSec)}}/s` : '';
                                        
                                        setProgress(
                                            uploadPct,
                                            `Uploading document (${{formatBytes(e.loaded)}} of ${{formatBytes(e.total)}})...`,
                                            `${{formatBytes(e.loaded)}} / ${{formatBytes(e.total)}}`,
                                            speedStr ? `Speed: ${{speedStr}}` : 'Uploading'
                                        );
                                    }}
                                }});

                                // 2. Real Server Optimization Phase: 45% to 95%
                                xhr.upload.addEventListener('load', () => {{
                                    setProgress(46, 'Stream received. Analyzing PDF structure...', 'All bytes received', 'Optimizing');
                                    startCompressionTicker();
                                }});

                                function startCompressionTicker() {{
                                    if (activeTickerInterval) clearInterval(activeTickerInterval);
                                    
                                    // Smoothly advances with organic increments and descriptive messages
                                    activeTickerInterval = setInterval(() => {{
                                        if (currentProgress < 95) {{
                                            // As it gets higher, increments naturally taper for realistic feel
                                            const step = currentProgress < 65 ? (0.6 + Math.random() * 0.9) : (currentProgress < 85 ? (0.4 + Math.random() * 0.6) : (0.15 + Math.random() * 0.3));
                                            const nextVal = Math.min(95, currentProgress + step);

                                            let message = 'Optimizing PDF streams...';
                                            if (nextVal < 58) {{
                                                message = 'Analyzing object streams & font tables...';
                                            }} else if (nextVal < 72) {{
                                                message = 'Downsampling raster images to target DPI...';
                                            }} else if (nextVal < 86) {{
                                                message = 'Re-encoding DCT streams & deduplicating XObjects...';
                                            }} else {{
                                                message = 'Stripping uncompressed metadata & finalizing...';
                                            }}

                                            setProgress(nextVal, message, 'RAM Processing Active', 'Rust Engine');
                                        }}
                                    }}, 100);
                                }}

                                xhr.onload = () => {{
                                    if (activeTickerInterval) clearInterval(activeTickerInterval);

                                    if (xhr.status >= 200 && xhr.status < 300) {{
                                        setProgress(100, 'Optimization complete! Loading result...', 'Finished', '100%');
                                        try {{
                                            const data = JSON.parse(xhr.responseText);
                                            setTimeout(() => showResults(data), 250);
                                        }} catch (e) {{
                                            showError('Invalid server response format.');
                                            if (progressStage) progressStage.style.display = 'none';
                                            if (fileQueueSection) fileQueueSection.style.display = 'block';
                                        }}
                                    }} else {{
                                        if (progressStage) progressStage.style.display = 'none';
                                        if (fileQueueSection) fileQueueSection.style.display = 'block';
                                        showError(xhr.responseText || 'Compression encountered an error. Please try again.');
                                    }}
                                }};

                                xhr.onerror = () => {{
                                    if (activeTickerInterval) clearInterval(activeTickerInterval);
                                    if (progressStage) progressStage.style.display = 'none';
                                    if (fileQueueSection) fileQueueSection.style.display = 'block';
                                    showError('Network error connecting to the compression engine.');
                                }};

                                xhr.send(formData);
                            }});
                        }}

                        function showResults(data) {{
                            if (progressStage) progressStage.style.display = 'none';
                            if (resultStage) resultStage.style.display = 'block';

                            const totalOriginal = data.total_original_bytes;
                            const totalCompressed = data.total_compressed_bytes;
                            const totalSavedPercent = data.total_saved_percent;

                            if (resultHeadline) {{
                                resultHeadline.textContent = `${{totalSavedPercent}}% Smaller`;
                            }}

                            const sub = document.getElementById('result-subtitle');
                            if (sub) {{
                                sub.textContent = `${{formatBytes(totalOriginal)}}  ➔  ${{formatBytes(totalCompressed)}}`;
                            }}

                            if (resultList) {{
                                resultList.innerHTML = '';
                                data.files.forEach(item => {{
                                    const row = document.createElement('div');
                                    row.className = 'file-queue-item';
                                    row.style.background = '#ffffff';
                                    row.style.padding = '1.1rem';
                                    row.style.flexWrap = 'wrap';
                                    row.style.gap = '0.75rem';
                                    row.innerHTML = `
                                        <div style="display:flex; align-items:center; gap:0.85rem; min-width:0; flex:1;">
                                            <div style="width:38px; height:38px; background:#DCFCE7; border-radius:8px; display:flex; align-items:center; justify-content:center; color:#16A34A; font-weight:700; font-size:0.75rem; flex-shrink:0;">
                                                ✓
                                            </div>
                                            <div style="min-width:0; flex:1;">
                                                <div style="font-weight:700; font-size:0.95rem; color:var(--slate-800); white-space:nowrap; overflow:hidden; text-overflow:ellipsis;">${{item.original_name}}</div>
                                                <div style="font-size:0.82rem; color:var(--slate-500); margin-top:2px;">
                                                    ${{formatBytes(item.original_size)}} ➔ <strong style="color:#16A34A;">${{formatBytes(item.compressed_size)}}</strong> 
                                                    <span style="color:#16A34A; font-weight:600; margin-left:4px;">(-${{item.percent_saved}}%)</span>
                                                </div>
                                                ${{item.message ? `<div style="font-size:0.78rem; color:#D97706; margin-top:3px;">${{item.message}}</div>` : ''}}
                                            </div>
                                        </div>
                                        <a href="${{item.download_url}}" download="${{item.original_name}}" class="btn btn-success btn-sm" style="white-space:nowrap;">
                                            Download ↓
                                        </a>
                                    `;
                                    resultList.appendChild(row);
                                }});
                            }}

                            if (data.files.length === 1) {{
                                singleDownloadBtn.href = data.files[0].download_url;
                                singleDownloadBtn.download = data.files[0].original_name;
                                singleDownloadBtn.style.display = 'inline-flex';
                                zipDownloadBtn.style.display = 'none';
                            }} else {{
                                zipDownloadBtn.href = data.zip_download_url;
                                zipDownloadBtn.style.display = 'inline-flex';
                                singleDownloadBtn.style.display = 'none';
                            }}
                        }}

                        updateModeCards();
                    "#, target_preset)))
                }
            }
        }
    }
}
