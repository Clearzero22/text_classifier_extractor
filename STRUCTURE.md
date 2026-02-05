# Project Structure Map

## Directory Tree

```
text_classifier_extractor/
├── .git/                    # Git repository
├── target/                  # Build artifacts (ignored)
│
├── .env                     # Environment variables (local, not committed)
├── .env.example             # Environment template
├── .gitignore               # Git exclusions
├── Cargo.lock               # Dependency lock file
├── Cargo.toml               # Project manifest
├── CLAUDE.md                # ⬅️ Root AI context (NEW)
│
└── src/
    ├── main.rs              # Entry point & all logic
    └── CLAUDE.md            # ⬅️ Module AI context (NEW)
```

## Module Relationships

```
┌─────────────────────────────────────────────────────────┐
│                    Root Context                          │
│                    CLAUDE.md                             │
│                  (377 lines)                             │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ├──► Project Vision
                        ├──► Architecture Overview
                        ├──► Technology Stack
                        ├──► Module Structure
                        ├──► Development Standards
                        ├──► API Integration Details
                        └── Mermaid Diagram
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│                  Module Context                          │
│                  src/CLAUDE.md                           │
│                  (296 lines)                             │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ├──► Data Model Documentation
                        ├──► Function Signatures
                        ├──► Active vs Commented Code
                        ├──► Testing Strategy
                        ├──► Refactoring Opportunities
                        └── Navigation Breadcrumbs
```

## Code Structure (main.rs)

```
src/main.rs (54 lines)
│
├── [1] Imports (lines 1-11)
│   ├── anyhow::Result
│   ├── rig::completion::Prompt
│   ├── rig::providers::openai
│   ├── schemars::JsonSchema
│   └── serde traits
│
├── [2] Data Models (lines 6-17)
│   ├── Sentiment (enum)
│   │   ├── Positive
│   │   ├── Negative
│   │   └── Neutral
│   └── SentimentClassification (struct)
│       ├── sentiment: Sentiment
│       └── confidence: f32
│
└── [3] Main Function (lines 19-53)
    ├── Environment setup (line 21)
    ├── API client init (lines 23-27)
    ├── Agent builder (lines 29-32) [ACTIVE]
    ├── Prompt execution (lines 34-36) [ACTIVE]
    └── Extractor builder (lines 40-50) [COMMENTED]
```

## Dependency Graph

```
┌─────────────────────────────────────────────────────────┐
│                   text_classifier_extractor              │
│                      (Cargo.toml)                        │
└───────┬─────────────┬─────────────┬─────────────┬───────┘
        │             │             │             │
        ▼             ▼             ▼             ▼
    ┌────────┐   ┌────────┐   ┌────────┐   ┌────────┐
    │rig-core│   │ tokio  │   │ serde  │   │anyhow  │
    │ 0.11.1 │   │ 1.34.0 │   │  1.0   │   │ 1.0.75 │
    └────┬───┘   └────┬───┘   └────┬───┘   └────┬───┘
         │            │            │            │
         ▼            ▼            ▼            ▼
    ┌────────────────────────────────────────────────┐
    │              External Services                  │
    │  ┌──────────────────────────────────────────┐  │
    │  │  Zhipu AI GLM-4.7 API                    │  │
    │  │  Endpoint: open.bigmodel.cn              │  │
    │  └──────────────────────────────────────────┘  │
    └────────────────────────────────────────────────┘
```

## Data Flow Diagram

```
┌─────────┐     ┌──────────┐     ┌─────────────┐
│  .env   │────▶│  Config  │────▶│ API Client  │
│ File    │     │ Loading  │     │ Initialization│
└─────────┘     └──────────┘     └──────┬───────┘
                                         │
                                         ▼
                                  ┌─────────────┐
                                  │   Agent     │
                                  │   Builder   │
                                  └──────┬──────┘
                                         │
                                         ▼
                                  ┌─────────────┐
                                  │   Prompt    │
                                  │ Execution   │
                                  └──────┬──────┘
                                         │
                                         ▼
                                  ┌─────────────┐
                                  │  Response   │
                                  │   Output    │
                                  └─────────────┘
```

## File Sizes & Metrics

| File | Lines | Purpose | Status |
|------|-------|---------|--------|
| `CLAUDE.md` | 377 | Root AI context | ✅ Created |
| `src/CLAUDE.md` | 296 | Module AI context | ✅ Created |
| `src/main.rs` | 54 | Application code | 📝 Active |
| `Cargo.toml` | 14 | Dependencies | ✅ Configured |
| `.env.example` | 9 | Environment template | ✅ Present |
| `.gitignore` | 4 | Git exclusions | ✅ Minimal |

## Documentation Coverage

```
Coverage Assessment
═══════════════════════════════════════════════════

✅ Root Documentation
   ├── Project vision           [COMPLETE]
   ├── Architecture overview    [COMPLETE]
   ├── Technology stack         [COMPLETE]
   ├── Module structure         [COMPLETE]
   ├── Development standards    [COMPLETE]
   ├── API integration details  [COMPLETE]
   └── Mermaid diagram          [COMPLETE]

✅ Module Documentation
   ├── Interface documentation  [COMPLETE]
   ├── Dependency mapping       [COMPLETE]
   ├── Entry points             [COMPLETE]
   ├── Testing strategy         [COMPLETE]
   ├── Code structure           [COMPLETE]
   └── Navigation breadcrumbs   [COMPLETE]

✅ Configuration
   ├── Environment template     [COMPLETE]
   └── Git exclusions           [COMPLETE]

⚠️  Code Quality (Needs Work)
   ├── Unit tests               [MISSING]
   ├── Integration tests        [MISSING]
   ├── Documentation comments   [MISSING]
   └── Error handling           [BASIC]
```

## Navigation Map

```
From Root (CLAUDE.md):
│
├──► Module Documentation
│   └── src/CLAUDE.md
│       ├── Data Models
│       ├── Interfaces
│       ├── Dependencies
│       └── Code Structure
│
├──► Configuration Files
│   ├── Cargo.toml
│   ├── .env.example
│   └── .gitignore
│
├──► Source Code
│   └── src/main.rs
│
└──► External Resources
    ├── rig-core docs
    ├── Zhipu AI platform
    └── serde/schemars docs
```

## Next Steps Recommendations

1. **Immediate (Priority 1)**
   - [ ] Uncomment and test structured extraction
   - [ ] Add basic unit tests for data models
   - [ ] Fix typo in agent preamble

2. **Short-term (Priority 2)**
   - [ ] Modularize main.rs into separate files
   - [ ] Add integration tests with mock API
   - [ ] Implement proper error handling

3. **Long-term (Priority 3)**
   - [ ] Add CLI argument parsing
   - [ ] Implement batch processing
   - [ ] Add logging and metrics

---

**Generated**: 2026-02-06
**Project**: text_classifier_extractor v0.1.0
**Status**: Early Development
