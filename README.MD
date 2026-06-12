# Nigerian Developer Survey — High-Level Technical Spec

---

## What this document is

A high-level technical spec for a web-based annual developer survey platform targeting the Nigerian software engineering community. It covers system purpose, architecture shape, core entities, primary flows, and the technology stack. It does not cover implementation detail — that lives in the lower-level spec.

---

## Problem statement

No annual, publicly available, Nigeria-specific developer survey exists. The Stack Overflow Developer Survey covers global respondents but misses the structural realities of the Nigerian developer ecosystem: naira-denominated compensation, remote work with foreign clients, NYSC/SIWES as career entry points, infrastructure constraints (power, internet), and the Lagos-vs-everywhere-else dynamic. This platform exists to produce that data.

**Secondary goal:** This is a learning project for the Rust programming language. The backend API is written in Axum. Design decisions favour explicitness and learning surface over abstraction.

---

## Scope

### In scope (v1)

- Survey submission by respondents (name and email collected, no account required)
- Partial submission — answers saved per question as the respondent progresses
- Resume from where they left off via email lookup
- Admin-controlled survey lifecycle (draft → active → closed → results published)
- Survey structure: surveys are organised into sections, sections contain questions
- Question management (create, order, type, required/optional) within sections
- Per-question response storage tied to a user
- Structured user profile built from static question-to-field mappings
- Aggregate results view (admin)
- Public results page (after admin publishes)
- Enum definitions served via API so the frontend never hardcodes display values
- API-only backend (Axum + PostgreSQL)
- Separate frontend (JavaScript-based, likely React)

### Out of scope (v1)

- User accounts or passwords for respondents
- Email sending of any kind (magic links, confirmations, newsletters)
- Respondent-facing results during an active survey
- Multi-language support
- Embeddable widgets
- Data export (CSV/PDF) — `Future Consideration`
- Sponsor or partner management — `Future Consideration`
- Multi-admin support — `Future Consideration`

---

## Users

There are two roles in the system.

A **User** is any developer who participates in the survey. They are identified by name and email. No password or account is required. Their email is the key used to resume a partial submission. Certain answers they give automatically populate a structured profile attached to their record. The name captured on first entry is kept as-is — if the same email returns with a different name, the original name is not overwritten.

An **Admin** is the survey operator. They manage sections and questions, control the survey lifecycle, view results, and decide when results are published publicly. There is a single admin in v1.

---

## System architecture

The system has three logical layers:

```
[ Frontend SPA ]  ──────►  [ Axum REST API ]  ──────►  [ PostgreSQL ]
                                   │
                              [ Auth middleware ]
                              (admin routes only)
```

- The frontend is a separate repository and deployment. It consumes the public API for survey participation and the authenticated API for admin operations.
- The API is stateless. No server-side session. Admin authentication uses short-lived access tokens and long-lived refresh tokens.
- The database is PostgreSQL.
- No message queue, no background jobs, no caching layer in v1.

---

## Core entities

### User

Represents a survey respondent. Identified by email, which must be unique across the system. Holds a first name and last name. Also carries a structured profile that is populated automatically as the user answers designated profile-building questions. The profile field mappings are static — defined at build time, not configurable by the admin.

Profile fields stored on the User record: primary job role, Nigerian state, age range, years of experience range, gender, highest education level, employment type, primary programming language, work arrangement, NYSC status, and whether they earn in a foreign currency. Keyed fields store the short code (e.g. `LA`, `BE`); direct fields store the snake_case value (e.g. `remote`, `completed`). Display names are derived at the application layer, never stored.

Age and years of experience are collected as ranges rather than exact values, for both privacy and aggregation utility.

---

### Survey

Represents one edition of the annual survey (e.g. "Nigeria Developer Survey 2026"). Has a title, a URL-safe slug, and a lifecycle status. Carries `opens_at` and `closes_at` timestamps. Also carries a `results_published` boolean, which is independent of the lifecycle status — a survey can be closed without its results being public. Results can only be published once the survey is closed.

**Status lifecycle:**

```
draft → active → closed
                   └──► results_published: false (default)
                   └──► results_published: true (separate admin action)
```

Status transitions are one-directional. A closed survey cannot be reopened.

---

### Section

Sits between Survey and Question. Groups questions under a named heading (e.g. "Technologies", "Salaries", "Demographics"). Has a title, a display order, and belongs to a survey. Sections cannot be added, edited, or deleted once the survey is `active`.

---

### Question

Belongs to a Section and, by denormalisation, also carries a direct `survey_id` for query convenience. Has a type (`single_choice`, `multi_choice`, `text`, `scale`), display order within its section, required flag, and — for choice-based types — a list of options stored as JSONB. For questions that feed the user profile, the mapping to the target profile field is defined in code, not in the database.

Questions cannot be edited or deleted once the survey is `active`. The question with the highest `order` value across the entire survey is implicitly the last question. When a response is submitted for that question, the backend marks the UserSurvey as `complete` automatically — no separate completion call is needed from the frontend.

---

### Response

Represents one user's answer to one question in one survey. This is the atomic unit of answer storage. A user who answers a 40-question survey produces up to 40 `Response` rows.

Each Response is tied to a User, a Survey, and a Question. The answer is stored as JSONB to accommodate all question types in a single column: a string for `single_choice` and `text`, an array of strings for `multi_choice`, and an integer for `scale`.

When a Response is created or updated for a question that maps to a profile field, the corresponding field on the User record is also updated in the same database transaction.

---

### UserSurvey

The join between a User and a Survey. Tracks the state of a user's participation in a specific survey. Created when the user submits their first answer. A user can have one UserSurvey row per survey edition.

Holds a `status` (`in_progress` or `complete`), a `current_question_order` integer used to resume the user at the correct question, and `started_at` and `completed_at` timestamps. The `current_question_order` tracks individual questions, not sections — the current section is derived from the current question.

---

### Admin

Single operator account for v1. Holds a username and an Argon2id password hash. Authentication issues a short-lived access token (1 hour TTL) and a long-lived refresh token (30 days TTL). The refresh token is stored hashed server-side and rotated on every use.

---

## Enum types

The following domain values are defined as Rust enums and served to the frontend via `/meta/enums` so the frontend never hardcodes display values. Two storage strategies are used depending on value length and stability.

Where values are long or their display names could drift from their stored form, a short stable key is stored in the database and the display name is derived at the application layer. Where values are already short and stable, the value is stored directly in snake_case.

### Keyed enums (short key stored in DB, display name derived in code)

| Enum | Sample keys |
|---|---|
| `NigerianState` | `LA` (Lagos), `RV` (Rivers), `FCT` (Abuja), `KN` (Kano), … all 36 states + FCT |
| `JobRole` | `BE` (Backend Engineer), `FE` (Frontend Engineer), `FS` (Fullstack Engineer), `DE` (DevOps / Infrastructure), … |
| `PrimaryLanguage` | `JS` (JavaScript / TypeScript), `PY` (Python), `RU` (Rust), `GO` (Go), … |

### Direct enums (snake_case value stored as-is in DB)

| Enum | Sample values |
|---|---|
| `AgeRange` | `under_18`, `18_24`, `25_34`, `35_44`, … |
| `ExperienceRange` | `0_1`, `1_3`, `3_5`, `5_10`, `10_plus` |
| `Gender` | `male`, `female`, `non_binary`, `prefer_not_to_say` |
| `EducationLevel` | `nd_nce`, `hnd`, `bsc`, `msc`, `phd`, `self_taught`, `other` |
| `EmploymentType` | `full_time`, `part_time`, `freelance`, `founder`, `student`, `unemployed`, `other` |
| `WorkArrangement` | `remote`, `hybrid`, `on_site` |
| `NyscStatus` | `completed`, `in_progress`, `exempted`, `not_applicable` |
| `EarningCurrency` | `naira_only`, `foreign_only`, `both` |

Full enum definitions with all keys and display names are in the lower-level spec.

---

## Primary flows

### 1. User starts a survey

1. User navigates to the survey page.
2. Frontend calls `GET /surveys/:slug` — returns survey metadata, sections, and questions.
3. Frontend calls `GET /meta/enums` — returns all enum definitions for populating dropdowns.
4. User enters their name and email.
5. Frontend calls `POST /surveys/:slug/start` with name and email.
6. Server checks: if a User with that email exists and has a UserSurvey row for this survey, return their `current_question_order` and all existing responses. If not, create the User (or locate existing user by email without updating their name) and create a UserSurvey row with status `in_progress`.
7. Frontend routes the user to the correct question.

**Failure cases:**
- Survey is not active → 403 with reason.
- Email format invalid → 422.

---

### 2. User answers a question

1. User answers a question and moves to the next one.
2. Frontend calls `POST /surveys/:slug/responses` with `user_id`, `question_id`, and answer value.
3. Server validates: survey is active, user has an `in_progress` UserSurvey for this survey, question belongs to this survey, answer value is valid for the question type.
4. Server upserts the Response row (insert, or update if the user is revising a previous answer).
5. If the question maps to a profile field, the corresponding User profile field is updated in the same transaction.
6. Server updates `current_question_order` on the UserSurvey row.
7. If the question is the last in the survey (highest `order` value), the server sets UserSurvey status to `complete` and sets `completed_at`.
8. Server returns 200.

**Failure cases:**
- Survey not active → 403.
- Question not part of this survey → 422.
- Answer value invalid for question type → 422.
- User has no UserSurvey row for this survey → 404.

---

### 3. User resumes a survey

1. User returns to the survey page and enters their email.
2. Frontend calls `POST /surveys/:slug/start` with email.
3. Server finds the existing User and UserSurvey row. Returns `current_question_order` and all existing responses so the frontend can pre-populate answers.
4. Frontend routes the user to their last question.

---

### 4. Admin manages a survey

1. Admin calls `POST /admin/auth/login` with credentials → receives access token (1hr) and refresh token (30 days).
2. Admin uses the access token as a Bearer token on all `/admin/*` routes.
3. When the access token expires, admin calls `POST /admin/auth/refresh` with the refresh token → receives a new access token and a rotated refresh token.
4. Admin creates a survey, adds sections, adds questions within sections, and publishes via status transition.
5. Admin closes the survey manually or it closes automatically when `closes_at` is reached.
6. After analysis, admin calls `PATCH /admin/surveys/:id/publish-results` → sets `results_published = true`.

**Failure cases:**
- Invalid credentials → 401.
- Expired or invalid access token → 401.
- Expired or invalid refresh token → 401 (forces re-login).
- Invalid status transition → 422.
- Publishing a survey with no questions → 422.
- Publishing results on a survey that is not closed → 422.

---

### 5. Admin views results

1. Admin calls `GET /admin/surveys/:id/results`.
2. Server returns aggregated response data per question, available regardless of survey status.
3. Aggregation per type: `single_choice` and `multi_choice` return option counts and percentages. `text` returns paginated raw responses. `scale` returns mean, median, and distribution.

---

### 6. Public results page

1. Once `results_published = true`, `GET /surveys/:slug/results` returns aggregate results without auth.
2. While `results_published = false`, this endpoint returns 404.

---

## API surface (high-level)

### Public endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/meta/enums` | Return all enum definitions for frontend use |
| GET | `/surveys/:slug` | Fetch survey metadata, sections, and questions |
| POST | `/surveys/:slug/start` | Start or resume a survey (creates or looks up user) |
| POST | `/surveys/:slug/responses` | Submit or update a single answer |
| GET | `/surveys/:slug/results` | Fetch public results (published surveys only) |

### Admin endpoints (access token required)

| Method | Path | Description |
|---|---|---|
| POST | `/admin/auth/login` | Authenticate, receive access and refresh tokens |
| POST | `/admin/auth/refresh` | Rotate access token using refresh token |
| GET | `/admin/surveys` | List all surveys |
| POST | `/admin/surveys` | Create a survey |
| GET | `/admin/surveys/:id` | Get a survey with sections and questions |
| PATCH | `/admin/surveys/:id` | Update survey metadata |
| PATCH | `/admin/surveys/:id/status` | Transition survey status |
| PATCH | `/admin/surveys/:id/publish-results` | Publish results publicly |
| DELETE | `/admin/surveys/:id` | Delete a draft survey only |
| POST | `/admin/surveys/:id/sections` | Add a section |
| PUT | `/admin/surveys/:id/sections/:sid` | Update a section (draft surveys only) |
| DELETE | `/admin/surveys/:id/sections/:sid` | Remove a section (draft surveys only) |
| POST | `/admin/surveys/:id/sections/:sid/questions` | Add a question to a section |
| PUT | `/admin/surveys/:id/questions/:qid` | Update a question (draft surveys only) |
| DELETE | `/admin/surveys/:id/questions/:qid` | Remove a question (draft surveys only) |
| GET | `/admin/surveys/:id/results` | View aggregated results |
| GET | `/admin/surveys/:id/responses` | List raw responses (paginated) |
| GET | `/admin/users` | List all users |
| GET | `/admin/users/:id` | Get a user with profile and responses |

---

## Technology stack

| Layer | Choice | Reason |
|---|---|---|
| Language | Rust (stable) | Primary learning goal |
| Web framework | Axum | Async-native, Tokio-based, best ecosystem fit |
| Database | PostgreSQL | Mature, strong Rust ecosystem, JSONB for answer storage |
| DB access | SQLx | Compile-time query checking, no ORM abstraction overhead |
| Migrations | sqlx-cli | Version-controlled SQL files, simple runner |
| Async runtime | Tokio | Required by Axum |
| Serialization | serde + serde_json | Standard for Rust web dev |
| Auth | jsonwebtoken crate | Access + refresh token pattern, stateless access tokens |
| Password hashing | argon2 crate | Argon2id for admin password and refresh token storage |
| Validation | validator crate | Struct-level validation, integrates with serde |
| Error handling | thiserror (library errors) + anyhow (app errors) | Teaches Rust error patterns correctly |
| Config | dotenvy | Familiar .env pattern from Node |
| Frontend | JavaScript-based SPA, likely React | Separate repo, consumes the API |

---

## Non-functional considerations

| Concern | Approach for v1 |
|---|---|
| Identity | Name and email collected. Email is the unique identifier for resume and deduplication. First-entry name is preserved — not overwritten on subsequent visits. |
| Duplicate submissions | Email-based deduplication. One UserSurvey row per user per survey. Answers are upserted, not duplicated. |
| Data retention | Indefinite. No deletion policy in v1. |
| Rate limiting | Global rate limiter applied across all endpoints. Per-endpoint thresholds deferred. |
| HTTPS | Mandatory. TLS termination at the reverse proxy. |
| Refresh token storage | Stored hashed (Argon2id) server-side. Rotated on every use. Invalidated on logout. |

---

## What the lower-level spec will cover

- Full database schema with column types, constraints, and indexes
- Full API contract per endpoint (request body, response shape, error codes)
- Axum router structure and handler organisation
- SQLx query patterns and error handling strategy
- Dual-write logic for profile-building answers
- Access and refresh token implementation details
- Validation rules per field
- Full enum definitions with all keys and display names
- `/meta/enums` response shape
- Migration file structure and naming convention
- Project directory layout
- Rust-specific implementation notes (trait design, error types, shared state)