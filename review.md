# Code Review

Project Path: /Users/harshpratapsingh/Desktop/taiko

Project Structure:
taiko
├── Cargo.toml
├── LICENSE
├── target
│   ├── CACHEDIR.TAG
│   ├── release
│   │   ├── taiko.d
│   │   ├── incremental
│   │   ├── examples
│   │   ├── taiko
│   │   ├── deps
│   │   │   ├── libfancy_regex-0c8eb8d613294a6e.rlib
│   │   │   ├── libclap_lex-178e9b36476d4e9a.rmeta
│   │   │   ├── libregex_automata-552ff0161f53a959.rlib
│   │   │   ├── pest-8433031303875c9a.d
│   │   │   ├── libsyn-04936f912c05bdc0.rlib
│   │   │   ├── libutf8parse-241bde393834c15a.rlib
│   │   │   ├── libbstr-62c8bef8e949b2ba.rlib
│   │   │   ├── libthiserror-b8e957c9de280bc3.rmeta
│   │   │   ├── anstyle_parse-827533bcb22b05fe.d
│   │   │   ├── liblock_api-2e0ba9efb1d81afa.rlib
│   │   │   ├── smallvec-c3648dc45b17a810.d
│   │   │   ├── libanyhow-fcf5cec6dd3dfc3e.rlib
│   │   │   ├── libfancy_regex-0c8eb8d613294a6e.rmeta
│   │   │   ├── libclap_lex-b9cf91260205bec3.rlib
│   │   │   ├── termtree-ba9786ca0305b61e.d
│   │   │   ├── libbstr-bdf27114869b0552.rlib
│   │   │   ├── libstrsim-676837c575daf54e.rmeta
│   │   │   ├── libheck-72df6f2fd216bb0b.rmeta
│   │   │   ├── prompt_macro-ed7428a7428d8f57.d
│   │   │   ├── libmalloc_buf-066d648d990ecced.rmeta
│   │   │   ├── libbase64-336c9298377507a2.rlib
│   │   │   ├── libthiserror_impl-126013681a33cb39.dylib
│   │   │   ├── libpest_generator-2d21a298232c8ebc.rmeta
│   │   │   ├── libconsole-aa2c177e5962a4d2.rmeta
│   │   │   ├── libnumber_prefix-dbbd565b6da08b33.rlib
│   │   │   ├── log-1763037ba5e87016.d
│   │   │   ├── liblazy_static-59e7fd1a9e99a10b.rmeta
│   │   │   ├── libucd_trie-b6376f1e4159930f.rmeta
│   │   │   ├── ryu-4ff3e018a55a59fb.d
│   │   │   ├── libquote-50dcc41d8c869384.rlib
│   │   │   ├── libryu-4ff3e018a55a59fb.rmeta
│   │   │   ├── libanyhow-30c7250151378d39.rmeta
│   │   │   ├── libclap_lex-0e39550645e2de8c.rlib
│   │   │   ├── libsyn-eefd0846265850c8.rmeta
│   │   │   ├── libautocfg-b1ddef1e4bff8483.rmeta
│   │   │   ├── console-691781ea730960dd.d
│   │   │   ├── libanstream-8b1a55f7013a63de.rlib
│   │   │   ├── libanstyle_parse-10471dc433a0c21a.rmeta
│   │   │   ├── libsyn-eefd0846265850c8.rlib
│   │   │   ├── libunicode_width-e0be5e6b79a54c74.rlib
│   │   │   ├── libclap_lex-b9cf91260205bec3.rmeta
│   │   │   ├── syn-04936f912c05bdc0.d
│   │   │   ├── ignore-4a87f3ab4ddc78c6.d
│   │   │   ├── libautocfg-999daad8563db8a9.rlib
│   │   │   ├── liblock_api-439e265ca667b05b.rlib
│   │   │   ├── libserde_json-9a69fa8f3bd55de9.rlib
│   │   │   ├── crossbeam_utils-b669fe86b38ddee9.d
│   │   │   ├── libcolored-f39e864bdda671a4.rmeta
│   │   │   ├── libquote-ba95521fa6b0fe01.rmeta
│   │   │   ├── libglobset-42b9568d3419ec9a.rmeta
│   │   │   ├── libsame_file-c9a50bc6410f240f.rmeta
│   │   │   ├── libanstream-37bf9bad64549f21.rlib
│   │   │   ├── libautocfg-c0c39e025eafae2a.rmeta
│   │   │   ├── libtiktoken_rs-62137a7f978c2c61.rmeta
│   │   │   ├── libtermtree-00916f8ce7930209.rlib
│   │   │   ├── libignore-24e2b64405dae3c3.rmeta
│   │   │   ├── libryu-f854f3e2d0bc7970.rlib
│   │   │   ├── tiktoken_rs-0225d32dda2f5a6d.d
│   │   │   ├── aho_corasick-81db9c5633441623.d
│   │   │   ├── liblazy_static-fd8aebbacaa947fb.rlib
│   │   │   ├── crossbeam_deque-aa2614d487e78a9a.d
│   │   │   ├── libcrossbeam-30a03ffc8858233b.rlib
│   │   │   ├── serde-b3e3f6caab443231.d
│   │   │   ├── aho_corasick-daae1a193e77c697.d
│   │   │   ├── libbase64-2d79316584b5f227.rlib
│   │   │   ├── libcrossbeam_channel-4af5578a8855579c.rmeta
│   │   │   ├── libclap_derive-8e7fba42bc14a27f.dylib
│   │   │   ├── scopeguard-efdb6bab98ff555d.d
│   │   │   ├── libproc_macro2-6f972a4c0d651058.rlib
│   │   │   ├── serde_json-74b1893772dffc57.d
│   │   │   ├── crossbeam_epoch-20ca9b22b8077bdc.d
│   │   │   ├── libanstyle-a3283da5c0f274f3.rmeta
│   │   │   ├── console-aa2c177e5962a4d2.d
│   │   │   ├── proc_macro2-ff957dbded91c6b1.d
│   │   │   ├── parking_lot_core-cdf873ae7c907ae1.d
│   │   │   ├── liblibc-a19978263b9b2a8c.rlib
│   │   │   ├── libcli_clipboard-9664b3bb3697ff58.rmeta
│   │   │   ├── libbit_vec-a11fe0a33f56b1aa.rmeta
│   │   │   ├── libportable_atomic-00a8e7bb09605d7e.rlib
│   │   │   ├── libcrossbeam_epoch-20ca9b22b8077bdc.rlib
│   │   │   ├── bit_set-8990b6bec7af34c8.d
│   │   │   ├── number_prefix-dbbd565b6da08b33.d
│   │   │   ├── librayon-d788a88286e03a57.rlib
│   │   │   ├── libscopeguard-efdb6bab98ff555d.rlib
│   │   │   ├── libsame_file-97cee3f9797c5614.rmeta
│   │   │   ├── libis_terminal_polyfill-b0c6c4fcc5a4dfdc.rlib
│   │   │   ├── libtiktoken_rs-0225d32dda2f5a6d.rlib
│   │   │   ├── libanstyle_parse-10471dc433a0c21a.rlib
│   │   │   ├── libpest-8433031303875c9a.rlib
│   │   │   ├── libcrossbeam_utils-b669fe86b38ddee9.rmeta
│   │   │   ├── libcrossbeam_epoch-a5a2e5c50e172349.rlib
│   │   │   ├── libonce_cell-7cf040c7a18f5efc.rmeta
│   │   │   ├── libbit_set-332521716022bab1.rlib
│   │   │   ├── librustc_hash-fd56defa4d4e0515.rlib
│   │   │   ├── either-49d65d10f088e6bd.d
│   │   │   ├── libonce_cell-e327dfb7dbd56683.rlib
│   │   │   ├── liblock_api-2e0ba9efb1d81afa.rmeta
│   │   │   ├── libanyhow-c1636100da306932.rlib
│   │   │   ├── libnumber_prefix-dbbd565b6da08b33.rmeta
│   │   │   ├── clap-340c1f12a53f5ef7.d
│   │   │   ├── libclap_builder-03171f2d20bf1d34.rmeta
│   │   │   ├── portable_atomic-00a8e7bb09605d7e.d
│   │   │   ├── liblog-df0c3a43a2d81184.rmeta
│   │   │   ├── libproc_macro2-ff957dbded91c6b1.rlib
│   │   │   ├── libglobset-e28de3350cb4d24a.rmeta
│   │   │   ├── liblibc-25bb789ecd625a35.rmeta
│   │   │   ├── parking_lot_core-ee9d663c04bf84d7.d
│   │   │   ├── libeither-49d65d10f088e6bd.rmeta
│   │   │   ├── libclap_derive-669d895d5854291e.dylib
│   │   │   ├── number_prefix-feef2e60a784e16b.d
│   │   │   ├── libsmallvec-793c8bfd17fc7acd.rmeta
│   │   │   ├── libregex-c34b18d69c7d70a9.rmeta
│   │   │   ├── portable_atomic-ff4399801b1a298d.d
│   │   │   ├── libbit_vec-a11fe0a33f56b1aa.rlib
│   │   │   ├── libcfg_if-407b3ce8be06afa8.rmeta
│   │   │   ├── crossbeam_queue-cbd3cfb4ee35919c.d
│   │   │   ├── libmemchr-47ad014842134a2e.rlib
│   │   │   ├── libanstream-8b1a55f7013a63de.rmeta
│   │   │   ├── libclap_derive-cce90712b1194b53.dylib
│   │   │   ├── scopeguard-f744bb069bca91ef.d
│   │   │   ├── libcfg_if-f1eac65717ac6884.rmeta
│   │   │   ├── memchr-f81761c04e7279ee.d
│   │   │   ├── clap_builder-a3a43c0869e4f951.d
│   │   │   ├── crossbeam_utils-c0cf686b755ca63d.d
│   │   │   ├── libignore-a6ee6b89288ab772.rmeta
│   │   │   ├── base64-336c9298377507a2.d
│   │   │   ├── libwalkdir-cd19d39f5b265f3d.rmeta
│   │   │   ├── lazy_static-fd8aebbacaa947fb.d
│   │   │   ├── libthiserror-c448c2c299fbbf71.rmeta
│   │   │   ├── libregex_syntax-3dc0aea71a24ebb7.rlib
│   │   │   ├── pest_derive-0d8c12727e936794.d
│   │   │   ├── smallvec-793c8bfd17fc7acd.d
│   │   │   ├── indicatif-14b3f8ae01f24abd.d
│   │   │   ├── libanstyle-a10b55c910c671e5.rmeta
│   │   │   ├── libanstream-f24d63c1eea0a444.rmeta
│   │   │   ├── libanstyle_parse-827533bcb22b05fe.rlib
│   │   │   ├── librustc_hash-19b2a7552500a495.rlib
│   │   │   ├── libnumber_prefix-feef2e60a784e16b.rlib
│   │   │   ├── globset-243c7c1921aad0a7.d
│   │   │   ├── base64-f3dae55d80987eb8.d
│   │   │   ├── libcolored-f39e864bdda671a4.rlib
│   │   │   ├── libparse_format-d4f6b9211f8e5fe1.rmeta
│   │   │   ├── fancy_regex-f197c19fd02d3f04.d
│   │   │   ├── libproc_macro2-6f972a4c0d651058.rmeta
│   │   │   ├── libitoa-42c9d698521056f5.rmeta
│   │   │   ├── libc-a19978263b9b2a8c.d
│   │   │   ├── libcolorchoice-ec8ac6fa39b94aaa.rmeta
│   │   │   ├── crossbeam_channel-4af5578a8855579c.d
│   │   │   ├── libclap-340c1f12a53f5ef7.rmeta
│   │   │   ├── libscopeguard-7d025d7bea8d1c0a.rlib
│   │   │   ├── unicode_ident-5929dd8d837653ce.d
│   │   │   ├── libunicode_ident-e63abeae6024e13f.rlib
│   │   │   ├── libclap-da87c120b9bdbdbc.rlib
│   │   │   ├── rayon-d788a88286e03a57.d
│   │   │   ├── libparking_lot_core-cdf873ae7c907ae1.rlib
│   │   │   ├── itoa-1d5aa20e784c1230.d
│   │   │   ├── unindent-7fd69610340f4601.d
│   │   │   ├── fancy_regex-ba7b780a240ccf29.d
│   │   │   ├── libquote-c2116f873725ad3b.rmeta
│   │   │   ├── unicode_ident-db0e24d7d1309e5f.d
│   │   │   ├── libcrossbeam_utils-169afb23be825b5c.rmeta
│   │   │   ├── libscopeguard-efdb6bab98ff555d.rmeta
│   │   │   ├── crossbeam_deque-103683c298e2c133.d
│   │   │   ├── libobjc_id-f018c0d90b54e8c4.rlib
│   │   │   ├── crossbeam_epoch-a5a2e5c50e172349.d
│   │   │   ├── parse_format-d4f6b9211f8e5fe1.d
│   │   │   ├── globset-42b9568d3419ec9a.d
│   │   │   ├── libunicode_width-24baffe62a47adf8.rlib
│   │   │   ├── libsyn-04936f912c05bdc0.rmeta
│   │   │   ├── libaho_corasick-daae1a193e77c697.rmeta
│   │   │   ├── libportable_atomic-ff4399801b1a298d.rlib
│   │   │   ├── libstrsim-dad156f3ed68cf6f.rlib
│   │   │   ├── cfg_if-fad10b9109b556b8.d
│   │   │   ├── libutf8parse-234f8135fee3007f.rmeta
│   │   │   ├── libheck-362cf1f27b4900dd.rmeta
│   │   │   ├── libanstyle-a3283da5c0f274f3.rlib
│   │   │   ├── strsim-676837c575daf54e.d
│   │   │   ├── once_cell-7cf040c7a18f5efc.d
│   │   │   ├── liblog-1763037ba5e87016.rlib
│   │   │   ├── number_prefix-f55dca790810ab17.d
│   │   │   ├── librustc_hash-fd56defa4d4e0515.rmeta
│   │   │   ├── malloc_buf-066d648d990ecced.d
│   │   │   ├── librustc_hash-21e4b86214e09860.rlib
│   │   │   ├── libucd_trie-12c6fb9c1feff04c.rmeta
│   │   │   ├── libucd_trie-12c6fb9c1feff04c.rlib
│   │   │   ├── jwalk-0f9377beb62cf3d3.d
│   │   │   ├── libproc_macro2-919e63ef4dc60bd1.rlib
│   │   │   ├── libcrossbeam_queue-cbd3cfb4ee35919c.rmeta
│   │   │   ├── anstyle-a3283da5c0f274f3.d
│   │   │   ├── libheck-362cf1f27b4900dd.rlib
│   │   │   ├── bit_set-332521716022bab1.d
│   │   │   ├── heck-eeeb58b6f76d070c.d
│   │   │   ├── libregex_syntax-3dc0aea71a24ebb7.rmeta
│   │   │   ├── libpest-99335c19396f192f.rlib
│   │   │   ├── libobjc-8bf9cf4cdd832e27.rmeta
│   │   │   ├── libunindent-edeee594be933f44.rmeta
│   │   │   ├── libaho_corasick-079516c0c3447023.rlib
│   │   │   ├── libanstyle_parse-827533bcb22b05fe.rmeta
│   │   │   ├── liblog-df0c3a43a2d81184.rlib
│   │   │   ├── cli_clipboard-9664b3bb3697ff58.d
│   │   │   ├── libunindent-7fd69610340f4601.rmeta
│   │   │   ├── libcolored-3c7106d6aeef1a52.rmeta
│   │   │   ├── libblock-b6691600ce4c15c0.rlib
│   │   │   ├── portable_atomic-2dd4b74eb6ccbb4e.d
│   │   │   ├── libunicode_ident-e63abeae6024e13f.rmeta
│   │   │   ├── libheck-eeeb58b6f76d070c.rmeta
│   │   │   ├── libglobset-243c7c1921aad0a7.rmeta
│   │   │   ├── libglobset-e28de3350cb4d24a.rlib
│   │   │   ├── libregex_syntax-e94c39df99344eca.rlib
│   │   │   ├── once_cell-e327dfb7dbd56683.d
│   │   │   ├── libcrossbeam_deque-103683c298e2c133.rlib
│   │   │   ├── libonce_cell-22db72f8e301a324.rlib
│   │   │   ├── libutf8parse-241bde393834c15a.rmeta
│   │   │   ├── libmalloc_buf-066d648d990ecced.rlib
│   │   │   ├── colored-3c7106d6aeef1a52.d
│   │   │   ├── libbstr-62c8bef8e949b2ba.rmeta
│   │   │   ├── libparking_lot_core-ee9d663c04bf84d7.rmeta
│   │   │   ├── libclap_builder-a3a43c0869e4f951.rlib
│   │   │   ├── regex_automata-bc4ea5f7d2bd90cf.d
│   │   │   ├── libanstream-f24d63c1eea0a444.rlib
│   │   │   ├── libstrsim-942658eb75fe1300.rlib
│   │   │   ├── crossbeam_deque-0eac58662325bbde.d
│   │   │   ├── libhandlebars-e0b1082b3829ca3a.rmeta
│   │   │   ├── libcfg_if-f1eac65717ac6884.rlib
│   │   │   ├── once_cell-22db72f8e301a324.d
│   │   │   ├── libbit_set-8990b6bec7af34c8.rmeta
│   │   │   ├── itoa-089fde234b4f92a8.d
│   │   │   ├── walkdir-82834c6751d8ae99.d
│   │   │   ├── libserde-02bc200ce28fb8f4.rlib
│   │   │   ├── lazy_static-05264fb63cd23ad0.d
│   │   │   ├── libquote-ba95521fa6b0fe01.rlib
│   │   │   ├── libunicode_width-0737db22197487ca.rmeta
│   │   │   ├── libbit_set-3db5febc0a4a4dde.rlib
│   │   │   ├── libjwalk-0f9377beb62cf3d3.rmeta
│   │   │   ├── libpest-8433031303875c9a.rmeta
│   │   │   ├── bit_vec-f69f69641a0e5953.d
│   │   │   ├── libstrsim-942658eb75fe1300.rmeta
│   │   │   ├── libsmallvec-793c8bfd17fc7acd.rlib
│   │   │   ├── libnumber_prefix-feef2e60a784e16b.rmeta
│   │   │   ├── libthiserror-c448c2c299fbbf71.rlib
│   │   │   ├── libmemchr-ce996ae63a872f89.rmeta
│   │   │   ├── objc_id-f018c0d90b54e8c4.d
│   │   │   ├── libanstream-37bf9bad64549f21.rmeta
│   │   │   ├── libportable_atomic-00a8e7bb09605d7e.rmeta
│   │   │   ├── libc-1eb43e0b10e2cb73.d
│   │   │   ├── libparking_lot-70ba145b5e6a5164.rlib
│   │   │   ├── aho_corasick-079516c0c3447023.d
│   │   │   ├── libpest_generator-2d21a298232c8ebc.rlib
│   │   │   ├── libclap_builder-03171f2d20bf1d34.rlib
│   │   │   ├── libcli_clipboard-9664b3bb3697ff58.rlib
│   │   │   ├── libparking_lot_core-ee9d663c04bf84d7.rlib
│   │   │   ├── anstyle_query-e6ccdcd626343fa4.d
│   │   │   ├── libis_terminal_polyfill-52e45efd9a7905ac.rlib
│   │   │   ├── libregex_automata-bc4ea5f7d2bd90cf.rmeta
│   │   │   ├── libserde_json-9a69fa8f3bd55de9.rmeta
│   │   │   ├── liblock_api-193ac776843c816c.rlib
│   │   │   ├── librayon_core-507e7a546e621bbf.rmeta
│   │   │   ├── libignore-24e2b64405dae3c3.rlib
│   │   │   ├── base64-2d79316584b5f227.d
│   │   │   ├── libitoa-089fde234b4f92a8.rlib
│   │   │   ├── libbit_vec-06961ae571d31ebc.rmeta
│   │   │   ├── anstyle_parse-b0edfc8eafe3f601.d
│   │   │   ├── liblog-1763037ba5e87016.rmeta
│   │   │   ├── libserde_json-2aad57aa6c9128be.rmeta
│   │   │   ├── serde-84d3b8a8449a15b0.d
│   │   │   ├── libcrossbeam_epoch-47b3a4eb6ccdce77.rmeta
│   │   │   ├── serde-02bc200ce28fb8f4.d
│   │   │   ├── bit_vec-a11fe0a33f56b1aa.d
│   │   │   ├── objc_foundation-e3e805579b657518.d
│   │   │   ├── libcrossbeam_deque-103683c298e2c133.rmeta
│   │   │   ├── serde_json-9a69fa8f3bd55de9.d
│   │   │   ├── regex_automata-552ff0161f53a959.d
│   │   │   ├── libconsole-691781ea730960dd.rmeta
│   │   │   ├── libbase64-f3dae55d80987eb8.rmeta
│   │   │   ├── libconsole-02a6f3989daa7f57.rlib
│   │   │   ├── libparking_lot-9ed5a2f4348fa93a.rmeta
│   │   │   ├── libsmallvec-c3648dc45b17a810.rmeta
│   │   │   ├── taiko-f55931db94913f27.d
│   │   │   ├── syn-eefd0846265850c8.d
│   │   │   ├── libclap_lex-178e9b36476d4e9a.rlib
│   │   │   ├── libaho_corasick-079516c0c3447023.rmeta
│   │   │   ├── rustc_hash-19b2a7552500a495.d
│   │   │   ├── libis_terminal_polyfill-52e45efd9a7905ac.rmeta
│   │   │   ├── anstyle-c89c052f1dcc0847.d
│   │   │   ├── libindicatif-14b3f8ae01f24abd.rlib
│   │   │   ├── libbit_set-332521716022bab1.rmeta
│   │   │   ├── libbit_vec-f69f69641a0e5953.rlib
│   │   │   ├── libsame_file-5bb213682242dec5.rlib
│   │   │   ├── libonce_cell-7cf040c7a18f5efc.rlib
│   │   │   ├── libregex_automata-552ff0161f53a959.rmeta
│   │   │   ├── libclap_lex-0e39550645e2de8c.rmeta
│   │   │   ├── libsyn-1f4a51a15691025b.rmeta
│   │   │   ├── liblock_api-439e265ca667b05b.rmeta
│   │   │   ├── libryu-f854f3e2d0bc7970.rmeta
│   │   │   ├── libanstyle_query-e6ccdcd626343fa4.rlib
│   │   │   ├── quote-ba95521fa6b0fe01.d
│   │   │   ├── heck-362cf1f27b4900dd.d
│   │   │   ├── libobjc-8bf9cf4cdd832e27.rlib
│   │   │   ├── colored-f39e864bdda671a4.d
│   │   │   ├── utf8parse-241bde393834c15a.d
│   │   │   ├── taiko-7d945277cdce9ad2.d
│   │   │   ├── regex-ace3a047af7df117.d
│   │   │   ├── libindicatif-26e032c4c48d7a58.rmeta
│   │   │   ├── libtermtree-b50d791c107835c3.rmeta
│   │   │   ├── libignore-4a87f3ab4ddc78c6.rlib
│   │   │   ├── libbstr-bdf27114869b0552.rmeta
│   │   │   ├── libcfg_if-fad10b9109b556b8.rlib
│   │   │   ├── same_file-c9a50bc6410f240f.d
│   │   │   ├── syn-1f4a51a15691025b.d
│   │   │   ├── libmemchr-47ad014842134a2e.rmeta
│   │   │   ├── liblog-2de049cddbc115fd.rmeta
│   │   │   ├── libbit_set-8990b6bec7af34c8.rlib
│   │   │   ├── libtiktoken_rs-14dffd894fc80d37.rmeta
│   │   │   ├── anstream-37bf9bad64549f21.d
│   │   │   ├── libanstyle_query-565df28b464e5999.rmeta
│   │   │   ├── libobjc_foundation-e3e805579b657518.rlib
│   │   │   ├── lock_api-439e265ca667b05b.d
│   │   │   ├── regex_syntax-3dc0aea71a24ebb7.d
│   │   │   ├── libunindent-7fd69610340f4601.rlib
│   │   │   ├── libonce_cell-22db72f8e301a324.rmeta
│   │   │   ├── bit_vec-06961ae571d31ebc.d
│   │   │   ├── libcolorchoice-ec8ac6fa39b94aaa.rlib
│   │   │   ├── taiko-7d945277cdce9ad2
│   │   │   ├── libserde_json-2aad57aa6c9128be.rlib
│   │   │   ├── libprompt_macro-1212f8861409456d.dylib
│   │   │   ├── libbase64-336c9298377507a2.rmeta
│   │   │   ├── libparking_lot_core-060f246501188fa1.rmeta
│   │   │   ├── libsmallvec-c3648dc45b17a810.rlib
│   │   │   ├── memchr-ce996ae63a872f89.d
│   │   │   ├── libtermtree-ba9786ca0305b61e.rmeta
│   │   │   ├── libonce_cell-e327dfb7dbd56683.rmeta
│   │   │   ├── libclap-340c1f12a53f5ef7.rlib
│   │   │   ├── libglobset-42b9568d3419ec9a.rlib
│   │   │   ├── libproc_macro2-919e63ef4dc60bd1.rmeta
│   │   │   ├── libtermtree-b50d791c107835c3.rlib
│   │   │   ├── unicode_width-24baffe62a47adf8.d
│   │   │   ├── libcrossbeam_utils-c0cf686b755ca63d.rlib
│   │   │   ├── libitoa-1d5aa20e784c1230.rmeta
│   │   │   ├── bstr-864c64da054cbdda.d
│   │   │   ├── libparse_format-d4f6b9211f8e5fe1.rlib
│   │   │   ├── libclap-ae186d62c68ba56b.rlib
│   │   │   ├── libhandlebars-e0b1082b3829ca3a.rlib
│   │   │   ├── libnumber_prefix-f55dca790810ab17.rlib
│   │   │   ├── libcolored-9dbbfa6ce05a3ae9.rlib
│   │   │   ├── anstyle_parse-10471dc433a0c21a.d
│   │   │   ├── libparking_lot_core-060f246501188fa1.rlib
│   │   │   ├── libcrossbeam-30a03ffc8858233b.rmeta
│   │   │   ├── anstream-f24d63c1eea0a444.d
│   │   │   ├── clap_lex-0e39550645e2de8c.d
│   │   │   ├── libparking_lot-70ba145b5e6a5164.rmeta
│   │   │   ├── libprompt_macro-ed7428a7428d8f57.dylib
│   │   │   ├── anyhow-30c7250151378d39.d
│   │   │   ├── crossbeam_utils-169afb23be825b5c.d
│   │   │   ├── libbstr-864c64da054cbdda.rmeta
│   │   │   ├── autocfg-c0c39e025eafae2a.d
│   │   │   ├── libanstyle_query-565df28b464e5999.rlib
│   │   │   ├── regex_syntax-e94c39df99344eca.d
│   │   │   ├── libconsole-691781ea730960dd.rlib
│   │   │   ├── ucd_trie-b6376f1e4159930f.d
│   │   │   ├── liblazy_static-59e7fd1a9e99a10b.rlib
│   │   │   ├── librayon_core-507e7a546e621bbf.rlib
│   │   │   ├── libbase64-f3dae55d80987eb8.rlib
│   │   │   ├── libcrossbeam_deque-aa2614d487e78a9a.rlib
│   │   │   ├── libutf8parse-13bdde9eeeac7f70.rmeta
│   │   │   ├── libwalkdir-a068a60a07f55af8.rmeta
│   │   │   ├── cfg_if-f1eac65717ac6884.d
│   │   │   ├── bstr-bdf27114869b0552.d
│   │   │   ├── libmemchr-23c1f02c01ab1843.rmeta
│   │   │   ├── libanstyle_parse-b0edfc8eafe3f601.rmeta
│   │   │   ├── libcrossbeam_queue-cbd3cfb4ee35919c.rlib
│   │   │   ├── libcfg_if-fad10b9109b556b8.rmeta
│   │   │   ├── libsmallvec-dedf8e8465e06278.rmeta
│   │   │   ├── libmemchr-f81761c04e7279ee.rmeta
│   │   │   ├── libsame_file-c9a50bc6410f240f.rlib
│   │   │   ├── thiserror-c448c2c299fbbf71.d
│   │   │   ├── thiserror_impl-126013681a33cb39.d
│   │   │   ├── taiko-dcd62d6b1c9858cd
│   │   │   ├── colorchoice-4a3a43883c734655.d
│   │   │   ├── libquote-50dcc41d8c869384.rmeta
│   │   │   ├── libbit_set-3db5febc0a4a4dde.rmeta
│   │   │   ├── block-b6691600ce4c15c0.d
│   │   │   ├── libscopeguard-7d025d7bea8d1c0a.rmeta
│   │   │   ├── libunicode_width-24baffe62a47adf8.rmeta
│   │   │   ├── libserde-84d3b8a8449a15b0.rlib
│   │   │   ├── libfancy_regex-f197c19fd02d3f04.rlib
│   │   │   ├── libproc_macro2-ff957dbded91c6b1.rmeta
│   │   │   ├── liblibc-1eb43e0b10e2cb73.rmeta
│   │   │   ├── liblibc-25bb789ecd625a35.rlib
│   │   │   ├── libindicatif-14b3f8ae01f24abd.rmeta
│   │   │   ├── liblazy_static-fd8aebbacaa947fb.rmeta
│   │   │   ├── clap_derive-669d895d5854291e.d
│   │   │   ├── libconsole-02a6f3989daa7f57.rmeta
│   │   │   ├── anstream-8b1a55f7013a63de.d
│   │   │   ├── libcrossbeam_deque-aa2614d487e78a9a.rmeta
│   │   │   ├── clap-da87c120b9bdbdbc.d
│   │   │   ├── libscopeguard-f744bb069bca91ef.rmeta
│   │   │   ├── liblazy_static-05264fb63cd23ad0.rlib
│   │   │   ├── libc-25bb789ecd625a35.d
│   │   │   ├── libheck-eeeb58b6f76d070c.rlib
│   │   │   ├── libbase64-2d79316584b5f227.rmeta
│   │   │   ├── libbstr-864c64da054cbdda.rlib
│   │   │   ├── ryu-f854f3e2d0bc7970.d
│   │   │   ├── bit_set-3db5febc0a4a4dde.d
│   │   │   ├── ignore-a6ee6b89288ab772.d
│   │   │   ├── parse_format-6994095d36e95c76.d
│   │   │   ├── strsim-dad156f3ed68cf6f.d
│   │   │   ├── libanstyle_query-8ab2c4db6a4c18e5.rlib
│   │   │   ├── parking_lot_core-060f246501188fa1.d
│   │   │   ├── libsame_file-5bb213682242dec5.rmeta
│   │   │   ├── clap_derive-cce90712b1194b53.d
│   │   │   ├── colorchoice-55672a04332e1a9b.d
│   │   │   ├── libparking_lot-57121538e2a65a76.rmeta
│   │   │   ├── libanyhow-c1636100da306932.rmeta
│   │   │   ├── libclap_builder-c4fe569c7473fac2.rmeta
│   │   │   ├── libunicode_width-0737db22197487ca.rlib
│   │   │   ├── libcrossbeam_utils-169afb23be825b5c.rlib
│   │   │   ├── libcolored-9dbbfa6ce05a3ae9.rmeta
│   │   │   ├── libcrossbeam_utils-c0cf686b755ca63d.rmeta
│   │   │   ├── libryu-4ff3e018a55a59fb.rlib
│   │   │   ├── unicode_ident-e63abeae6024e13f.d
│   │   │   ├── ignore-24e2b64405dae3c3.d
│   │   │   ├── libunicode_ident-db0e24d7d1309e5f.rlib
│   │   │   ├── libthiserror-b8e957c9de280bc3.rlib
│   │   │   ├── libclap_builder-c4fe569c7473fac2.rlib
│   │   │   ├── libobjc_foundation-e3e805579b657518.rmeta
│   │   │   ├── parking_lot-9ed5a2f4348fa93a.d
│   │   │   ├── regex-33bf253257e85501.d
│   │   │   ├── libryu-d113a3a4eafc30cf.rmeta
│   │   │   ├── libregex_syntax-0d7fe56b75ad5078.rmeta
│   │   │   ├── liblazy_static-05264fb63cd23ad0.rmeta
│   │   │   ├── libtiktoken_rs-62137a7f978c2c61.rlib
│   │   │   ├── libunindent-edeee594be933f44.rlib
│   │   │   ├── quote-50dcc41d8c869384.d
│   │   │   ├── libclap_builder-a3a43c0869e4f951.rmeta
│   │   │   ├── bstr-62c8bef8e949b2ba.d
│   │   │   ├── regex-c34b18d69c7d70a9.d
│   │   │   ├── libparse_format-6994095d36e95c76.rmeta
│   │   │   ├── rustc_hash-21e4b86214e09860.d
│   │   │   ├── utf8parse-234f8135fee3007f.d
│   │   │   ├── proc_macro2-6f972a4c0d651058.d
│   │   │   ├── walkdir-cd19d39f5b265f3d.d
│   │   │   ├── libobjc_id-f018c0d90b54e8c4.rmeta
│   │   │   ├── utf8parse-13bdde9eeeac7f70.d
│   │   │   ├── libregex-33bf253257e85501.rmeta
│   │   │   ├── itoa-42c9d698521056f5.d
│   │   │   ├── is_terminal_polyfill-b0c6c4fcc5a4dfdc.d
│   │   │   ├── libregex_syntax-e94c39df99344eca.rmeta
│   │   │   ├── libanstyle-c89c052f1dcc0847.rlib
│   │   │   ├── liblibc-a19978263b9b2a8c.rmeta
│   │   │   ├── strsim-942658eb75fe1300.d
│   │   │   ├── libregex_syntax-0d7fe56b75ad5078.rlib
│   │   │   ├── handlebars-e0b1082b3829ca3a.d
│   │   │   ├── clap_builder-03171f2d20bf1d34.d
│   │   │   ├── libmemchr-f81761c04e7279ee.rlib
│   │   │   ├── regex_syntax-0d7fe56b75ad5078.d
│   │   │   ├── libstrsim-dad156f3ed68cf6f.rmeta
│   │   │   ├── globset-e28de3350cb4d24a.d
│   │   │   ├── libbit_vec-f69f69641a0e5953.rmeta
│   │   │   ├── libserde_json-74b1893772dffc57.rlib
│   │   │   ├── libtiktoken_rs-14dffd894fc80d37.rlib
│   │   │   ├── libmemchr-23c1f02c01ab1843.rlib
│   │   │   ├── libparking_lot-9ed5a2f4348fa93a.rlib
│   │   │   ├── libsame_file-97cee3f9797c5614.rlib
│   │   │   ├── autocfg-b1ddef1e4bff8483.d
│   │   │   ├── libutf8parse-234f8135fee3007f.rlib
│   │   │   ├── clap_builder-c4fe569c7473fac2.d
│   │   │   ├── libwalkdir-82834c6751d8ae99.rmeta
│   │   │   ├── libbit_vec-06961ae571d31ebc.rlib
│   │   │   ├── libryu-d113a3a4eafc30cf.rlib
│   │   │   ├── parking_lot-57121538e2a65a76.d
│   │   │   ├── libanyhow-fcf5cec6dd3dfc3e.rmeta
│   │   │   ├── libignore-a6ee6b89288ab772.rlib
│   │   │   ├── libblock-b6691600ce4c15c0.rmeta
│   │   │   ├── librustc_hash-21e4b86214e09860.rmeta
│   │   │   ├── prompt_macro-1212f8861409456d.d
│   │   │   ├── libignore-4a87f3ab4ddc78c6.rmeta
│   │   │   ├── libpest_meta-2d0dbfc24ff8c66e.rmeta
│   │   │   ├── libcrossbeam_epoch-47b3a4eb6ccdce77.rlib
│   │   │   ├── cfg_if-407b3ce8be06afa8.d
│   │   │   ├── libwalkdir-82834c6751d8ae99.rlib
│   │   │   ├── is_terminal_polyfill-52e45efd9a7905ac.d
│   │   │   ├── libheck-72df6f2fd216bb0b.rlib
│   │   │   ├── libglobset-243c7c1921aad0a7.rlib
│   │   │   ├── libwalkdir-a068a60a07f55af8.rlib
│   │   │   ├── lock_api-2e0ba9efb1d81afa.d
│   │   │   ├── libmemchr-ce996ae63a872f89.rlib
│   │   │   ├── libscopeguard-f744bb069bca91ef.rlib
│   │   │   ├── objc-8bf9cf4cdd832e27.d
│   │   │   ├── parking_lot-70ba145b5e6a5164.d
│   │   │   ├── libserde-b3e3f6caab443231.rmeta
│   │   │   ├── walkdir-a068a60a07f55af8.d
│   │   │   ├── libanstyle_query-e6ccdcd626343fa4.rmeta
│   │   │   ├── clap_lex-178e9b36476d4e9a.d
│   │   │   ├── log-df0c3a43a2d81184.d
│   │   │   ├── libtermtree-00916f8ce7930209.rmeta
│   │   │   ├── thiserror-b8e957c9de280bc3.d
│   │   │   ├── clap_derive-8e7fba42bc14a27f.d
│   │   │   ├── libucd_trie-b6376f1e4159930f.rlib
│   │   │   ├── libfancy_regex-ba7b780a240ccf29.rmeta
│   │   │   ├── libaho_corasick-81db9c5633441623.rlib
│   │   │   ├── autocfg-999daad8563db8a9.d
│   │   │   ├── libis_terminal_polyfill-b0c6c4fcc5a4dfdc.rmeta
│   │   │   ├── libregex-c34b18d69c7d70a9.rlib
│   │   │   ├── pest-99335c19396f192f.d
│   │   │   ├── regex_automata-5ddb58acb04b6c99.d
│   │   │   ├── libcrossbeam_channel-4af5578a8855579c.rlib
│   │   │   ├── libcfg_if-407b3ce8be06afa8.rlib
│   │   │   ├── quote-c2116f873725ad3b.d
│   │   │   ├── libstrsim-676837c575daf54e.rlib
│   │   │   ├── libfancy_regex-f197c19fd02d3f04.rmeta
│   │   │   ├── libanstyle-c89c052f1dcc0847.rmeta
│   │   │   ├── libserde-84d3b8a8449a15b0.rmeta
│   │   │   ├── libconsole-aa2c177e5962a4d2.rlib
│   │   │   ├── libregex_automata-5ddb58acb04b6c99.rlib
│   │   │   ├── libfancy_regex-ba7b780a240ccf29.rlib
│   │   │   ├── ryu-d113a3a4eafc30cf.d
│   │   │   ├── libunicode_ident-5929dd8d837653ce.rmeta
│   │   │   ├── clap_lex-b9cf91260205bec3.d
│   │   │   ├── libeither-49d65d10f088e6bd.rlib
│   │   │   ├── libindicatif-d22f6c15ea25134b.rlib
│   │   │   ├── anstyle_query-8ab2c4db6a4c18e5.d
│   │   │   ├── libcrossbeam_deque-0eac58662325bbde.rlib
│   │   │   ├── libsyn-1f4a51a15691025b.rlib
│   │   │   ├── libautocfg-999daad8563db8a9.rmeta
│   │   │   ├── taiko-f55931db94913f27
│   │   │   ├── libitoa-089fde234b4f92a8.rmeta
│   │   │   ├── libquote-c2116f873725ad3b.rlib
│   │   │   ├── console-02a6f3989daa7f57.d
│   │   │   ├── log-2de049cddbc115fd.d
│   │   │   ├── anstyle_query-565df28b464e5999.d
│   │   │   ├── libunicode_width-e0be5e6b79a54c74.rmeta
│   │   │   ├── colorchoice-ec8ac6fa39b94aaa.d
│   │   │   ├── libregex_automata-5ddb58acb04b6c99.rmeta
│   │   │   ├── libparse_format-6994095d36e95c76.rlib
│   │   │   ├── unicode_width-0737db22197487ca.d
│   │   │   ├── libregex-33bf253257e85501.rlib
│   │   │   ├── same_file-97cee3f9797c5614.d
│   │   │   ├── proc_macro2-919e63ef4dc60bd1.d
│   │   │   ├── clap-ae186d62c68ba56b.d
│   │   │   ├── libpest_derive-0d8c12727e936794.dylib
│   │   │   ├── libpest-99335c19396f192f.rmeta
│   │   │   ├── rustc_hash-fd56defa4d4e0515.d
│   │   │   ├── libindicatif-26e032c4c48d7a58.rlib
│   │   │   ├── librustc_hash-19b2a7552500a495.rmeta
│   │   │   ├── libanstyle-a10b55c910c671e5.rlib
│   │   │   ├── smallvec-dedf8e8465e06278.d
│   │   │   ├── libcolorchoice-55672a04332e1a9b.rmeta
│   │   │   ├── libindicatif-d22f6c15ea25134b.rmeta
│   │   │   ├── memchr-23c1f02c01ab1843.d
│   │   │   ├── crossbeam-30a03ffc8858233b.d
│   │   │   ├── scopeguard-7d025d7bea8d1c0a.d
│   │   │   ├── libclap-da87c120b9bdbdbc.rmeta
│   │   │   ├── heck-72df6f2fd216bb0b.d
│   │   │   ├── libparking_lot-57121538e2a65a76.rlib
│   │   │   ├── libautocfg-c0c39e025eafae2a.rlib
│   │   │   ├── libserde_json-74b1893772dffc57.rmeta
│   │   │   ├── libanyhow-30c7250151378d39.rlib
│   │   │   ├── fancy_regex-0c8eb8d613294a6e.d
│   │   │   ├── libtiktoken_rs-0225d32dda2f5a6d.rmeta
│   │   │   ├── same_file-5bb213682242dec5.d
│   │   │   ├── libserde-b3e3f6caab443231.rlib
│   │   │   ├── libutf8parse-13bdde9eeeac7f70.rlib
│   │   │   ├── libitoa-1d5aa20e784c1230.rlib
│   │   │   ├── libwalkdir-cd19d39f5b265f3d.rlib
│   │   │   ├── indicatif-26e032c4c48d7a58.d
│   │   │   ├── libcolorchoice-4a3a43883c734655.rmeta
│   │   │   ├── libclap-ae186d62c68ba56b.rmeta
│   │   │   ├── unindent-edeee594be933f44.d
│   │   │   ├── unicode_width-e0be5e6b79a54c74.d
│   │   │   ├── libcrossbeam_epoch-a5a2e5c50e172349.rmeta
│   │   │   ├── librayon-d788a88286e03a57.rmeta
│   │   │   ├── rayon_core-507e7a546e621bbf.d
│   │   │   ├── libunicode_ident-db0e24d7d1309e5f.rmeta
│   │   │   ├── tiktoken_rs-62137a7f978c2c61.d
│   │   │   ├── libcolored-3c7106d6aeef1a52.rlib
│   │   │   ├── libanstyle_parse-b0edfc8eafe3f601.rlib
│   │   │   ├── taiko-dcd62d6b1c9858cd.d
│   │   │   ├── libaho_corasick-81db9c5633441623.rmeta
│   │   │   ├── libautocfg-b1ddef1e4bff8483.rlib
│   │   │   ├── libcrossbeam_deque-0eac58662325bbde.rmeta
│   │   │   ├── libregex_automata-bc4ea5f7d2bd90cf.rlib
│   │   │   ├── libportable_atomic-ff4399801b1a298d.rmeta
│   │   │   ├── libjwalk-0f9377beb62cf3d3.rlib
│   │   │   ├── libsmallvec-dedf8e8465e06278.rlib
│   │   │   ├── libtermtree-ba9786ca0305b61e.rlib
│   │   │   ├── indicatif-d22f6c15ea25134b.d
│   │   │   ├── libportable_atomic-2dd4b74eb6ccbb4e.rlib
│   │   │   ├── termtree-00916f8ce7930209.d
│   │   │   ├── anyhow-c1636100da306932.d
│   │   │   ├── libanstyle_query-8ab2c4db6a4c18e5.rmeta
│   │   │   ├── libnumber_prefix-f55dca790810ab17.rmeta
│   │   │   ├── anyhow-fcf5cec6dd3dfc3e.d
│   │   │   ├── memchr-47ad014842134a2e.d
│   │   │   ├── libcrossbeam_utils-b669fe86b38ddee9.rlib
│   │   │   ├── libcolorchoice-4a3a43883c734655.rlib
│   │   │   ├── termtree-b50d791c107835c3.d
│   │   │   ├── libpest_meta-2d0dbfc24ff8c66e.rlib
│   │   │   ├── liblog-2de049cddbc115fd.rlib
│   │   │   ├── crossbeam_epoch-47b3a4eb6ccdce77.d
│   │   │   ├── libserde-02bc200ce28fb8f4.rmeta
│   │   │   ├── libcrossbeam_epoch-20ca9b22b8077bdc.rmeta
│   │   │   ├── libregex-ace3a047af7df117.rlib
│   │   │   ├── pest_generator-2d21a298232c8ebc.d
│   │   │   ├── tiktoken_rs-14dffd894fc80d37.d
│   │   │   ├── ucd_trie-12c6fb9c1feff04c.d
│   │   │   ├── serde_json-2aad57aa6c9128be.d
│   │   │   ├── pest_meta-2d0dbfc24ff8c66e.d
│   │   │   ├── lazy_static-59e7fd1a9e99a10b.d
│   │   │   ├── anstyle-a10b55c910c671e5.d
│   │   │   ├── liblock_api-193ac776843c816c.rmeta
│   │   │   ├── libitoa-42c9d698521056f5.rlib
│   │   │   ├── libregex-ace3a047af7df117.rmeta
│   │   │   ├── liblibc-1eb43e0b10e2cb73.rlib
│   │   │   ├── libaho_corasick-daae1a193e77c697.rlib
│   │   │   ├── libunicode_ident-5929dd8d837653ce.rlib
│   │   │   ├── libportable_atomic-2dd4b74eb6ccbb4e.rmeta
│   │   │   ├── libcolorchoice-55672a04332e1a9b.rlib
│   │   │   ├── libparking_lot_core-cdf873ae7c907ae1.rmeta
│   │   │   ├── lock_api-193ac776843c816c.d
│   │   │   └── colored-9dbbfa6ce05a3ae9.d
│   │   └── build
│   │       ├── lock_api-93f46a0f6600e65e
│   │       │   ├── build_script_build-93f46a0f6600e65e
│   │       │   ├── build_script_build-93f46a0f6600e65e.d
│   │       │   └── build-script-build
│   │       ├── parking_lot_core-7b414e808cc8a664
│   │       │   ├── build_script_build-7b414e808cc8a664
│   │       │   ├── build_script_build-7b414e808cc8a664.d
│   │       │   └── build-script-build
│   │       ├── anyhow-a091b4cd7dc48367
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde_json-8cf628c6f64ea3b3
│   │       │   ├── build_script_build-8cf628c6f64ea3b3
│   │       │   ├── build_script_build-8cf628c6f64ea3b3.d
│   │       │   └── build-script-build
│   │       ├── portable-atomic-cbe761dbb2e56387
│   │       │   ├── build_script_build-cbe761dbb2e56387.d
│   │       │   ├── build_script_build-cbe761dbb2e56387
│   │       │   └── build-script-build
│   │       ├── crossbeam-utils-26865dba3e481b21
│   │       │   ├── build_script_build-26865dba3e481b21.d
│   │       │   ├── build_script_build-26865dba3e481b21
│   │       │   └── build-script-build
│   │       ├── serde_json-7d1be0e058c15c26
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── rayon-core-76abf9d66ccd57d9
│   │       │   ├── build_script_build-76abf9d66ccd57d9
│   │       │   ├── build-script-build
│   │       │   └── build_script_build-76abf9d66ccd57d9.d
│   │       ├── anyhow-a200ab83f2e6d3a2
│   │       │   ├── build_script_build-a200ab83f2e6d3a2
│   │       │   ├── build_script_build-a200ab83f2e6d3a2.d
│   │       │   └── build-script-build
│   │       ├── lock_api-a400f8a5cc30ee13
│   │       │   ├── out
│   │       │   │   └── autocfg_c626b9f9d739aed9_0.ll
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde_json-2e2fb7eff5db2c08
│   │       │   ├── build_script_build-2e2fb7eff5db2c08.d
│   │       │   ├── build_script_build-2e2fb7eff5db2c08
│   │       │   └── build-script-build
│   │       ├── proc-macro2-cf5f604d36dc7a38
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── lock_api-0ffbaafdf4ad1c9c
│   │       │   ├── out
│   │       │   │   └── autocfg_54e0952b1b5c1d91_0.ll
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── portable-atomic-42432f849b3c7cb4
│   │       │   ├── build_script_build-42432f849b3c7cb4.d
│   │       │   ├── build_script_build-42432f849b3c7cb4
│   │       │   └── build-script-build
│   │       ├── portable-atomic-124a97b8ccfc4948
│   │       │   ├── build_script_build-124a97b8ccfc4948.d
│   │       │   ├── build_script_build-124a97b8ccfc4948
│   │       │   └── build-script-build
│   │       ├── lock_api-582ee046782621db
│   │       │   ├── out
│   │       │   │   └── probe0.ll
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── proc-macro2-7859dadbd2d1ad76
│   │       │   ├── build_script_build-7859dadbd2d1ad76.d
│   │       │   ├── build_script_build-7859dadbd2d1ad76
│   │       │   └── build-script-build
│   │       ├── portable-atomic-8e08ac94a2738537
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── thiserror-1ee99c6e96769780
│   │       │   ├── out
│   │       │   │   └── thiserror.d
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── thiserror-b378c8de0bf03c56
│   │       │   ├── build_script_build-b378c8de0bf03c56.d
│   │       │   ├── build_script_build-b378c8de0bf03c56
│   │       │   └── build-script-build
│   │       ├── libc-6a4dea2c0a9fd5e2
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── libc-5a2ec3725e0e8cef
│   │       │   ├── build_script_build-5a2ec3725e0e8cef
│   │       │   ├── build_script_build-5a2ec3725e0e8cef.d
│   │       │   └── build-script-build
│   │       ├── serde_json-40e4a683080fed50
│   │       │   ├── build_script_build-40e4a683080fed50
│   │       │   ├── build-script-build
│   │       │   └── build_script_build-40e4a683080fed50.d
│   │       ├── crossbeam-utils-02bcd52b002fb486
│   │       │   ├── build_script_build-02bcd52b002fb486.d
│   │       │   ├── build_script_build-02bcd52b002fb486
│   │       │   └── build-script-build
│   │       ├── crossbeam-utils-3c63712aee8ad31a
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde_json-5f266a2ffb04b8e9
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── rayon-core-ffc8f37c34c6bcd2
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── parking_lot_core-dd56bfc521ddfbf2
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde-5b0156270c4031bb
│   │       │   ├── build_script_build-5b0156270c4031bb
│   │       │   ├── build-script-build
│   │       │   └── build_script_build-5b0156270c4031bb.d
│   │       ├── crossbeam-utils-ffe9c2e226adc67f
│   │       │   ├── build_script_build-ffe9c2e226adc67f
│   │       │   ├── build_script_build-ffe9c2e226adc67f.d
│   │       │   └── build-script-build
│   │       ├── portable-atomic-ecd13d08fd0b38ec
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── libc-3cd929236be2f1f6
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── proc-macro2-d893a28c929bfebd
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde-c4b3fbec58b8f717
│   │       │   ├── build_script_build-c4b3fbec58b8f717
│   │       │   ├── build_script_build-c4b3fbec58b8f717.d
│   │       │   └── build-script-build
│   │       ├── lock_api-4c2547368e352307
│   │       │   ├── build_script_build-4c2547368e352307
│   │       │   ├── build_script_build-4c2547368e352307.d
│   │       │   └── build-script-build
│   │       ├── lock_api-14c47a8264d6e481
│   │       │   ├── build_script_build-14c47a8264d6e481
│   │       │   ├── build_script_build-14c47a8264d6e481.d
│   │       │   └── build-script-build
│   │       ├── parking_lot_core-555fca84dcd699e3
│   │       │   ├── build_script_build-555fca84dcd699e3
│   │       │   ├── build_script_build-555fca84dcd699e3.d
│   │       │   └── build-script-build
│   │       ├── serde_json-f84b20b85c3c2187
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── anyhow-06615564e6f1c958
│   │       │   ├── build_script_build-06615564e6f1c958
│   │       │   ├── build-script-build
│   │       │   └── build_script_build-06615564e6f1c958.d
│   │       ├── parking_lot_core-e3e5be45e3154b11
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── libc-d1e2fb59fe6f5ff4
│   │       │   ├── build_script_build-d1e2fb59fe6f5ff4
│   │       │   ├── build-script-build
│   │       │   └── build_script_build-d1e2fb59fe6f5ff4.d
│   │       ├── thiserror-d92368118985b354
│   │       │   ├── out
│   │       │   │   └── thiserror.d
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── parking_lot_core-3c3480ffa2df55d0
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde-3bc99fc33d70cf87
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde-7a6314b4a78f6a93
│   │       │   ├── build_script_build-7a6314b4a78f6a93
│   │       │   ├── build_script_build-7a6314b4a78f6a93.d
│   │       │   └── build-script-build
│   │       ├── libc-2858bc19af6b2c4e
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── crossbeam-utils-2c61182232f0172d
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── parking_lot_core-f0918f1023f612e2
│   │       │   ├── build_script_build-f0918f1023f612e2.d
│   │       │   ├── build_script_build-f0918f1023f612e2
│   │       │   └── build-script-build
│   │       ├── crossbeam-utils-56c0ed1b478c118e
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde-a157e6cd2cc2aa25
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── anyhow-04efd0959cd0d7e1
│   │       │   ├── build_script_build-04efd0959cd0d7e1.d
│   │       │   ├── build_script_build-04efd0959cd0d7e1
│   │       │   └── build-script-build
│   │       ├── portable-atomic-ce8e51eda864b3b6
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── serde-bd1507a17571b635
│   │       │   ├── out
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── proc-macro2-a43fe1062415b5be
│   │       │   ├── out
│   │       │   │   └── proc_macro2.d
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── anyhow-161213d1cc945bff
│   │       │   ├── out
│   │       │   │   └── anyhow.d
│   │       │   ├── stderr
│   │       │   ├── output
│   │       │   ├── root-output
│   │       │   └── invoked.timestamp
│   │       ├── libc-2223c27cfa6ecf70
│   │       │   ├── build_script_build-2223c27cfa6ecf70.d
│   │       │   ├── build_script_build-2223c27cfa6ecf70
│   │       │   └── build-script-build
│   │       ├── proc-macro2-f1fdfaef733fc8d1
│   │       │   ├── build_script_build-f1fdfaef733fc8d1.d
│   │       │   ├── build_script_build-f1fdfaef733fc8d1
│   │       │   └── build-script-build
│   │       ├── proc-macro2-0eb273b8504a6cca
│   │       │   ├── build_script_build-0eb273b8504a6cca.d
│   │       │   ├── build_script_build-0eb273b8504a6cca
│   │       │   └── build-script-build
│   │       └── anyhow-157b02da119bf892
│   │           ├── out
│   │           ├── stderr
│   │           ├── output
│   │           ├── root-output
│   │           └── invoked.timestamp
│   └── debug
│       ├── taiko.d
│       ├── incremental
│       │   ├── taiko-2hfaz2d3h6k3k
│       │   │   ├── s-h363oqxqi7-05qkufl-4djysof61is1p8qguny8eegvh
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-h363oqxqi7-05qkufl.lock
│       │   ├── taiko-1mpt094ptlyyl
│       │   │   ├── s-gucqrhrcxo-1qjhta0.lock
│       │   │   └── s-gucqrhrcxo-1qjhta0-id5xpy83cl5udc7as6vxi85u
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   ├── taiko-3656ipg7xrpml
│       │   │   ├── s-gwwpu4vu8z-1x6uwx3.lock
│       │   │   └── s-gwwpu4vu8z-1x6uwx3-aozy0p7t1j7b5jiwvn45rbygn
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   ├── code2prompt-32etjpfteqxfv
│       │   │   ├── s-gucr53gui5-1rk0gpn-3417gbx8rgfjjyw2cb8m1meat
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-gucr53gui5-1rk0gpn.lock
│       │   ├── taiko-ghdbuulbgopm
│       │   │   ├── s-gudqphu0nh-1ix5k92.lock
│       │   │   └── s-gudqphu0nh-1ix5k92-bgxxs8ir0s0wkx2qsm8uivv4k
│       │   │       ├── 24yzy8m9w4yr4diy.o
│       │   │       ├── 26k7fh38dj7pf20i.o
│       │   │       ├── 1hhoo4rerd11y0n3.o
│       │   │       ├── 38ps3ab9xikm79mc.o
│       │   │       ├── 1app0h7cjp1v7gui.o
│       │   │       ├── 2jkvv7jbolzny3i.o
│       │   │       ├── 1yhsq1aczi4dgjy0.o
│       │   │       ├── 4txfsig44eip83du.o
│       │   │       ├── 4wz8jsbck3c2e9by.o
│       │   │       ├── 3cn0bh76q38ki1on.o
│       │   │       ├── 2eb8hqgkjdpgoo97.o
│       │   │       ├── q71g0mayyajssbc.o
│       │   │       ├── 40hrvv4ddb6fyt2z.o
│       │   │       ├── 41otuwf3pxp6hjab.o
│       │   │       ├── 14tihf9h5rss9kvy.o
│       │   │       ├── ru37bqi7y0d4sif.o
│       │   │       ├── 1fk0y92xfp8yx7lq.o
│       │   │       ├── 4e9ce35ptscxgt6l.o
│       │   │       ├── xy62o1xeynn6zck.o
│       │   │       ├── 4l6pjztib7q2kdj5.o
│       │   │       ├── 2djygn87l3fcp5ab.o
│       │   │       ├── 443bnhynv3g0ltnw.o
│       │   │       ├── 3vwb3rd1bg0vwzzj.o
│       │   │       ├── 30i3uwesufd2ga57.o
│       │   │       ├── 3mzadfyojbg48g48.o
│       │   │       ├── 2smxdnkjonxooqi7.o
│       │   │       ├── 4e47vuwwx3zw1mu4.o
│       │   │       ├── r0dfy7rj9qqd4ia.o
│       │   │       ├── 4wxtfrlpiyp4axhv.o
│       │   │       ├── 357w6yde94a1rjrc.o
│       │   │       ├── ct58ozvf54cbnnk.o
│       │   │       ├── 4or24krol3u9p7x.o
│       │   │       ├── 587vwv27xrlz7gc5.o
│       │   │       ├── 3jr0tiddpq7oxcq9.o
│       │   │       ├── 1qcpk7paqwuwog80.o
│       │   │       ├── 1l9bv1l55ubc6bf0.o
│       │   │       ├── 26r61b9oymxj5bj6.o
│       │   │       ├── 4ig3nlhcd4cqyowo.o
│       │   │       ├── 2aysip9oi0iu989k.o
│       │   │       ├── 400erkvo81kk5c9b.o
│       │   │       ├── 24lqk1tq9pqzqefs.o
│       │   │       ├── 36p584sk276h648y.o
│       │   │       ├── 53k8o2oqxnzb5ynh.o
│       │   │       ├── 5ckoge74ehk0oboj.o
│       │   │       ├── 4vxtzz7j739mji19.o
│       │   │       ├── 234mmik7zlm3ignv.o
│       │   │       ├── 27vk6pcr0kyskrcy.o
│       │   │       ├── 2yvhcb0mt0nincxy.o
│       │   │       ├── 4cp4ckdcxgw1y31s.o
│       │   │       ├── 77v5mozdpm42kz.o
│       │   │       ├── 1xkk5uo8l7vm7ut1.o
│       │   │       ├── k8moi8de20ivu73.o
│       │   │       ├── 595sew38pf6l0hpt.o
│       │   │       ├── zhzcm7yrvamrbkj.o
│       │   │       ├── query-cache.bin
│       │   │       ├── zq9upsxmwybvmob.o
│       │   │       ├── 7imnbsgddsm3x6h.o
│       │   │       ├── 49twn26df46l5zwe.o
│       │   │       ├── 43wjj04ugjn92w25.o
│       │   │       ├── 3cpe0k7r03o1szon.o
│       │   │       ├── 344jaybo67ov5gq6.o
│       │   │       ├── 1eqkw6xifl786fcx.o
│       │   │       ├── w6qw59kh5lr0j5f.o
│       │   │       ├── 4fxwjam1pj8b1e4e.o
│       │   │       ├── 329f18b751pung3b.o
│       │   │       ├── 1cbgbyzvo95y4wyw.o
│       │   │       ├── 31tiwzjsxz7w7gux.o
│       │   │       ├── 3i8dafgzc90u5za6.o
│       │   │       ├── 1ndzsjo32har2ftc.o
│       │   │       ├── dep-graph.bin
│       │   │       ├── 2rb8aofti8peg4s8.o
│       │   │       ├── 3gx1il9qsteymfkp.o
│       │   │       ├── 2vkqxnkt4flsjaby.o
│       │   │       ├── exrqpc0l22kt700.o
│       │   │       ├── 2kyphk6ya49r6q8z.o
│       │   │       ├── work-products.bin
│       │   │       ├── 1fnr94fr66c4kpca.o
│       │   │       ├── 538c7k1bjcwk3t6j.o
│       │   │       ├── zt7i6emmecz7zj8.o
│       │   │       ├── 2buf9o9p8n830ug6.o
│       │   │       ├── 2oi91zgtyseheqwj.o
│       │   │       ├── e87v8dwv2310jwi.o
│       │   │       ├── 4psxxmhbzsmcuvqm.o
│       │   │       ├── 53550xyw71saqcnj.o
│       │   │       ├── 4h4gnkd47f8w4t78.o
│       │   │       ├── 59zk6h2ymlrkol25.o
│       │   │       ├── 36r255p5bi0eojvn.o
│       │   │       ├── 34cnf4b80vdz5gwm.o
│       │   │       ├── 45py8zrlfave2bgg.o
│       │   │       ├── 51wbm0v3ee98k6ip.o
│       │   │       ├── 26z3o0htvtvx1zo4.o
│       │   │       ├── 2kjy3koojtr3n589.o
│       │   │       ├── 4obeu6vcpkvxik2c.o
│       │   │       ├── 2rnpiabfdp77sh1q.o
│       │   │       ├── 1ou8tv2ptzpsnygm.o
│       │   │       ├── 4t8heu737mc3a60d.o
│       │   │       └── 39uezqq1rtbk50lt.o
│       │   ├── macro-3bebr4rno0m7c
│       │   │   ├── s-h345sw68lc-1uvxcub-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345svpt92-0j05r58-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345sz195x-0mcryw0.lock
│       │   │   ├── s-h345sviua3-0o6bk9l.lock
│       │   │   ├── s-h345sw68lc-1uvxcub.lock
│       │   │   ├── s-h345svpt92-0j05r58.lock
│       │   │   ├── s-h345sz195x-0mcryw0-9185mhlknx7qnys7uzqgqebcm
│       │   │   │   ├── 5ih7r1hpqeqow4smkpd22r6k8.o
│       │   │   │   ├── 8197w6wzq4lyj8i6r434nev6h.o
│       │   │   │   ├── 59ygxsn37rkavnmrg5akbmvi2.o
│       │   │   │   ├── bqjdqvil8eptzvpby9p8ugylv.o
│       │   │   │   ├── 86ttm5q3zisp2ko1878blumb0.o
│       │   │   │   ├── d6lqo2hzqmdtqtu9lwrhoe0kr.o
│       │   │   │   ├── 7spp2cz9qhmlypctdaq55rykt.o
│       │   │   │   ├── a6783ftvi4r681u1crg6zr3yj.o
│       │   │   │   ├── dh2u25u0wd3cslq0med0qmlau.o
│       │   │   │   ├── 8qoftcqiudrew3lam0xwfnyh2.o
│       │   │   │   ├── 84qrgwux79fl56pnb9z7l2bl7.o
│       │   │   │   ├── 0iy0w0jc7y6tvtedzcymqiqgn.o
│       │   │   │   ├── 34w2w1xap2m5zouv8i0gv0xwb.o
│       │   │   │   ├── 69f6ajoft6ngkecuq61q6rrbr.o
│       │   │   │   ├── 8vj4hvq8x55y4df8a9npgg5ho.o
│       │   │   │   ├── 9kd6ml3jxwmzv7ibfie1zogum.o
│       │   │   │   ├── cbx5bgxtu7t0wijlktspe692q.o
│       │   │   │   ├── 7ozn3zriitfv3yjjiua86r1n6.o
│       │   │   │   ├── czfhhus0ojg5xsy6zkli5loca.o
│       │   │   │   ├── bpc408l3izi6os2y0z05rsiz0.o
│       │   │   │   ├── 2vkms33gczsutuiz3rfk5pgo3.o
│       │   │   │   ├── 9gbmcr9xe0j7wtbkxqq6py0lk.o
│       │   │   │   ├── 28psvte0saatgk5gc4ez2714t.o
│       │   │   │   ├── 0r49v5bwjvhv693cg0hktd8r2.o
│       │   │   │   ├── cs80tiwzed2wl6l45te088ggl.o
│       │   │   │   ├── b84tmk0mt38g6624csneqvnip.o
│       │   │   │   ├── 8et0gvmynspspmah41u4549pf.o
│       │   │   │   ├── 48r8m1jp7cg20lbl40ngay9jb.o
│       │   │   │   ├── 9b69rfz3lacrs25eo3y59mjdk.o
│       │   │   │   ├── 9d2ok4tmm6p1gqm22w996dfnp.o
│       │   │   │   ├── 6322ag56pyneqd6cretql94fr.o
│       │   │   │   ├── 1t2vawq0hq0qx70rip5d9k8r0.o
│       │   │   │   ├── 1yph16pi0a4m9936bzdgx3lrb.o
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── 3oquf95pnsbdreqjii0xem938.o
│       │   │   │   ├── 3kyn2u8mnvx1kd04bp8qvbjqz.o
│       │   │   │   ├── 0v4b2xt1owhqia6tqwot37533.o
│       │   │   │   ├── 7anlg0117yrqpxzi297x6ioq5.o
│       │   │   │   ├── 9s8k66marzqf9eshmxejyzxun.o
│       │   │   │   ├── 3gjqpl2qkj399hufnldpa5eqk.o
│       │   │   │   ├── 2b8ihr5u4ryml6r13itt3jd6o.o
│       │   │   │   ├── 5ydgzt3m0khxnhe2nt4ax4bhg.o
│       │   │   │   ├── 3vdh20u3n80x7iy7bqa3l1yev.o
│       │   │   │   ├── 6w94l81t0vp5lrcd2d8xgvgu9.o
│       │   │   │   ├── 4aqz4qqf8eqgulhn3muclnl69.o
│       │   │   │   ├── 1s3eqefh7dgj5gnzyuhy21zzm.o
│       │   │   │   ├── c704xjrbqdfxefhxlv1shm8vf.o
│       │   │   │   ├── 56fnpepkgvcyxacid66prda75.o
│       │   │   │   ├── dep-graph.bin
│       │   │   │   ├── 4n4xwtycneligrv2ovvvisl6w.o
│       │   │   │   ├── 2g2dvehddis8d0jd3r4pd8mpz.o
│       │   │   │   ├── 5wzq7ybw4q2j7e1uezb4j17y2.o
│       │   │   │   ├── 38n9h7nehxbp3xoc9cm0qtev7.o
│       │   │   │   ├── work-products.bin
│       │   │   │   ├── 779mqrip7eti8g4g27wl6ywtr.o
│       │   │   │   ├── 9tflsx3em0v7ys8uobks8glwz.o
│       │   │   │   ├── 7w2f86hyv4gajgeo93jqrwnul.o
│       │   │   │   ├── 9l3z05efuf0ua54apg3k2p3j6.o
│       │   │   │   ├── 0vk4pcmq7tk9c8j8nx2ahqsa1.o
│       │   │   │   └── e7vk5eyst12qq6ssmnpxezt8n.o
│       │   │   └── s-h345sviua3-0o6bk9l-working
│       │   │       └── dep-graph.part.bin
│       │   ├── taiko-2rtvm112rt1z2
│       │   │   ├── s-h345ty2huz-18hqydp.lock
│       │   │   └── s-h345ty2huz-18hqydp-working
│       │   │       └── dep-graph.part.bin
│       │   ├── prompt_macro-1tpgv56hckinv
│       │   │   ├── s-h34bgiyyks-1av13re.lock
│       │   │   └── s-h34bgiyyks-1av13re-0igd6seseb08p1eog7c3iwg4a
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   ├── taiko_macro-2g6s8sgowx55o
│       │   │   ├── s-h34597apt6-05qhnqi-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   └── s-h34597apt6-05qhnqi.lock
│       │   ├── taiko-0fg8wlpbtgr5p
│       │   │   ├── s-h363oqxqe9-0bhwsqk.lock
│       │   │   └── s-h363oqxqe9-0bhwsqk-4ki09y0cyg4ewyxq3ipla3958
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   ├── taiko-330rik32fj1iq
│       │   │   ├── s-h4rmmqo5dm-07ikjyg-ce3nle1gm68oqnfuwxwujh8vc
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-h4rmmqo5dm-07ikjyg.lock
│       │   ├── taiko-2qrskcxybjdrx
│       │   │   ├── s-h34585i7rm-1nij193.lock
│       │   │   ├── s-h34588nefg-15xq3be-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h34588nefg-15xq3be.lock
│       │   │   └── s-h34585i7rm-1nij193-working
│       │   │       └── dep-graph.part.bin
│       │   ├── taiko-12zj6su92u20z
│       │   │   ├── s-gucqrhrcy0-8za5c3-eonc8r3qu4pxy56lwg5ec8tdl
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-gucqrhrcy0-8za5c3.lock
│       │   ├── taiko-1dwz2ysnp89xb
│       │   │   ├── s-guhdsuyw31-vx9by4-9wd63uxy2nfba1ofa0xkv0imz
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-guhdsuyw31-vx9by4.lock
│       │   ├── macro-2nfzf9gg52u6j
│       │   │   ├── s-h345sw68lk-192sngu-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345sviu9s-1naeba8-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345sz1994-038983n-9vob9ezmwceeht59v36xjdw41
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   ├── s-h345sw68lk-192sngu.lock
│       │   │   ├── s-h345sviu9s-1naeba8.lock
│       │   │   └── s-h345sz1994-038983n.lock
│       │   ├── taiko-286nlt2xj6nlk
│       │   │   ├── s-h3593a08xy-1q4j3fh-3rjbg8giwp3dtmpxqn0a67f8m
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-h3593a08xy-1q4j3fh.lock
│       │   ├── taiko_macro-1j297q5r74nzs
│       │   │   ├── s-h34597apse-1jznrtf-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   └── s-h34597apse-1jznrtf.lock
│       │   ├── taiko-3l02klothpxuu
│       │   │   ├── s-h1353lha70-1bdio7a.lock
│       │   │   └── s-h1353lha70-1bdio7a-9jg04s3f9o8z9d9iozd4hxnrs
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   ├── taiko-cyu92fw90zsk
│       │   │   ├── s-gucr5yjku5-13ccnm0-9mhymi5vhkq149n5pc5a677a0
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-gucr5yjku5-13ccnm0.lock
│       │   ├── taiko_macro-1kgv422498mgz
│       │   │   ├── s-h34592umk7-07x6lvz.lock
│       │   │   ├── s-h345962hmz-0vmdo5c-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345962hmz-0vmdo5c.lock
│       │   │   └── s-h34592umk7-07x6lvz-working
│       │   │       └── dep-graph.part.bin
│       │   ├── prompt_macro-3qbqarxznryyy
│       │   │   ├── s-h4rmmp2tw7-1bgkp4c-4gyhk3lfgzoqweqhbgn11naj2
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-h4rmmp2tw7-1bgkp4c.lock
│       │   ├── prompt_macro-08penxfxl4mxe
│       │   │   ├── s-h4rmmfql9r-1mcmpgm.lock
│       │   │   └── s-h4rmmfql9r-1mcmpgm-5m4ajfocwjn0y1a83syq2nig5
│       │   │       ├── 88lmssdmljd84t9hw4c8u36fx.o
│       │   │       ├── 0strn6mtyb231ygelhyno366t.o
│       │   │       ├── 38z5wwfaktbp75vkrkggwdz3i.o
│       │   │       ├── egmhy82yfzsaw0xq250evcc26.o
│       │   │       ├── 0l6i19x87dtrnj8wvzh69r4e0.o
│       │   │       ├── 0lajsbsf7rwkgrm48f5wa445t.o
│       │   │       ├── 3lk9bphkudd7udir21vy9z7lk.o
│       │   │       ├── 2suvsh642jjbbabfju1lj3pnp.o
│       │   │       ├── 3dr0oah5ih2qoumtva82o3kca.o
│       │   │       ├── ak5wpodhx4zho1jn6po2ot86p.o
│       │   │       ├── 1fxsh136umon34dlmhbgpdqt2.o
│       │   │       ├── awxjhq05b587cxr1zfqgtmrf6.o
│       │   │       ├── 0p9uj9f2pemdi2ep5p35vvp6x.o
│       │   │       ├── 0smzyy4kths8ph48ku75zjdj5.o
│       │   │       ├── 6g4ryt3fxk57bw0wpgsu3e07j.o
│       │   │       ├── 02obfdr1m7r5b1jsq8i87ozsr.o
│       │   │       ├── btmito57xrmgf5fmlhi193k29.o
│       │   │       ├── 798yp0of29e8wl3gup00i8673.o
│       │   │       ├── eyjd47t29xq9al6jjux05zlb7.o
│       │   │       ├── czbnlgmtj3fq3mrgnsquh3ukd.o
│       │   │       ├── 9wn3iy6h2545e9xnildflppm0.o
│       │   │       ├── 9e1thle0a3b0imyeo4qlyb0ib.o
│       │   │       ├── 7fv8agmjmknrt0qbm164gkq3q.o
│       │   │       ├── 43nvshpvy5hw940grcwkmrz43.o
│       │   │       ├── 57wcdt8bdls0wwavp47orsqg3.o
│       │   │       ├── 2corl48yjqh2uvks178wlwngl.o
│       │   │       ├── 5tmawzz6mt3ncf3t0jtj01itf.o
│       │   │       ├── 0lr9f0tu50tvxgbx174pm14li.o
│       │   │       ├── dvi60j57cce4imzxiwcr1xjlu.o
│       │   │       ├── 7g9bssd1wtvaj58rv7l1htjlf.o
│       │   │       ├── 014y7cf4gc4wys8fa8q1ac72q.o
│       │   │       ├── 90dhprdov6la4j7hrmunygemb.o
│       │   │       ├── 57hxw4nbg5fnztsw49i2qn8x2.o
│       │   │       ├── 59s72doh9e9v1b022an3zduxm.o
│       │   │       ├── 9ztw0y1zn61i1nx65jz9j3aso.o
│       │   │       ├── 71zrdd7e7fqpkku30gefxsv0u.o
│       │   │       ├── b2is5ck5b297zkq3cvuj616jc.o
│       │   │       ├── b1ne1l55uy2hjmed4g0kk5g7k.o
│       │   │       ├── query-cache.bin
│       │   │       ├── 4vruejr03djvwe74v6fe61434.o
│       │   │       ├── 4dm7ai8t1tvygfq7r9pt3im3l.o
│       │   │       ├── 3i2mzie6rjkr9b0lkky2re1f2.o
│       │   │       ├── acuanjekzg8g1b16gd25koayy.o
│       │   │       ├── 5j6jrzwuipi7rt6vzwmhvvc2s.o
│       │   │       ├── 1wnzygvemygx66zodub3e1xgz.o
│       │   │       ├── dep-graph.bin
│       │   │       ├── ct99frs1x5etl4a50anq2w679.o
│       │   │       ├── 6sasmswio74bp85fkw79sbh2w.o
│       │   │       ├── 4ajakp8rvv7kyqrg1hgtx8ywa.o
│       │   │       ├── work-products.bin
│       │   │       ├── 3zdkgsqjzzgtwk4y7svw57ds9.o
│       │   │       ├── 97ac5amkz428uxor2ockxz6sy.o
│       │   │       ├── 9n89zd7k53w7nk1rkdvqbzwbe.o
│       │   │       ├── bstr8votgwkrrtjf4qnrwukv3.o
│       │   │       ├── 02ltavw7gnyrb5zx69arh6fa6.o
│       │   │       ├── cele8h54vvcoti7337y3v67lz.o
│       │   │       ├── 2gl76x7fzddzqpqj01ohqyb6d.o
│       │   │       └── 4yhdpdu1tw4lzdqj491yregr0.o
│       │   ├── prompt_macro-0bwibi7r0qen4
│       │   │   ├── s-h34bgiyylp-1jidoxb-am8re5spf1u8fen2oy2iqarll
│       │   │   │   ├── bi151e6edloqu2js51mj61rqt.o
│       │   │   │   ├── 5w8g4d8xlrh83uqwmikmr0lma.o
│       │   │   │   ├── cyn9ppblutkjiqyif5atjasuz.o
│       │   │   │   ├── evb2cvwmxrpu0i8orke3kqqfh.o
│       │   │   │   ├── 0xowutb8zn4x6sxcobpjh9rih.o
│       │   │   │   ├── 2lffn6j8xiqda1m3coqtp17iq.o
│       │   │   │   ├── 7ncs6hwjsyhs13byus6rz9i57.o
│       │   │   │   ├── 9f0smx4vq6ns6jsom1svtqf0n.o
│       │   │   │   ├── 8hfxko7v3l4x3rmx07qorrkyx.o
│       │   │   │   ├── bnij2cagzhf6bs5mbr166re6r.o
│       │   │   │   ├── 2mzp7c14smdeo9fzch8du901g.o
│       │   │   │   ├── 7obxhie633ae9ecomfoo90r73.o
│       │   │   │   ├── 9oyj3w0xumsd04zu4guvr58xi.o
│       │   │   │   ├── 9g7voes1bc3tu9h6k14jvpky1.o
│       │   │   │   ├── 9q109wnk72urrya744f21e8z6.o
│       │   │   │   ├── 2wdzzxriw9c06ctyiutno8bwb.o
│       │   │   │   ├── 7utryyv73tmd31925vgtvq0tn.o
│       │   │   │   ├── 05rqswre3vfn5netia0vler8u.o
│       │   │   │   ├── 0djcpm11yv78hw0afvki4njdu.o
│       │   │   │   ├── 1kep12r3r4vr0ugbdgxf5ibdn.o
│       │   │   │   ├── 88vlp5ont7ewtk653cdsfb1l5.o
│       │   │   │   ├── aal52jwvcxz0vl51l8rdntmzp.o
│       │   │   │   ├── 5ind7wyjqu2jl9nnnb7ovsxqa.o
│       │   │   │   ├── f58a5y6vq6j57hp0bsuqa6la0.o
│       │   │   │   ├── 3w8luw3ssi11xpcofim82rrgu.o
│       │   │   │   ├── 0a69emz5dclq3bhpb4nvwgywk.o
│       │   │   │   ├── 75wn0l9djlefynkzhw2pbv99j.o
│       │   │   │   ├── 6pu0styichouo9c9647rs2dom.o
│       │   │   │   ├── 2vvwa1kp8fcy5ztbktymv9l3p.o
│       │   │   │   ├── 5vausxkvqpy50y2p35945pja8.o
│       │   │   │   ├── 2j6nr7dblqf2cv0t50umt73t1.o
│       │   │   │   ├── 9gng8sbv9kvl2lmqcntbfal6d.o
│       │   │   │   ├── 703ly9smkyayihiu7z81gjh8j.o
│       │   │   │   ├── c77of5tqfcxlggaia5cf62lya.o
│       │   │   │   ├── 3owkmbrn5n6wkjfwp6vdhwmnv.o
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── b4k89rvs82cza74b6h32ijjta.o
│       │   │   │   ├── afzxt25c4pypfjc60r8yidmtp.o
│       │   │   │   ├── aaq3i1dcctjtljewo8xj8aqgy.o
│       │   │   │   ├── 6lzul5q5wgnck06jofq57lsl2.o
│       │   │   │   ├── dep-graph.bin
│       │   │   │   ├── 8az4lox4hkghxabejcc7yna5e.o
│       │   │   │   ├── cf0y9xmlvtvtrm6dtze4oaj1s.o
│       │   │   │   ├── bqkksyu2emhdiwvghbrig00wo.o
│       │   │   │   ├── 0usvr0yry16n26igt2vst3ag0.o
│       │   │   │   ├── 7hkvdh9hjz15qmep6s19scgqc.o
│       │   │   │   ├── work-products.bin
│       │   │   │   ├── 8tqk7rorf3tmunuxqm00svlxc.o
│       │   │   │   ├── d9s138o62ninwoso97hkrbmgu.o
│       │   │   │   ├── bzdze8nwz66yj4k9uyuny1gj7.o
│       │   │   │   ├── a9jbxa26n4lf9ldvo0hij2ohw.o
│       │   │   │   ├── cr5h8n55ue6jw5e5y7ft8x6sy.o
│       │   │   │   ├── 47pxeocj5z2yr8s1ixsv7jbgq.o
│       │   │   │   ├── f17hiyv1gxk6dwgegtv0s6oz1.o
│       │   │   │   ├── 9yjsxr7sop8061b1q2ynt3asu.o
│       │   │   │   ├── d6c040ty8blyp32teutljixhz.o
│       │   │   │   ├── 5i1b6a7orwifjge3d514cwq7y.o
│       │   │   │   ├── f346kzndhd9h62kae3f8awmq0.o
│       │   │   │   ├── 2syi762p1vyrk2fl951pvo3jf.o
│       │   │   │   └── 0zv3huy7iigr9r99xnk5xfksn.o
│       │   │   └── s-h34bgiyylp-1jidoxb.lock
│       │   ├── macro-1yg67u3dbw74m
│       │   │   ├── s-h345sw68lf-0syy7x0-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345sviua7-102jss0-working
│       │   │   │   └── dep-graph.part.bin
│       │   │   ├── s-h345sz1961-0jpzlhr-8rb5mritfzbz3mtwgvx81hmdz
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   ├── s-h345sviua7-102jss0.lock
│       │   │   ├── s-h345sw68lf-0syy7x0.lock
│       │   │   └── s-h345sz1961-0jpzlhr.lock
│       │   ├── taiko-22ptsxjtotcrd
│       │   │   ├── s-h344xokhjz-085eo10-ct0el9mv34ocug2f1p06tt8qt
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   ├── s-h344xokhjz-085eo10.lock
│       │   │   ├── s-h344yvztsg-0dwefqk-working
│       │   │   │   ├── dep-graph.part.bin
│       │   │   │   ├── query-cache.bin
│       │   │   │   ├── dep-graph.bin
│       │   │   │   └── work-products.bin
│       │   │   └── s-h344yvztsg-0dwefqk.lock
│       │   ├── taiko-3nxsk73dbbw0p
│       │   │   ├── s-h4rmmqo5d3-1tmt56g.lock
│       │   │   └── s-h4rmmqo5d3-1tmt56g-8erel33ov17rlj5v5oq6mv2sp
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   ├── prompt_macro-02nprea9q6gbt
│       │   │   ├── s-h4rmmp2ty3-0z37w6f.lock
│       │   │   └── s-h4rmmp2ty3-0z37w6f-770j96i1v50maokbavqplyh5h
│       │   │       ├── query-cache.bin
│       │   │       ├── dep-graph.bin
│       │   │       └── work-products.bin
│       │   └── prompt_macro-2vukl6emjpbwb
│       │       ├── s-h34bgiyyle-0arc4fc-73qsjbnnxtb8vpc4o7c5nnhuw
│       │       │   ├── query-cache.bin
│       │       │   ├── dep-graph.bin
│       │       │   └── work-products.bin
│       │       └── s-h34bgiyyle-0arc4fc.lock
│       ├── examples
│       ├── taiko
│       ├── deps
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.10.rcgu.o
│       │   ├── libsyn-cd4a8424ebf8304f.rlib
│       │   ├── clap_lex-19b7f35199c12602.d
│       │   ├── libanstyle-3e72b357153a842f.rmeta
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.05.rcgu.o
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.13.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.k8moi8de20ivu73.rcgu.o
│       │   ├── libthiserror_impl-5ec8bf9d562dc2a5.dylib
│       │   ├── rustc_hash-0d95bed8e9e70cbe.d
│       │   ├── strsim-fadff44222618b4b.strsim.e1140db0c11a0d78-cgu.1.rcgu.o
│       │   ├── libtaiko-53c112d7896ac128.rmeta
│       │   ├── libc-a3e4d221db19ab38.d
│       │   ├── taiko-f18a957e5fef2c78.d
│       │   ├── memchr-d1514e95b786a778.d
│       │   ├── libheck-fe4aad64d30b965f.rlib
│       │   ├── libsyn-79418542530a1a9b.rlib
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.2.rcgu.o
│       │   ├── libquote-bb3c202bc622fea6.rmeta
│       │   ├── libunicode_ident-ddc98497afda250d.rmeta
│       │   ├── quote-cdfe020df82b4d1d.d
│       │   ├── autocfg-d0bb26a33a75c25c.d
│       │   ├── libproc_macro2-5b0542e781e56cc7.rlib
│       │   ├── globset-79a72d153d181801.d
│       │   ├── libquote-97b83299b1024699.rmeta
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.11.rcgu.o
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.02.rcgu.o
│       │   ├── libcrossbeam_deque-499b579d3c611256.rmeta
│       │   ├── memchr-3a5327134c6e17ec.d
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.2.rcgu.o
│       │   ├── libonce_cell-508ab5f09252e905.rmeta
│       │   ├── libwalkdir-f97164972ff9ea9d.rmeta
│       │   ├── pest-54f9af20f2c3fa2f.d
│       │   ├── libserde_json-4d2f2b02b377dd94.rmeta
│       │   ├── libthiserror-2adf4c655155bef1.rlib
│       │   ├── taiko-5f5482d76af3ad3e.39uezqq1rtbk50lt.rcgu.o
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.01.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.1app0h7cjp1v7gui.rcgu.o
│       │   ├── libcfg_if-f63cf2e845327ac6.rmeta
│       │   ├── libstrsim-fadff44222618b4b.rlib
│       │   ├── libhandlebars-90b3957116bc7052.rmeta
│       │   ├── globset-d77835a08215d720.d
│       │   ├── libucd_trie-dcd6580dd443e17a.rlib
│       │   ├── libthiserror_impl-09cb00ac5d14a0cf.dylib
│       │   ├── libpest_generator-0e47e79a53ca6246.rlib
│       │   ├── rayon-08b4c57f583c0024.d
│       │   ├── libpest_generator-fc27fdfcc846e3cc.rlib
│       │   ├── libthiserror-dd7f932d747b83fd.rlib
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.03.rcgu.o
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.09.rcgu.o
│       │   ├── libclap_builder-fe335d7668ae5da9.rlib
│       │   ├── libscopeguard-707e2ad09f4fa99f.rmeta
│       │   ├── librayon_core-9f7d444320b975be.rmeta
│       │   ├── libclap_lex-1030d2fa2711077f.rmeta
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.2.rcgu.o
│       │   ├── portable_atomic-fca8db96086604f2.d
│       │   ├── libjwalk-4c072e0256a54789.rlib
│       │   ├── colored-f86f6016082595f1.d
│       │   ├── libmacro-8730cf09808ccf8c.rmeta
│       │   ├── libcli_clipboard-1de30f25a56af333.rlib
│       │   ├── libc-377ada10dcb65998.d
│       │   ├── bstr-e83c5b67e0a2b2dd.d
│       │   ├── thiserror-dd7f932d747b83fd.d
│       │   ├── libitoa-e7583f895a51e75e.rmeta
│       │   ├── libsame_file-d377ec210631ce56.rmeta
│       │   ├── rayon_core-9f7d444320b975be.d
│       │   ├── parking_lot_core-132a77ce2b7cc0d0.d
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.4.rcgu.o
│       │   ├── libmemchr-53c5fe84ba9abe21.rmeta
│       │   ├── libheck-24d0bb0cde6f2d31.rlib
│       │   ├── libcrossbeam_queue-ec50bd6f228abdbd.rmeta
│       │   ├── libunicode_ident-eb62d7b284154f68.rlib
│       │   ├── libblock-a814ef6c1ee6440a.rmeta
│       │   ├── libanstyle_query-a6e7118bf77d1478.rmeta
│       │   ├── libjwalk-b89993c9e5148856.rmeta
│       │   ├── libunicode_ident-4fa8e70de8323614.rmeta
│       │   ├── crossbeam_channel-8c094417765e7827.d
│       │   ├── libhandlebars-5dcd2ebf69d9205f.rmeta
│       │   ├── crossbeam_epoch-1695a116cad47243.crossbeam_epoch.848f1f04288c74a7-cgu.1.rcgu.o
│       │   ├── libignore-3f8d697aa460cd76.rlib
│       │   ├── unicode_ident-114fa19babfeb915.d
│       │   ├── fancy_regex-c6034dfe6ec4bdda.d
│       │   ├── jwalk-9935bac444b172dd.d
│       │   ├── regex_automata-2b47dcdb846aa3d6.d
│       │   ├── ucd_trie-0e6e94eba92d6a77.d
│       │   ├── libwalkdir-bdbf780e9d753ec0.rmeta
│       │   ├── portable_atomic-d7a74020c9b9fe12.d
│       │   ├── libcrossbeam_deque-35e98e3c2d441351.rmeta
│       │   ├── libproc_macro2-5b0542e781e56cc7.rmeta
│       │   ├── ucd_trie-f1a751efa5939de4.ucd_trie.271f527e5aea8b64-cgu.4.rcgu.o
│       │   ├── malloc_buf-53a7bf1de681c142.d
│       │   ├── libobjc-2476d8a1277d70b4.rmeta
│       │   ├── libmemchr-3a5327134c6e17ec.rmeta
│       │   ├── libonce_cell-00ea8b35a4a0316d.rlib
│       │   ├── libtermtree-d611376f57ba0335.rmeta
│       │   ├── serde-82b2836e0bab74b1.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.14.rcgu.o
│       │   ├── libignore-fffd30910e9307dc.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.03.rcgu.o
│       │   ├── libobjc-9aceb7739b194f45.rlib
│       │   ├── libcrossbeam_epoch-011387be3d43c679.rmeta
│       │   ├── librayon-f8933dc420cf2428.rmeta
│       │   ├── libfancy_regex-a99532343fd1d180.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.09.rcgu.o
│       │   ├── libquote-5b4df7b9712eaceb.rmeta
│       │   ├── libindicatif-c6b15a49e77c4728.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.3i8dafgzc90u5za6.rcgu.o
│       │   ├── libpest_generator-98c6362ac1cac2f8.rlib
│       │   ├── libregex_automata-5699297d539bd36f.rmeta
│       │   ├── libclap_derive-5a22d46402a77847.dylib
│       │   ├── libserde_json-89cb9c549439971c.rmeta
│       │   ├── libthiserror_impl-b284c8f3d873283e.dylib
│       │   ├── taiko-5f5482d76af3ad3e.3gx1il9qsteymfkp.rcgu.o
│       │   ├── colorchoice-2065da109b279150.d
│       │   ├── pest_meta-03395d72a0dfe259.d
│       │   ├── librustc_hash-59b564b5a1e74548.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.45py8zrlfave2bgg.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.14tihf9h5rss9kvy.rcgu.o
│       │   ├── libunicode_ident-323a696b6fd93af8.rmeta
│       │   ├── bstr-3516875550e89b46.bstr.db2bb989631abf3e-cgu.3.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.1qcpk7paqwuwog80.rcgu.o
│       │   ├── colored-77bb8fdb6306fbb7.colored.15f374face372787-cgu.3.rcgu.o
│       │   ├── libthiserror-7d04976b43fa628e.rlib
│       │   ├── utf8parse-970b336da10722b6.utf8parse.131d40064bdbf144-cgu.0.rcgu.o
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.11.rcgu.o
│       │   ├── liblibc-18495412a3a8bf98.rmeta
│       │   ├── libfancy_regex-11b1b72092e141c5.rmeta
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.06.rcgu.o
│       │   ├── ucd_trie-40e4c8a9f503bc66.d
│       │   ├── libonce_cell-e8773422b0f43014.rlib
│       │   ├── bit_set-4059202125707996.d
│       │   ├── taiko-5f5482d76af3ad3e.77v5mozdpm42kz.rcgu.o
│       │   ├── libaho_corasick-bcc434a446492054.rmeta
│       │   ├── libclap_lex-19b7f35199c12602.rmeta
│       │   ├── libbase64-2fa71140a3bbff25.rmeta
│       │   ├── libbit_set-d7322ea878b61688.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.05.rcgu.o
│       │   ├── pest_generator-5aeaa8baad32201a.d
│       │   ├── anyhow-b80c101af730b57a.d
│       │   ├── libpest-4932018ce684cc23.rmeta
│       │   ├── libglobset-42ddf0677cf59874.rlib
│       │   ├── clap_builder-76adf2488bf8955d.d
│       │   ├── aho_corasick-bcc434a446492054.d
│       │   ├── ignore-a70f92eb98e46d19.d
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.06.rcgu.o
│       │   ├── libquote-98d955d671fa9ae9.rmeta
│       │   ├── quote-98d955d671fa9ae9.d
│       │   ├── libserde-e8a75b8079968bd6.rmeta
│       │   ├── libignore-8646944c799ad137.rmeta
│       │   ├── libproc_macro2-858b3b96a32c88fa.rlib
│       │   ├── libclap_builder-fe335d7668ae5da9.rmeta
│       │   ├── bstr-4748e8eea95eebcc.d
│       │   ├── libproc_macro2-e3ead2790c8a6ade.rmeta
│       │   ├── libportable_atomic-efc16aa6ba00ca29.rmeta
│       │   ├── regex-79f28fcb5b054383.d
│       │   ├── clap-183d96304d7c6951.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.04.rcgu.o
│       │   ├── rayon-a641aab5cd97d3de.rayon.81f2f342fe3ac30b-cgu.0.rcgu.o
│       │   ├── colored-77bb8fdb6306fbb7.d
│       │   ├── pest_derive-11c6b22c1ab0ffdc.d
│       │   ├── ucd_trie-e13425de40a4d29d.d
│       │   ├── scopeguard-e2949efb413bface.d
│       │   ├── pest_meta-dc9c6f7dea0d78f4.d
│       │   ├── libbit_vec-de5920a6f9721359.rmeta
│       │   ├── libnumber_prefix-4b0f92ccd4a73404.rmeta
│       │   ├── jwalk-b89993c9e5148856.d
│       │   ├── libhandlebars-bf6d86a23749eb49.rmeta
│       │   ├── libthiserror-7fd1eef083610ebb.rmeta
│       │   ├── handlebars-940b1fe4c2f989a9.d
│       │   ├── smallvec-6de58aa964b2431f.d
│       │   ├── cli_clipboard-7a13cdfe04ad4c75.d
│       │   ├── libcrossbeam_epoch-32d4247daab5bcc4.rmeta
│       │   ├── libignore-a70f92eb98e46d19.rmeta
│       │   ├── libucd_trie-40e4c8a9f503bc66.rlib
│       │   ├── libcli_clipboard-21af94370cbb9111.rmeta
│       │   ├── ucd_trie-f1a751efa5939de4.d
│       │   ├── libanstyle_query-549661baf91ae106.rmeta
│       │   ├── anstream-4945a0c46b1be249.d
│       │   ├── taiko-5f5482d76af3ad3e.zq9upsxmwybvmob.rcgu.o
│       │   ├── libcrossbeam_queue-6ffa5f04fb97f232.rlib
│       │   ├── libanstyle-f05d8c7856d7e78e.rlib
│       │   ├── libblock-ba265c29da368d38.rmeta
│       │   ├── clap-ecd14b188f0d2455.d
│       │   ├── ucd_trie-68172e19f10149cf.d
│       │   ├── scopeguard-1a41f4f101669df5.d
│       │   ├── libutf8parse-970b336da10722b6.rlib
│       │   ├── taiko-5f5482d76af3ad3e.26k7fh38dj7pf20i.rcgu.o
│       │   ├── ignore-c4f376d660a6f596.d
│       │   ├── libtaiko-f18a957e5fef2c78.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.1yhsq1aczi4dgjy0.rcgu.o
│       │   ├── regex-e853df995e92a0fb.d
│       │   ├── libcrossbeam_epoch-5484dbb6acc3f8a5.rmeta
│       │   ├── libbstr-4748e8eea95eebcc.rmeta
│       │   ├── libryu-468c4aa142a2ea4e.rmeta
│       │   ├── liblog-e4b3d7d40b5ea863.rmeta
│       │   ├── libbase64-6e96c5a9915be65f.rlib
│       │   ├── libsyn-f7a43a238a4253d3.rmeta
│       │   ├── lazy_static-2a7ede781c6dce4a.d
│       │   ├── unicode_width-16a1984432142341.d
│       │   ├── libunicode_ident-114fa19babfeb915.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.08.rcgu.o
│       │   ├── libunindent-60d7bc434b501888.rmeta
│       │   ├── libcrossbeam_utils-360c9ee5275f1872.rmeta
│       │   ├── libstrsim-59ec25f123d79732.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.43wjj04ugjn92w25.rcgu.o
│       │   ├── heck-79e04a5211bdd404.d
│       │   ├── libonce_cell-508ab5f09252e905.rlib
│       │   ├── taiko-5f5482d76af3ad3e.e87v8dwv2310jwi.rcgu.o
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.07.rcgu.o
│       │   ├── libpest-7ca1fc2e97cc4b6c.rmeta
│       │   ├── termtree-d611376f57ba0335.d
│       │   ├── libanstyle-f4f197a2e42921a4.rmeta
│       │   ├── crossbeam_epoch-821d298372abaa1d.d
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.04.rcgu.o
│       │   ├── libpest-54f9af20f2c3fa2f.rlib
│       │   ├── libanstyle-289580a72738fded.rmeta
│       │   ├── libbit_vec-6784665201503254.rmeta
│       │   ├── pest_generator-0e47e79a53ca6246.d
│       │   ├── libcrossbeam_epoch-ff0e744e239caf0b.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.13.rcgu.o
│       │   ├── heck-5b224f32e954472c.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.04.rcgu.o
│       │   ├── crossbeam_utils-18686986dd98f802.d
│       │   ├── libaho_corasick-4690a1ce781d3d30.rmeta
│       │   ├── libparking_lot-c365af0e5de08631.rmeta
│       │   ├── itoa-7afa4844fdfbbd60.d
│       │   ├── heck-24d0bb0cde6f2d31.d
│       │   ├── crossbeam_channel-fdc0ae76d9c3397f.crossbeam_channel.6929f6013531f7a0-cgu.1.rcgu.o
│       │   ├── libtiktoken_rs-1011b622425df025.rmeta
│       │   ├── thiserror-2adf4c655155bef1.d
│       │   ├── taiko-5f5482d76af3ad3e.1eqkw6xifl786fcx.rcgu.o
│       │   ├── libtaiko-3164c78022e0e85e.rmeta
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.08.rcgu.o
│       │   ├── libaho_corasick-91cacaa652cc5153.rmeta
│       │   ├── libtaiko-76736ea0e179a76d.rmeta
│       │   ├── taiko-53c112d7896ac128.d
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.15.rcgu.o
│       │   ├── libpest-2b50acd6dff662ba.rlib
│       │   ├── thiserror_impl-b284c8f3d873283e.d
│       │   ├── libpest_meta-0b5f49852cd582a0.rmeta
│       │   ├── colored-77bb8fdb6306fbb7.colored.15f374face372787-cgu.1.rcgu.o
│       │   ├── prompt_macro-4108ea4371bd192f.d
│       │   ├── memchr-e48cf06112b0cb00.d
│       │   ├── libregex-76833208390bba40.rmeta
│       │   ├── libbit_set-7c3c824b8000ac3c.rmeta
│       │   ├── bit_vec-0afb4ba3382ab368.d
│       │   ├── bstr-3516875550e89b46.bstr.db2bb989631abf3e-cgu.1.rcgu.o
│       │   ├── libproc_macro2-53f1f5c378e4910d.rmeta
│       │   ├── same_file-32d6dcce2e0d14c3.d
│       │   ├── taiko-eddd48b28a2017fa.d
│       │   ├── libheck-698dd90d905fb3d9.rmeta
│       │   ├── taiko-5061137b45c3c774.d
│       │   ├── libpest_meta-c3a5db06ffcfb496.rmeta
│       │   ├── libclap_lex-ab8face252e39df4.rmeta
│       │   ├── parking_lot_core-d102f8c1d60826b4.d
│       │   ├── unicode_width-ca1ace74c70dfb2a.d
│       │   ├── libautocfg-87cda2e1edfcdf6c.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.01.rcgu.o
│       │   ├── libsyn-d2b4a3bd7aae0608.rlib
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.08.rcgu.o
│       │   ├── libclap-183d96304d7c6951.rlib
│       │   ├── base64-bdbacd6f5912bab4.d
│       │   ├── taiko-5f5482d76af3ad3e.exrqpc0l22kt700.rcgu.o
│       │   ├── regex-76833208390bba40.d
│       │   ├── proc_macro2-5b0542e781e56cc7.d
│       │   ├── thiserror_impl-cc61fd152678c91d.d
│       │   ├── pest_meta-763306f9a1f70e0e.d
│       │   ├── aho_corasick-4690a1ce781d3d30.d
│       │   ├── libmalloc_buf-eaef8ee3f1de8168.rlib
│       │   ├── unicode_width-f704b80496ed2425.d
│       │   ├── libmacro-fcf57fd3cf106838.dylib
│       │   ├── libpest_generator-5af4eca3dc36d733.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.26r61b9oymxj5bj6.rcgu.o
│       │   ├── ryu-bc0b344f98438129.d
│       │   ├── liblog-0bb24d5082652099.rmeta
│       │   ├── pest_meta-757e14534d182046.d
│       │   ├── proc_macro2-53f1f5c378e4910d.d
│       │   ├── taiko-5f5482d76af3ad3e.53550xyw71saqcnj.rcgu.o
│       │   ├── libclap-183d96304d7c6951.rmeta
│       │   ├── anstream-c9c41d61ff842eeb.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.14.rcgu.o
│       │   ├── libconsole-6f82366262245499.rmeta
│       │   ├── libpest_meta-646a0dbae8a78fde.rlib
│       │   ├── taiko-5f5482d76af3ad3e.d
│       │   ├── anstyle_parse-544d360a395d43a6.anstyle_parse.a5a86d66167dd00b-cgu.0.rcgu.o
│       │   ├── macro-8730cf09808ccf8c.d
│       │   ├── taiko-5f5482d76af3ad3e.2smxdnkjonxooqi7.rcgu.o
│       │   ├── libthiserror-83a66a01eb224bff.rmeta
│       │   ├── termtree-63ffc5db97ed923c.termtree.3432f4eab2c35a-cgu.0.rcgu.o
│       │   ├── bit_vec-1ceac5fefb9229e6.d
│       │   ├── parking_lot_core-d102f8c1d60826b4.parking_lot_core.ed50d05b6b4b6b92-cgu.1.rcgu.o
│       │   ├── crossbeam-fc5ab7c357eb0886.d
│       │   ├── libunicode_ident-8c0673ce5c824490.rmeta
│       │   ├── libbase64-bdbacd6f5912bab4.rmeta
│       │   ├── libcli_clipboard-8ee37ab0b7ce884f.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.06.rcgu.o
│       │   ├── rayon-a641aab5cd97d3de.rayon.81f2f342fe3ac30b-cgu.2.rcgu.o
│       │   ├── log-e4b3d7d40b5ea863.d
│       │   ├── libprompt_macro-b787227993fb113d.dylib
│       │   ├── either-6ffdf15de26bd676.either.fc73ab5e3b6b03b2-cgu.0.rcgu.o
│       │   ├── console-add040e40b2f8f94.d
│       │   ├── libcrossbeam_deque-3f19b7bc5f5cbf56.rmeta
│       │   ├── libsyn-d2b4a3bd7aae0608.rmeta
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.01.rcgu.o
│       │   ├── libautocfg-08f5cf05d6405d00.rmeta
│       │   ├── libmemchr-9e426edd08692ba7.rlib
│       │   ├── smallvec-5a30387ee9f221d9.d
│       │   ├── librayon-a641aab5cd97d3de.rmeta
│       │   ├── libregex-0f8107ff5ca70c14.rlib
│       │   ├── libcrossbeam_channel-8c094417765e7827.rmeta
│       │   ├── ucd_trie-c6b083631db69857.d
│       │   ├── parse_format-d12b5b25e64c4bdb.d
│       │   ├── cli_clipboard-21af94370cbb9111.d
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.0.rcgu.o
│       │   ├── globset-b3bed3e7224f2a1d.d
│       │   ├── libserde_json-46bd2ddb93a26abd.rmeta
│       │   ├── regex-215f349788d17753.d
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.00.rcgu.o
│       │   ├── libpest_meta-03395d72a0dfe259.rmeta
│       │   ├── libmemchr-e48cf06112b0cb00.rlib
│       │   ├── crossbeam_channel-fdc0ae76d9c3397f.d
│       │   ├── libpest-d3e9ce9e178f265d.rmeta
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.03.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.2kyphk6ya49r6q8z.rcgu.o
│       │   ├── libcrossbeam_epoch-1695a116cad47243.rmeta
│       │   ├── libblock-f5cb22d2e0ed675f.rmeta
│       │   ├── libcrossbeam_epoch-821d298372abaa1d.rmeta
│       │   ├── libeither-3983297f66cd6218.rmeta
│       │   ├── libproc_macro2-18e1e4b8cfd0c269.rmeta
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.0.rcgu.o
│       │   ├── colorchoice-79ef33375f88230b.colorchoice.eae1d236d1a6ac87-cgu.0.rcgu.o
│       │   ├── strsim-fadff44222618b4b.strsim.e1140db0c11a0d78-cgu.3.rcgu.o
│       │   ├── librayon_core-a8051b96fd8cee48.rmeta
│       │   ├── portable_atomic-86e97a657a48c7f6.d
│       │   ├── libmemchr-e48cf06112b0cb00.rmeta
│       │   ├── libpest_meta-dc9c6f7dea0d78f4.rmeta
│       │   ├── code2prompt-d94e4e13a9165e5e.d
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.13.rcgu.o
│       │   ├── libregex_syntax-0a1f0f14aeb00775.rmeta
│       │   ├── libsmallvec-5a30387ee9f221d9.rlib
│       │   ├── libanyhow-1b066440c0121ea7.rmeta
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.12.rcgu.o
│       │   ├── libobjc_id-d8e2530d5b0ab1b8.rmeta
│       │   ├── clap_derive-5a22d46402a77847.d
│       │   ├── anstyle_query-a6e7118bf77d1478.d
│       │   ├── libparse_format-65b5d42e6ef4c317.rlib
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.07.rcgu.o
│       │   ├── librayon_core-fc07d5c6ab3bcf2c.rmeta
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.11.rcgu.o
│       │   ├── libclap_builder-3bf11762de3d2225.rmeta
│       │   ├── crossbeam_queue-63b742e294933c25.d
│       │   ├── libtermtree-120ecd86391cc49e.rmeta
│       │   ├── walkdir-c9c753dae4c51eae.d
│       │   ├── libregex_automata-2c01a250e90ac542.rmeta
│       │   ├── libobjc-6bc02222399e6eef.rmeta
│       │   ├── libcrossbeam_utils-18686986dd98f802.rlib
│       │   ├── libonce_cell-9e5aa9cec64e1de2.rmeta
│       │   ├── syn-47e79c06fd625548.d
│       │   ├── libstrsim-fadff44222618b4b.rmeta
│       │   ├── anstream-c9c41d61ff842eeb.anstream.9266a8ef28ea17e3-cgu.1.rcgu.o
│       │   ├── libonce_cell-e8773422b0f43014.rmeta
│       │   ├── libprompt_macro-00aa2d85d6355b44.dylib
│       │   ├── libthiserror-ab23e760f2b3bad1.rlib
│       │   ├── libcrossbeam_epoch-1695a116cad47243.rlib
│       │   ├── libthiserror-7fd1eef083610ebb.rlib
│       │   ├── libserde-f47b43f94aadc098.rlib
│       │   ├── ryu-468c4aa142a2ea4e.d
│       │   ├── walkdir-744b0cb7cd472aaa.d
│       │   ├── libportable_atomic-d7a74020c9b9fe12.rmeta
│       │   ├── serde_json-3766680e5b132a36.d
│       │   ├── libcolored-77bb8fdb6306fbb7.rlib
│       │   ├── taiko-5f5482d76af3ad3e.w6qw59kh5lr0j5f.rcgu.o
│       │   ├── clap_derive-66cadc6df51f5d9d.d
│       │   ├── libobjc_id-e778945f2e2d0c47.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.3cpe0k7r03o1szon.rcgu.o
│       │   ├── libprompt_macro-d3fe94433cb69dc4.rmeta
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.6.rcgu.o
│       │   ├── unicode_ident-eb62d7b284154f68.d
│       │   ├── thiserror-b0e51cc25e506228.d
│       │   ├── libsame_file-5b4e7ccbf3e1f175.rmeta
│       │   ├── libitoa-6e50e7815faad2c1.rmeta
│       │   ├── libmemchr-d6546b159e1b463b.rmeta
│       │   ├── libcrossbeam_utils-88b9235b0b0f49fb.rmeta
│       │   ├── quote-b7048526dbec7acb.d
│       │   ├── bit_vec-d0100a597cc971f3.bit_vec.7a831650774f41bb-cgu.0.rcgu.o
│       │   ├── objc-6bc02222399e6eef.d
│       │   ├── rayon_core-a2856f9f57a1e6c2.d
│       │   ├── crossbeam_deque-7808d91b25723717.crossbeam_deque.9aec7cbf5e0aaf8e-cgu.0.rcgu.o
│       │   ├── base64-69a5e3c040cfede6.d
│       │   ├── libpest_derive-8b5502e962597fb9.dylib
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.0.rcgu.o
│       │   ├── liblibc-a3e4d221db19ab38.rlib
│       │   ├── libclap-d3f8da414072d201.rmeta
│       │   ├── libregex-215f349788d17753.rmeta
│       │   ├── libserde_json-46bd2ddb93a26abd.rlib
│       │   ├── anstyle-3e72b357153a842f.d
│       │   ├── taiko-5f5482d76af3ad3e.344jaybo67ov5gq6.rcgu.o
│       │   ├── libbit_set-6f81487daedf45e2.rmeta
│       │   ├── libmemchr-9af1c8a56ee8a9d6.rlib
│       │   ├── libobjc_foundation-c6bbb7d5207330fa.rmeta
│       │   ├── serde-ac3a19c9b2414b1f.d
│       │   ├── libunindent-b0bd2a506b19c0a4.rmeta
│       │   ├── libclap_builder-07298f382acc0d93.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.2vkqxnkt4flsjaby.rcgu.o
│       │   ├── libanstyle_parse-0f351125273fc2d4.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.31tiwzjsxz7w7gux.rcgu.o
│       │   ├── libjwalk-9935bac444b172dd.rmeta
│       │   ├── termtree-120ecd86391cc49e.d
│       │   ├── libbase64-293239c257061bcb.rmeta
│       │   ├── libsmallvec-4106a1e233fbf2ba.rmeta
│       │   ├── clap_lex-4292f37165d83514.d
│       │   ├── libparking_lot-937ce451f7998a83.rmeta
│       │   ├── aho_corasick-94342bc35c3ad340.d
│       │   ├── libpest-8a80f6c810d9ebf2.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.02.rcgu.o
│       │   ├── libclap_lex-4292f37165d83514.rmeta
│       │   ├── libanstyle_parse-a6c619ba8ed0ba68.rmeta
│       │   ├── libglobset-d77835a08215d720.rmeta
│       │   ├── pest_derive-422a3fa6425b8f73.d
│       │   ├── libunicode_ident-eb62d7b284154f68.rmeta
│       │   ├── clap_lex-1030d2fa2711077f.clap_lex.da2cdf95e2f2a55-cgu.0.rcgu.o
│       │   ├── prompt_macro-b787227993fb113d.d
│       │   ├── libcrossbeam_queue-63b742e294933c25.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.3mzadfyojbg48g48.rcgu.o
│       │   ├── block-a814ef6c1ee6440a.block.897eb148a5edd8a4-cgu.0.rcgu.o
│       │   ├── librayon_core-c5baa12ac57958ec.rmeta
│       │   ├── libobjc_foundation-f22c8b27505b154a.rmeta
│       │   ├── libcolored-f86f6016082595f1.rmeta
│       │   ├── libcolorchoice-0e0af3dd6900341b.rmeta
│       │   ├── memchr-9b9ca7881fffae76.memchr.c950af87f4bab50d-cgu.0.rcgu.o
│       │   ├── libautocfg-d0bb26a33a75c25c.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.4ig3nlhcd4cqyowo.rcgu.o
│       │   ├── colorchoice-3af447a85e1f9cfe.d
│       │   ├── block-8f64c5ae264aca5f.d
│       │   ├── libthiserror-9684af3710b94797.rmeta
│       │   ├── number_prefix-531fe2cfc1d66054.d
│       │   ├── bstr-d0720bfc6931e58a.d
│       │   ├── anstyle-cc3da9f40336992b.d
│       │   ├── pest_generator-3d5fc24121b48bba.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.10.rcgu.o
│       │   ├── libclap_derive-f50b7d70f89c4204.dylib
│       │   ├── libparse_format-5f7c65faf1c4422a.rlib
│       │   ├── libthiserror-dd7f932d747b83fd.rmeta
│       │   ├── walkdir-723ddb56cce4eeb6.walkdir.2fd6da2df1898303-cgu.0.rcgu.o
│       │   ├── objc_foundation-2eea5b8768bd59e9.objc_foundation.8d264bf540fb910d-cgu.0.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.2buf9o9p8n830ug6.rcgu.o
│       │   ├── ucd_trie-31ce46028bbae57f.d
│       │   ├── crossbeam_epoch-8a7b82de4db57161.d
│       │   ├── pest-2b50acd6dff662ba.pest.dd21cf03355f4f0a-cgu.1.rcgu.o
│       │   ├── proc_macro2-ace53266be815410.d
│       │   ├── crossbeam_utils-18686986dd98f802.crossbeam_utils.712b2907679f27b8-cgu.1.rcgu.o
│       │   ├── libcfg_if-45b3cfd05c61975a.rmeta
│       │   ├── anyhow-2df9d65eaaaa7324.anyhow.61955e5b11b0378d-cgu.1.rcgu.o
│       │   ├── libobjc_id-95409c6e5bfb8fd3.rlib
│       │   ├── crossbeam_utils-f72012fcfdf033a4.d
│       │   ├── libunicode_width-ca1ace74c70dfb2a.rmeta
│       │   ├── log-0bb24d5082652099.d
│       │   ├── libscopeguard-78b42e8fc5e857b8.rmeta
│       │   ├── libconsole-db545e6f7af4471e.rmeta
│       │   ├── libtaiko-9fa3b50fce309f87.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.11.rcgu.o
│       │   ├── liblibc-a3e4d221db19ab38.rmeta
│       │   ├── libcolored-b2c368a2264b59e0.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.2jkvv7jbolzny3i.rcgu.o
│       │   ├── pest_derive-c166c2eba4646155.d
│       │   ├── libclap_derive-5ad44a07f9b0b512.dylib
│       │   ├── taiko-5f5482d76af3ad3e.2rb8aofti8peg4s8.rcgu.o
│       │   ├── libtaiko-fcb18f34e23635e6.rmeta
│       │   ├── unicode_ident-4fa8e70de8323614.d
│       │   ├── crossbeam_queue-2aef4dbdcfccc3c1.d
│       │   ├── libunicode_ident-81e6f0701cb43f8a.rlib
│       │   ├── crossbeam_utils-88b9235b0b0f49fb.d
│       │   ├── libucd_trie-f1a751efa5939de4.rlib
│       │   ├── libquote-2aec2c4a0083645f.rmeta
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.12.rcgu.o
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.05.rcgu.o
│       │   ├── libcolored-77bb8fdb6306fbb7.rmeta
│       │   ├── libproc_macro2-ace53266be815410.rmeta
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.00.rcgu.o
│       │   ├── libcli_clipboard-1de30f25a56af333.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.09.rcgu.o
│       │   ├── rayon-965a1ceb4216c982.d
│       │   ├── libbit_set-4059202125707996.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.03.rcgu.o
│       │   ├── libcolored-722d01c60457fd3f.rmeta
│       │   ├── libcrossbeam-999b280ae2ae4dd9.rmeta
│       │   ├── termtree-dafa81c0289883c2.d
│       │   ├── console-7ae5fb73e1277bb2.d
│       │   ├── same_file-5b4e7ccbf3e1f175.d
│       │   ├── clap_builder-fe335d7668ae5da9.d
│       │   ├── libaho_corasick-a4b46a431e5663f4.rmeta
│       │   ├── libregex_syntax-45becf883096970e.rlib
│       │   ├── objc_id-5439da7a615eaaf7.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.00.rcgu.o
│       │   ├── serde-9116e3003993b927.d
│       │   ├── thiserror-e448c25cc0294170.d
│       │   ├── libmacro-184818f65e53cea8.rmeta
│       │   ├── crossbeam_deque-867204b66eaea281.d
│       │   ├── thiserror-7d04976b43fa628e.d
│       │   ├── libsyn-2ab39f0e4fc565c6.rmeta
│       │   ├── libcfg_if-e9f2eedc90029417.rmeta
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.2.rcgu.o
│       │   ├── parking_lot-c365af0e5de08631.d
│       │   ├── libportable_atomic-fca8db96086604f2.rmeta
│       │   ├── clap_lex-ab8face252e39df4.d
│       │   ├── regex_automata-e3223faa738ad36b.d
│       │   ├── lock_api-7c6bc2669b134fb3.d
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.4.rcgu.o
│       │   ├── once_cell-508ab5f09252e905.d
│       │   ├── libsyn-d7643833d974eb60.rlib
│       │   ├── globset-18f2a14144fb0c54.d
│       │   ├── cli_clipboard-1de30f25a56af333.cli_clipboard.8aa09a2b7f50a4e4-cgu.0.rcgu.o
│       │   ├── libthiserror-e448c25cc0294170.rlib
│       │   ├── indicatif-0ba12c0f222c86ae.d
│       │   ├── base64-6e96c5a9915be65f.base64.c69e558fad926954-cgu.0.rcgu.o
│       │   ├── libmemchr-210dc6bad0ef55ff.rmeta
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.8.rcgu.o
│       │   ├── libquote-b7048526dbec7acb.rmeta
│       │   ├── libunicode_ident-4fa8e70de8323614.rlib
│       │   ├── regex-0f8107ff5ca70c14.d
│       │   ├── libquote-cdfe020df82b4d1d.rlib
│       │   ├── ucd_trie-f1a751efa5939de4.ucd_trie.271f527e5aea8b64-cgu.2.rcgu.o
│       │   ├── rustc_hash-952c00a7f09c9aa7.d
│       │   ├── libmemchr-9b9ca7881fffae76.rlib
│       │   ├── taiko-76736ea0e179a76d.d
│       │   ├── libautocfg-87cda2e1edfcdf6c.rlib
│       │   ├── unicode_ident-81e6f0701cb43f8a.d
│       │   ├── liblibc-6aecec00ac62ff86.rmeta
│       │   ├── libutf8parse-93787ca028a6d438.rmeta
│       │   ├── objc_foundation-5b46b178f3d630b7.d
│       │   ├── libonce_cell-1b3d204c83a12703.rlib
│       │   ├── libregex-79f28fcb5b054383.rmeta
│       │   ├── serde_json-4d2f2b02b377dd94.d
│       │   ├── libmemchr-210dc6bad0ef55ff.rlib
│       │   ├── libparking_lot-ab409976f3a9a8f3.rmeta
│       │   ├── clap_derive-eba727efa8166a4e.d
│       │   ├── libc-a3e4d221db19ab38.libc.b1f5482b71adc603-cgu.0.rcgu.o
│       │   ├── autocfg-87cda2e1edfcdf6c.d
│       │   ├── libheck-24d0bb0cde6f2d31.rmeta
│       │   ├── clap_derive-5ad44a07f9b0b512.d
│       │   ├── libpest_meta-646a0dbae8a78fde.rmeta
│       │   ├── libproc_macro2-e3ead2790c8a6ade.rlib
│       │   ├── taiko-5f5482d76af3ad3e.3cn0bh76q38ki1on.rcgu.o
│       │   ├── libthiserror-2adf4c655155bef1.rmeta
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.4.rcgu.o
│       │   ├── parse_format-8ca4bf5ed49416ac.d
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.08.rcgu.o
│       │   ├── liblazy_static-2a7ede781c6dce4a.rmeta
│       │   ├── quote-bb3c202bc622fea6.d
│       │   ├── libsyn-d7643833d974eb60.rmeta
│       │   ├── parking_lot-75ed7d3a6b2d6614.d
│       │   ├── quote-2aec2c4a0083645f.d
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.03.rcgu.o
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.09.rcgu.o
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.15.rcgu.o
│       │   ├── libunicode_ident-8c0673ce5c824490.rlib
│       │   ├── libcrossbeam_utils-a7c3d28de1cc7e82.rmeta
│       │   ├── libobjc_id-95409c6e5bfb8fd3.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.51wbm0v3ee98k6ip.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.ru37bqi7y0d4sif.rcgu.o
│       │   ├── block-ba265c29da368d38.d
│       │   ├── crossbeam_queue-f370a9ca94baf21b.d
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.05.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.1hhoo4rerd11y0n3.rcgu.o
│       │   ├── syn-db3f05bad1825a44.d
│       │   ├── regex_syntax-e240d2964da64048.d
│       │   ├── librayon_core-a2856f9f57a1e6c2.rmeta
│       │   ├── libpest_meta-757e14534d182046.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.24lqk1tq9pqzqefs.rcgu.o
│       │   ├── libthiserror-ab23e760f2b3bad1.rmeta
│       │   ├── libmalloc_buf-5265224e247f551c.rmeta
│       │   ├── ignore-a5403807be371545.d
│       │   ├── taiko-5f5482d76af3ad3e.2oi91zgtyseheqwj.rcgu.o
│       │   ├── libfancy_regex-f9979b02061d09b7.rlib
│       │   ├── serde-5b71833873e30944.d
│       │   ├── taiko-5f5482d76af3ad3e.4vxtzz7j739mji19.rcgu.o
│       │   ├── libindicatif-c97a0576239e6a63.rmeta
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.07.rcgu.o
│       │   ├── libucd_trie-c6b083631db69857.rmeta
│       │   ├── pest-348d1225c08310b0.d
│       │   ├── malloc_buf-5265224e247f551c.d
│       │   ├── libquote-98d955d671fa9ae9.rlib
│       │   ├── libobjc_foundation-2eea5b8768bd59e9.rmeta
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.4.rcgu.o
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.8.rcgu.o
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.04.rcgu.o
│       │   ├── libucd_trie-dcd6580dd443e17a.rmeta
│       │   ├── libregex-e853df995e92a0fb.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.59zk6h2ymlrkol25.rcgu.o
│       │   ├── libeither-6ffdf15de26bd676.rmeta
│       │   ├── anyhow-2df9d65eaaaa7324.d
│       │   ├── libunicode_ident-c820b7f471869748.rmeta
│       │   ├── libanstream-c9c41d61ff842eeb.rmeta
│       │   ├── anstyle_parse-0f351125273fc2d4.d
│       │   ├── libthiserror-5ae0973ba1aab17e.rmeta
│       │   ├── libtaiko-39abda136aead6d6.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.40hrvv4ddb6fyt2z.rcgu.o
│       │   ├── taiko-f08293747cba18c3.d
│       │   ├── libthiserror-e448c25cc0294170.rmeta
│       │   ├── libcolored-e9a596b387e8b72c.rmeta
│       │   ├── libpest_derive-422a3fa6425b8f73.dylib
│       │   ├── ucd_trie-f1a751efa5939de4.ucd_trie.271f527e5aea8b64-cgu.0.rcgu.o
│       │   ├── crossbeam_channel-9b64f4cef3c2d868.d
│       │   ├── same_file-6e6845bfa543b20f.d
│       │   ├── crossbeam-5e9fbe378655c40e.d
│       │   ├── libanstyle-cc3da9f40336992b.rmeta
│       │   ├── libsame_file-6e6845bfa543b20f.rmeta
│       │   ├── termtree-9eacc883d229c4f3.d
│       │   ├── serde_json-36b0216e9098d50f.d
│       │   ├── clap_lex-1030d2fa2711077f.d
│       │   ├── anstyle_query-3bc001f5a383188e.d
│       │   ├── unicode_width-2f86821300067d55.d
│       │   ├── taiko-5f5482d76af3ad3e.1fk0y92xfp8yx7lq.rcgu.o
│       │   ├── scopeguard-78b42e8fc5e857b8.d
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.6.rcgu.o
│       │   ├── libanstyle_query-7aa23c30b201a805.rmeta
│       │   ├── base64-de66e0b6f7ed0906.d
│       │   ├── libclap-8d71986fb942cffc.rmeta
│       │   ├── unicode_ident-323a696b6fd93af8.d
│       │   ├── libindicatif-fa070afa23fb479a.rmeta
│       │   ├── pest_derive-8fcf80f809a489d5.d
│       │   ├── libbit_vec-d0100a597cc971f3.rmeta
│       │   ├── libstrsim-56672850028f243e.rmeta
│       │   ├── clap_derive-2cac4eeb373a37aa.d
│       │   ├── strsim-56672850028f243e.d
│       │   ├── libserde_json-3cc7104dfc7c5b4e.rmeta
│       │   ├── libc-6aecec00ac62ff86.d
│       │   ├── libsame_file-32d6dcce2e0d14c3.rmeta
│       │   ├── liblog-9a66223a5e030806.rlib
│       │   ├── regex_automata-fbc90720657fe65b.d
│       │   ├── scopeguard-5a9fa7d4f030a5c0.d
│       │   ├── taiko-5f5482d76af3ad3e.4e9ce35ptscxgt6l.rcgu.o
│       │   ├── libscopeguard-e2949efb413bface.rmeta
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.0.rcgu.o
│       │   ├── rayon-f8933dc420cf2428.d
│       │   ├── libparking_lot-9e0c399cf98f8a24.rmeta
│       │   ├── libanstyle_parse-544d360a395d43a6.rmeta
│       │   ├── regex_syntax-0a1f0f14aeb00775.d
│       │   ├── same_file-d377ec210631ce56.d
│       │   ├── libglobset-79a72d153d181801.rmeta
│       │   ├── cfg_if-13a8e19d179f00de.d
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.05.rcgu.o
│       │   ├── syn-f20dfd11b09216d9.d
│       │   ├── libnumber_prefix-bb7a15a300755127.rmeta
│       │   ├── libobjc_foundation-5b46b178f3d630b7.rmeta
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.06.rcgu.o
│       │   ├── anstyle_query-af4c0252b5bddea8.d
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.6.rcgu.o
│       │   ├── libregex_syntax-45becf883096970e.rmeta
│       │   ├── libfancy_regex-5c74bb0ac19aa96e.rmeta
│       │   ├── pest-a336ecea6014468c.d
│       │   ├── liblog-4f8d2beb91e2c81d.rmeta
│       │   ├── syn-a22e3a50ce0cd6e9.d
│       │   ├── walkdir-f97164972ff9ea9d.d
│       │   ├── libclap_derive-2cac4eeb373a37aa.dylib
│       │   ├── libnumber_prefix-dade78a5a916dddb.rmeta
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.07.rcgu.o
│       │   ├── libclap_lex-8d638cc28f6d33df.rmeta
│       │   ├── libquote-ebea092d1391ccb4.rmeta
│       │   ├── libpest_meta-03395d72a0dfe259.rlib
│       │   ├── crossbeam_deque-7808d91b25723717.d
│       │   ├── libstrsim-07b9a41bde477617.rmeta
│       │   ├── unindent-b0bd2a506b19c0a4.d
│       │   ├── memchr-d050dac8c0ef965f.d
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.01.rcgu.o
│       │   ├── libobjc-72251fe41e6eaf6d.rmeta
│       │   ├── libsame_file-6f1ef61c7135d1cb.rmeta
│       │   ├── anstyle_parse-fe31e2ba51a3cf1b.d
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.14.rcgu.o
│       │   ├── libanyhow-f6d938147879d3de.rmeta
│       │   ├── libsmallvec-e53885e764a5bbff.rmeta
│       │   ├── libpest-426fd3f80d09c9fa.rmeta
│       │   ├── pest-7ca1fc2e97cc4b6c.d
│       │   ├── thiserror-4ceb9fe7530f729d.d
│       │   ├── lock_api-82983c2133f5a1f6.d
│       │   ├── taiko-5f5482d76af3ad3e.4txfsig44eip83du.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.4l6pjztib7q2kdj5.rcgu.o
│       │   ├── console-6f82366262245499.d
│       │   ├── libutf8parse-c4e9f128df76b4db.rmeta
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.09.rcgu.o
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.15.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.41otuwf3pxp6hjab.rcgu.o
│       │   ├── strsim-fadff44222618b4b.strsim.e1140db0c11a0d78-cgu.5.rcgu.o
│       │   ├── rustc_hash-eff6d23f1ed3d804.d
│       │   ├── libparking_lot_core-5341009d00e1e7bb.rmeta
│       │   ├── objc-9aceb7739b194f45.objc.1dbd854cedaf6697-cgu.0.rcgu.o
│       │   ├── libparking_lot_core-d102f8c1d60826b4.rlib
│       │   ├── crossbeam_channel-b311370f4de6afb9.d
│       │   ├── librustc_hash-0d95bed8e9e70cbe.rmeta
│       │   ├── bit_set-d7322ea878b61688.d
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.6.rcgu.o
│       │   ├── libanstyle_query-830166fdf9e655c7.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.12.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.595sew38pf6l0hpt.rcgu.o
│       │   ├── librustc_hash-fefca1a8f4a2c364.rmeta
│       │   ├── libpest-9b28f6cac5ba28d8.rmeta
│       │   ├── unicode_ident-24c05b45cc2da0bf.d
│       │   ├── regex-0f8107ff5ca70c14.regex.d7700752044f9ef2-cgu.1.rcgu.o
│       │   ├── walkdir-723ddb56cce4eeb6.walkdir.2fd6da2df1898303-cgu.2.rcgu.o
│       │   ├── libhandlebars-97344c36524f5209.rmeta
│       │   ├── cli_clipboard-105138324c549219.d
│       │   ├── globset-42ddf0677cf59874.d
│       │   ├── libproc_macro2-203b1862d9ee796a.rlib
│       │   ├── handlebars-97344c36524f5209.d
│       │   ├── memchr-9af1c8a56ee8a9d6.d
│       │   ├── taiko-5f5482d76af3ad3e.587vwv27xrlz7gc5.rcgu.o
│       │   ├── unicode_ident-8c0673ce5c824490.d
│       │   ├── macro-184818f65e53cea8.d
│       │   ├── libnumber_prefix-531fe2cfc1d66054.rmeta
│       │   ├── clap_builder-971a7de4ef516dae.d
│       │   ├── libunindent-5d879711c2b503b6.rmeta
│       │   ├── libsame_file-04b1f130144ea03a.rmeta
│       │   ├── anstyle-f05d8c7856d7e78e.d
│       │   ├── libparking_lot_core-c5651f062de4fb05.rmeta
│       │   ├── tiktoken_rs-fd3595d3a846a6ac.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.00.rcgu.o
│       │   ├── objc_foundation-c6bbb7d5207330fa.d
│       │   ├── parking_lot-937ce451f7998a83.parking_lot.3b0075210154421a-cgu.1.rcgu.o
│       │   ├── parking_lot-ab409976f3a9a8f3.d
│       │   ├── proc_macro2-f7065532d3eae9df.d
│       │   ├── libwalkdir-744b0cb7cd472aaa.rmeta
│       │   ├── once_cell-9e5aa9cec64e1de2.d
│       │   ├── libwalkdir-ddd7218531b92be4.rmeta
│       │   ├── ryu-7dfefb4f0eac89cd.d
│       │   ├── libregex_automata-357a41b18ec6022f.rmeta
│       │   ├── autocfg-08f5cf05d6405d00.d
│       │   ├── libthiserror-d715994f55f32c4b.rmeta
│       │   ├── prompt_macro-00aa2d85d6355b44.d
│       │   ├── libpest_generator-3d5fc24121b48bba.rmeta
│       │   ├── libnumber_prefix-2ed70394c0e1d1a6.rmeta
│       │   ├── anyhow-f6d938147879d3de.d
│       │   ├── libquote-2aec2c4a0083645f.rlib
│       │   ├── colorchoice-bb8ac1d01a3b8369.d
│       │   ├── libryu-7dfefb4f0eac89cd.rlib
│       │   ├── anstream-a0d112ff6640ce06.d
│       │   ├── malloc_buf-6e1f00ba28e7b2a6.d
│       │   ├── rayon_core-a8051b96fd8cee48.d
│       │   ├── libpest-a336ecea6014468c.rmeta
│       │   ├── libproc_macro2-29c5e0e4ca278bae.rmeta
│       │   ├── libobjc_foundation-2eea5b8768bd59e9.rlib
│       │   ├── cfg_if-e9f2eedc90029417.d
│       │   ├── log-9a66223a5e030806.d
│       │   ├── tiktoken_rs-631e7da1ceaab6f2.d
│       │   ├── libanstyle_parse-fe31e2ba51a3cf1b.rmeta
│       │   ├── libclap-e7b61dcae432590e.rmeta
│       │   ├── cfg_if-f63cf2e845327ac6.d
│       │   ├── libwalkdir-723ddb56cce4eeb6.rmeta
│       │   ├── libmemchr-9b9ca7881fffae76.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.15.rcgu.o
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.02.rcgu.o
│       │   ├── console-db545e6f7af4471e.d
│       │   ├── libclap_lex-1030d2fa2711077f.rlib
│       │   ├── utf8parse-f2acd427c0a95aab.d
│       │   ├── taiko-5f5482d76af3ad3e.zt7i6emmecz7zj8.rcgu.o
│       │   ├── libconsole-16f998361d7a45ce.rmeta
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.02.rcgu.o
│       │   ├── itoa-ee973ff0bc0fc6f2.d
│       │   ├── memchr-9e426edd08692ba7.d
│       │   ├── crossbeam_epoch-1695a116cad47243.d
│       │   ├── objc_id-95409c6e5bfb8fd3.d
│       │   ├── libutf8parse-ce5e6d0fbc2807eb.rmeta
│       │   ├── libtermtree-63ffc5db97ed923c.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.01.rcgu.o
│       │   ├── libproc_macro2-f7065532d3eae9df.rmeta
│       │   ├── block-a814ef6c1ee6440a.d
│       │   ├── serde_json-3cc7104dfc7c5b4e.d
│       │   ├── libcrossbeam-fc5ab7c357eb0886.rmeta
│       │   ├── libparking_lot-937ce451f7998a83.rlib
│       │   ├── autocfg-0eecde6036b9415d.d
│       │   ├── strsim-fadff44222618b4b.d
│       │   ├── syn-79418542530a1a9b.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.10.rcgu.o
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.07.rcgu.o
│       │   ├── libanstream-c9c41d61ff842eeb.rlib
│       │   ├── crossbeam_utils-18686986dd98f802.crossbeam_utils.712b2907679f27b8-cgu.3.rcgu.o
│       │   ├── libheck-79e04a5211bdd404.rmeta
│       │   ├── anyhow-2df9d65eaaaa7324.anyhow.61955e5b11b0378d-cgu.3.rcgu.o
│       │   ├── number_prefix-4b0f92ccd4a73404.number_prefix.a87f77bc40836102-cgu.0.rcgu.o
│       │   ├── indicatif-c6b15a49e77c4728.d
│       │   ├── libthiserror_impl-538e6a41d5bf27ac.dylib
│       │   ├── libparking_lot-9e5145b6ec7b28b8.rmeta
│       │   ├── unindent-60d7bc434b501888.d
│       │   ├── portable_atomic-d7a74020c9b9fe12.portable_atomic.7d1dd0cdd52ae42b-cgu.0.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.400erkvo81kk5c9b.rcgu.o
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.13.rcgu.o
│       │   ├── libmemchr-d050dac8c0ef965f.rmeta
│       │   ├── librayon-965a1ceb4216c982.rmeta
│       │   ├── colored-6587aeda69587c72.d
│       │   ├── bstr-3516875550e89b46.d
│       │   ├── rayon_core-fc07d5c6ab3bcf2c.d
│       │   ├── taiko-5f5482d76af3ad3e.4t8heu737mc3a60d.rcgu.o
│       │   ├── libanstyle_parse-38193868a4bc2feb.rmeta
│       │   ├── parking_lot_core-d102f8c1d60826b4.parking_lot_core.ed50d05b6b4b6b92-cgu.2.rcgu.o
│       │   ├── ryu-c053dfd3b64364c1.d
│       │   ├── clap_lex-9e784cf71c4dbe13.d
│       │   ├── utf8parse-ce5e6d0fbc2807eb.d
│       │   ├── libanstyle_parse-9d6e9e70ccaa3642.rmeta
│       │   ├── librustc_hash-cb11f00b55fa406c.rlib
│       │   ├── memchr-825b4f36e101cadf.d
│       │   ├── libheck-3c14e1c860b4929e.rlib
│       │   ├── libtaiko-e3f3f14e7a4f2dc3.rmeta
│       │   ├── lazy_static-4fc00963a7215728.d
│       │   ├── scopeguard-1a41f4f101669df5.scopeguard.1a9f97d81b38826e-cgu.0.rcgu.o
│       │   ├── handlebars-7682939d551c053d.d
│       │   ├── pest_meta-646a0dbae8a78fde.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.05.rcgu.o
│       │   ├── rayon-a641aab5cd97d3de.rayon.81f2f342fe3ac30b-cgu.1.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.1ou8tv2ptzpsnygm.rcgu.o
│       │   ├── ignore-8646944c799ad137.d
│       │   ├── libbstr-e83c5b67e0a2b2dd.rmeta
│       │   ├── libglobset-42ddf0677cf59874.rmeta
│       │   ├── memchr-53c5fe84ba9abe21.d
│       │   ├── lazy_static-e3bceafda619eadf.d
│       │   ├── libsyn-2ab39f0e4fc565c6.rlib
│       │   ├── parking_lot_core-ecd370daa92df0e4.d
│       │   ├── jwalk-0c6633ef064c2b32.d
│       │   ├── libtiktoken_rs-6b9d2c49470fe1b4.rmeta
│       │   ├── libpest_generator-fc27fdfcc846e3cc.rmeta
│       │   ├── libutf8parse-a5a14685b2cf96f0.rmeta
│       │   ├── libcrossbeam-044650bffcd98472.rmeta
│       │   ├── crossbeam_utils-01dce0ec683fd523.d
│       │   ├── libryu-b38cca26303b1aac.rmeta
│       │   ├── itoa-b219816d8a894515.d
│       │   ├── libserde_json-3766680e5b132a36.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.09.rcgu.o
│       │   ├── libcrossbeam_queue-f370a9ca94baf21b.rmeta
│       │   ├── parse_format-e3d136611627ca9d.d
│       │   ├── libpest_meta-dc9c6f7dea0d78f4.rlib
│       │   ├── anyhow-157204463f5593ed.d
│       │   ├── malloc_buf-eaef8ee3f1de8168.malloc_buf.bb3b44082c169ad8-cgu.0.rcgu.o
│       │   ├── globset-1a6df8c66f8fd019.d
│       │   ├── taiko-5f5482d76af3ad3e.1xkk5uo8l7vm7ut1.rcgu.o
│       │   ├── libpest_meta-763306f9a1f70e0e.rlib
│       │   ├── libcfg_if-13a8e19d179f00de.rlib
│       │   ├── libproc_macro2-203b1862d9ee796a.rmeta
│       │   ├── libucd_trie-68172e19f10149cf.rmeta
│       │   ├── unicode_width-2f841ed759394eeb.d
│       │   ├── tiktoken_rs-13783de26c5a1c29.d
│       │   ├── quote-0d787cd1d6792bdd.d
│       │   ├── taiko_macro-8195bb2f72d2614e.d
│       │   ├── libcrossbeam_deque-7808d91b25723717.rmeta
│       │   ├── objc_foundation-f22c8b27505b154a.d
│       │   ├── lazy_static-2a7ede781c6dce4a.lazy_static.8395fc51f6b34063-cgu.0.rcgu.o
│       │   ├── liblock_api-0159b92381d00b94.rmeta
│       │   ├── libheck-3c14e1c860b4929e.rmeta
│       │   ├── libaho_corasick-94342bc35c3ad340.rmeta
│       │   ├── liblog-b05324ccb384617e.rmeta
│       │   ├── bstr-3516875550e89b46.bstr.db2bb989631abf3e-cgu.2.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.q71g0mayyajssbc.rcgu.o
│       │   ├── prompt_macro-7c6d31ccf50d03b5.d
│       │   ├── unicode_width-ae1ffe84cbe7b16b.d
│       │   ├── colored-77bb8fdb6306fbb7.colored.15f374face372787-cgu.2.rcgu.o
│       │   ├── libunicode_ident-ddc98497afda250d.rlib
│       │   ├── unicode_width-16a1984432142341.unicode_width.4932874858b1b8f9-cgu.0.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.5ckoge74ehk0oboj.rcgu.o
│       │   ├── libobjc-02366c74c03250b5.rmeta
│       │   ├── libpest-bdcbc99b6b0e3535.rlib
│       │   ├── taiko-5f5482d76af3ad3e.1cbgbyzvo95y4wyw.rcgu.o
│       │   ├── libregex_automata-fbc90720657fe65b.rmeta
│       │   ├── bit_set-ebed3e5cb085b134.d
│       │   ├── libclap-ce668e3a0d326b36.rmeta
│       │   ├── pest_derive-8b5502e962597fb9.d
│       │   ├── libryu-7dfefb4f0eac89cd.rmeta
│       │   ├── libregex_syntax-9494d37cd8500675.rmeta
│       │   ├── libunicode_width-f704b80496ed2425.rmeta
│       │   ├── libpest_generator-5aeaa8baad32201a.rmeta
│       │   ├── anstyle_parse-38193868a4bc2feb.d
│       │   ├── ignore-3f8d697aa460cd76.d
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.02.rcgu.o
│       │   ├── proc_macro2-858b3b96a32c88fa.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.15.rcgu.o
│       │   ├── libclap-42a443b901c57b66.rmeta
│       │   ├── cli_clipboard-1de30f25a56af333.d
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.08.rcgu.o
│       │   ├── libhandlebars-7682939d551c053d.rmeta
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.07.rcgu.o
│       │   ├── libquote-0d787cd1d6792bdd.rmeta
│       │   ├── pest-bdcbc99b6b0e3535.d
│       │   ├── fancy_regex-8b42a60874425991.d
│       │   ├── libclap-93a9579a2f5f6918.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.04.rcgu.o
│       │   ├── liblazy_static-2a7ede781c6dce4a.rlib
│       │   ├── libthiserror-9cd844cd13e52635.rmeta
│       │   ├── libregex_automata-2b47dcdb846aa3d6.rmeta
│       │   ├── libquote-5b4df7b9712eaceb.rlib
│       │   ├── taiko-5f5482d76af3ad3e.3jr0tiddpq7oxcq9.rcgu.o
│       │   ├── pest-4932018ce684cc23.d
│       │   ├── taiko-5f5482d76af3ad3e.38ps3ab9xikm79mc.rcgu.o
│       │   ├── clap_derive-f50b7d70f89c4204.d
│       │   ├── anstyle_parse-544d360a395d43a6.d
│       │   ├── libignore-a5403807be371545.rmeta
│       │   ├── anstream-7f6af7b63c566228.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.07.rcgu.o
│       │   ├── libpest_generator-98c6362ac1cac2f8.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.10.rcgu.o
│       │   ├── strsim-07b9a41bde477617.d
│       │   ├── libanyhow-2df9d65eaaaa7324.rlib
│       │   ├── either-be1612519a653c32.d
│       │   ├── taiko-5f5482d76af3ad3e.3vwb3rd1bg0vwzzj.rcgu.o
│       │   ├── lazy_static-b27d32fa506d233a.d
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.5.rcgu.o
│       │   ├── libmalloc_buf-3eb27260df86a9e9.rmeta
│       │   ├── libsyn-79418542530a1a9b.rmeta
│       │   ├── regex_syntax-45becf883096970e.d
│       │   ├── autocfg-fd5382870ef4df31.d
│       │   ├── libobjc_id-2b177a12223e105b.rmeta
│       │   ├── heck-3c14e1c860b4929e.d
│       │   ├── regex_syntax-9494d37cd8500675.d
│       │   ├── libautocfg-d60d3d7f96968932.rlib
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.3.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.4psxxmhbzsmcuvqm.rcgu.o
│       │   ├── liblock_api-afee0d44cb672303.rmeta
│       │   ├── pest-d3e9ce9e178f265d.d
│       │   ├── liblazy_static-9f6e603ede85beb1.rmeta
│       │   ├── libpest-68b61f401184a463.rmeta
│       │   ├── rustc_hash-cb11f00b55fa406c.rustc_hash.cd136026b637812e-cgu.0.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.2eb8hqgkjdpgoo97.rcgu.o
│       │   ├── ucd_trie-dcd6580dd443e17a.d
│       │   ├── libcrossbeam_channel-9b64f4cef3c2d868.rmeta
│       │   ├── ryu-c350fc729588bc75.d
│       │   ├── lazy_static-9f6e603ede85beb1.d
│       │   ├── proc_macro2-18e1e4b8cfd0c269.d
│       │   ├── libanstream-7f6af7b63c566228.rmeta
│       │   ├── libmemchr-9af1c8a56ee8a9d6.rmeta
│       │   ├── autocfg-d60d3d7f96968932.d
│       │   ├── strsim-287c9ce1468a762c.d
│       │   ├── crossbeam_epoch-1695a116cad47243.crossbeam_epoch.848f1f04288c74a7-cgu.0.rcgu.o
│       │   ├── clap_lex-8d638cc28f6d33df.d
│       │   ├── clap-183d96304d7c6951.clap.e7e7c6cb7c57cf88-cgu.0.rcgu.o
│       │   ├── crossbeam_epoch-011387be3d43c679.d
│       │   ├── indicatif-fa070afa23fb479a.d
│       │   ├── clap-d3f8da414072d201.d
│       │   ├── libpest_meta-763306f9a1f70e0e.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.36p584sk276h648y.rcgu.o
│       │   ├── libstrsim-287c9ce1468a762c.rmeta
│       │   ├── objc_id-95409c6e5bfb8fd3.objc_id.7c992673bec576e2-cgu.0.rcgu.o
│       │   ├── libunicode_width-16a1984432142341.rmeta
│       │   ├── libautocfg-08f5cf05d6405d00.rlib
│       │   ├── portable_atomic-efc16aa6ba00ca29.d
│       │   ├── serde_json-89cb9c549439971c.d
│       │   ├── taiko-5f5482d76af3ad3e.53k8o2oqxnzb5ynh.rcgu.o
│       │   ├── libproc_macro2-858b3b96a32c88fa.rmeta
│       │   ├── libmemchr-9e426edd08692ba7.rmeta
│       │   ├── libunicode_width-ae1ffe84cbe7b16b.rmeta
│       │   ├── libpest_meta-0b5f49852cd582a0.rlib
│       │   ├── pest_generator-5af4eca3dc36d733.d
│       │   ├── libcrossbeam_deque-fea05bd22443a097.rmeta
│       │   ├── libscopeguard-7d723bac1d47c066.rmeta
│       │   ├── libeither-6ffdf15de26bd676.rlib
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.10.rcgu.o
│       │   ├── liblibc-377ada10dcb65998.rmeta
│       │   ├── libtiktoken_rs-631e7da1ceaab6f2.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.zhzcm7yrvamrbkj.rcgu.o
│       │   ├── libhandlebars-499fd614942c8270.rmeta
│       │   ├── libucd_trie-ef4c8f2907ae0490.rmeta
│       │   ├── strsim-fadff44222618b4b.strsim.e1140db0c11a0d78-cgu.0.rcgu.o
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.3.rcgu.o
│       │   ├── libcfg_if-f063294a1a369ae9.rmeta
│       │   ├── libucd_trie-56d1345b48666f6a.rlib
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.12.rcgu.o
│       │   ├── libparking_lot_core-132a77ce2b7cc0d0.rmeta
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.04.rcgu.o
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.11.rcgu.o
│       │   ├── anstream-aaba10b07d433098.d
│       │   ├── libanstream-4945a0c46b1be249.rmeta
│       │   ├── libtermtree-9eacc883d229c4f3.rmeta
│       │   ├── libcolorchoice-79ef33375f88230b.rlib
│       │   ├── objc_id-d8e2530d5b0ab1b8.d
│       │   ├── thiserror-7fd1eef083610ebb.thiserror.353237263af74ba1-cgu.0.rcgu.o
│       │   ├── libconsole-c1f5d1f60de9f375.rmeta
│       │   ├── libclap_builder-971a7de4ef516dae.rmeta
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.08.rcgu.o
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.02.rcgu.o
│       │   ├── libanyhow-2df9d65eaaaa7324.rmeta
│       │   ├── libthiserror-9684af3710b94797.rlib
│       │   ├── colored-722d01c60457fd3f.d
│       │   ├── console-16f998361d7a45ce.d
│       │   ├── utf8parse-a5a14685b2cf96f0.d
│       │   ├── libanstyle_query-3bc001f5a383188e.rmeta
│       │   ├── either-3983297f66cd6218.d
│       │   ├── libanyhow-b80c101af730b57a.rmeta
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.00.rcgu.o
│       │   ├── libportable_atomic-d6e205ab3c0ba513.rmeta
│       │   ├── libcolorchoice-2065da109b279150.rmeta
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.03.rcgu.o
│       │   ├── liblibc-7c6e71b586fd4b21.rmeta
│       │   ├── liblog-9a66223a5e030806.rmeta
│       │   ├── crossbeam_utils-a91e03eda2e116e2.d
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.3.rcgu.o
│       │   ├── libbstr-3516875550e89b46.rlib
│       │   ├── taiko-5f5482d76af3ad3e.49twn26df46l5zwe.rcgu.o
│       │   ├── libregex_syntax-0d6136cd0bf5fa53.rmeta
│       │   ├── libignore-3f8d697aa460cd76.rmeta
│       │   ├── libcrossbeam-5e9fbe378655c40e.rlib
│       │   ├── libportable_atomic-d7a74020c9b9fe12.rlib
│       │   ├── anstream-c9c41d61ff842eeb.anstream.9266a8ef28ea17e3-cgu.0.rcgu.o
│       │   ├── libutf8parse-f2acd427c0a95aab.rmeta
│       │   ├── libitoa-b13010d569e2163d.rmeta
│       │   ├── libclap_lex-9e784cf71c4dbe13.rmeta
│       │   ├── libindicatif-6ecc1cea8c4dac4f.rmeta
│       │   ├── libanstyle-d2b2265849d241e8.rmeta
│       │   ├── bit_vec-6784665201503254.d
│       │   ├── heck-fe4aad64d30b965f.d
│       │   ├── libprompt_macro-4108ea4371bd192f.rmeta
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.1.rcgu.o
│       │   ├── utf8parse-93787ca028a6d438.d
│       │   ├── itoa-7afa4844fdfbbd60.itoa.3fc4b069589d6209-cgu.0.rcgu.o
│       │   ├── librayon-a641aab5cd97d3de.rlib
│       │   ├── taiko-5f5482d76af3ad3e.2djygn87l3fcp5ab.rcgu.o
│       │   ├── libserde_json-1aaf06abfb500a24.rmeta
│       │   ├── pest-8a80f6c810d9ebf2.d
│       │   ├── anstyle_query-549661baf91ae106.d
│       │   ├── libheck-698dd90d905fb3d9.rlib
│       │   ├── libserde-9116e3003993b927.rmeta
│       │   ├── strsim-59ec25f123d79732.d
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.7.rcgu.o
│       │   ├── thiserror-9cd844cd13e52635.d
│       │   ├── utf8parse-c4e9f128df76b4db.d
│       │   ├── thiserror-83a66a01eb224bff.d
│       │   ├── librayon_core-a2856f9f57a1e6c2.rlib
│       │   ├── libconsole-7ae5fb73e1277bb2.rmeta
│       │   ├── libwalkdir-c9c753dae4c51eae.rmeta
│       │   ├── liblock_api-7c6bc2669b134fb3.rmeta
│       │   ├── regex_automata-357a41b18ec6022f.d
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.02.rcgu.o
│       │   ├── libserde-f47b43f94aadc098.rmeta
│       │   ├── ignore-c85d7efb1af0a7ed.d
│       │   ├── libclap_derive-eba727efa8166a4e.dylib
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.1.rcgu.o
│       │   ├── tiktoken_rs-1011b622425df025.d
│       │   ├── libpest_derive-50b1d03c52ebd1e6.dylib
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.01.rcgu.o
│       │   ├── libpest-2b50acd6dff662ba.rmeta
│       │   ├── lock_api-afee0d44cb672303.lock_api.ab663dfc8edcece1-cgu.0.rcgu.o
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.00.rcgu.o
│       │   ├── libportable_atomic-86e97a657a48c7f6.rmeta
│       │   ├── liblock_api-c4667e452a675ace.rmeta
│       │   ├── objc_foundation-b5962e5667862d5f.d
│       │   ├── libbase64-6e96c5a9915be65f.rmeta
│       │   ├── libonce_cell-9e5aa9cec64e1de2.rlib
│       │   ├── crossbeam_utils-360c9ee5275f1872.d
│       │   ├── libcolored-6587aeda69587c72.rmeta
│       │   ├── libcrossbeam_utils-18686986dd98f802.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.443bnhynv3g0ltnw.rcgu.o
│       │   ├── base64-2fa71140a3bbff25.d
│       │   ├── bstr-8598bd38e349842f.d
│       │   ├── libucd_trie-e13425de40a4d29d.rlib
│       │   ├── thiserror-ab23e760f2b3bad1.d
│       │   ├── libobjc-9aceb7739b194f45.rmeta
│       │   ├── pest_meta-0b5f49852cd582a0.d
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.10.rcgu.o
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.06.rcgu.o
│       │   ├── crossbeam_epoch-ff0e744e239caf0b.d
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.13.rcgu.o
│       │   ├── libnumber_prefix-b73371543e850b43.rmeta
│       │   ├── libsmallvec-5a30387ee9f221d9.rmeta
│       │   ├── regex_syntax-0d6136cd0bf5fa53.d
│       │   ├── walkdir-723ddb56cce4eeb6.d
│       │   ├── libautocfg-fd5382870ef4df31.rmeta
│       │   ├── libutf8parse-970b336da10722b6.rmeta
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.12.rcgu.o
│       │   ├── libsyn-f7a43a238a4253d3.rlib
│       │   ├── thiserror-9684af3710b94797.d
│       │   ├── taiko_macro-32e128c10b862bc2.d
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.1.rcgu.o
│       │   ├── libregex-0f8107ff5ca70c14.rmeta
│       │   ├── libpest_generator-e747617606104524.rlib
│       │   ├── libsyn-a22e3a50ce0cd6e9.rmeta
│       │   ├── libucd_trie-ef4c8f2907ae0490.rlib
│       │   ├── strsim-fadff44222618b4b.strsim.e1140db0c11a0d78-cgu.2.rcgu.o
│       │   ├── libtiktoken_rs-fd3595d3a846a6ac.rmeta
│       │   ├── regex-8506a54e982bcf47.d
│       │   ├── libserde-82b2836e0bab74b1.rmeta
│       │   ├── anstyle_query-3bc001f5a383188e.anstyle_query.458e41846de4633e-cgu.0.rcgu.o
│       │   ├── taiko-39abda136aead6d6.d
│       │   ├── libfancy_regex-c6034dfe6ec4bdda.rmeta
│       │   ├── libobjc_id-5439da7a615eaaf7.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.15.rcgu.o
│       │   ├── libautocfg-fd5382870ef4df31.rlib
│       │   ├── libpest-149ca50ad3efa4ae.rmeta
│       │   ├── libclap-ecd14b188f0d2455.rmeta
│       │   ├── bstr-88cad40a68500408.d
│       │   ├── taiko-5f5482d76af3ad3e.2kjy3koojtr3n589.rcgu.o
│       │   ├── liblazy_static-c94958108887daa5.rmeta
│       │   ├── objc_id-e778945f2e2d0c47.d
│       │   ├── libthiserror-4ceb9fe7530f729d.rmeta
│       │   ├── scopeguard-707e2ad09f4fa99f.d
│       │   ├── ucd_trie-56d1345b48666f6a.d
│       │   ├── taiko-5f5482d76af3ad3e.27vk6pcr0kyskrcy.rcgu.o
│       │   ├── regex_syntax-a0b5b3df3d0e1c36.d
│       │   ├── libbit_vec-d0100a597cc971f3.rlib
│       │   ├── handlebars-5269e56c0793138a.d
│       │   ├── libregex_syntax-e240d2964da64048.rmeta
│       │   ├── serde_json-46bd2ddb93a26abd.d
│       │   ├── utf8parse-970b336da10722b6.d
│       │   ├── libparse_format-e3d136611627ca9d.rmeta
│       │   ├── lock_api-afee0d44cb672303.d
│       │   ├── handlebars-bf6d86a23749eb49.d
│       │   ├── unicode_ident-ddc98497afda250d.d
│       │   ├── anstyle-f4f197a2e42921a4.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.07.rcgu.o
│       │   ├── libtermtree-dafa81c0289883c2.rmeta
│       │   ├── base64-6e96c5a9915be65f.d
│       │   ├── libobjc_foundation-b5962e5667862d5f.rmeta
│       │   ├── libquote-cdfe020df82b4d1d.rmeta
│       │   ├── libtermtree-63ffc5db97ed923c.rlib
│       │   ├── libheck-79e04a5211bdd404.rlib
│       │   ├── libcrossbeam_queue-2aef4dbdcfccc3c1.rmeta
│       │   ├── handlebars-90b3957116bc7052.d
│       │   ├── taiko-c1d38b211a2bda2e.d
│       │   ├── syn-d7643833d974eb60.d
│       │   ├── libpest-348d1225c08310b0.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.d
│       │   ├── parking_lot_core-d102f8c1d60826b4.parking_lot_core.ed50d05b6b4b6b92-cgu.0.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.1l9bv1l55ubc6bf0.rcgu.o
│       │   ├── lock_api-0dd4773c669ee2df.d
│       │   ├── syn-f7a43a238a4253d3.d
│       │   ├── taiko-5f5482d76af3ad3e.7imnbsgddsm3x6h.rcgu.o
│       │   ├── libhandlebars-940b1fe4c2f989a9.rmeta
│       │   ├── libsmallvec-edb7a4846548a822.rmeta
│       │   ├── libthiserror-7d04976b43fa628e.rmeta
│       │   ├── parse_format-65b5d42e6ef4c317.d
│       │   ├── libucd_trie-56d1345b48666f6a.rmeta
│       │   ├── colorchoice-0e0af3dd6900341b.d
│       │   ├── rustc_hash-fefca1a8f4a2c364.d
│       │   ├── anstream-6d78168790225816.d
│       │   ├── crossbeam_channel-fdc0ae76d9c3397f.crossbeam_channel.6929f6013531f7a0-cgu.0.rcgu.o
│       │   ├── libregex_automata-e3223faa738ad36b.rmeta
│       │   ├── thiserror-7fd1eef083610ebb.d
│       │   ├── termtree-63ffc5db97ed923c.d
│       │   ├── strsim-8fff5fb944a39f3a.d
│       │   ├── ucd_trie-ef4c8f2907ae0490.d
│       │   ├── crossbeam-5e9fbe378655c40e.crossbeam.a2109ce82c94f356-cgu.0.rcgu.o
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.05.rcgu.o
│       │   ├── libjwalk-e55f9b1b97b0d9fa.rmeta
│       │   ├── libnumber_prefix-4b0f92ccd4a73404.rlib
│       │   ├── libtaiko-5c3c546742a88a7e.rmeta
│       │   ├── libcrossbeam_channel-fdc0ae76d9c3397f.rlib
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.12.rcgu.o
│       │   ├── pest_generator-e747617606104524.d
│       │   ├── libtaiko-c1d38b211a2bda2e.rmeta
│       │   ├── libparking_lot_core-ecd370daa92df0e4.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.r0dfy7rj9qqd4ia.rcgu.o
│       │   ├── regex_automata-5699297d539bd36f.d
│       │   ├── taiko-5c3c546742a88a7e.d
│       │   ├── proc_macro2-29c5e0e4ca278bae.d
│       │   ├── memchr-210dc6bad0ef55ff.d
│       │   ├── libregex-8506a54e982bcf47.rmeta
│       │   ├── lazy_static-c94958108887daa5.d
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.05.rcgu.o
│       │   ├── libbit_vec-c25c525deccf9549.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.4or24krol3u9p7x.rcgu.o
│       │   ├── quote-97b83299b1024699.d
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.06.rcgu.o
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.09.rcgu.o
│       │   ├── libcrossbeam-5e9fbe378655c40e.rmeta
│       │   ├── libcrossbeam_channel-b7295634b6d1e64f.rmeta
│       │   ├── libtermtree-38a1aa0746d77933.rmeta
│       │   ├── heck-698dd90d905fb3d9.d
│       │   ├── liblock_api-afee0d44cb672303.rlib
│       │   ├── once_cell-e8773422b0f43014.d
│       │   ├── libpest_derive-8fcf80f809a489d5.dylib
│       │   ├── prompt_macro-d3fe94433cb69dc4.d
│       │   ├── rayon_core-c5baa12ac57958ec.d
│       │   ├── libheck-5b224f32e954472c.rmeta
│       │   ├── parking_lot_core-7562cea3fa4ad8a4.d
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.00.rcgu.o
│       │   ├── anstyle_query-830166fdf9e655c7.d
│       │   ├── colored-77bb8fdb6306fbb7.colored.15f374face372787-cgu.0.rcgu.o
│       │   ├── smallvec-e53885e764a5bbff.d
│       │   ├── crossbeam_queue-ec50bd6f228abdbd.d
│       │   ├── libmemchr-d1514e95b786a778.rmeta
│       │   ├── bstr-3516875550e89b46.bstr.db2bb989631abf3e-cgu.0.rcgu.o
│       │   ├── crossbeam_utils-18686986dd98f802.crossbeam_utils.712b2907679f27b8-cgu.4.rcgu.o
│       │   ├── colored-e9a596b387e8b72c.d
│       │   ├── walkdir-ddd7218531b92be4.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.09.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.36r255p5bi0eojvn.rcgu.o
│       │   ├── fancy_regex-a99532343fd1d180.d
│       │   ├── taiko-5f5482d76af3ad3e.4e47vuwwx3zw1mu4.rcgu.o
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.14.rcgu.o
│       │   ├── objc_id-2b177a12223e105b.d
│       │   ├── libpest_generator-0e47e79a53ca6246.rmeta
│       │   ├── clap_builder-3bf11762de3d2225.d
│       │   ├── libsyn-cd4a8424ebf8304f.rmeta
│       │   ├── libblock-a814ef6c1ee6440a.rlib
│       │   ├── taiko-5f5482d76af3ad3e.4cp4ckdcxgw1y31s.rcgu.o
│       │   ├── libautocfg-0eecde6036b9415d.rlib
│       │   ├── libanstream-aaba10b07d433098.rmeta
│       │   ├── libportable_atomic-cfcde1b41ba11c6a.rmeta
│       │   ├── console-c1f5d1f60de9f375.d
│       │   ├── libproc_macro2-29c5e0e4ca278bae.rlib
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.08.rcgu.o
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.14.rcgu.o
│       │   ├── libindicatif-0ba12c0f222c86ae.rmeta
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.02.rcgu.o
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.5.rcgu.o
│       │   ├── serde-ad7ab06ed1a5d069.d
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.09.rcgu.o
│       │   ├── parking_lot-9e5145b6ec7b28b8.d
│       │   ├── cfg_if-5054d9bf08a0c80c.d
│       │   ├── thiserror_impl-09cb00ac5d14a0cf.d
│       │   ├── cfg_if-45b3cfd05c61975a.d
│       │   ├── libglobset-b3bed3e7224f2a1d.rmeta
│       │   ├── libanstream-6d78168790225816.rmeta
│       │   ├── itoa-e7583f895a51e75e.d
│       │   ├── libucd_trie-0e6e94eba92d6a77.rmeta
│       │   ├── libcrossbeam_channel-b311370f4de6afb9.rmeta
│       │   ├── liblog-9737222543b9683b.rmeta
│       │   ├── libclap_derive-29cfff08b5f02c48.dylib
│       │   ├── taiko-5f5482d76af3ad3e.4fxwjam1pj8b1e4e.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.329f18b751pung3b.rcgu.o
│       │   ├── same_file-04b1f130144ea03a.same_file.f082061347029239-cgu.0.rcgu.o
│       │   ├── syn-d2b4a3bd7aae0608.d
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.5.rcgu.o
│       │   ├── same_file-04b1f130144ea03a.d
│       │   ├── librustc_hash-952c00a7f09c9aa7.rmeta
│       │   ├── anyhow-7aa4bdd60e197a62.d
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.05.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.2yvhcb0mt0nincxy.rcgu.o
│       │   ├── libcode2prompt-d94e4e13a9165e5e.rmeta
│       │   ├── crossbeam_queue-6ffa5f04fb97f232.d
│       │   ├── libsyn-47e79c06fd625548.rmeta
│       │   ├── libsyn-f20dfd11b09216d9.rlib
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.06.rcgu.o
│       │   ├── libwalkdir-723ddb56cce4eeb6.rlib
│       │   ├── once_cell-00ea8b35a4a0316d.d
│       │   ├── libthiserror_impl-cc61fd152678c91d.dylib
│       │   ├── libscopeguard-1a41f4f101669df5.rlib
│       │   ├── libthiserror-b0e51cc25e506228.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.1fnr94fr66c4kpca.rcgu.o
│       │   ├── libquote-ebea092d1391ccb4.rlib
│       │   ├── anstyle_parse-9d6e9e70ccaa3642.d
│       │   ├── libblock-afd704776bd056d4.rmeta
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.04.rcgu.o
│       │   ├── libpest_meta-c3a5db06ffcfb496.rlib
│       │   ├── cfg_if-f063294a1a369ae9.d
│       │   ├── cli_clipboard-1de30f25a56af333.cli_clipboard.8aa09a2b7f50a4e4-cgu.1.rcgu.o
│       │   ├── quote-5b4df7b9712eaceb.d
│       │   ├── librustc_hash-cb11f00b55fa406c.rmeta
│       │   ├── base64-6e96c5a9915be65f.base64.c69e558fad926954-cgu.1.rcgu.o
│       │   ├── crossbeam_epoch-5484dbb6acc3f8a5.d
│       │   ├── libpest-68b61f401184a463.rlib
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.d
│       │   ├── libautocfg-d0bb26a33a75c25c.rlib
│       │   ├── unindent-5d879711c2b503b6.d
│       │   ├── libpest_derive-11c6b22c1ab0ffdc.dylib
│       │   ├── pest_meta-c3a5db06ffcfb496.d
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.5.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.xy62o1xeynn6zck.rcgu.o
│       │   ├── libfancy_regex-f9979b02061d09b7.rmeta
│       │   ├── libpest-149ca50ad3efa4ae.rlib
│       │   ├── libconsole-16f998361d7a45ce.rlib
│       │   ├── log-9737222543b9683b.d
│       │   ├── libpest_generator-5aeaa8baad32201a.rlib
│       │   ├── libclap_builder-2b85ab4d0dabe091.rmeta
│       │   ├── portable_atomic-cfcde1b41ba11c6a.d
│       │   ├── macro-fcf57fd3cf106838.d
│       │   ├── libconsole-add040e40b2f8f94.rmeta
│       │   ├── libonce_cell-1b3d204c83a12703.rmeta
│       │   ├── crossbeam_deque-35e98e3c2d441351.d
│       │   ├── libunicode_ident-81e6f0701cb43f8a.rmeta
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.3.rcgu.o
│       │   ├── taiko_macro-23f8f29e33e51888.d
│       │   ├── handlebars-5dcd2ebf69d9205f.d
│       │   ├── libpest-54f9af20f2c3fa2f.rmeta
│       │   ├── bit_set-6f81487daedf45e2.d
│       │   ├── anstyle_parse-a6c619ba8ed0ba68.d
│       │   ├── smallvec-edb7a4846548a822.d
│       │   ├── pest-426fd3f80d09c9fa.d
│       │   ├── libsmallvec-227f32d6884fb60c.rmeta
│       │   ├── parking_lot-9e0c399cf98f8a24.d
│       │   ├── memchr-d6546b159e1b463b.d
│       │   ├── unindent-02808f17bd4a482b.d
│       │   ├── clap_builder-2b85ab4d0dabe091.d
│       │   ├── libautocfg-d60d3d7f96968932.rmeta
│       │   ├── libpest_meta-757e14534d182046.rlib
│       │   ├── log-4f8d2beb91e2c81d.d
│       │   ├── ryu-b38cca26303b1aac.d
│       │   ├── libfancy_regex-8b42a60874425991.rmeta
│       │   ├── ignore-fffd30910e9307dc.d
│       │   ├── libindicatif-bc93d441eb23aa5e.rlib
│       │   ├── libsyn-db3f05bad1825a44.rmeta
│       │   ├── libpest-9b28f6cac5ba28d8.rlib
│       │   ├── libunindent-b0bd2a506b19c0a4.rlib
│       │   ├── thiserror_impl-538e6a41d5bf27ac.d
│       │   ├── rayon-f1863ce1e1832310.d
│       │   ├── number_prefix-2ed70394c0e1d1a6.d
│       │   ├── crossbeam_deque-499b579d3c611256.d
│       │   ├── thiserror_impl-5ec8bf9d562dc2a5.d
│       │   ├── quote-ebea092d1391ccb4.d
│       │   ├── librustc_hash-eff6d23f1ed3d804.rmeta
│       │   ├── ucd_trie-f1a751efa5939de4.ucd_trie.271f527e5aea8b64-cgu.3.rcgu.o
│       │   ├── libblock-8f64c5ae264aca5f.rmeta
│       │   ├── clap-42a443b901c57b66.d
│       │   ├── libitoa-b219816d8a894515.rmeta
│       │   ├── crossbeam_epoch-32d4247daab5bcc4.d
│       │   ├── anstyle-d2b2265849d241e8.d
│       │   ├── taiko-fcb18f34e23635e6.d
│       │   ├── smallvec-5a30387ee9f221d9.smallvec.ead87cc81ebfeca5-cgu.0.rcgu.o
│       │   ├── libryu-c053dfd3b64364c1.rmeta
│       │   ├── clap_derive-29cfff08b5f02c48.d
│       │   ├── clap-e7b61dcae432590e.d
│       │   ├── libclap_builder-76adf2488bf8955d.rmeta
│       │   ├── libparse_format-8ca4bf5ed49416ac.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.04.rcgu.o
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.13.rcgu.o
│       │   ├── libcrossbeam_utils-01dce0ec683fd523.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.1ndzsjo32har2ftc.rcgu.o
│       │   ├── number_prefix-bb7a15a300755127.d
│       │   ├── portable_atomic-d6e205ab3c0ba513.d
│       │   ├── libeither-be1612519a653c32.rmeta
│       │   ├── malloc_buf-3eb27260df86a9e9.d
│       │   ├── memchr-9b9ca7881fffae76.d
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.10.rcgu.o
│       │   ├── libunicode_ident-24c05b45cc2da0bf.rmeta
│       │   ├── fancy_regex-11b1b72092e141c5.d
│       │   ├── taiko-5f5482d76af3ad3e.357w6yde94a1rjrc.rcgu.o
│       │   ├── libcfg_if-5054d9bf08a0c80c.rmeta
│       │   ├── crossbeam-999b280ae2ae4dd9.d
│       │   ├── libheck-5b224f32e954472c.rlib
│       │   ├── base64-293239c257061bcb.d
│       │   ├── libserde_json-36b0216e9098d50f.rmeta
│       │   ├── itoa-6e50e7815faad2c1.d
│       │   ├── libcolorchoice-bb8ac1d01a3b8369.rmeta
│       │   ├── anyhow-2df9d65eaaaa7324.anyhow.61955e5b11b0378d-cgu.0.rcgu.o
│       │   ├── pest-2b50acd6dff662ba.pest.dd21cf03355f4f0a-cgu.0.rcgu.o
│       │   ├── libpest-4932018ce684cc23.rlib
│       │   ├── crossbeam_utils-18686986dd98f802.crossbeam_utils.712b2907679f27b8-cgu.0.rcgu.o
│       │   ├── libbit_set-ebed3e5cb085b134.rmeta
│       │   ├── libbstr-8598bd38e349842f.rmeta
│       │   ├── libparse_format-65b5d42e6ef4c317.rmeta
│       │   ├── libitoa-7afa4844fdfbbd60.rmeta
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.01.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.2rnpiabfdp77sh1q.rcgu.o
│       │   ├── libquote-c84f5129941373e7.rmeta
│       │   ├── libc-18495412a3a8bf98.d
│       │   ├── bit_vec-c25c525deccf9549.d
│       │   ├── libbase64-de66e0b6f7ed0906.rmeta
│       │   ├── libucd_trie-e13425de40a4d29d.rmeta
│       │   ├── cli_clipboard-8ee37ab0b7ce884f.d
│       │   ├── libbit_vec-1ceac5fefb9229e6.rmeta
│       │   ├── number_prefix-4b0f92ccd4a73404.d
│       │   ├── colored-b2c368a2264b59e0.d
│       │   ├── termtree-38a1aa0746d77933.d
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.08.rcgu.o
│       │   ├── rustc_hash-59b564b5a1e74548.d
│       │   ├── number_prefix-b73371543e850b43.d
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.02.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.4obeu6vcpkvxik2c.rcgu.o
│       │   ├── libtaiko-e901070ac87d03b6.rmeta
│       │   ├── crossbeam_deque-fea05bd22443a097.d
│       │   ├── rayon-a641aab5cd97d3de.d
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.01.rcgu.o
│       │   ├── libignore-c85d7efb1af0a7ed.rmeta
│       │   ├── libscopeguard-5a9fa7d4f030a5c0.rmeta
│       │   ├── clap_lex-1030d2fa2711077f.clap_lex.da2cdf95e2f2a55-cgu.1.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.538c7k1bjcwk3t6j.rcgu.o
│       │   ├── libserde-5b71833873e30944.rmeta
│       │   ├── bit_set-fb191915270cea7a.d
│       │   ├── either-432053b3e7d4b6db.d
│       │   ├── libc-7c6e71b586fd4b21.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.03.rcgu.o
│       │   ├── parking_lot-937ce451f7998a83.parking_lot.3b0075210154421a-cgu.2.rcgu.o
│       │   ├── libcolorchoice-79ef33375f88230b.rmeta
│       │   ├── liblock_api-82983c2133f5a1f6.rmeta
│       │   ├── libglobset-18f2a14144fb0c54.rmeta
│       │   ├── malloc_buf-eaef8ee3f1de8168.d
│       │   ├── libeither-0f2a5df081714e01.rmeta
│       │   ├── quote-c84f5129941373e7.d
│       │   ├── libtaiko-eddd48b28a2017fa.rmeta
│       │   ├── liblazy_static-b27d32fa506d233a.rmeta
│       │   ├── libcrossbeam_deque-867204b66eaea281.rmeta
│       │   ├── libproc_macro2-18e1e4b8cfd0c269.rlib
│       │   ├── libaho_corasick-0d01aad8c1a49b2c.rmeta
│       │   ├── libsyn-f20dfd11b09216d9.rmeta
│       │   ├── libcrossbeam_queue-6ffa5f04fb97f232.rmeta
│       │   ├── colorchoice-79ef33375f88230b.d
│       │   ├── objc-02366c74c03250b5.d
│       │   ├── libstrsim-8fff5fb944a39f3a.rmeta
│       │   ├── libmalloc_buf-eaef8ee3f1de8168.rmeta
│       │   ├── lock_api-c4667e452a675ace.d
│       │   ├── libbit_set-fb191915270cea7a.rmeta
│       │   ├── ryu-7dfefb4f0eac89cd.ryu.3d08e38d7147eac5-cgu.0.rcgu.o
│       │   ├── libmalloc_buf-53a7bf1de681c142.rmeta
│       │   ├── libonce_cell-00ea8b35a4a0316d.rmeta
│       │   ├── unindent-95aa104d088359ad.d
│       │   ├── libbstr-d0720bfc6931e58a.rmeta
│       │   ├── walkdir-723ddb56cce4eeb6.walkdir.2fd6da2df1898303-cgu.1.rcgu.o
│       │   ├── libregex_syntax-a0b5b3df3d0e1c36.rmeta
│       │   ├── objc_foundation-2eea5b8768bd59e9.d
│       │   ├── clap-8d71986fb942cffc.d
│       │   ├── serde-e8a75b8079968bd6.d
│       │   ├── bit_vec-de5920a6f9721359.d
│       │   ├── libjwalk-4c072e0256a54789.rmeta
│       │   ├── libitoa-ee973ff0bc0fc6f2.rmeta
│       │   ├── libcrossbeam_utils-a91e03eda2e116e2.rmeta
│       │   ├── libbase64-69a5e3c040cfede6.rmeta
│       │   ├── libmemchr-825b4f36e101cadf.rmeta
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.11.rcgu.o
│       │   ├── libryu-bc0b344f98438129.rmeta
│       │   ├── libpest_derive-30e7a9d1a2598605.dylib
│       │   ├── objc-2476d8a1277d70b4.d
│       │   ├── indicatif-c97a0576239e6a63.d
│       │   ├── libbstr-3516875550e89b46.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.30i3uwesufd2ga57.rcgu.o
│       │   ├── libparking_lot_core-d102f8c1d60826b4.rmeta
│       │   ├── proc_macro2-86318a00fa10a098.d
│       │   ├── libserde-ac3a19c9b2414b1f.rmeta
│       │   ├── rustc_hash-cb11f00b55fa406c.d
│       │   ├── aho_corasick-91cacaa652cc5153.d
│       │   ├── libcfg_if-13a8e19d179f00de.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.00.rcgu.o
│       │   ├── crossbeam_channel-b7295634b6d1e64f.d
│       │   ├── taiko-e3f3f14e7a4f2dc3.d
│       │   ├── taiko-5f5482d76af3ad3e.234mmik7zlm3ignv.rcgu.o
│       │   ├── libignore-c4f376d660a6f596.rmeta
│       │   ├── libeither-432053b3e7d4b6db.rmeta
│       │   ├── fancy_regex-f9979b02061d09b7.fancy_regex.ba7ca2c5a79501d9-cgu.03.rcgu.o
│       │   ├── anyhow-1b066440c0121ea7.d
│       │   ├── crossbeam_utils-a7c3d28de1cc7e82.d
│       │   ├── libsmallvec-6de58aa964b2431f.rmeta
│       │   ├── libproc_macro2-53f1f5c378e4910d.rlib
│       │   ├── log-b05324ccb384617e.d
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.03.rcgu.o
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.14.rcgu.o
│       │   ├── jwalk-4c072e0256a54789.d
│       │   ├── libbstr-88cad40a68500408.rmeta
│       │   ├── anstyle-f05d8c7856d7e78e.anstyle.3786fe4584b4f7ad-cgu.0.rcgu.o
│       │   ├── liblibc-a93187a4ecae664b.rmeta
│       │   ├── libanstream-a0d112ff6640ce06.rmeta
│       │   ├── parking_lot_core-5341009d00e1e7bb.d
│       │   ├── pest-149ca50ad3efa4ae.d
│       │   ├── libcrossbeam-94dad57f069ce4b6.rmeta
│       │   ├── same_file-6f1ef61c7135d1cb.d
│       │   ├── libglobset-1a6df8c66f8fd019.rmeta
│       │   ├── ignore-3f8d697aa460cd76.ignore.64da15cac8df83bd-cgu.12.rcgu.o
│       │   ├── clap-93a9579a2f5f6918.d
│       │   ├── lock_api-0159b92381d00b94.d
│       │   ├── walkdir-bdbf780e9d753ec0.d
│       │   ├── proc_macro2-e3ead2790c8a6ade.d
│       │   ├── libanstyle_parse-544d360a395d43a6.rlib
│       │   ├── parking_lot-937ce451f7998a83.d
│       │   ├── clap-ce668e3a0d326b36.d
│       │   ├── pest_generator-fc27fdfcc846e3cc.d
│       │   ├── anyhow-2df9d65eaaaa7324.anyhow.61955e5b11b0378d-cgu.2.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.24yzy8m9w4yr4diy.rcgu.o
│       │   ├── crossbeam_utils-18686986dd98f802.crossbeam_utils.712b2907679f27b8-cgu.2.rcgu.o
│       │   ├── serde-f47b43f94aadc098.serde.1b7bb8750c6b7445-cgu.0.rcgu.o
│       │   ├── libparking_lot_core-7562cea3fa4ad8a4.rmeta
│       │   ├── regex_automata-2c01a250e90ac542.regex_automata.5f613560626ef1b-cgu.06.rcgu.o
│       │   ├── libhandlebars-7682939d551c053d.rlib
│       │   ├── libtiktoken_rs-13783de26c5a1c29.rmeta
│       │   ├── clap_builder-fe335d7668ae5da9.clap_builder.dd7ff24e2ab134cf-cgu.11.rcgu.o
│       │   ├── libserde-ad7ab06ed1a5d069.rmeta
│       │   ├── thiserror_impl-0c990f87796132c7.d
│       │   ├── libparse_format-d12b5b25e64c4bdb.rmeta
│       │   ├── libquote-bb3c202bc622fea6.rlib
│       │   ├── libclap_derive-66cadc6df51f5d9d.dylib
│       │   ├── libunicode_width-2f841ed759394eeb.rmeta
│       │   ├── objc-72251fe41e6eaf6d.d
│       │   ├── libucd_trie-31ce46028bbae57f.rmeta
│       │   ├── prompt_macro-d26dacd45b5d50a2.d
│       │   ├── either-6ffdf15de26bd676.d
│       │   ├── smallvec-227f32d6884fb60c.d
│       │   ├── smallvec-4106a1e233fbf2ba.d
│       │   ├── libanstyle_query-af4c0252b5bddea8.rmeta
│       │   ├── libprompt_macro-d26dacd45b5d50a2.rmeta
│       │   ├── walkdir-723ddb56cce4eeb6.walkdir.2fd6da2df1898303-cgu.3.rcgu.o
│       │   ├── anstyle-289580a72738fded.d
│       │   ├── libtiktoken_rs-6b9d2c49470fe1b4.rlib
│       │   ├── pest-2b50acd6dff662ba.d
│       │   ├── fancy_regex-f9979b02061d09b7.d
│       │   ├── either-0f2a5df081714e01.d
│       │   ├── taiko-5f5482d76af3ad3e.2aysip9oi0iu989k.rcgu.o
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.13.rcgu.o
│       │   ├── regex-0f8107ff5ca70c14.regex.d7700752044f9ef2-cgu.0.rcgu.o
│       │   ├── libpest_generator-3d5fc24121b48bba.rlib
│       │   ├── bit_set-7c3c824b8000ac3c.d
│       │   ├── taiko-5f5482d76af3ad3e
│       │   ├── colorchoice-3a3e68f61f1b1683.d
│       │   ├── libbit_set-d7322ea878b61688.rlib
│       │   ├── libunindent-5d879711c2b503b6.rlib
│       │   ├── aho_corasick-a4b46a431e5663f4.d
│       │   ├── libcolorchoice-3af447a85e1f9cfe.rmeta
│       │   ├── parking_lot_core-c5651f062de4fb05.d
│       │   ├── libpest_derive-c166c2eba4646155.dylib
│       │   ├── libcrossbeam_channel-fdc0ae76d9c3397f.rmeta
│       │   ├── libucd_trie-40e4c8a9f503bc66.rmeta
│       │   ├── scopeguard-7d723bac1d47c066.d
│       │   ├── libcolorchoice-3a3e68f61f1b1683.rmeta
│       │   ├── taiko-3164c78022e0e85e.d
│       │   ├── clap_builder-07298f382acc0d93.d
│       │   ├── aho_corasick-bcc434a446492054.aho_corasick.11cdef273708fa5f-cgu.01.rcgu.o
│       │   ├── parking_lot-937ce451f7998a83.parking_lot.3b0075210154421a-cgu.0.rcgu.o
│       │   ├── libanyhow-157204463f5593ed.rmeta
│       │   ├── taiko-9fa3b50fce309f87.d
│       │   ├── libanstyle-f05d8c7856d7e78e.rmeta
│       │   ├── liblock_api-0dd4773c669ee2df.rmeta
│       │   ├── block-afd704776bd056d4.d
│       │   ├── objc-9aceb7739b194f45.d
│       │   ├── libquote-b7048526dbec7acb.rlib
│       │   ├── libaho_corasick-bcc434a446492054.rlib
│       │   ├── librayon-f1863ce1e1832310.rmeta
│       │   ├── libsame_file-04b1f130144ea03a.rlib
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.06.rcgu.o
│       │   ├── libproc_macro2-86318a00fa10a098.rmeta
│       │   ├── libcli_clipboard-7a13cdfe04ad4c75.rmeta
│       │   ├── libunicode_ident-c820b7f471869748.rlib
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.07.rcgu.o
│       │   ├── libindicatif-bc93d441eb23aa5e.rmeta
│       │   ├── console-16f998361d7a45ce.console.c91fd4e067f3fe59-cgu.7.rcgu.o
│       │   ├── libanyhow-7aa4bdd60e197a62.rmeta
│       │   ├── number_prefix-dade78a5a916dddb.d
│       │   ├── indicatif-bc93d441eb23aa5e.d
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.04.rcgu.o
│       │   ├── libtiktoken_rs-7cacd8bbce1680ab.rmeta
│       │   ├── bit_set-d7322ea878b61688.bit_set.bfc6c8fe623b81b7-cgu.0.rcgu.o
│       │   ├── syn-2ab39f0e4fc565c6.d
│       │   ├── serde-f47b43f94aadc098.d
│       │   ├── liblazy_static-4fc00963a7215728.rmeta
│       │   ├── libunindent-95aa104d088359ad.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.4h4gnkd47f8w4t78.rcgu.o
│       │   ├── strsim-fadff44222618b4b.strsim.e1140db0c11a0d78-cgu.4.rcgu.o
│       │   ├── rayon_core-a2856f9f57a1e6c2.rayon_core.fe77e9b22c5e02db-cgu.7.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.ct58ozvf54cbnnk.rcgu.o
│       │   ├── objc-9aceb7739b194f45.objc.1dbd854cedaf6697-cgu.1.rcgu.o
│       │   ├── libucd_trie-f1a751efa5939de4.rmeta
│       │   ├── syn-cd4a8424ebf8304f.d
│       │   ├── pest_derive-30e7a9d1a2598605.d
│       │   ├── libthiserror_impl-0c990f87796132c7.dylib
│       │   ├── bit_vec-d0100a597cc971f3.d
│       │   ├── jwalk-4c072e0256a54789.jwalk.1707f2f275189ab8-cgu.0.rcgu.o
│       │   ├── handlebars-7682939d551c053d.handlebars.d95cd9d261f6d407-cgu.08.rcgu.o
│       │   ├── indicatif-bc93d441eb23aa5e.indicatif.b31dacfb960fbb2e-cgu.14.rcgu.o
│       │   ├── libanstyle_query-3bc001f5a383188e.rlib
│       │   ├── regex_syntax-45becf883096970e.regex_syntax.a6641376d637ea5d-cgu.15.rcgu.o
│       │   ├── handlebars-499fd614942c8270.d
│       │   ├── libautocfg-0eecde6036b9415d.rmeta
│       │   ├── libpest_generator-5af4eca3dc36d733.rlib
│       │   ├── once_cell-1b3d204c83a12703.d
│       │   ├── libmemchr-53c5fe84ba9abe21.rlib
│       │   ├── globset-42ddf0677cf59874.globset.c9574fab2d9e7190-cgu.00.rcgu.o
│       │   ├── taiko-5f5482d76af3ad3e.4wxtfrlpiyp4axhv.rcgu.o
│       │   ├── libunindent-02808f17bd4a482b.rmeta
│       │   ├── libprompt_macro-7c6d31ccf50d03b5.rmeta
│       │   ├── crossbeam_deque-34556cdbd670b867.d
│       │   ├── pest_derive-50b1d03c52ebd1e6.d
│       │   ├── ucd_trie-f1a751efa5939de4.ucd_trie.271f527e5aea8b64-cgu.1.rcgu.o
│       │   ├── libcrossbeam_epoch-8a7b82de4db57161.rmeta
│       │   ├── libcrossbeam_utils-f72012fcfdf033a4.rmeta
│       │   ├── librayon-08b4c57f583c0024.rmeta
│       │   ├── libhandlebars-5269e56c0793138a.rmeta
│       │   ├── liblazy_static-e3bceafda619eadf.rmeta
│       │   ├── crossbeam-044650bffcd98472.d
│       │   ├── rmetaebMtyP
│       │   │   └── lib.rmeta
│       │   ├── libparse_format-5f7c65faf1c4422a.rmeta
│       │   ├── crossbeam_queue-6ffa5f04fb97f232.crossbeam_queue.7ec1aaa6b93a337c-cgu.0.rcgu.o
│       │   ├── pest-9b28f6cac5ba28d8.d
│       │   ├── libscopeguard-1a41f4f101669df5.rmeta
│       │   ├── libheck-fe4aad64d30b965f.rmeta
│       │   ├── pest-68b61f401184a463.d
│       │   ├── unicode_ident-c820b7f471869748.d
│       │   ├── libpest_generator-e747617606104524.rmeta
│       │   ├── indicatif-6ecc1cea8c4dac4f.d
│       │   ├── serde_json-1aaf06abfb500a24.d
│       │   ├── libunicode_width-16a1984432142341.rlib
│       │   ├── libitoa-7afa4844fdfbbd60.rlib
│       │   ├── jwalk-e55f9b1b97b0d9fa.d
│       │   ├── libpest-bdcbc99b6b0e3535.rmeta
│       │   ├── libc-a93187a4ecae664b.d
│       │   ├── crossbeam-94dad57f069ce4b6.d
│       │   ├── taiko-5f5482d76af3ad3e.34cnf4b80vdz5gwm.rcgu.o
│       │   ├── libcli_clipboard-105138324c549219.rmeta
│       │   ├── aho_corasick-0d01aad8c1a49b2c.d
│       │   ├── libparking_lot-75ed7d3a6b2d6614.rmeta
│       │   ├── libcrossbeam_deque-7808d91b25723717.rlib
│       │   ├── cfg_if-13a8e19d179f00de.cfg_if.c832da7cbd1e9ed0-cgu.0.rcgu.o
│       │   ├── libregex_automata-2c01a250e90ac542.rlib
│       │   ├── serde_json-46bd2ddb93a26abd.serde_json.9574b3b9f529d731-cgu.1.rcgu.o
│       │   ├── libcrossbeam_deque-34556cdbd670b867.rmeta
│       │   ├── anstyle_query-7aa23c30b201a805.d
│       │   ├── thiserror-d715994f55f32c4b.d
│       │   ├── libbit_vec-0afb4ba3382ab368.rmeta
│       │   ├── taiko-5f5482d76af3ad3e.26z3o0htvtvx1zo4.rcgu.o
│       │   ├── libmalloc_buf-6e1f00ba28e7b2a6.rmeta
│       │   ├── itoa-b13010d569e2163d.d
│       │   ├── crossbeam_deque-3f19b7bc5f5cbf56.d
│       │   ├── taiko-e901070ac87d03b6.d
│       │   ├── fancy_regex-5c74bb0ac19aa96e.d
│       │   ├── block-f5cb22d2e0ed675f.d
│       │   ├── tiktoken_rs-7cacd8bbce1680ab.d
│       │   ├── tiktoken_rs-6b9d2c49470fe1b4.tiktoken_rs.f1e27d42bd45159-cgu.7.rcgu.o
│       │   ├── proc_macro2-203b1862d9ee796a.d
│       │   ├── taiko-5f5482d76af3ad3e.4wz8jsbck3c2e9by.rcgu.o
│       │   ├── log-9a66223a5e030806.log.68c472aded1c97b0-cgu.0.rcgu.o
│       │   ├── libjwalk-0c6633ef064c2b32.rmeta
│       │   ├── libryu-c350fc729588bc75.rmeta
│       │   ├── pest_generator-98c6362ac1cac2f8.d
│       │   ├── libunicode_width-2f86821300067d55.rmeta
│       │   ├── parse_format-5f7c65faf1c4422a.d
│       │   └── thiserror-5ae0973ba1aab17e.d
│       └── build
│           ├── proc-macro2-71b852fa649793eb
│           │   ├── build_script_build-71b852fa649793eb.build_script_build.86a6642e7e87370c-cgu.1.rcgu.o
│           │   ├── build_script_build-71b852fa649793eb.43r0rs0dg3dp9kpj.rcgu.o
│           │   ├── build_script_build-71b852fa649793eb.build_script_build.86a6642e7e87370c-cgu.0.rcgu.o
│           │   └── build_script_build-71b852fa649793eb.d
│           ├── proc-macro2-7b33bf05052de6ba
│           │   ├── build_script_build-7b33bf05052de6ba
│           │   ├── build_script_build-7b33bf05052de6ba.d
│           │   └── build-script-build
│           ├── serde_json-2700675899c561ee
│           │   ├── build_script_build-2700675899c561ee
│           │   ├── build_script_build-2700675899c561ee.d
│           │   └── build-script-build
│           ├── serde_json-0c88b2b71e3ca72e
│           │   ├── build_script_build-0c88b2b71e3ca72e
│           │   ├── build_script_build-0c88b2b71e3ca72e.d
│           │   └── build-script-build
│           ├── portable-atomic-61fd60073dd4b8b1
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── anyhow-f5f09e440259ab33
│           │   ├── out
│           │   │   └── anyhow.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── crossbeam-utils-b7c546d562ade16a
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── rayon-core-9aa8f630351beb3b
│           │   ├── build_script_build-9aa8f630351beb3b
│           │   ├── build_script_build-9aa8f630351beb3b.d
│           │   └── build-script-build
│           ├── portable-atomic-a382796ffbf9cf75
│           │   ├── build_script_build-a382796ffbf9cf75.d
│           │   ├── build_script_build-a382796ffbf9cf75
│           │   └── build-script-build
│           ├── serde-94d89c9341a7084d
│           │   ├── build_script_build-94d89c9341a7084d.d
│           │   ├── build-script-build
│           │   └── build_script_build-94d89c9341a7084d
│           ├── rayon-core-8d5da163b8fed8cf
│           │   ├── build_script_build-8d5da163b8fed8cf
│           │   ├── build_script_build-8d5da163b8fed8cf.d
│           │   └── build-script-build
│           ├── proc-macro2-47b3be70cfbd8223
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── parking_lot_core-adff1e52a301ae2a
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde_json-b6920857dcdc32eb
│           │   ├── build_script_build-b6920857dcdc32eb.16mvphmou2rtoymb.rcgu.o
│           │   ├── build_script_build-b6920857dcdc32eb.build_script_build.c2887dafaff13177-cgu.0.rcgu.o
│           │   └── build_script_build-b6920857dcdc32eb.d
│           ├── lock_api-e14509da734c2d54
│           │   ├── out
│           │   │   └── probe0.ll
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── parking_lot_core-a31b0fda74e682d7
│           │   ├── build_script_build-a31b0fda74e682d7
│           │   ├── build_script_build-a31b0fda74e682d7.d
│           │   └── build-script-build
│           ├── anyhow-21303603be6bb241
│           │   ├── build_script_build-21303603be6bb241.d
│           │   ├── build_script_build-21303603be6bb241
│           │   └── build-script-build
│           ├── proc-macro2-349eefc81df036f2
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde_json-fd2c26297c425c63
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── thiserror-c15fed6c1ab4b76d
│           │   ├── build_script_build-c15fed6c1ab4b76d.d
│           │   ├── build_script_build-c15fed6c1ab4b76d.43rm72el7so7obqa.rcgu.o
│           │   └── build_script_build-c15fed6c1ab4b76d.build_script_build.124cb6343ec5e2da-cgu.0.rcgu.o
│           ├── lock_api-c2cc38d2fe74d092
│           │   ├── out
│           │   │   └── probe0.ll
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── parking_lot_core-28739aa3822acdb8
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── crossbeam-utils-f1ab3b74eb49e525
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── parking_lot_core-8444d975184dad65
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── lock_api-fecfef5d950ea8e6
│           │   ├── build_script_build-fecfef5d950ea8e6
│           │   ├── build-script-build
│           │   └── build_script_build-fecfef5d950ea8e6.d
│           ├── rayon-core-f147dc92fcf8a1b9
│           │   └── out
│           ├── crossbeam-utils-4aa54a6ba37249b2
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── parking_lot_core-d6654147e146abf2
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── proc-macro2-4b2d5ab7c15a7841
│           │   ├── out
│           │   │   └── proc_macro2.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── thiserror-6814e2147fa7f90a
│           │   ├── build_script_build-6814e2147fa7f90a.d
│           │   ├── build_script_build-6814e2147fa7f90a
│           │   └── build-script-build
│           ├── parking_lot_core-1ec50e0a9439f42f
│           │   ├── build_script_build-1ec50e0a9439f42f
│           │   ├── build_script_build-1ec50e0a9439f42f.d
│           │   └── build-script-build
│           ├── serde-f4d29662f586b1ad
│           │   ├── build_script_build-f4d29662f586b1ad
│           │   ├── build-script-build
│           │   └── build_script_build-f4d29662f586b1ad.d
│           ├── crossbeam-utils-f785f4679f2d2fc4
│           │   ├── build_script_build-f785f4679f2d2fc4.d
│           │   ├── build-script-build
│           │   └── build_script_build-f785f4679f2d2fc4
│           ├── parking_lot_core-c72518f2222858e6
│           │   ├── build_script_build-c72518f2222858e6.d
│           │   ├── build_script_build-c72518f2222858e6
│           │   └── build-script-build
│           ├── lock_api-f37e667b77a3cbf6
│           │   ├── build_script_build-f37e667b77a3cbf6.d
│           │   ├── build_script_build-f37e667b77a3cbf6
│           │   └── build-script-build
│           ├── proc-macro2-1bac009bb7e1645d
│           │   ├── build_script_build-1bac009bb7e1645d
│           │   ├── build_script_build-1bac009bb7e1645d.d
│           │   └── build-script-build
│           ├── parking_lot_core-1310bf90b95215f4
│           │   ├── build_script_build-1310bf90b95215f4.d
│           │   ├── build_script_build-1310bf90b95215f4
│           │   └── build-script-build
│           ├── proc-macro2-5e1800b72e2bcd37
│           │   ├── build_script_build-5e1800b72e2bcd37.d
│           │   ├── build_script_build-5e1800b72e2bcd37
│           │   └── build-script-build
│           ├── proc-macro2-0d77a8bb2eb18aac
│           │   └── out
│           ├── proc-macro2-3d041c88ac13b7b2
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde-51e9b92bb5385dd3
│           │   ├── build_script_build-51e9b92bb5385dd3
│           │   ├── build_script_build-51e9b92bb5385dd3.d
│           │   └── build-script-build
│           ├── rayon-core-85d69c3fed040b0b
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── anyhow-694f0bf564b7f760
│           │   └── out
│           ├── serde-9cbba86eec346dfd
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-f88559519fdda68a
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── anyhow-c328af7e82a9ff5a
│           │   ├── build_script_build-c328af7e82a9ff5a.build_script_build.36c848aad5132615-cgu.1.rcgu.o
│           │   ├── build_script_build-c328af7e82a9ff5a.1thd6v71brywc5na.rcgu.o
│           │   ├── build_script_build-c328af7e82a9ff5a.build_script_build.36c848aad5132615-cgu.0.rcgu.o
│           │   └── build_script_build-c328af7e82a9ff5a.d
│           ├── proc-macro2-c018b269e41b4945
│           │   ├── build_script_build-c018b269e41b4945.d
│           │   ├── build_script_build-c018b269e41b4945
│           │   └── build-script-build
│           ├── serde-e320218ce2ea030f
│           │   ├── build_script_build-e320218ce2ea030f
│           │   ├── build_script_build-e320218ce2ea030f.d
│           │   └── build-script-build
│           ├── crossbeam-utils-5e7b9a99208763ac
│           │   └── out
│           ├── thiserror-6946fc1648e43c4d
│           │   ├── build_script_build-6946fc1648e43c4d.d
│           │   ├── build_script_build-6946fc1648e43c4d
│           │   └── build-script-build
│           ├── crossbeam-utils-7dc5b415c10355de
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── proc-macro2-34d22fc484647129
│           │   ├── out
│           │   │   └── proc_macro2.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde-258c9c74048a698a
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── thiserror-8d71d83770ec0d1e
│           │   ├── build_script_build-8d71d83770ec0d1e
│           │   ├── build_script_build-8d71d83770ec0d1e.d
│           │   └── build-script-build
│           ├── crossbeam-utils-8c4a123449621551
│           │   ├── build_script_build-8c4a123449621551
│           │   ├── build_script_build-8c4a123449621551.d
│           │   └── build-script-build
│           ├── lock_api-43b5baccf2761d58
│           │   ├── out
│           │   │   └── probe0.ll
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── proc-macro2-2960ce1923938b06
│           │   ├── out
│           │   │   └── proc_macro2.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-bfb29dfdfe86d3b5
│           │   ├── build_script_build-bfb29dfdfe86d3b5.d
│           │   ├── build-script-build
│           │   └── build_script_build-bfb29dfdfe86d3b5
│           ├── serde_json-513cda1226c2709e
│           │   └── out
│           ├── rayon-core-f1951c452243f234
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── anyhow-f75b32b55c786955
│           │   ├── build_script_build-f75b32b55c786955.d
│           │   ├── build-script-build
│           │   └── build_script_build-f75b32b55c786955
│           ├── lock_api-d8c796c9d5186783
│           │   ├── build_script_build-d8c796c9d5186783.d
│           │   ├── build-script-build
│           │   └── build_script_build-d8c796c9d5186783
│           ├── anyhow-69dad769cf04aec3
│           │   ├── build_script_build-69dad769cf04aec3.d
│           │   ├── build-script-build
│           │   └── build_script_build-69dad769cf04aec3
│           ├── parking_lot_core-195e9b6ef181b17e
│           │   ├── build_script_build-195e9b6ef181b17e.d
│           │   ├── build_script_build-195e9b6ef181b17e.4ndvtwgbmlu9cask.rcgu.o
│           │   └── build_script_build-195e9b6ef181b17e.build_script_build.234ad5104da37bb3-cgu.0.rcgu.o
│           ├── parking_lot_core-cb766df791a66321
│           │   └── out
│           ├── lock_api-4176eb344898284d
│           │   └── out
│           ├── serde-f34b0a63c68e2b28
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── lock_api-3a2d4e573bd6ab89
│           │   ├── build_script_build-3a2d4e573bd6ab89.build_script_build.46b39aaa38c4d8b-cgu.0.rcgu.o
│           │   ├── build_script_build-3a2d4e573bd6ab89.d
│           │   └── build_script_build-3a2d4e573bd6ab89.2uklgffilwb3of0c.rcgu.o
│           ├── libc-b8cc196fd93b7573
│           │   ├── build_script_build-b8cc196fd93b7573.build_script_build.4beee6d1bd2278b1-cgu.1.rcgu.o
│           │   ├── build_script_build-b8cc196fd93b7573.3mjwqbmkt9a07krz.rcgu.o
│           │   ├── build_script_build-b8cc196fd93b7573.build_script_build.4beee6d1bd2278b1-cgu.3.rcgu.o
│           │   ├── build_script_build-b8cc196fd93b7573.d
│           │   ├── build_script_build-b8cc196fd93b7573.build_script_build.4beee6d1bd2278b1-cgu.0.rcgu.o
│           │   └── build_script_build-b8cc196fd93b7573.build_script_build.4beee6d1bd2278b1-cgu.2.rcgu.o
│           ├── thiserror-a2716e01c1f71a25
│           │   ├── out
│           │   │   └── thiserror.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-0b6f3c5169fb2332
│           │   ├── build_script_build-0b6f3c5169fb2332.d
│           │   ├── build-script-build
│           │   └── build_script_build-0b6f3c5169fb2332
│           ├── libc-f83c8660959772d2
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde_json-e01ac2acfe810987
│           │   ├── build_script_build-e01ac2acfe810987
│           │   ├── build_script_build-e01ac2acfe810987.d
│           │   └── build-script-build
│           ├── serde-1280fa902d100c35
│           │   ├── build_script_build-1280fa902d100c35.124bhwk32jfz089c.rcgu.o
│           │   ├── build_script_build-1280fa902d100c35.d
│           │   └── build_script_build-1280fa902d100c35.build_script_build.8a1f837ec8484f8a-cgu.0.rcgu.o
│           ├── libc-7d49f0dd348944e7
│           │   ├── build_script_build-7d49f0dd348944e7
│           │   ├── build_script_build-7d49f0dd348944e7.d
│           │   └── build-script-build
│           ├── serde-15ef351b926016da
│           │   ├── build_script_build-15ef351b926016da
│           │   ├── build_script_build-15ef351b926016da.d
│           │   └── build-script-build
│           ├── serde_json-ceb6742366fbe817
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── anyhow-4d8661d83f222ced
│           │   ├── out
│           │   │   └── anyhow.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── libc-e8c2ebee99eca0ea
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-b180948938fa23ec
│           │   ├── build_script_build-b180948938fa23ec
│           │   ├── build_script_build-b180948938fa23ec.d
│           │   └── build-script-build
│           ├── portable-atomic-6bd9235b10a31d01
│           │   ├── build_script_build-6bd9235b10a31d01
│           │   ├── build_script_build-6bd9235b10a31d01.d
│           │   └── build-script-build
│           ├── lock_api-cc150cec7c9b3e6b
│           │   ├── out
│           │   │   └── probe0.ll
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── libc-ef19e0a46c35f68e
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── parking_lot_core-a7554249a02c828d
│           │   ├── build_script_build-a7554249a02c828d
│           │   ├── build-script-build
│           │   └── build_script_build-a7554249a02c828d.d
│           ├── thiserror-917e6197ca09e5cb
│           │   ├── out
│           │   │   └── thiserror.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── proc-macro2-5914dfc89e6c01c8
│           │   ├── out
│           │   │   └── proc_macro2.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── rayon-core-96f4ef6fdedbf457
│           │   ├── build_script_build-96f4ef6fdedbf457.d
│           │   ├── build_script_build-96f4ef6fdedbf457
│           │   └── build-script-build
│           ├── lock_api-f62d494bf69c7a29
│           │   ├── build_script_build-f62d494bf69c7a29
│           │   ├── build_script_build-f62d494bf69c7a29.d
│           │   └── build-script-build
│           ├── portable-atomic-18451c41678abd99
│           │   ├── build_script_build-18451c41678abd99.build_script_build.c097003a755c5d81-cgu.3.rcgu.o
│           │   ├── build_script_build-18451c41678abd99.build_script_build.c097003a755c5d81-cgu.1.rcgu.o
│           │   ├── build_script_build-18451c41678abd99.ddwa4iktwao9b6i.rcgu.o
│           │   ├── build_script_build-18451c41678abd99.d
│           │   ├── build_script_build-18451c41678abd99.build_script_build.c097003a755c5d81-cgu.2.rcgu.o
│           │   └── build_script_build-18451c41678abd99.build_script_build.c097003a755c5d81-cgu.0.rcgu.o
│           ├── crossbeam-utils-294497481999ce73
│           │   ├── build_script_build-294497481999ce73.build_script_build.f791489844f62608-cgu.0.rcgu.o
│           │   ├── build_script_build-294497481999ce73.d
│           │   ├── build_script_build-294497481999ce73.build_script_build.f791489844f62608-cgu.1.rcgu.o
│           │   └── build_script_build-294497481999ce73.we9eewemckz48p1.rcgu.o
│           ├── parking_lot_core-3b8e996bca7d399a
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-a13e051751f7cf84
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde-f1abb45418c4c481
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── crossbeam-utils-12057a0bc6ecc3ca
│           │   ├── build_script_build-12057a0bc6ecc3ca
│           │   ├── build_script_build-12057a0bc6ecc3ca.d
│           │   └── build-script-build
│           ├── portable-atomic-b1e5bff091e618d2
│           │   └── out
│           ├── libc-f946f47e7407f08d
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── proc-macro2-57ec4510a8178437
│           │   ├── build_script_build-57ec4510a8178437
│           │   ├── build-script-build
│           │   └── build_script_build-57ec4510a8178437.d
│           ├── thiserror-312da2e49e7367df
│           │   └── out
│           ├── thiserror-05f09dc7460e1f7b
│           │   ├── build_script_build-05f09dc7460e1f7b
│           │   ├── build-script-build
│           │   └── build_script_build-05f09dc7460e1f7b.d
│           ├── crossbeam-utils-a21ae51dabfdac5e
│           │   ├── build_script_build-a21ae51dabfdac5e
│           │   ├── build-script-build
│           │   └── build_script_build-a21ae51dabfdac5e.d
│           ├── anyhow-884aa501d02da459
│           │   ├── build_script_build-884aa501d02da459
│           │   ├── build_script_build-884aa501d02da459.d
│           │   └── build-script-build
│           ├── serde-3b411f723ea4e60e
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── rayon-core-7f2e74a336e475fe
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde_json-40d13c6728b7cde0
│           │   ├── build_script_build-40d13c6728b7cde0.d
│           │   ├── build_script_build-40d13c6728b7cde0
│           │   └── build-script-build
│           ├── anyhow-0126dbf0d6bf1421
│           │   ├── build_script_build-0126dbf0d6bf1421
│           │   ├── build_script_build-0126dbf0d6bf1421.d
│           │   └── build-script-build
│           ├── crossbeam-utils-2f1344100f9d4784
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── thiserror-e22779b02053e86e
│           │   ├── out
│           │   │   └── thiserror.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── crossbeam-utils-b86b090deaa07f31
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde_json-14fca741fe22317c
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── libc-86c557807c3c7c82
│           │   └── out
│           ├── crossbeam-utils-dbfafa18b0a911eb
│           │   ├── build_script_build-dbfafa18b0a911eb
│           │   ├── build-script-build
│           │   └── build_script_build-dbfafa18b0a911eb.d
│           ├── rayon-core-a4047371f319ed01
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-8927d37f21084d26
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde-d555c6b832c055cc
│           │   ├── build_script_build-d555c6b832c055cc
│           │   ├── build_script_build-d555c6b832c055cc.d
│           │   └── build-script-build
│           ├── libc-7c14432322c24806
│           │   ├── build_script_build-7c14432322c24806
│           │   ├── build_script_build-7c14432322c24806.d
│           │   └── build-script-build
│           ├── libc-4d9c835ed33afc41
│           │   ├── build_script_build-4d9c835ed33afc41
│           │   ├── build_script_build-4d9c835ed33afc41.d
│           │   └── build-script-build
│           ├── serde_json-04f30e6423e78c58
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── libc-bfa4e3e8a2a22d17
│           │   ├── build_script_build-bfa4e3e8a2a22d17
│           │   ├── build-script-build
│           │   └── build_script_build-bfa4e3e8a2a22d17.d
│           ├── thiserror-e32ce5bababc08dd
│           │   ├── out
│           │   │   └── thiserror.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── libc-e0eb1e838a8f7c1a
│           │   ├── build_script_build-e0eb1e838a8f7c1a.d
│           │   ├── build_script_build-e0eb1e838a8f7c1a
│           │   └── build-script-build
│           ├── anyhow-98cfd45e744f3825
│           │   ├── out
│           │   │   └── anyhow.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde_json-9602a6c20b2da187
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── lock_api-044addbb5279c67e
│           │   ├── out
│           │   │   └── probe0.ll
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde-c7fe8afb9177998f
│           │   └── out
│           ├── lock_api-63b0ddbffb201236
│           │   ├── build_script_build-63b0ddbffb201236
│           │   ├── build_script_build-63b0ddbffb201236.d
│           │   └── build-script-build
│           ├── serde_json-d90e3cb43b4b65d6
│           │   ├── build_script_build-d90e3cb43b4b65d6.d
│           │   ├── build-script-build
│           │   └── build_script_build-d90e3cb43b4b65d6
│           ├── libc-fc2e0bf46424e06c
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── proc-macro2-1007b32837bdd3d2
│           │   ├── build_script_build-1007b32837bdd3d2.d
│           │   ├── build_script_build-1007b32837bdd3d2
│           │   └── build-script-build
│           ├── rayon-core-cba18d45bf387724
│           │   ├── build_script_build-cba18d45bf387724.d
│           │   ├── build_script_build-cba18d45bf387724.build_script_build.f261a664d423d2fb-cgu.0.rcgu.o
│           │   └── build_script_build-cba18d45bf387724.5a4he4sbqy3xpwik.rcgu.o
│           ├── anyhow-7b81de0558c140d6
│           │   ├── out
│           │   │   └── anyhow.d
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── serde-a8f107b475504fab
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── portable-atomic-97929674aaf1fcaa
│           │   ├── out
│           │   ├── stderr
│           │   ├── output
│           │   ├── root-output
│           │   └── invoked.timestamp
│           ├── rayon-core-a1ab18239ea5d8d5
│           │   ├── build_script_build-a1ab18239ea5d8d5.d
│           │   ├── build-script-build
│           │   └── build_script_build-a1ab18239ea5d8d5
│           ├── crossbeam-utils-fa78451dbc620ac9
│           │   ├── build_script_build-fa78451dbc620ac9.d
│           │   ├── build_script_build-fa78451dbc620ac9
│           │   └── build-script-build
│           └── anyhow-1d35aa32df47f0fd
│               ├── out
│               │   └── anyhow.d
│               ├── stderr
│               ├── output
│               ├── root-output
│               └── invoked.timestamp
├── prompt-macro
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       └── lib.rs
├── README.md
└── src
    ├── prompts.rs
    └── main.rs


Files Content:
/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-a091b4cd7dc48367/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-check-cfg=cfg(anyhow_nightly_testing)
cargo:rustc-check-cfg=cfg(anyhow_no_core_error)
cargo:rustc-check-cfg=cfg(anyhow_no_core_unwind_safe)
cargo:rustc-check-cfg=cfg(anyhow_no_fmt_arguments_as_str)
cargo:rustc-check-cfg=cfg(anyhow_no_ptr_addr_of)
cargo:rustc-check-cfg=cfg(anyhow_no_unsafe_op_in_unsafe_fn_lint)
cargo:rustc-check-cfg=cfg(error_generic_member_access)
cargo:rustc-check-cfg=cfg(std_backtrace)
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-a091b4cd7dc48367/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-a091b4cd7dc48367/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-7d1be0e058c15c26/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=limb_width_64

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-7d1be0e058c15c26/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-7d1be0e058c15c26/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-a400f8a5cc30ee13/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-a400f8a5cc30ee13/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-a400f8a5cc30ee13/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-cf5f604d36dc7a38/output:

```````
cargo:rustc-check-cfg=cfg(fuzzing)
cargo:rustc-check-cfg=cfg(no_is_available)
cargo:rustc-check-cfg=cfg(no_literal_byte_character)
cargo:rustc-check-cfg=cfg(no_literal_c_string)
cargo:rustc-check-cfg=cfg(no_source_text)
cargo:rustc-check-cfg=cfg(proc_macro_span)
cargo:rustc-check-cfg=cfg(procmacro2_backtrace)
cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)
cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)
cargo:rustc-check-cfg=cfg(randomize_layout)
cargo:rustc-check-cfg=cfg(span_locations)
cargo:rustc-check-cfg=cfg(super_unstable)
cargo:rustc-check-cfg=cfg(wrap_proc_macro)
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-cf5f604d36dc7a38/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-cf5f604d36dc7a38/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-0ffbaafdf4ad1c9c/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-0ffbaafdf4ad1c9c/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-0ffbaafdf4ad1c9c/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-582ee046782621db/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-582ee046782621db/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/lock_api-582ee046782621db/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-8e08ac94a2738537/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rustc-check-cfg=cfg(target_feature,values("experimental-zacas","fast-serialization","load-store-on-cond","distinct-ops","miscellaneous-extensions-3"))
cargo:rustc-check-cfg=cfg(portable_atomic_disable_fiq,portable_atomic_force_amo,portable_atomic_ll_sc_rmw,portable_atomic_new_atomic_intrinsics,portable_atomic_no_asm,portable_atomic_no_asm_maybe_uninit,portable_atomic_no_atomic_64,portable_atomic_no_atomic_cas,portable_atomic_no_atomic_load_store,portable_atomic_no_atomic_min_max,portable_atomic_no_cfg_target_has_atomic,portable_atomic_no_cmpxchg16b_intrinsic,portable_atomic_no_cmpxchg16b_target_feature,portable_atomic_no_const_mut_refs,portable_atomic_no_const_raw_ptr_deref,portable_atomic_no_const_transmute,portable_atomic_no_core_unwind_safe,portable_atomic_no_diagnostic_namespace,portable_atomic_no_offset_of,portable_atomic_no_stronger_failure_ordering,portable_atomic_no_track_caller,portable_atomic_no_unsafe_op_in_unsafe_fn,portable_atomic_pre_llvm_15,portable_atomic_pre_llvm_16,portable_atomic_pre_llvm_18,portable_atomic_s_mode,portable_atomic_sanitize_thread,portable_atomic_target_feature,portable_atomic_unsafe_assume_single_core,portable_atomic_unstable_asm,portable_atomic_unstable_asm_experimental_arch,portable_atomic_unstable_cfg_target_has_atomic,portable_atomic_unstable_isa_attribute)
cargo:rustc-check-cfg=cfg(portable_atomic_target_feature,values("cmpxchg16b","distinct-ops","experimental-zacas","fast-serialization","load-store-on-cond","lse","lse128","lse2","mclass","miscellaneous-extensions-3","quadword-atomics","rcpc3","v6","zaamo","zabha"))
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-8e08ac94a2738537/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-8e08ac94a2738537/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/thiserror-1ee99c6e96769780/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/thiserror-1ee99c6e96769780/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/thiserror-1ee99c6e96769780/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-6a4dea2c0a9fd5e2/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_FREEBSD_VERSION
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_const_extern_fn
cargo:rustc-check-cfg=cfg(emscripten_new_stat_abi)
cargo:rustc-check-cfg=cfg(espidf_time32)
cargo:rustc-check-cfg=cfg(freebsd10)
cargo:rustc-check-cfg=cfg(freebsd11)
cargo:rustc-check-cfg=cfg(freebsd12)
cargo:rustc-check-cfg=cfg(freebsd13)
cargo:rustc-check-cfg=cfg(freebsd14)
cargo:rustc-check-cfg=cfg(freebsd15)
cargo:rustc-check-cfg=cfg(libc_const_extern_fn)
cargo:rustc-check-cfg=cfg(libc_deny_warnings)
cargo:rustc-check-cfg=cfg(libc_thread_local)
cargo:rustc-check-cfg=cfg(libc_ctest)
cargo:rustc-check-cfg=cfg(target_os,values("switch","aix","ohos","hurd","rtems","visionos","nuttx"))
cargo:rustc-check-cfg=cfg(target_env,values("illumos","wasi","aix","ohos"))
cargo:rustc-check-cfg=cfg(target_arch,values("loongarch64","mips32r6","mips64r6","csky"))

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-6a4dea2c0a9fd5e2/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-6a4dea2c0a9fd5e2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-3c63712aee8ad31a/output:

```````
cargo:rerun-if-changed=no_atomic.rs
cargo:rustc-check-cfg=cfg(crossbeam_no_atomic,crossbeam_sanitize_thread)

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-3c63712aee8ad31a/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-3c63712aee8ad31a/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-5f266a2ffb04b8e9/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-5f266a2ffb04b8e9/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/rayon-core-ffc8f37c34c6bcd2/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/rayon-core-ffc8f37c34c6bcd2/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/rayon-core-ffc8f37c34c6bcd2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-dd56bfc521ddfbf2/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-dd56bfc521ddfbf2/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-dd56bfc521ddfbf2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-ecd13d08fd0b38ec/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS
cargo:rustc-cfg=portable_atomic_llvm_16
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-ecd13d08fd0b38ec/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-ecd13d08fd0b38ec/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-3cd929236be2f1f6/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_FREEBSD_VERSION
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_const_extern_fn
cargo:rustc-check-cfg=cfg(emscripten_new_stat_abi)
cargo:rustc-check-cfg=cfg(espidf_time32)
cargo:rustc-check-cfg=cfg(freebsd10)
cargo:rustc-check-cfg=cfg(freebsd11)
cargo:rustc-check-cfg=cfg(freebsd12)
cargo:rustc-check-cfg=cfg(freebsd13)
cargo:rustc-check-cfg=cfg(freebsd14)
cargo:rustc-check-cfg=cfg(freebsd15)
cargo:rustc-check-cfg=cfg(libc_const_extern_fn)
cargo:rustc-check-cfg=cfg(libc_deny_warnings)
cargo:rustc-check-cfg=cfg(libc_thread_local)
cargo:rustc-check-cfg=cfg(libc_ctest)
cargo:rustc-check-cfg=cfg(target_os,values("switch","aix","ohos","hurd","rtems","visionos","nuttx"))
cargo:rustc-check-cfg=cfg(target_env,values("illumos","wasi","aix","ohos"))
cargo:rustc-check-cfg=cfg(target_arch,values("loongarch64","mips32r6","mips64r6","csky"))

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-3cd929236be2f1f6/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-3cd929236be2f1f6/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-d893a28c929bfebd/output:

```````
cargo:rustc-check-cfg=cfg(fuzzing)
cargo:rustc-check-cfg=cfg(no_is_available)
cargo:rustc-check-cfg=cfg(no_literal_byte_character)
cargo:rustc-check-cfg=cfg(no_literal_c_string)
cargo:rustc-check-cfg=cfg(no_source_text)
cargo:rustc-check-cfg=cfg(proc_macro_span)
cargo:rustc-check-cfg=cfg(procmacro2_backtrace)
cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)
cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)
cargo:rustc-check-cfg=cfg(randomize_layout)
cargo:rustc-check-cfg=cfg(span_locations)
cargo:rustc-check-cfg=cfg(super_unstable)
cargo:rustc-check-cfg=cfg(wrap_proc_macro)
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-d893a28c929bfebd/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-d893a28c929bfebd/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-f84b20b85c3c2187/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde_json-f84b20b85c3c2187/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-e3e5be45e3154b11/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-e3e5be45e3154b11/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-e3e5be45e3154b11/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/thiserror-d92368118985b354/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/thiserror-d92368118985b354/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/thiserror-d92368118985b354/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-3c3480ffa2df55d0/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-3c3480ffa2df55d0/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/parking_lot_core-3c3480ffa2df55d0/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-3bc99fc33d70cf87/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-check-cfg=cfg(no_core_cstr)
cargo:rustc-check-cfg=cfg(no_core_error)
cargo:rustc-check-cfg=cfg(no_core_net)
cargo:rustc-check-cfg=cfg(no_core_num_saturating)
cargo:rustc-check-cfg=cfg(no_core_try_from)
cargo:rustc-check-cfg=cfg(no_diagnostic_namespace)
cargo:rustc-check-cfg=cfg(no_float_copysign)
cargo:rustc-check-cfg=cfg(no_num_nonzero_signed)
cargo:rustc-check-cfg=cfg(no_relaxed_trait_bounds)
cargo:rustc-check-cfg=cfg(no_serde_derive)
cargo:rustc-check-cfg=cfg(no_std_atomic)
cargo:rustc-check-cfg=cfg(no_std_atomic64)
cargo:rustc-check-cfg=cfg(no_systemtime_checked_add)
cargo:rustc-check-cfg=cfg(no_target_has_atomic)

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-3bc99fc33d70cf87/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-3bc99fc33d70cf87/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-2858bc19af6b2c4e/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_int128
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN
cargo:rustc-cfg=libc_cfg_target_vendor
cargo:rustc-cfg=libc_non_exhaustive
cargo:rustc-cfg=libc_long_array
cargo:rustc-cfg=libc_ptr_addr_of
cargo:rustc-cfg=libc_underscore_const_names
cargo:rustc-cfg=libc_const_extern_fn

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-2858bc19af6b2c4e/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/libc-2858bc19af6b2c4e/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-2c61182232f0172d/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-2c61182232f0172d/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-2c61182232f0172d/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-56c0ed1b478c118e/output:

```````
cargo:rerun-if-changed=no_atomic.rs
cargo:rustc-check-cfg=cfg(crossbeam_no_atomic,crossbeam_sanitize_thread)

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-56c0ed1b478c118e/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/crossbeam-utils-56c0ed1b478c118e/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-a157e6cd2cc2aa25/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-a157e6cd2cc2aa25/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-a157e6cd2cc2aa25/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-ce8e51eda864b3b6/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rustc-check-cfg=cfg(target_feature,values("experimental-zacas","fast-serialization","load-store-on-cond","distinct-ops","miscellaneous-extensions-3"))
cargo:rustc-check-cfg=cfg(portable_atomic_disable_fiq,portable_atomic_force_amo,portable_atomic_ll_sc_rmw,portable_atomic_new_atomic_intrinsics,portable_atomic_no_asm,portable_atomic_no_asm_maybe_uninit,portable_atomic_no_atomic_64,portable_atomic_no_atomic_cas,portable_atomic_no_atomic_load_store,portable_atomic_no_atomic_min_max,portable_atomic_no_cfg_target_has_atomic,portable_atomic_no_cmpxchg16b_intrinsic,portable_atomic_no_cmpxchg16b_target_feature,portable_atomic_no_const_mut_refs,portable_atomic_no_const_raw_ptr_deref,portable_atomic_no_const_transmute,portable_atomic_no_core_unwind_safe,portable_atomic_no_diagnostic_namespace,portable_atomic_no_offset_of,portable_atomic_no_stronger_failure_ordering,portable_atomic_no_track_caller,portable_atomic_no_unsafe_op_in_unsafe_fn,portable_atomic_pre_llvm_15,portable_atomic_pre_llvm_16,portable_atomic_pre_llvm_18,portable_atomic_s_mode,portable_atomic_sanitize_thread,portable_atomic_target_feature,portable_atomic_unsafe_assume_single_core,portable_atomic_unstable_asm,portable_atomic_unstable_asm_experimental_arch,portable_atomic_unstable_cfg_target_has_atomic,portable_atomic_unstable_isa_attribute)
cargo:rustc-check-cfg=cfg(portable_atomic_target_feature,values("cmpxchg16b","distinct-ops","experimental-zacas","fast-serialization","load-store-on-cond","lse","lse128","lse2","mclass","miscellaneous-extensions-3","quadword-atomics","rcpc3","v6","zaamo","zabha"))
cargo:rustc-cfg=portable_atomic_no_const_mut_refs
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-ce8e51eda864b3b6/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/portable-atomic-ce8e51eda864b3b6/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-bd1507a17571b635/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-check-cfg=cfg(no_core_cstr)
cargo:rustc-check-cfg=cfg(no_core_error)
cargo:rustc-check-cfg=cfg(no_core_net)
cargo:rustc-check-cfg=cfg(no_core_num_saturating)
cargo:rustc-check-cfg=cfg(no_core_try_from)
cargo:rustc-check-cfg=cfg(no_diagnostic_namespace)
cargo:rustc-check-cfg=cfg(no_float_copysign)
cargo:rustc-check-cfg=cfg(no_num_nonzero_signed)
cargo:rustc-check-cfg=cfg(no_relaxed_trait_bounds)
cargo:rustc-check-cfg=cfg(no_serde_derive)
cargo:rustc-check-cfg=cfg(no_std_atomic)
cargo:rustc-check-cfg=cfg(no_std_atomic64)
cargo:rustc-check-cfg=cfg(no_systemtime_checked_add)
cargo:rustc-check-cfg=cfg(no_target_has_atomic)

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-bd1507a17571b635/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/serde-bd1507a17571b635/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-a43fe1062415b5be/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-a43fe1062415b5be/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/proc-macro2-a43fe1062415b5be/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-161213d1cc945bff/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-161213d1cc945bff/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-161213d1cc945bff/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-157b02da119bf892/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-check-cfg=cfg(anyhow_nightly_testing)
cargo:rustc-check-cfg=cfg(anyhow_no_core_error)
cargo:rustc-check-cfg=cfg(anyhow_no_core_unwind_safe)
cargo:rustc-check-cfg=cfg(anyhow_no_fmt_arguments_as_str)
cargo:rustc-check-cfg=cfg(anyhow_no_ptr_addr_of)
cargo:rustc-check-cfg=cfg(anyhow_no_unsafe_op_in_unsafe_fn_lint)
cargo:rustc-check-cfg=cfg(error_generic_member_access)
cargo:rustc-check-cfg=cfg(std_backtrace)
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-157b02da119bf892/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/release/build/anyhow-157b02da119bf892/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-61fd60073dd4b8b1/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS
cargo:rustc-cfg=portable_atomic_llvm_16
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-61fd60073dd4b8b1/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-61fd60073dd4b8b1/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-f5f09e440259ab33/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-f5f09e440259ab33/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-f5f09e440259ab33/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-b7c546d562ade16a/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-b7c546d562ade16a/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/crossbeam-utils-b7c546d562ade16a/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-47b3be70cfbd8223/output:

```````
cargo:rustc-check-cfg=cfg(fuzzing)
cargo:rustc-check-cfg=cfg(no_is_available)
cargo:rustc-check-cfg=cfg(no_literal_byte_character)
cargo:rustc-check-cfg=cfg(no_literal_c_string)
cargo:rustc-check-cfg=cfg(no_source_text)
cargo:rustc-check-cfg=cfg(proc_macro_span)
cargo:rustc-check-cfg=cfg(procmacro2_backtrace)
cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)
cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)
cargo:rustc-check-cfg=cfg(randomize_layout)
cargo:rustc-check-cfg=cfg(span_locations)
cargo:rustc-check-cfg=cfg(super_unstable)
cargo:rustc-check-cfg=cfg(wrap_proc_macro)
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-47b3be70cfbd8223/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/proc-macro2-47b3be70cfbd8223/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-adff1e52a301ae2a/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-adff1e52a301ae2a/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-adff1e52a301ae2a/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-e14509da734c2d54/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-e14509da734c2d54/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/lock_api-e14509da734c2d54/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-349eefc81df036f2/output:

```````
cargo:rustc-check-cfg=cfg(fuzzing)
cargo:rustc-check-cfg=cfg(no_is_available)
cargo:rustc-check-cfg=cfg(no_literal_byte_character)
cargo:rustc-check-cfg=cfg(no_literal_c_string)
cargo:rustc-check-cfg=cfg(no_source_text)
cargo:rustc-check-cfg=cfg(proc_macro_span)
cargo:rustc-check-cfg=cfg(procmacro2_backtrace)
cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)
cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)
cargo:rustc-check-cfg=cfg(randomize_layout)
cargo:rustc-check-cfg=cfg(span_locations)
cargo:rustc-check-cfg=cfg(super_unstable)
cargo:rustc-check-cfg=cfg(wrap_proc_macro)
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-349eefc81df036f2/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-349eefc81df036f2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-fd2c26297c425c63/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=limb_width_64

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-fd2c26297c425c63/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/serde_json-fd2c26297c425c63/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-c2cc38d2fe74d092/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-c2cc38d2fe74d092/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-c2cc38d2fe74d092/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-28739aa3822acdb8/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-28739aa3822acdb8/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/parking_lot_core-28739aa3822acdb8/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-f1ab3b74eb49e525/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-f1ab3b74eb49e525/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/crossbeam-utils-f1ab3b74eb49e525/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-8444d975184dad65/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-8444d975184dad65/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/parking_lot_core-8444d975184dad65/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-4aa54a6ba37249b2/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-4aa54a6ba37249b2/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/crossbeam-utils-4aa54a6ba37249b2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-d6654147e146abf2/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-d6654147e146abf2/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-d6654147e146abf2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-4b2d5ab7c15a7841/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-4b2d5ab7c15a7841/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/proc-macro2-4b2d5ab7c15a7841/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-3d041c88ac13b7b2/output:

```````
cargo:rustc-check-cfg=cfg(fuzzing)
cargo:rustc-check-cfg=cfg(no_is_available)
cargo:rustc-check-cfg=cfg(no_literal_byte_character)
cargo:rustc-check-cfg=cfg(no_literal_c_string)
cargo:rustc-check-cfg=cfg(no_source_text)
cargo:rustc-check-cfg=cfg(proc_macro_span)
cargo:rustc-check-cfg=cfg(procmacro2_backtrace)
cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)
cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)
cargo:rustc-check-cfg=cfg(randomize_layout)
cargo:rustc-check-cfg=cfg(span_locations)
cargo:rustc-check-cfg=cfg(super_unstable)
cargo:rustc-check-cfg=cfg(wrap_proc_macro)
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-3d041c88ac13b7b2/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/proc-macro2-3d041c88ac13b7b2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-85d69c3fed040b0b/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-85d69c3fed040b0b/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/rayon-core-85d69c3fed040b0b/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-9cbba86eec346dfd/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-9cbba86eec346dfd/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-9cbba86eec346dfd/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-f88559519fdda68a/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS
cargo:rustc-cfg=portable_atomic_llvm_16
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-f88559519fdda68a/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-f88559519fdda68a/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-7dc5b415c10355de/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-7dc5b415c10355de/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-7dc5b415c10355de/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-34d22fc484647129/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-34d22fc484647129/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/proc-macro2-34d22fc484647129/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-258c9c74048a698a/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-258c9c74048a698a/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/serde-258c9c74048a698a/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-43b5baccf2761d58/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-43b5baccf2761d58/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-43b5baccf2761d58/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-2960ce1923938b06/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-2960ce1923938b06/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-2960ce1923938b06/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-f1951c452243f234/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-f1951c452243f234/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/rayon-core-f1951c452243f234/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-f34b0a63c68e2b28/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-f34b0a63c68e2b28/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-f34b0a63c68e2b28/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-a2716e01c1f71a25/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-a2716e01c1f71a25/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/thiserror-a2716e01c1f71a25/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-f83c8660959772d2/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_int128
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN
cargo:rustc-cfg=libc_cfg_target_vendor
cargo:rustc-cfg=libc_non_exhaustive
cargo:rustc-cfg=libc_long_array
cargo:rustc-cfg=libc_ptr_addr_of
cargo:rustc-cfg=libc_underscore_const_names
cargo:rustc-cfg=libc_const_extern_fn

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-f83c8660959772d2/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-f83c8660959772d2/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-ceb6742366fbe817/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=limb_width_64

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-ceb6742366fbe817/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-ceb6742366fbe817/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-4d8661d83f222ced/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-4d8661d83f222ced/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/anyhow-4d8661d83f222ced/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-e8c2ebee99eca0ea/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_int128
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN
cargo:rustc-cfg=libc_cfg_target_vendor
cargo:rustc-cfg=libc_non_exhaustive
cargo:rustc-cfg=libc_long_array
cargo:rustc-cfg=libc_ptr_addr_of
cargo:rustc-cfg=libc_underscore_const_names
cargo:rustc-cfg=libc_const_extern_fn

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-e8c2ebee99eca0ea/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/libc-e8c2ebee99eca0ea/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-cc150cec7c9b3e6b/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-cc150cec7c9b3e6b/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/lock_api-cc150cec7c9b3e6b/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-ef19e0a46c35f68e/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_int128
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN
cargo:rustc-cfg=libc_cfg_target_vendor
cargo:rustc-cfg=libc_non_exhaustive
cargo:rustc-cfg=libc_long_array
cargo:rustc-cfg=libc_ptr_addr_of
cargo:rustc-cfg=libc_underscore_const_names
cargo:rustc-cfg=libc_const_extern_fn

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-ef19e0a46c35f68e/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/libc-ef19e0a46c35f68e/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-917e6197ca09e5cb/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-917e6197ca09e5cb/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/thiserror-917e6197ca09e5cb/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-5914dfc89e6c01c8/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rustc-cfg=wrap_proc_macro
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/proc-macro2-5914dfc89e6c01c8/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/proc-macro2-5914dfc89e6c01c8/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-3b8e996bca7d399a/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/parking_lot_core-3b8e996bca7d399a/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/parking_lot_core-3b8e996bca7d399a/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-a13e051751f7cf84/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS
cargo:rustc-cfg=portable_atomic_llvm_16
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-a13e051751f7cf84/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/portable-atomic-a13e051751f7cf84/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-f1abb45418c4c481/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-f1abb45418c4c481/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-f1abb45418c4c481/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-f946f47e7407f08d/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_int128
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN
cargo:rustc-cfg=libc_cfg_target_vendor
cargo:rustc-cfg=libc_non_exhaustive
cargo:rustc-cfg=libc_long_array
cargo:rustc-cfg=libc_ptr_addr_of
cargo:rustc-cfg=libc_underscore_const_names
cargo:rustc-cfg=libc_const_extern_fn

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-f946f47e7407f08d/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-f946f47e7407f08d/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-3b411f723ea4e60e/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-3b411f723ea4e60e/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/serde-3b411f723ea4e60e/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-7f2e74a336e475fe/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-7f2e74a336e475fe/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-7f2e74a336e475fe/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-2f1344100f9d4784/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-2f1344100f9d4784/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-2f1344100f9d4784/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-e22779b02053e86e/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-e22779b02053e86e/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-e22779b02053e86e/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-b86b090deaa07f31/output:

```````
cargo:rerun-if-changed=no_atomic.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-b86b090deaa07f31/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/crossbeam-utils-b86b090deaa07f31/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-14fca741fe22317c/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=limb_width_64

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-14fca741fe22317c/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-14fca741fe22317c/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-a4047371f319ed01/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/rayon-core-a4047371f319ed01/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/rayon-core-a4047371f319ed01/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-8927d37f21084d26/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS
cargo:rustc-cfg=portable_atomic_llvm_16
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-8927d37f21084d26/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/portable-atomic-8927d37f21084d26/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-04f30e6423e78c58/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=limb_width_64

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-04f30e6423e78c58/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/serde_json-04f30e6423e78c58/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-e32ce5bababc08dd/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/thiserror-e32ce5bababc08dd/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/thiserror-e32ce5bababc08dd/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-98cfd45e744f3825/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-98cfd45e744f3825/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/anyhow-98cfd45e744f3825/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-9602a6c20b2da187/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=limb_width_64

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde_json-9602a6c20b2da187/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/serde_json-9602a6c20b2da187/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-044addbb5279c67e/output:

```````
cargo:rustc-cfg=has_const_fn_trait_bound

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/lock_api-044addbb5279c67e/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/lock_api-044addbb5279c67e/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-fc2e0bf46424e06c/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_int128
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN
cargo:rustc-cfg=libc_cfg_target_vendor
cargo:rustc-cfg=libc_non_exhaustive
cargo:rustc-cfg=libc_long_array
cargo:rustc-cfg=libc_ptr_addr_of
cargo:rustc-cfg=libc_underscore_const_names
cargo:rustc-cfg=libc_const_extern_fn

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/libc-fc2e0bf46424e06c/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/libc-fc2e0bf46424e06c/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-7b81de0558c140d6/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-7b81de0558c140d6/root-output:

```````
/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-7b81de0558c140d6/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-a8f107b475504fab/output:

```````
cargo:rerun-if-changed=build.rs

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/serde-a8f107b475504fab/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/serde-a8f107b475504fab/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-97929674aaf1fcaa/output:

```````
cargo:rerun-if-changed=build.rs
cargo:rerun-if-changed=no_atomic.rs
cargo:rerun-if-changed=version.rs
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS
cargo:rustc-cfg=portable_atomic_llvm_16
cargo:rustc-cfg=portable_atomic_target_feature="lse2"
cargo:rustc-cfg=portable_atomic_ll_sc_rmw

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/portable-atomic-97929674aaf1fcaa/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/portable-atomic-97929674aaf1fcaa/out
```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-1d35aa32df47f0fd/output:

```````
cargo:rerun-if-changed=build/probe.rs
cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP
cargo:rustc-cfg=std_backtrace

```````

/Users/harshpratapsingh/Desktop/taiko/target/debug/build/anyhow-1d35aa32df47f0fd/root-output:

```````
/Users/harshpratapsingh/Desktop/cool/taiko/target/debug/build/anyhow-1d35aa32df47f0fd/out
```````


Please perform a thorough code review of the above codebase, focusing on:
1. Code quality and best practices
2. Performance considerations
3. Error handling
4. Potential bugs or issues
5. Architecture and design patterns
6. Suggestions for improvements

Provide specific examples and recommendations where applicable.