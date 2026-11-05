# Reconnaissance Map: Linear

**Target URL / Surface**: https://linear.app
**Analyzed At**: 2026-11-05T03:56:53.212593+00:00

## 1. Screens & Surfaces

| Screen ID | Title | Route Path | Interactive Elements | States |
|:---|:---|:---|:---|:---|
| `S-HOME` | Main Dashboard | `/` | Create Event, Share Link | empty, populated |

## 2. User Flows

### Flow: Guest Booking Journey (`F-BOOKING`)
- **Screens Involved**: S-HOME
- **Happy Path**:
  1. Open link
  1. Pick slot
  1. Confirm
- **Edge Cases & Failure Modes**:
  * Concurrency clash

## 3. Hard Architectural Constraints

- Zero double-booking (serializable tx)
- UTC-only timestamps

## 4. Cryptographic Evidence Ledger

| Evidence ID | Source | SHA-256 Digest | Key Observation |
|:---|:---|:---|:---|
| `ev-6eda5b0236aa` | `directory_artifact` | `sha256:6eda5b0236aa8c51a225c5661bc3911aaa6487b5e3ea337dcca0aece540bc29d` | Artifact discovered: Cargo.toml |
| `ev-e8ecfe28ddfc` | `directory_artifact` | `sha256:e8ecfe28ddfcc16cfa9847ff21049cd3dc050a422aece5bf13aa9ae633488a6d` | Artifact discovered: .rustc_info.json |
| `ev-6d9d1d216e0f` | `directory_artifact` | `sha256:6d9d1d216e0f83abc5e5662ca62c92b4f23009466b54fa27321a69acdb778bb2` | Artifact discovered: CACHEDIR.TAG |
| `ev-52ab6e1a243c` | `directory_artifact` | `sha256:52ab6e1a243ce4835424781009c012bc247ebf10845006f7199efdbd77ddb867` | Artifact discovered: replica-square.d |
| `ev-1714cf0d3eec` | `directory_artifact` | `sha256:1714cf0d3eec4ecae0de0cca27ff62b198dafa94593dde0c193599c8574b5ad5` | Artifact discovered: libreplica_square.d |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-build-lock |
| `ev-c46a1f09ca6b` | `directory_artifact` | `sha256:c46a1f09ca6bec47e260fe450d170a7fc3c902aa1453d10a403e2b90ada21680` | Artifact discovered: libreplica_square.rlib |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-lock |
| `ev-db667a05035f` | `directory_artifact` | `sha256:db667a05035f7a986c945d1195f06f2dad0c55dfb522db38520cc9da67386626` | Artifact discovered: replica-square |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-artifact-lock |
| `ev-783008cae0a6` | `directory_artifact` | `sha256:783008cae0a69fa2e8847703bd5f2a99a8bcbe95d8498b914edbe8ee49cfb267` | Artifact discovered: replica-agent.d |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-build-lock |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-lock |
| `ev-82438dcec5f3` | `directory_artifact` | `sha256:82438dcec5f371f5519d64165437ebbe4851ae0d4ea3ea20b4f6cb3ec66ab454` | Artifact discovered: replica-square |
| `ev-e3b0c44298fc` | `directory_artifact` | `sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Artifact discovered: .cargo-artifact-lock |
| `ev-24fcd2a0ee90` | `directory_artifact` | `sha256:24fcd2a0ee90f2721f32b9f6d045cb6c5e3e58d7c3e985bfd022266b16d6f380` | Artifact discovered: replica-agent |
| `ev-5b5c749c639a` | `directory_artifact` | `sha256:5b5c749c639abe27921bfd5515a56a46b00366aaa2019d0ee085140f1816afe0` | Artifact discovered: engine_tests.rs |
| `ev-8d839432c79c` | `directory_artifact` | `sha256:8d839432c79c9002e9701b606ece60d172755d519dac35f71ac8b18af5302fd5` | Artifact discovered: agentic_pipeline_tests.rs |
| `ev-87a724b17404` | `directory_artifact` | `sha256:87a724b1740460560f8b27fb37e7c5309a35fd5c56ae23881a750b7aff97137a` | Artifact discovered: Cargo.lock |
| `ev-f44a79f0dc8d` | `directory_artifact` | `sha256:f44a79f0dc8da0a5d950e93f9986807711468634422a9d2f6c5604056e951215` | Artifact discovered: agentic-replica-engine-architecture.md |
| `ev-9fefd63cbc2f` | `directory_artifact` | `sha256:9fefd63cbc2f185e926d9b5a858bafe69c11a3fa81e90cbeac06cc4da4b3f3e9` | Artifact discovered: README.md |
| `ev-2cdbc4093a27` | `directory_artifact` | `sha256:2cdbc4093a27fe116463c1f9cfb53c1a919617887529ff3a7c946cde4ece7456` | Artifact discovered: .gitignore |
| `ev-004a2d3eefa0` | `directory_artifact` | `sha256:004a2d3eefa064b206a5af95d20cfc2a671b68fad662190b0590fb15a2c2ab14` | Artifact discovered: marketplace.json |
| `ev-dedefde51b81` | `directory_artifact` | `sha256:dedefde51b8187f24f9504e0971af5670dac1fafc9abde86ab2a581b5da4f77a` | Artifact discovered: plugin.json |
| `ev-ec82d4e7f17e` | `directory_artifact` | `sha256:ec82d4e7f17e725ebb82c6da4c2e61cea5dbb2b46259f8041fdaf675747eb54a` | Artifact discovered: SKILL.md |
| `ev-e2adb4a18040` | `directory_artifact` | `sha256:e2adb4a18040809001a8c189c5ab2d283d70d17d92fedeefd0fe17aa36023de9` | Artifact discovered: SKILL.md |
| `ev-c187a95ff3fb` | `directory_artifact` | `sha256:c187a95ff3fbc5b7044d632dc6faad161b58aceaa151b08a03b16d4608a5c28a` | Artifact discovered: cleanroom-report.md |
| `ev-65610173691e` | `directory_artifact` | `sha256:65610173691e42076728ab057f2eb2357e810c71ce5f619b32ce57a550c6dad4` | Artifact discovered: architecture.md |
| `ev-f30dcced5eaf` | `directory_artifact` | `sha256:f30dcced5eaf09d905453bc99a7329cae11c278fc15fd535f41f96441705da3c` | Artifact discovered: reviews.csv |
| `ev-df453650e560` | `directory_artifact` | `sha256:df453650e5602434a1e84a488e0d32d0daa393f9313f3d24a8638e2f49eb63ac` | Artifact discovered: recon-map.json |
| `ev-e0385ed3ff57` | `directory_artifact` | `sha256:e0385ed3ff57c5d96cc056e99aece7144ad5bcc04486b606e050ea5e52c08ecb` | Artifact discovered: schema.sql |
| `ev-54ef6347a995` | `directory_artifact` | `sha256:54ef6347a9955ff5035b3189484596cb1c6956a5580d8f89fca8cbf4fb241014` | Artifact discovered: recon.md |
| `ev-38b3d7bc1593` | `directory_artifact` | `sha256:38b3d7bc1593c15a442cab20632c53b3ca9834de71b1bf214848ddb4a5f0a04b` | Artifact discovered: tailwind.tokens.js |
| `ev-0947df934b52` | `directory_artifact` | `sha256:0947df934b52e6a3531d92cc77b3954a9eca52e7b10f89130bd0426eb0211e77` | Artifact discovered: parity.md |
| `ev-9404e007b9ae` | `directory_artifact` | `sha256:9404e007b9ae14227d1bec633857eec59e783eac1e268d54e44fe28841fdae7f` | Artifact discovered: listing-audit.md |
| `ev-32b58886498c` | `directory_artifact` | `sha256:32b58886498cc651dce86114dbee117eda56319dee5600e11e7e1e6c5415ecb6` | Artifact discovered: features.csv |
| `ev-01c8bc71e77a` | `directory_artifact` | `sha256:01c8bc71e77adb6f27b721254d9c697d95c8cea3e5817b2e425829c8a11fda10` | Artifact discovered: tokens.json |
| `ev-286dc3f2f07a` | `directory_artifact` | `sha256:286dc3f2f07a2b94792438902a8d7154c3479a1f70b1406edaece7c148028809` | Artifact discovered: feedback.md |
| `ev-cae33efdb02c` | `directory_artifact` | `sha256:cae33efdb02cf774435c1ff9cb16bcc1014606908530c6e1dc727615fe3e8cda` | Artifact discovered: config |
| `ev-28d25bf82af4` | `directory_artifact` | `sha256:28d25bf82af4c0e2b72f50959b2beb859e3e60b9630a5e8c603dad4ddb2b6e80` | Artifact discovered: HEAD |
| `ev-6671fe83b7a0` | `directory_artifact` | `sha256:6671fe83b7a07c8932ee89164d1f2793b2318058eb8b98dc5c06ee0a5a3b0ec1` | Artifact discovered: exclude |
| `ev-821594c0f2ca` | `directory_artifact` | `sha256:821594c0f2ca319229c45c939efe96431aaa6cc12cf3a785592008b748d0652a` | Artifact discovered: HEAD |
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
| `ev-7bf0393f1bbd` | `directory_artifact` | `sha256:7bf0393f1bbdac575ccc5d70b5969d0a0b10ee00718c9b85f8f36887d5e67daf` | Artifact discovered: index |
| `ev-7f1606576847` | `directory_artifact` | `sha256:7f1606576847b9da909a75475d13b2e6cdb7cb6a4971d25a932287caa1bae61e` | Artifact discovered: COMMIT_EDITMSG |
| `ev-20909c0d3dd7` | `directory_artifact` | `sha256:20909c0d3dd760312cb95b7f774bf610c47a1f8d90d2ab189bb4f613f8623a80` | Artifact discovered: mod.rs |
| `ev-32df0ee24778` | `directory_artifact` | `sha256:32df0ee24778ab2374c0911ebc0d1794ef012524f8bce819fed25e0da71b3a7a` | Artifact discovered: types.rs |
| `ev-f42701380ca6` | `directory_artifact` | `sha256:f42701380ca63df5b71ae86b21b5816e5c6851c6b53a44c6d1fb86660a49379f` | Artifact discovered: mod.rs |
| `ev-4f67a2dd2436` | `directory_artifact` | `sha256:4f67a2dd243629112755e0dafc555ed9a1e265ca2c66f424864417cb16c1575b` | Artifact discovered: mod.rs |
| `ev-6c4074a3f6d3` | `directory_artifact` | `sha256:6c4074a3f6d3fdb1032180b3d28c7677abe11b854d16f02e97a5cd6123a46ddd` | Artifact discovered: lib.rs |
| `ev-9087ae964271` | `directory_artifact` | `sha256:9087ae9642711a7f64800e8e778f49e91cdab181e69f858a4e99305239634c3a` | Artifact discovered: mod.rs |
| `ev-8b14286afbcc` | `directory_artifact` | `sha256:8b14286afbcc28712de047b3e159bca6ad9c6faa30a95a51e37113ac5105e907` | Artifact discovered: mod.rs |
| `ev-8a8c3ed95c03` | `directory_artifact` | `sha256:8a8c3ed95c03aa1e2cead03b95c75e56012e079e20612ff9a12316e6bb87643e` | Artifact discovered: mod.rs |
| `ev-83705bd91f45` | `directory_artifact` | `sha256:83705bd91f4528c3e1459f30ed3257c522bec1a7dcca3eaf2ba3ec8c04cd4108` | Artifact discovered: mod.rs |
| `ev-78f0ef86b7b1` | `directory_artifact` | `sha256:78f0ef86b7b1387646dabc6679e4267ed66b5963874ae8abde98d08ab3b27a6a` | Artifact discovered: mod.rs |
| `ev-a9825d55df2c` | `directory_artifact` | `sha256:a9825d55df2c16bd72198d90f6293f62033f89cf957da05937d6a7320aa43f9e` | Artifact discovered: main.rs |
| `ev-9a8f461cd02d` | `directory_artifact` | `sha256:9a8f461cd02d249636859049243806a1ee92432894b72b85798e3238792117b3` | Artifact discovered: mod.rs |
| `ev-8bbe387c5367` | `directory_artifact` | `sha256:8bbe387c5367b3f58e95a174bd3cc3654b3dde274a81e635b64744287606025b` | Artifact discovered: mod.rs |
| `ev-8e9288cd6bba` | `directory_artifact` | `sha256:8e9288cd6bba75b113a499802b01eecd867603e0464e0a79d27f509a2230a366` | Artifact discovered: mod.rs |
| `ev-1350ce76155e` | `directory_artifact` | `sha256:1350ce76155eceeb4fe4cc18110da8e6e2db19c5f37083a860777b48e9848e86` | Artifact discovered: mod.rs |
| `ev-49620c5b727c` | `directory_artifact` | `sha256:49620c5b727cdeade0c6fc5e485b64351cabca787012df690a62458fed8dec18` | Artifact discovered: mod.rs |
