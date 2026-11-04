-- ==========================================================================
-- REPLICA DATABASE SCHEMA (PostgreSQL)
-- Clean-Room Architecture Specification
-- ==========================================================================

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Table: users
CREATE TABLE IF NOT EXISTS public.users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  email text NOT NULL,
  full_name text,
  created_at timestamptz DEFAULT now() NOT NULL,
  updated_at timestamptz DEFAULT now() NOT NULL
);

create unique index users_email_idx on users (lower(email));
ALTER TABLE public.users ENABLE ROW LEVEL SECURITY;
create policy users_select_own on users for select using (id = auth.uid());
create policy users_update_own on users for update using (id = auth.uid());

-- Table: homes
CREATE TABLE IF NOT EXISTS public.homes (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  user_id uuid NOT NULL REFERENCES public.users(id) on delete cascade,
  title text NOT NULL,
  status text DEFAULT 'active' NOT NULL check (status in ('active', 'draft', 'archived')),
  metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
  created_at timestamptz DEFAULT now() NOT NULL,
  updated_at timestamptz DEFAULT now() NOT NULL
);

create index homes_user_id_idx on homes (user_id);
create index homes_created_at_idx on homes (created_at desc);
ALTER TABLE public.homes ENABLE ROW LEVEL SECURITY;
create policy homes_all_own on homes for all using (user_id = auth.uid());

-- Table: bookings
CREATE TABLE IF NOT EXISTS public.bookings (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  user_id uuid NOT NULL REFERENCES public.users(id) on delete cascade,
  title text NOT NULL,
  status text DEFAULT 'active' NOT NULL check (status in ('active', 'draft', 'archived')),
  metadata jsonb DEFAULT '{}'::jsonb NOT NULL,
  created_at timestamptz DEFAULT now() NOT NULL,
  updated_at timestamptz DEFAULT now() NOT NULL
);

create index bookings_user_id_idx on bookings (user_id);
create index bookings_created_at_idx on bookings (created_at desc);
ALTER TABLE public.bookings ENABLE ROW LEVEL SECURITY;
create policy bookings_all_own on bookings for all using (user_id = auth.uid());

