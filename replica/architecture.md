# System Architecture & Technical Specification

## 1. Stack Selection

| Layer | Choice | Rationale |
|:---|:---|:---|
| Web & API | Next.js 15 (App Router) + TypeScript | Modern React server components, streaming SSR, type safety |
| Styling | Tailwind CSS + Radix UI Primitives | Utility-first, zero runtime CSS overhead, accessible tokens |
| Database | PostgreSQL on Supabase / Neon | ACID compliance, robust jsonb, native RLS, instant branching |
| ORM | Drizzle ORM | Type-safe SQL dialect, zero codegen latency, lightweight |
| Auth | Supabase Auth (JWT + Cookie sessions) | Secure session cookies, OAuth 2.0 / email OTP, native DB integration |

## 2. API Contracts & Route Registry

| Method | Path | Auth? | Description | Input | Output |
|:---|:---|:---|:---|:---|:---|
| `POST` | `/api/auth/register` | No | Register a new user account with hashed password | `{ email: string, password: string }` | `{ user_id: string, token: string }` |
| `POST` | `/api/auth/login` | No | Authenticate user and issue session token | `{ email: string, password: string }` | `{ user_id: string, token: string }` |
| `GET` | `/api/homes` | Yes | List all homes belonging to authenticated user | `{ limit?: number, cursor?: string }` | `{ items: homes[], next_cursor?: string }` |
| `POST` | `/api/homes` | Yes | Create a new entry in homes | `{ title: string, metadata?: Record<string, any> }` | `{ item: homes }` |
| `GET` | `/api/bookings` | Yes | List all bookings belonging to authenticated user | `{ limit?: number, cursor?: string }` | `{ items: bookings[], next_cursor?: string }` |
| `POST` | `/api/bookings` | Yes | Create a new entry in bookings | `{ title: string, metadata?: Record<string, any> }` | `{ item: bookings }` |

## 3. Critical Edge Cases & Risk Mitigations

### Idempotency
Enforce idempotency-key headers on all POST/PATCH state modifications and Stripe webhooks.

### Timezones
All timestamps strictly stored in UTC with timestamptz. Client converts via Intl.DateTimeFormat.

### Concurrency
Postgres exclusion constraints and optimistic concurrency locks (version integer) prevent double-actions.

## 4. Phased Build Order

1. Vertical Slice: Auth flow + Core Dashboard + Initial DB Migration
2. Primary CRUD Operations with RLS policies and validation
3. Interactive Frontend screens matching recon specs
4. Edge cases: Rate limiting, webhook idempotency, background job queues
5. Entrepreneur differentiators & competitor weakness mitigations
