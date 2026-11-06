use crate::types::{ApiRoute, ArchitecturePlan, DbColumn, DbTable, ReconMap};
use std::collections::HashMap;

pub struct ArchitectEngine;

impl ArchitectEngine {
    /// Synthesize an architecture plan from a ReconMap
    pub fn plan_from_recon(recon: &ReconMap) -> ArchitecturePlan {
        let mut tables = Vec::new();

        // 1. Core Users Table
        tables.push(DbTable {
            name: "users".to_string(),
            columns: vec![
                DbColumn {
                    name: "id".to_string(),
                    data_type: "uuid".to_string(),
                    is_primary_key: true,
                    is_nullable: false,
                    default_value: Some("gen_random_uuid()".to_string()),
                    check_constraint: None,
                    references: None,
                },
                DbColumn {
                    name: "email".to_string(),
                    data_type: "text".to_string(),
                    is_primary_key: false,
                    is_nullable: false,
                    default_value: None,
                    check_constraint: None,
                    references: None,
                },
                DbColumn {
                    name: "full_name".to_string(),
                    data_type: "text".to_string(),
                    is_primary_key: false,
                    is_nullable: true,
                    default_value: None,
                    check_constraint: None,
                    references: None,
                },
                DbColumn {
                    name: "created_at".to_string(),
                    data_type: "timestamptz".to_string(),
                    is_primary_key: false,
                    is_nullable: false,
                    default_value: Some("now()".to_string()),
                    check_constraint: None,
                    references: None,
                },
                DbColumn {
                    name: "updated_at".to_string(),
                    data_type: "timestamptz".to_string(),
                    is_primary_key: false,
                    is_nullable: false,
                    default_value: Some("now()".to_string()),
                    check_constraint: None,
                    references: None,
                },
            ],
            indexes: vec![
                "create unique index users_email_idx on users (lower(email));".to_string(),
            ],
            rls_policies: vec![
                "create policy users_select_own on users for select using (id = auth.uid());"
                    .to_string(),
                "create policy users_update_own on users for update using (id = auth.uid());"
                    .to_string(),
            ],
        });

        // 2. Derive Domain Entity Tables from Screens and Flows
        for screen in &recon.screens {
            let entity_name = screen.screen_id.trim_start_matches("S-").to_lowercase();
            if entity_name != "login" && entity_name != "auth" && entity_name != "settings" {
                let table_name = format!("{}s", entity_name);
                tables.push(DbTable {
                    name: table_name.clone(),
                    columns: vec![
                        DbColumn {
                            name: "id".to_string(),
                            data_type: "uuid".to_string(),
                            is_primary_key: true,
                            is_nullable: false,
                            default_value: Some("gen_random_uuid()".to_string()),
                            check_constraint: None,
                            references: None,
                        },
                        DbColumn {
                            name: "user_id".to_string(),
                            data_type: "uuid".to_string(),
                            is_primary_key: false,
                            is_nullable: false,
                            default_value: None,
                            check_constraint: None,
                            references: Some("users(id) on delete cascade".to_string()),
                        },
                        DbColumn {
                            name: "title".to_string(),
                            data_type: "text".to_string(),
                            is_primary_key: false,
                            is_nullable: false,
                            default_value: None,
                            check_constraint: None,
                            references: None,
                        },
                        DbColumn {
                            name: "status".to_string(),
                            data_type: "text".to_string(),
                            is_primary_key: false,
                            is_nullable: false,
                            default_value: Some("'active'".to_string()),
                            check_constraint: Some(
                                "check (status in ('active', 'draft', 'archived'))".to_string(),
                            ),
                            references: None,
                        },
                        DbColumn {
                            name: "metadata".to_string(),
                            data_type: "jsonb".to_string(),
                            is_primary_key: false,
                            is_nullable: false,
                            default_value: Some("'{}'::jsonb".to_string()),
                            check_constraint: None,
                            references: None,
                        },
                        DbColumn {
                            name: "created_at".to_string(),
                            data_type: "timestamptz".to_string(),
                            is_primary_key: false,
                            is_nullable: false,
                            default_value: Some("now()".to_string()),
                            check_constraint: None,
                            references: None,
                        },
                        DbColumn {
                            name: "updated_at".to_string(),
                            data_type: "timestamptz".to_string(),
                            is_primary_key: false,
                            is_nullable: false,
                            default_value: Some("now()".to_string()),
                            check_constraint: None,
                            references: None,
                        },
                    ],
                    indexes: vec![
                        format!(
                            "create index {}_user_id_idx on {} (user_id);",
                            table_name, table_name
                        ),
                        format!(
                            "create index {}_created_at_idx on {} (created_at desc);",
                            table_name, table_name
                        ),
                    ],
                    rls_policies: vec![format!(
                        "create policy {}_all_own on {} for all using (user_id = auth.uid());",
                        table_name, table_name
                    )],
                });
            }
        }

        // 3. API Routes
        let mut routes = Vec::new();
        routes.push(ApiRoute {
            method: "POST".to_string(),
            path: "/api/auth/register".to_string(),
            description: "Register a new user account with hashed password".to_string(),
            auth_required: false,
            input_schema: "{ email: string, password: string }".to_string(),
            output_schema: "{ user_id: string, token: string }".to_string(),
            flow_id: "F-AUTH".to_string(),
        });
        routes.push(ApiRoute {
            method: "POST".to_string(),
            path: "/api/auth/login".to_string(),
            description: "Authenticate user and issue session token".to_string(),
            auth_required: false,
            input_schema: "{ email: string, password: string }".to_string(),
            output_schema: "{ user_id: string, token: string }".to_string(),
            flow_id: "F-AUTH".to_string(),
        });

        for table in &tables {
            if table.name != "users" {
                routes.push(ApiRoute {
                    method: "GET".to_string(),
                    path: format!("/api/{}", table.name),
                    description: format!("List all {} belonging to authenticated user", table.name),
                    auth_required: true,
                    input_schema: "{ limit?: number, cursor?: string }".to_string(),
                    output_schema: format!("{{ items: {}[], next_cursor?: string }}", table.name),
                    flow_id: "F-CORE".to_string(),
                });
                routes.push(ApiRoute {
                    method: "POST".to_string(),
                    path: format!("/api/{}", table.name),
                    description: format!("Create a new entry in {}", table.name),
                    auth_required: true,
                    input_schema: "{ title: string, metadata?: Record<string, any> }".to_string(),
                    output_schema: format!("{{ item: {} }}", table.name),
                    flow_id: "F-CORE".to_string(),
                });
            }
        }

        // 4. Build Order (Vertical Slice First)
        let build_order = vec![
            "1. Vertical Slice: Auth flow + Core Dashboard + Initial DB Migration".to_string(),
            "2. Primary CRUD Operations with RLS policies and validation".to_string(),
            "3. Interactive Frontend screens matching recon specs".to_string(),
            "4. Edge cases: Rate limiting, webhook idempotency, background job queues".to_string(),
            "5. Entrepreneur differentiators & competitor weakness mitigations".to_string(),
        ];

        let mut risk_mitigations = HashMap::new();
        risk_mitigations.insert(
            "Idempotency".to_string(),
            "Enforce idempotency-key headers on all POST/PATCH state modifications and Stripe webhooks.".to_string(),
        );
        risk_mitigations.insert(
            "Timezones".to_string(),
            "All timestamps strictly stored in UTC with timestamptz. Client converts via Intl.DateTimeFormat.".to_string(),
        );
        risk_mitigations.insert(
            "Concurrency".to_string(),
            "Postgres exclusion constraints and optimistic concurrency locks (version integer) prevent double-actions.".to_string(),
        );

        ArchitecturePlan {
            web_framework: "Next.js 15 (App Router) + TypeScript".to_string(),
            styling_system: "Tailwind CSS + Radix UI Primitives".to_string(),
            database_engine: "PostgreSQL on Supabase / Neon".to_string(),
            orm_layer: "Drizzle ORM".to_string(),
            auth_provider: "Supabase Auth (JWT + Cookie sessions)".to_string(),
            tables,
            routes,
            build_order,
            risk_mitigations,
        }
    }

    /// Render SQL Schema DDL
    pub fn render_sql(plan: &ArchitecturePlan) -> String {
        let mut sql = String::new();
        sql.push_str(
            "-- ==========================================================================\n",
        );
        sql.push_str("-- REPLICA DATABASE SCHEMA (PostgreSQL)\n");
        sql.push_str("-- Clean-Room Architecture Specification\n");
        sql.push_str(
            "-- ==========================================================================\n\n",
        );
        sql.push_str("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";\n\n");

        for table in &plan.tables {
            sql.push_str(&format!("-- Table: {}\n", table.name));
            sql.push_str(&format!(
                "CREATE TABLE IF NOT EXISTS public.{} (\n",
                table.name
            ));

            let mut col_defs = Vec::new();
            for col in &table.columns {
                let mut def = format!("  {} {}", col.name, col.data_type);
                if col.is_primary_key {
                    def.push_str(" PRIMARY KEY");
                }
                if let Some(ref d) = col.default_value {
                    def.push_str(&format!(" DEFAULT {}", d));
                }
                if !col.is_nullable {
                    def.push_str(" NOT NULL");
                }
                if let Some(ref r) = col.references {
                    def.push_str(&format!(" REFERENCES public.{}", r));
                }
                if let Some(ref c) = col.check_constraint {
                    def.push_str(&format!(" {}", c));
                }
                col_defs.push(def);
            }

            sql.push_str(&col_defs.join(",\n"));
            sql.push_str("\n);\n\n");

            // Indexes
            for idx in &table.indexes {
                sql.push_str(&format!("{}\n", idx));
            }

            // RLS
            sql.push_str(&format!(
                "ALTER TABLE public.{} ENABLE ROW LEVEL SECURITY;\n",
                table.name
            ));
            for policy in &table.rls_policies {
                sql.push_str(&format!("{}\n", policy));
            }
            sql.push('\n');
        }

        sql
    }

    /// Render Architecture Markdown
    pub fn render_markdown(plan: &ArchitecturePlan) -> String {
        let mut md = String::new();
        md.push_str("# System Architecture & Technical Specification\n\n");
        md.push_str("## 1. Stack Selection\n\n");
        md.push_str("| Layer | Choice | Rationale |\n");
        md.push_str("|:---|:---|:---|\n");
        md.push_str(&format!(
            "| Web & API | {} | Modern React server components, streaming SSR, type safety |\n",
            plan.web_framework
        ));
        md.push_str(&format!(
            "| Styling | {} | Utility-first, zero runtime CSS overhead, accessible tokens |\n",
            plan.styling_system
        ));
        md.push_str(&format!(
            "| Database | {} | ACID compliance, robust jsonb, native RLS, instant branching |\n",
            plan.database_engine
        ));
        md.push_str(&format!(
            "| ORM | {} | Type-safe SQL dialect, zero codegen latency, lightweight |\n",
            plan.orm_layer
        ));
        md.push_str(&format!("| Auth | {} | Secure session cookies, OAuth 2.0 / email OTP, native DB integration |\n", plan.auth_provider));
        md.push('\n');

        md.push_str("## 2. API Contracts & Route Registry\n\n");
        md.push_str("| Method | Path | Auth? | Description | Input | Output |\n");
        md.push_str("|:---|:---|:---|:---|:---|:---|\n");
        for r in &plan.routes {
            md.push_str(&format!(
                "| `{}` | `{}` | {} | {} | `{}` | `{}` |\n",
                r.method,
                r.path,
                if r.auth_required { "Yes" } else { "No" },
                r.description,
                r.input_schema,
                r.output_schema
            ));
        }
        md.push('\n');

        md.push_str("## 3. Critical Edge Cases & Risk Mitigations\n\n");
        for (risk, mit) in &plan.risk_mitigations {
            md.push_str(&format!("### {}\n{}\n\n", risk, mit));
        }

        md.push_str("## 4. Phased Build Order\n\n");
        for step in &plan.build_order {
            md.push_str(&format!("{}\n", step));
        }

        md
    }
}
