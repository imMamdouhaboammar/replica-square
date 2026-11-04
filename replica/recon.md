# Reconnaissance Map: Calendly

**Target URL / Surface**: https://calendly.com
**Analyzed At**: 2026-11-04T22:05:21.470697+00:00

## 1. Screens & Surfaces

| Screen ID | Title | Route Path | Interactive Elements | States |
|:---|:---|:---|:---|:---|
| `S-HOME` | Main Dashboard / Landing | `/` | Create Event Button, Upcoming Schedule List, Share Link Trigger | empty, populated, loading |
| `S-BOOKING` | Public Booking Calendar | `/[user]/[slug]` | Date Picker, Time Slot Grid, Guest Form Input, Confirm Booking CTA | date_select, slot_select, confirmed |

## 2. User Flows

### Flow: Guest Booking Journey (`F-BOOKING`)
- **Screens Involved**: S-BOOKING -> S-CONFIRMATION
- **Happy Path**:
  1. Guest opens public link
  1. Selects available date and time slot
  1. Enters name and email
  1. Receives calendar invite & confirmation
- **Edge Cases & Failure Modes**:
  * Slot booked simultaneously by another guest (concurrency clash)
  * Guest in conflicting timezone (DST calculation)

## 3. Hard Architectural Constraints

- Absolute zero double-booking invariant (exclusion constraint or serializable transaction)
- All timestamps stored strictly in UTC with timezone offset preserved
- Public forms must implement client IP rate-limiting and honeypot validation

## 4. Cryptographic Evidence Ledger

| Evidence ID | Source | SHA-256 Digest | Key Observation |
|:---|:---|:---|:---|
| `ev-b1f25086f9ed` | `directory_artifact` | `sha256:b1f25086f9ed96ffde91e2b43f7649b83609a58ffd69043de99acf4388e20751` | Artifact discovered: Cargo.toml |
| `ev-e8ecfe28ddfc` | `directory_artifact` | `sha256:e8ecfe28ddfcc16cfa9847ff21049cd3dc050a422aece5bf13aa9ae633488a6d` | Artifact discovered: .rustc_info.json |
| `ev-6d9d1d216e0f` | `directory_artifact` | `sha256:6d9d1d216e0f83abc5e5662ca62c92b4f23009466b54fa27321a69acdb778bb2` | Artifact discovered: CACHEDIR.TAG |
| `ev-783008cae0a6` | `directory_artifact` | `sha256:783008cae0a69fa2e8847703bd5f2a99a8bcbe95d8498b914edbe8ee49cfb267` | Artifact discovered: replica-agent.d |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-build-lock |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-lock |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-artifact-lock |
| `ev-02e6fc12c5d3` | `directory_artifact` | `sha256:02e6fc12c5d3e89106f7ecef3b24c4c48b0ff381956373632323ce747ffb1cd3` | Artifact discovered: replica-agent |
| `ev-f6597341a5f9` | `directory_artifact` | `sha256:f6597341a5f9597825544d39dfdd3f75f74ab7cfd88a63d880b5076fc638f438` | Artifact discovered: Cargo.lock |
| `ev-44c92e3a70ad` | `directory_artifact` | `sha256:44c92e3a70ad3307b7056871c2bdb096d8bfa9373f5bf06a79bb6324a20ff2fb` | Artifact discovered: .gitignore |
| `ev-cae33efdb02c` | `directory_artifact` | `sha256:cae33efdb02cf774435c1ff9cb16bcc1014606908530c6e1dc727615fe3e8cda` | Artifact discovered: config |
| `ev-28d25bf82af4` | `directory_artifact` | `sha256:28d25bf82af4c0e2b72f50959b2beb859e3e60b9630a5e8c603dad4ddb2b6e80` | Artifact discovered: HEAD |
| `ev-6671fe83b7a0` | `directory_artifact` | `sha256:6671fe83b7a07c8932ee89164d1f2793b2318058eb8b98dc5c06ee0a5a3b0ec1` | Artifact discovered: exclude |
| `ev-85ab6c163d43` | `directory_artifact` | `sha256:85ab6c163d43a17ea9cf7788308bca1466f1b0a8d1cc92e26e9bf63da4062aee` | Artifact discovered: description |
| `ev-1f74d5e92929` | `directory_artifact` | `sha256:1f74d5e9292979b573ebd59741d46cb93ff391acdd083d340b94370753d92437` | Artifact discovered: commit-msg.sample |
| `ev-4febce867790` | `directory_artifact` | `sha256:4febce867790052338076f4e66cc47efb14879d18097d1d61c8261859eaaa7b3` | Artifact discovered: pre-rebase.sample |
| `ev-44ebfc923dc5` | `directory_artifact` | `sha256:44ebfc923dc5466bc009602f0ecf067b9c65459abfe8868ddc49b78e6ced7a92` | Artifact discovered: sendemail-validate.sample |
| `ev-57185b7b9f05` | `directory_artifact` | `sha256:57185b7b9f05239d7ab52db045f5b89eb31348d7b2177eab214f5eb872e1971b` | Artifact discovered: pre-commit.sample |
| `ev-0223497a0b8b` | `directory_artifact` | `sha256:0223497a0b8b033aa58a3a521b8629869386cf7ab0e2f101963d328aa62193f7` | Artifact discovered: applypatch-msg.sample |
| `ev-e0549964e938` | `directory_artifact` | `sha256:e0549964e93897b519bd8e333c037e51fff0f88ba13e086a331592bf801fa1d0` | Artifact discovered: fsmonitor-watchman.sample |
| `ev-a4c3d2b9c7bb` | `directory_artifact` | `sha256:a4c3d2b9c7bb3fd8d1441c31bd4ee71a595d66b44fcf49ddb310252320169989` | Artifact discovered: pre-receive.sample |
| `ev-e9ddcaa4189f` | `directory_artifact` | `sha256:e9ddcaa4189fddd25ed97fc8c789eca7b6ca16390b2392ae3276f0c8e1aa4619` | Artifact discovered: prepare-commit-msg.sample |
| `ev-81765af2daef` | `directory_artifact` | `sha256:81765af2daef323061dcbc5e61fc16481cb74b3bac9ad8a174b186523586f6c5` | Artifact discovered: post-update.sample |
| `ev-d3825a703379` | `directory_artifact` | `sha256:d3825a70337940ebbd0a5c072984e13245920cdf8898bd225c8d27a6dfc9cb53` | Artifact discovered: pre-merge-commit.sample |
| `ev-e15c5b469ea3` | `directory_artifact` | `sha256:e15c5b469ea3e0a695bea6f2c82bcf8e62821074939ddd85b77e0007ff165475` | Artifact discovered: pre-applypatch.sample |
| `ev-ecce9c7e04d3` | `directory_artifact` | `sha256:ecce9c7e04d3f5dd9d8ada81753dd1d549a9634b26770042b58dda00217d086a` | Artifact discovered: pre-push.sample |
| `ev-8d5f2fa83e10` | `directory_artifact` | `sha256:8d5f2fa83e103cf08b57eaa67521df9194f45cbdbcb37da52ad586097a14d106` | Artifact discovered: update.sample |
| `ev-a53d0741798b` | `directory_artifact` | `sha256:a53d0741798b287c6dd7afa64aee473f305e65d3f49463bb9d7408ec3b12bf5f` | Artifact discovered: push-to-checkout.sample |
| `ev-20909c0d3dd7` | `directory_artifact` | `sha256:20909c0d3dd760312cb95b7f774bf610c47a1f8d90d2ab189bb4f613f8623a80` | Artifact discovered: mod.rs |
| `ev-32df0ee24778` | `directory_artifact` | `sha256:32df0ee24778ab2374c0911ebc0d1794ef012524f8bce819fed25e0da71b3a7a` | Artifact discovered: types.rs |
| `ev-4f67a2dd2436` | `directory_artifact` | `sha256:4f67a2dd243629112755e0dafc555ed9a1e265ca2c66f424864417cb16c1575b` | Artifact discovered: mod.rs |
| `ev-717e55085f59` | `directory_artifact` | `sha256:717e55085f596f17d987ed6bf0c215ae1378fb690aa98b9e79e1acb4eb629da6` | Artifact discovered: mod.rs |
| `ev-8b14286afbcc` | `directory_artifact` | `sha256:8b14286afbcc28712de047b3e159bca6ad9c6faa30a95a51e37113ac5105e907` | Artifact discovered: mod.rs |
| `ev-3cc5964c0346` | `directory_artifact` | `sha256:3cc5964c0346019ecfa7e28d3ea6a88e7fabb8c70756304d235cca373a84ef9d` | Artifact discovered: mod.rs |
| `ev-78f0ef86b7b1` | `directory_artifact` | `sha256:78f0ef86b7b1387646dabc6679e4267ed66b5963874ae8abde98d08ab3b27a6a` | Artifact discovered: mod.rs |
| `ev-2ff196fa3140` | `directory_artifact` | `sha256:2ff196fa3140b944d877b41ff1d81276227234ad3a0c42592fa9284e6078e1b5` | Artifact discovered: main.rs |
| `ev-9a8f461cd02d` | `directory_artifact` | `sha256:9a8f461cd02d249636859049243806a1ee92432894b72b85798e3238792117b3` | Artifact discovered: mod.rs |
| `ev-8e9288cd6bba` | `directory_artifact` | `sha256:8e9288cd6bba75b113a499802b01eecd867603e0464e0a79d27f509a2230a366` | Artifact discovered: mod.rs |
| `ev-ae69d17ebbf5` | `directory_artifact` | `sha256:ae69d17ebbf5b4693dcf12178894531b76d44774e6bdbcd5eb65fe5443d7d5cc` | Artifact discovered: mod.rs |
